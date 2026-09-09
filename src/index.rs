//! Deterministic keyword/alias search index over the embedded OOP IR.
//!
//! No embeddings, no ML, no network calls: this builds a weighted inverted
//! index once (lazily, on first use) from a handful of textual fields per
//! member, and scores queries by `field_weight × IDF` token overlap. Ties are
//! always broken by member/class id so results are stable and reproducible.
//!
//! Two things are genuinely different from `4d-language-classic`:
//!
//! 1. **camelCase splitting.** `Collection.orderBy` must be reachable from the
//!    query "order by", and "entity selection" must reach `EntitySelection`.
//!    The vendored corpus statistics are computed under exactly this
//!    tokenization, so the IDF table and the index agree by construction.
//! 2. **Class-flooding control.** `WebServer` has 41 members, `Collection` 47,
//!    `EntitySelection` 34. A naive query for "web server" returns 41
//!    near-identical rows and buries everything else, so a class is capped at
//!    3 member rows, and additionally emits a single *class card* when the
//!    query names it or when enough of its members match. See [`search`].

use crate::data;
use crate::ir::{self, ClassIr, DocExample, MemberIr, Relationship, SynthBlock};
use serde_json::Value;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::OnceLock;

// ---------------------------------------------------------------------------
// Ranking constants
// ---------------------------------------------------------------------------

/// Field weights: how much a query-token match in this field contributes.
///
/// Class name and member name are **both** strong signals in OOP — this is the
/// main departure from classic, where the command id carried nearly everything.
/// A user who knows "orderBy" and a user who knows "Collection" should both
/// land somewhere useful.
pub const WEIGHT_MEMBER_NAME: f32 = 3.0;
pub const WEIGHT_CLASS_ID: f32 = 2.5;
pub const WEIGHT_SUMMARY: f32 = 1.0;
pub const WEIGHT_PARAM_NAMES: f32 = 0.6;
/// The OOP IR has no `constraints` array (classic's field). Its analogue here
/// is the small amount of remaining behavioural prose: `versionBehaviorChanges`
/// entries, `errorModel`, and a dynamic member's explanatory `note`.
pub const WEIGHT_CONSTRAINTS: f32 = 0.4;

/// A single class may contribute at most this many member rows to a result
/// set before the rest are dropped.
pub const CLASS_MEMBER_CAP: usize = 3;

/// When this many members of one class appear in the candidate pool, the class
/// also emits a class card so the query has somewhere to land that isn't a
/// wall of near-identical member rows.
pub const CLASS_COLLAPSE_THRESHOLD: usize = 5;

/// Size of the candidate pool that collapse and cap decisions are made over.
///
/// Fixed, and deliberately **not** a function of `limit`: if the pool grew with
/// the limit, `--limit 5` and `--limit 10` could disagree about whether a class
/// floods the results, and the first five rows of the two answers would differ.
/// A constant pool makes the ranking a pure function of the query, with
/// `limit` doing nothing but truncate.
pub const CANDIDATE_POOL: usize = 60;

/// No stopword list, deliberately. `ln(N / df)` is exactly 0.0 for a token
/// that appears in every member ("the", df 502) and near 0 for the rest of the
/// English function words, so IDF already does the job a hand-written list
/// would only approximate. See README, "How matching works".
fn idf(token: &str) -> f32 {
    data::idf(token).unwrap_or(0.0)
}

// ---------------------------------------------------------------------------
// Records
// ---------------------------------------------------------------------------

pub struct MemberRecord {
    pub ir: MemberIr,
    /// Verbatim text of this member's compiler-verified synthetic example.
    /// Every one of the 502 members has one.
    pub synth_raw: Option<&'static str>,
    /// Per-block line ranges into `synth_raw`, from the manifest.
    pub synth_blocks: Vec<SynthBlock>,
    /// Documentation examples attributed to this member, already ordered by
    /// the precedence rule in `model.rs` (compiler-verified first).
    pub doc_examples: Vec<DocExample>,
}

impl MemberRecord {
    pub fn class_id(&self) -> &str {
        &self.ir.receiver.class_id
    }
}

pub struct ClassRecord {
    pub ir: ClassIr,
    /// Ids of members declared on this class, sorted.
    pub own_members: Vec<String>,
    /// Ids of members reachable on this class but declared on an ancestor,
    /// sorted. Inherited members are modelled once on the declaring class
    /// (`.exists` lives on `Document`, not `File`), so this is what makes a
    /// lookup for `File.exists` resolve.
    pub inherited_members: Vec<String>,
}

pub struct Index {
    pub members: BTreeMap<String, MemberRecord>,
    pub classes: BTreeMap<String, ClassRecord>,
    pub enums: HashMap<String, Value>,
    pub relationships: Vec<Relationship>,
    /// token -> sorted (by member id) postings of (member id, weight × IDF).
    inverted: HashMap<String, Vec<(String, f32)>>,
    aliases: HashMap<String, Vec<String>>,
    /// Normalized name (`"4dfile"`, `"file"`, `"entityselection"`) -> class id.
    class_name_lookup: HashMap<String, String>,
    /// Space-joined camel-split class name (`"entity selection"`) -> class id.
    class_phrase_lookup: HashMap<String, String>,
    /// Normalized `<class>.<member>` (including inherited and `4D.`-prefixed
    /// spellings) -> declaring member id.
    member_name_lookup: HashMap<String, String>,
}

static INDEX: OnceLock<Index> = OnceLock::new();

pub fn get() -> &'static Index {
    INDEX.get_or_init(build_index)
}

// ---------------------------------------------------------------------------
// Tokenization
// ---------------------------------------------------------------------------

/// Splits on non-alphanumeric runs, **then splits camelCase humps**, and
/// lowercases. `".orderBy"` -> `["order", "by"]`; `"EntitySelection"` ->
/// `["entity", "selection"]`; `"4D.HTTPRequest.new"` -> `["4d", "http",
/// "request", "new"]`.
///
/// Single-character tokens are dropped (`"4D"`'s `"d"` when split from a
/// digit run, the `*` noise, etc.). This is exactly the tokenization
/// `oop_corpus_stats.json` documents, so the vendored IDF table lines up with
/// what we index.
pub fn tokenize(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for run in text.split(|c: char| !c.is_alphanumeric()) {
        if run.is_empty() {
            continue;
        }
        for hump in split_camel_humps(run) {
            if hump.chars().count() >= 2 {
                out.push(hump.to_lowercase());
            }
        }
    }
    out
}

/// Splits a single alphanumeric run at camelCase boundaries.
///
/// Boundaries are lower/digit -> upper (`orderBy` -> `order|By`) and the tail
/// of an acronym run followed by a capitalised word (`HTTPRequest` ->
/// `HTTP|Request`). An all-lowercase or all-uppercase run is returned intact,
/// so `"searchmails"` stays one token and `"HTTP"` is not shredded.
fn split_camel_humps(run: &str) -> Vec<String> {
    let chars: Vec<char> = run.chars().collect();
    let mut parts = Vec::new();
    let mut start = 0usize;
    for i in 1..chars.len() {
        let prev = chars[i - 1];
        let cur = chars[i];
        let next = chars.get(i + 1).copied();
        let lower_to_upper = !prev.is_uppercase() && cur.is_uppercase();
        let acronym_end =
            prev.is_uppercase() && cur.is_uppercase() && next.is_some_and(|n| n.is_lowercase());
        if lower_to_upper || acronym_end {
            parts.push(chars[start..i].iter().collect::<String>());
            start = i;
        }
    }
    parts.push(chars[start..].iter().collect::<String>());
    parts.retain(|p| !p.is_empty());
    parts
}

/// Lowercased, non-alphanumerics stripped: `"4D.File"` -> `"4dfile"`. Used for
/// exact id/name resolution, never for scoring.
fn normalize_name(text: &str) -> String {
    text.chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

/// Tokenizes the query, then expands each token via the alias table.
fn expand_query_tokens(query: &str, aliases: &HashMap<String, Vec<String>>) -> HashSet<String> {
    let mut expanded = HashSet::new();
    for tok in tokenize(query) {
        if let Some(extra) = aliases.get(&tok) {
            for e in extra {
                expanded.insert(e.clone());
            }
        }
        expanded.insert(tok);
    }
    expanded
}

fn parse_aliases(json: &str) -> HashMap<String, Vec<String>> {
    let raw: HashMap<String, Value> =
        serde_json::from_str(json).expect("embedded data/aliases.json failed to parse");
    raw.into_iter()
        .filter(|(k, _)| !k.starts_with('_'))
        .filter_map(|(k, v)| {
            let list = v
                .as_array()?
                .iter()
                .filter_map(|x| x.as_str().map(str::to_string))
                .collect();
            Some((k, list))
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Index construction
// ---------------------------------------------------------------------------

fn build_index() -> Index {
    let root = ir::parse_embedded();
    let doc_root = ir::parse_embedded_doc_examples();
    let manifest = ir::parse_embedded_synth_manifest();
    let aliases = parse_aliases(data::ALIASES_JSON);
    let synth_text: HashMap<&'static str, &'static str> =
        data::SYNTH_EXAMPLES.iter().copied().collect();

    // member id -> manifest blocks. Joined on `member_id`, never on filename.
    let mut synth_blocks: HashMap<String, Vec<SynthBlock>> = HashMap::new();
    for entry in manifest.files.into_values() {
        synth_blocks.insert(entry.member_id, entry.blocks);
    }

    // member id -> doc examples, in precedence order. `not 4D source` blocks
    // are dropped here so they can never reach a caller as code.
    let mut doc_by_member: HashMap<String, Vec<DocExample>> = HashMap::new();
    for ex in doc_root.examples {
        if ex.is_not_4d_source() {
            continue;
        }
        doc_by_member.entry(ex.member_id.clone()).or_default().push(ex);
    }
    for list in doc_by_member.values_mut() {
        list.sort_by(|a, b| {
            doc_tier(a)
                .cmp(&doc_tier(b))
                .then_with(|| a.example_id.cmp(&b.example_id))
        });
        // The harvester attributes an inherited block both to the declaring
        // class and to the subclass whose page it appeared on, so the same
        // code arrives twice under different `exampleId`s. Keep the first
        // (best-tier) copy of each distinct snippet.
        let mut seen: HashSet<String> = HashSet::new();
        list.retain(|e| seen.insert(e.code.clone()));
    }

    // token -> member id -> accumulated base weight (pre-IDF).
    let mut acc: HashMap<String, HashMap<String, f32>> = HashMap::new();
    let mut members: BTreeMap<String, MemberRecord> = BTreeMap::new();

    for m in root.commands {
        index_member(&m, &mut acc);
        let id = m.id.clone();
        members.insert(
            id.clone(),
            MemberRecord {
                synth_raw: synth_text.get(id.as_str()).copied(),
                synth_blocks: synth_blocks.remove(&id).unwrap_or_default(),
                doc_examples: doc_by_member.remove(&id).unwrap_or_default(),
                ir: m,
            },
        );
    }

    // Apply the precomputed `ln(N / df)` multiplier. Rarity across the corpus
    // matters as much as raw field weight: "imap" should outrank "function".
    let mut inverted: HashMap<String, Vec<(String, f32)>> = HashMap::new();
    for (token, by_member) in acc {
        let token_idf = idf(&token);
        if token_idf <= 0.0 {
            // df == N: the token carries no discriminating information.
            continue;
        }
        let mut postings: Vec<(String, f32)> = by_member
            .into_iter()
            .map(|(id, base)| (id, base * token_idf))
            .collect();
        postings.sort_by(|a, b| a.0.cmp(&b.0));
        inverted.insert(token, postings);
    }

    let classes = build_class_records(&root.classes);

    let mut class_name_lookup = HashMap::new();
    let mut class_phrase_lookup = HashMap::new();
    for (id, rec) in &classes {
        for name in [
            id.as_str(),
            rec.ir.type_name.as_str(),
            rec.ir.display_name.as_str(),
        ] {
            class_name_lookup.insert(normalize_name(name), id.clone());
            class_phrase_lookup.insert(tokenize(name).join(" "), id.clone());
        }
    }

    let member_name_lookup = build_member_name_lookup(&classes, &members);

    Index {
        members,
        classes,
        enums: root.enums,
        relationships: root.relationships,
        inverted,
        aliases,
        class_name_lookup,
        class_phrase_lookup,
        member_name_lookup,
    }
}

/// Example-precedence tier for a documentation example. Lower is better.
/// 0 = compiler-verified as published, 1 = compiler-verified only after being
/// mechanically hosted in a class file, 2 = not compiler-verified.
pub fn doc_tier(ex: &DocExample) -> u8 {
    if ex.is_compiler_verified() {
        if ex.is_wrapped() {
            1
        } else {
            0
        }
    } else {
        2
    }
}

fn index_member(m: &MemberIr, acc: &mut HashMap<String, HashMap<String, f32>>) {
    // Dedup tokens within one field so a word repeated several times in a long
    // summary doesn't out-accumulate a word that appears once in the member
    // name.
    let mut add = |text: &str, weight: f32| {
        let unique: HashSet<String> = tokenize(text).into_iter().collect();
        for tok in unique {
            *acc.entry(tok).or_default().entry(m.id.clone()).or_insert(0.0) += weight;
        }
    };

    add(&m.member_name, WEIGHT_MEMBER_NAME);
    add(&m.receiver.class_id, WEIGHT_CLASS_ID);
    add(&m.summary, WEIGHT_SUMMARY);

    let mut param_names = Vec::new();
    for overload in &m.overloads {
        if let Some(params) = overload.get("params").and_then(Value::as_array) {
            for p in params {
                if let Some(name) = p.get("name").and_then(Value::as_str) {
                    param_names.push(name.to_string());
                }
            }
        }
    }
    for name in &param_names {
        add(name, WEIGHT_PARAM_NAMES);
    }

    for change in &m.version_behavior_changes {
        if let Some(s) = change.get("change").and_then(Value::as_str) {
            add(s, WEIGHT_CONSTRAINTS);
        }
    }
    if let Some(em) = &m.error_model {
        if let Some(s) = em.get("style").and_then(Value::as_str) {
            add(s, WEIGHT_CONSTRAINTS);
        }
        if let Some(s) = em.get("catchableWith").and_then(Value::as_str) {
            add(s, WEIGHT_CONSTRAINTS);
        }
    }
    if let Some(dm) = &m.dynamic_member {
        if let Some(s) = dm.get("note").and_then(Value::as_str) {
            add(s, WEIGHT_CONSTRAINTS);
        }
    }
}

fn build_class_records(classes: &HashMap<String, ClassIr>) -> BTreeMap<String, ClassRecord> {
    let mut out = BTreeMap::new();
    for (id, cl) in classes {
        let mut own = cl.members.clone();
        own.sort();

        // Walk the superclass chain (at most 2 deep in this corpus, but guard
        // against a cycle anyway) collecting members declared upstream.
        let mut inherited = Vec::new();
        let mut seen: HashSet<&str> = HashSet::from([id.as_str()]);
        let mut cursor = cl.superclass.as_deref();
        while let Some(sup_id) = cursor {
            if !seen.insert(sup_id) {
                break;
            }
            let Some(sup) = classes.get(sup_id) else { break };
            inherited.extend(sup.members.iter().cloned());
            cursor = sup.superclass.as_deref();
        }
        inherited.sort();
        inherited.dedup();

        out.insert(
            id.clone(),
            ClassRecord {
                ir: cl.clone(),
                own_members: own,
                inherited_members: inherited,
            },
        );
    }
    out
}

/// Builds the `<class>.<member>` resolution table, including every spelling a
/// caller might reasonably type:
///
/// * the declared id itself (`Document.exists`),
/// * the id under a subclass that inherits it (`File.exists`),
/// * the `4D.`-qualified type name (`4D.File.exists`, `4D.Collection.orderBy`),
/// * the bare member name where it is unambiguous corpus-wide (`.orderBy`).
///
/// Declared spellings always win over inherited ones, so an override could
/// never be shadowed by its own base class.
fn build_member_name_lookup(
    classes: &BTreeMap<String, ClassRecord>,
    members: &BTreeMap<String, MemberRecord>,
) -> HashMap<String, String> {
    let mut map: HashMap<String, String> = HashMap::new();
    let insert = |key: String, id: &str, authoritative: bool, map: &mut HashMap<String, String>| {
        if authoritative {
            map.insert(key, id.to_string());
        } else {
            map.entry(key).or_insert_with(|| id.to_string());
        }
    };

    // Pass 1: authoritative spellings from the member's own id.
    for id in members.keys() {
        insert(normalize_name(id), id, true, &mut map);
    }

    // Pass 2: class-qualified spellings, declared before inherited.
    for (class_id, rec) in classes {
        let type_name = rec.ir.type_name.as_str();
        for (member_id, declared) in rec
            .own_members
            .iter()
            .map(|m| (m, true))
            .chain(rec.inherited_members.iter().map(|m| (m, false)))
        {
            let Some(m) = members.get(member_id) else {
                continue;
            };
            let short = m.ir.member_name.trim_start_matches('.');
            for prefix in [class_id.as_str(), type_name] {
                insert(
                    normalize_name(&format!("{prefix}.{short}")),
                    member_id,
                    declared,
                    &mut map,
                );
            }
        }
    }

    // Pass 3: bare member names, only where unambiguous corpus-wide. Never
    // authoritative — a bare name must not shadow a qualified one.
    let mut bare_counts: HashMap<String, HashSet<&str>> = HashMap::new();
    for (id, m) in members {
        bare_counts
            .entry(normalize_name(&m.ir.member_name))
            .or_default()
            .insert(id.as_str());
    }
    for (bare, ids) in bare_counts {
        if ids.len() == 1 {
            let only = ids.into_iter().next().expect("checked len == 1");
            insert(bare, only, false, &mut map);
        }
    }

    map
}

// ---------------------------------------------------------------------------
// Exact resolution
// ---------------------------------------------------------------------------

impl Index {
    /// Resolves a class by id (`File`), 4D type name (`4D.File`), display name,
    /// or any case/punctuation variation of those.
    pub fn resolve_class(&self, name: &str) -> Option<&ClassRecord> {
        let key = normalize_name(name);
        let id = self.class_name_lookup.get(&key)?;
        self.classes.get(id)
    }

    /// Resolves a member by id, by an inherited spelling (`File.exists` ->
    /// `Document.exists`), by 4D type name (`4D.Collection.orderBy`), or by an
    /// unambiguous bare member name (`.orderBy`).
    pub fn resolve_member(&self, name: &str) -> Option<&MemberRecord> {
        let key = normalize_name(name);
        let id = self.member_name_lookup.get(&key)?;
        self.members.get(id)
    }

    /// True when `member_id` is reachable on `class_id` but declared upstream.
    pub fn is_inherited_on(&self, class_id: &str, member_id: &str) -> bool {
        self.classes
            .get(class_id)
            .is_some_and(|c| c.inherited_members.iter().any(|m| m == member_id))
    }

    /// Every member id reachable on this class: declared first, then inherited.
    pub fn all_members_of(&self, class_id: &str) -> Vec<&str> {
        let Some(rec) = self.classes.get(class_id) else {
            return Vec::new();
        };
        rec.own_members
            .iter()
            .chain(rec.inherited_members.iter())
            .map(String::as_str)
            .collect()
    }

    /// Members whose `returns_instance_of` edge points at this class, plus the
    /// class's own `obtained_via` edges — i.e. "what produces one of these".
    pub fn producers_of(&self, class_id: &str) -> Vec<&Relationship> {
        self.relationships
            .iter()
            .filter(|r| match r.kind.as_str() {
                "returns_instance_of" => r.to.class_id.as_deref() == Some(class_id),
                "obtained_via" => r.from.class_id.as_deref() == Some(class_id),
                _ => false,
            })
            .collect()
    }
}

// ---------------------------------------------------------------------------
// Search
// ---------------------------------------------------------------------------

/// One row of a ranked result set: either a single member, or a class card
/// standing in for a class too many of whose members matched.
pub enum Hit<'a> {
    Member(&'a MemberRecord, f32),
    Class(&'a ClassRecord, f32, ClassCardReason),
}

/// Why a class card was emitted instead of member rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassCardReason {
    /// The query names the class exactly (`"collection"`, `"4D.File"`,
    /// `"entity selection"`).
    ExactNameMatch,
    /// `CLASS_COLLAPSE_THRESHOLD`+ of the class's members matched, so listing
    /// them individually would bury every other class.
    ManyMembersMatched,
}

impl Hit<'_> {
    pub fn score(&self) -> f32 {
        match self {
            Hit::Member(_, s) | Hit::Class(_, s, _) => *s,
        }
    }

    pub fn id(&self) -> &str {
        match self {
            Hit::Member(m, _) => &m.ir.id,
            Hit::Class(c, _, _) => &c.ir.id,
        }
    }
}

/// Ranked lookup for a free-text query.
///
/// Pipeline: tokenize -> alias-expand -> sum `field_weight × IDF` per member ->
/// class-flooding control -> sort by `(score desc, id asc)` -> truncate.
pub fn search(query: &str, limit: usize) -> Vec<Hit<'static>> {
    let idx = get();
    let expanded = expand_query_tokens(query, &idx.aliases);

    let mut scores: HashMap<&str, f32> = HashMap::new();
    for tok in &expanded {
        if let Some(postings) = idx.inverted.get(tok) {
            for (id, weight) in postings {
                *scores.entry(id.as_str()).or_insert(0.0) += weight;
            }
        }
    }

    let mut ranked: Vec<(&str, f32)> = scores.into_iter().collect();
    ranked.sort_by(|a, b| {
        b.1.partial_cmp(&a.1)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.0.cmp(b.0))
    });

    // The candidate pool is deliberately wider than `limit`: collapse and cap
    // decisions must be made before truncation, or a class that floods the
    // top-N would never be detected. Its size is a constant, so those
    // decisions -- and therefore the first N rows -- do not shift when the
    // caller changes `--limit`.
    let pool_size = CANDIDATE_POOL.min(ranked.len());
    let pool = &ranked[..pool_size];

    let mut per_class: HashMap<&str, usize> = HashMap::new();
    for (id, _) in pool {
        if let Some(m) = idx.members.get(*id) {
            *per_class.entry(m.class_id()).or_insert(0) += 1;
        }
    }

    let named_class = exact_class_match(idx, query, &expanded);

    // Which classes collapse to a card, and why.
    let mut collapsed: HashMap<&str, ClassCardReason> = HashMap::new();
    for (class_id, count) in &per_class {
        if *count >= CLASS_COLLAPSE_THRESHOLD {
            collapsed.insert(class_id, ClassCardReason::ManyMembersMatched);
        }
    }
    if let Some(cid) = named_class {
        collapsed.insert(cid, ClassCardReason::ExactNameMatch);
    }

    let mut hits: Vec<Hit<'static>> = Vec::new();
    let mut emitted_class: HashSet<&str> = HashSet::new();
    let mut shown_per_class: HashMap<&str, usize> = HashMap::new();

    // A class the query names outright leads the results: "collection",
    // "4D.File" and "entity selection" are asking about the class, so its card
    // -- which is what carries `instantiation`, the thing agents most often get
    // wrong -- must not sit below whichever member happened to score highest.
    let top_score = pool.first().map(|(_, s)| *s).unwrap_or(0.0);
    if let Some(cid) = named_class {
        if let Some(c) = idx.classes.get(cid) {
            emitted_class.insert(cid);
            hits.push(Hit::Class(c, top_score, ClassCardReason::ExactNameMatch));
        }
    }

    for (id, score) in pool {
        let Some(m) = idx.members.get(*id) else { continue };
        let class_id = m.class_id();
        if let Some(reason) = collapsed.get(class_id).copied() {
            if emitted_class.insert(class_id) {
                if let Some(c) = idx.classes.get(class_id) {
                    // The card inherits the best score its members achieved, so
                    // it ranks exactly where the class's strongest member would.
                    hits.push(Hit::Class(c, *score, reason));
                }
            }
        }
        // The per-class cap applies to every class, collapsed or not. A
        // collapsed class therefore contributes one card plus at most
        // `CLASS_MEMBER_CAP` member rows -- never the 41 rows that made the
        // rule necessary, but never zero either: collapsing `Collection` down
        // to only its card would hide `Collection.orderBy` from the query
        // "order a collection", which is the answer the caller wanted.
        let shown = shown_per_class.entry(class_id).or_insert(0);
        if *shown >= CLASS_MEMBER_CAP {
            continue;
        }
        *shown += 1;
        hits.push(Hit::Member(m, *score));
    }

    // A class named outright by the query must appear even if none of its
    // members scored (e.g. an abstract class whose members all live elsewhere).
    if let Some(cid) = named_class {
        if !emitted_class.contains(cid) {
            if let Some(c) = idx.classes.get(cid) {
                hits.push(Hit::Class(c, top_score, ClassCardReason::ExactNameMatch));
            }
        }
    }

    hits.sort_by(|a, b| {
        b.score()
            .partial_cmp(&a.score())
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| {
                // At equal score: the class the query named outright first,
                // then other class cards (the more general answer, and what
                // carries `instantiation`), then member rows.
                rank_key(a).cmp(&rank_key(b))
            })
            .then_with(|| a.id().cmp(b.id()))
    });
    hits.truncate(limit);
    hits
}

/// Tiebreak class for a hit at equal score. Lower sorts first.
fn rank_key(hit: &Hit<'_>) -> u8 {
    match hit {
        Hit::Class(_, _, ClassCardReason::ExactNameMatch) => 0,
        Hit::Class(_, _, ClassCardReason::ManyMembersMatched) => 1,
        Hit::Member(..) => 2,
    }
}

/// Does the query name a class outright? Matches the whole query against class
/// ids/type names (`"4D.File"`, `"collection"`) and against the space-joined
/// camel-split form (`"entity selection"` -> `EntitySelection`), and also
/// accepts a single query token that is itself a class name.
fn exact_class_match(
    idx: &'static Index,
    query: &str,
    expanded: &HashSet<String>,
) -> Option<&'static str> {
    if let Some(id) = idx.class_name_lookup.get(&normalize_name(query)) {
        return Some(id.as_str());
    }
    let phrase = tokenize(query).join(" ");
    if let Some(id) = idx.class_phrase_lookup.get(&phrase) {
        return Some(id.as_str());
    }
    if expanded.len() == 1 {
        let only = expanded.iter().next().expect("checked len == 1");
        if let Some(id) = idx.class_name_lookup.get(only) {
            return Some(id.as_str());
        }
    }
    None
}
