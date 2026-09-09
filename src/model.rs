//! Response shapes shared by the CLI (`--json`) and the HTTP server.
//!
//! Two result kinds are serialized under a `resultType` discriminator:
//! `"member"` (one function/property/constructor) and `"class"` (a whole class
//! collapsed into one card — see `index::search`).

use crate::index::{self, ClassCardReason, ClassRecord, Hit, MemberRecord};
use crate::ir::{DocExample, SynthBlock};
use serde::Serialize;
use serde_json::Value;

// ---------------------------------------------------------------------------
// Examples
// ---------------------------------------------------------------------------

const SYNTH_PROVENANCE: &str = "compiler-verified synthetic example (tool4d check-syntax via the 4d-static-docs pipeline's OOP stage O6); raw text, not idiomatic hand-written code";

/// Classic's hard-won lesson, carried over: without an explicit machine-readable
/// warning, agents copied the synthesizer's placeholder tokens into real code
/// verbatim. The OOP synthesizer's placeholder vocabulary differs from
/// classic's, so the token list is OOP-specific.
const PLACEHOLDER_NOTE: &str = "Tokens such as $result1/$blobScal2/$options3, [SynthTable], SynthRelated, cs.SynthOOPHandler and SynthOOPCallback in `raw` are auto-generated placeholder variable names, table/dataclass references and stub class names from the check project -- substitute your own receiver expression, variable names, and table/field/class references when adapting this example; do not copy them verbatim.";

const UNVERIFIED_DOC_WARNING: &str = "This documentation example is NOT compiler-verified: it failed `tool4d check-syntax`, usually because it references a project table, ORDA dataclass or user class that only exists in the doc page's imaginary database, or contains a literal `...` elision. It is supplied only alongside the compiler-verified synthetic example above, never instead of it -- read it for intent, take syntax from the verified example.";

const WRAPPED_NOTE: &str = "This documentation example is a class-body fragment: it was compiler-verified only after being mechanically hosted inside a 4D class file. Reproduce that hosting (place it inside a class) when adapting it.";

/// Where an example's text came from.
#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExampleSource {
    /// Hand-written, harvested from the 4D documentation.
    Documentation,
    /// Machine-synthesized to exercise every overload/variant, then compiled.
    Synthetic,
}

#[derive(Debug, Serialize)]
pub struct ExampleBlock {
    pub source: ExampleSource,
    pub provenance: String,
    #[serde(rename = "compilerVerified")]
    pub compiler_verified: bool,
    pub raw: String,
    #[serde(rename = "docPage", skip_serializing_if = "Option::is_none")]
    pub doc_page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// True when this doc example was attributed to the member via the class's
    /// inheritance chain rather than appearing on the member's own doc page.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub inherited: bool,
    /// Per-overload/variant line ranges into `raw` (synthetic examples only).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub blocks: Vec<SynthBlock>,
    /// Explicit, machine-readable warning that `raw`'s literal-looking tokens
    /// are auto-generated placeholders, not required syntax. Present on
    /// synthetic examples only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder_note: Option<&'static str>,
    /// Present when this block is *not* compiler-verified, or was verified
    /// only after mechanical wrapping.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<&'static str>,
}

/// The `example` field of a result.
///
/// `primary` follows the precedence rule:
///   1. documentation example, compiler-verified
///   2. documentation example, wrapped and compiler-verified
///   3. compiler-verified synthetic example (all 502 members have one)
///   4. documentation example, not compiler-verified — **never primary**
///
/// A not-compiler-verified doc example only ever appears in `alternates`,
/// alongside the synthetic one, carrying an explicit `warning`. Doc blocks
/// whose provenance is "not 4D source" (JSON, query grammar, console output)
/// are dropped at index-build time and never appear here at all.
#[derive(Debug, Serialize)]
pub struct ExampleInfo {
    pub available: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<ExampleBlock>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub alternates: Vec<ExampleBlock>,
}

fn synth_block(record: &MemberRecord) -> Option<ExampleBlock> {
    record.synth_raw.map(|raw| ExampleBlock {
        source: ExampleSource::Synthetic,
        provenance: SYNTH_PROVENANCE.to_string(),
        compiler_verified: true,
        raw: raw.to_string(),
        doc_page: None,
        title: None,
        caption: None,
        inherited: false,
        blocks: record.synth_blocks.clone(),
        placeholder_note: Some(PLACEHOLDER_NOTE),
        warning: None,
    })
}

fn doc_block(ex: &DocExample) -> ExampleBlock {
    let verified = ex.is_compiler_verified();
    let wrapped = ex.is_wrapped();
    ExampleBlock {
        source: ExampleSource::Documentation,
        provenance: if verified && wrapped {
            "documentation example, wrapped and compiler-verified".to_string()
        } else {
            ex.provenance.clone()
        },
        compiler_verified: verified,
        raw: ex.code.clone(),
        doc_page: ex.doc_page.clone(),
        title: ex.title.clone(),
        caption: ex.caption.clone(),
        inherited: ex.inherited,
        blocks: Vec::new(),
        placeholder_note: None,
        warning: if !verified {
            Some(UNVERIFIED_DOC_WARNING)
        } else if wrapped {
            Some(WRAPPED_NOTE)
        } else {
            None
        },
    }
}

pub fn build_example(record: &MemberRecord) -> ExampleInfo {
    let verified_doc = record
        .doc_examples
        .iter()
        .find(|e| e.is_compiler_verified());

    match verified_doc {
        // Tiers 1-2: a compiler-verified documentation example leads. The
        // synthetic one still follows as an alternate, because it is the only
        // thing that exercises *every* overload and variant.
        Some(best) => {
            let mut alternates: Vec<ExampleBlock> = record
                .doc_examples
                .iter()
                .filter(|e| e.example_id != best.example_id && e.is_compiler_verified())
                .map(doc_block)
                .collect();
            alternates.extend(synth_block(record));
            alternates.extend(
                record
                    .doc_examples
                    .iter()
                    .filter(|e| !e.is_compiler_verified())
                    .map(doc_block),
            );
            ExampleInfo {
                available: true,
                primary: Some(doc_block(best)),
                alternates,
            }
        }
        // Tier 3: the synthetic example leads. Any unverified documentation
        // examples follow it, clearly labelled -- never in its place.
        None => {
            let primary = synth_block(record);
            let alternates: Vec<ExampleBlock> = record.doc_examples.iter().map(doc_block).collect();
            ExampleInfo {
                available: primary.is_some(),
                primary,
                alternates,
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Member results
// ---------------------------------------------------------------------------

/// The verbatim source line(s) for a member, byte-identical to the 4D docs.
/// Prefer rendering this over reconstructing a signature: for a multi-variant
/// property it is the only place all variants appear in the docs' own words.
/// `None` for the 7 dynamic pseudo-members, which have no fixed source line.
fn raw_syntax(record: &MemberRecord) -> Vec<String> {
    if let Some(acc) = &record.ir.accessor {
        if let Some(s) = acc.get("rawSyntax").and_then(Value::as_str) {
            return vec![s.to_string()];
        }
    }
    record
        .ir
        .overloads
        .iter()
        .filter_map(|o| o.get("rawSyntax").and_then(Value::as_str))
        .map(str::to_string)
        .collect()
}

/// The declared type(s) of a property. **Always a list**: 9 properties are
/// multi-variant (`Email.bcc` is `Text | Object | Collection`) and rendering
/// only the first is a bug this project has already been bitten by once.
fn accessor_types(record: &MemberRecord) -> Vec<Value> {
    let Some(acc) = &record.ir.accessor else {
        return Vec::new();
    };
    match acc.get("type") {
        Some(Value::Array(items)) => items.clone(),
        Some(other) => vec![other.clone()],
        None => Vec::new(),
    }
}

#[derive(Debug, Serialize)]
pub struct MemberResult {
    #[serde(rename = "resultType")]
    pub result_type: &'static str,
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub kind: String,
    #[serde(rename = "memberName")]
    pub member_name: String,
    #[serde(rename = "classId")]
    pub class_id: String,
    /// The 4D type name of the receiver's class, e.g. `4D.File`.
    #[serde(rename = "classTypeName")]
    pub class_type_name: String,
    /// `instance` or `class` (a `class`-kind receiver is called on the class
    /// itself, e.g. `4D.File.new(...)`).
    #[serde(rename = "receiverKind")]
    pub receiver_kind: String,
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
    /// Verbatim source line(s) from the 4D docs. Empty only for the 7 dynamic
    /// pseudo-members.
    #[serde(rename = "rawSyntax")]
    pub raw_syntax: Vec<String>,
    /// Raw IR overload objects, echoed as-is. Empty for properties.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub overloads: Vec<Value>,
    /// Raw IR accessor object, echoed as-is. Absent for callables.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessor: Option<Value>,
    /// Every declared type of a property, flattened to a list so multi-variant
    /// properties can never be truncated to their first variant.
    #[serde(rename = "accessorTypes", skip_serializing_if = "Vec::is_empty")]
    pub accessor_types: Vec<Value>,
    /// True when `accessorTypes` holds more than one alternative.
    #[serde(rename = "multiVariant", skip_serializing_if = "std::ops::Not::not")]
    pub multi_variant: bool,
    /// Set for the 7 dynamic pseudo-members (`.attributeName` and friends):
    /// the member name is a *pattern*, not a literal name.
    #[serde(rename = "dynamicMember", skip_serializing_if = "Option::is_none")]
    pub dynamic_member: Option<Value>,
    #[serde(rename = "returnsShape", skip_serializing_if = "Option::is_none")]
    pub returns_shape: Option<Value>,
    #[serde(rename = "errorModel", skip_serializing_if = "Option::is_none")]
    pub error_model: Option<Value>,
    #[serde(rename = "sinceVersion", skip_serializing_if = "Option::is_none")]
    pub since_version: Option<String>,
    #[serde(
        rename = "versionBehaviorChanges",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub version_behavior_changes: Vec<Value>,
    #[serde(rename = "docPage", skip_serializing_if = "Option::is_none")]
    pub doc_page: Option<String>,
    /// Only present when the caller asked for this member under a subclass
    /// that inherits it (`File.exists` -> declared on `Document`).
    #[serde(rename = "inheritedFrom", skip_serializing_if = "Option::is_none")]
    pub inherited_from: Option<String>,
    #[serde(rename = "requestedAs", skip_serializing_if = "Option::is_none")]
    pub requested_as: Option<String>,
    pub example: ExampleInfo,
}

/// Whether a class's IR `typeName` genuinely belongs to it, i.e. is `<id>` or
/// `4D.<id>`.
///
/// This is an invariant the regression suite enforces over the whole corpus
/// rather than a case the renderer handles. It exists because it was once
/// violated: `Document.typeName` was recorded upstream as `4D.File` and
/// `Directory.typeName` as `4D.Folder` — the type of one of their *two*
/// concrete subclasses, which was arbitrary (why `File` over `ZipFile`?) and
/// wrong. `tool4d` accepts `var $x : 4D.Document` and rejects `4D.DocumentX`,
/// so those are real class-store types and are the correct declaration type
/// for a value that may be either subclass. Fixed upstream in
/// `4d-static-docs` `fb0a69ac`.
///
/// Deliberately a test-enforced invariant, not a fallback branch: if the data
/// regresses we want a failing test naming the class, not a renderer quietly
/// papering over it.
pub fn owns_its_type_name(class_id: &str, type_name: &str) -> bool {
    type_name == class_id
        || type_name
            .strip_prefix("4D.")
            .is_some_and(|rest| rest == class_id)
}

/// One-line header for a class, shared by `class`, `query` class cards,
/// `members` and `returns` so they can never disagree with each other.
///
/// Abstractness is deliberately *not* in the header. `isAbstract` records the
/// documented instantiation guidance ("obtain an instance of one of its
/// concrete subclasses"), not whether the type may be named — and since the
/// type may be named, saying so next to the type name would misinform.
/// The card body carries the guidance instead.
pub fn class_heading(class_id: &str, type_name: &str) -> String {
    if type_name == class_id {
        type_name.to_string()
    } else {
        format!("{type_name} (class {class_id})")
    }
}

pub fn build_member_result(record: &MemberRecord, score: Option<f32>) -> MemberResult {
    let idx = index::get();
    let class_type_name = idx
        .classes
        .get(record.class_id())
        .map(|c| c.ir.type_name.clone())
        .unwrap_or_else(|| record.class_id().to_string());
    let types = accessor_types(record);

    MemberResult {
        result_type: "member",
        id: record.ir.id.clone(),
        display_name: record.ir.display_name.clone(),
        kind: record.ir.kind.clone(),
        member_name: record.ir.member_name.clone(),
        class_id: record.class_id().to_string(),
        class_type_name,
        receiver_kind: record.ir.receiver.kind.clone(),
        summary: record.ir.summary.clone(),
        score,
        raw_syntax: raw_syntax(record),
        overloads: record.ir.overloads.clone(),
        accessor: record.ir.accessor.clone(),
        multi_variant: types.len() > 1,
        accessor_types: types,
        dynamic_member: record.ir.dynamic_member.clone(),
        returns_shape: record.ir.returns_shape.clone(),
        error_model: record.ir.error_model.clone(),
        since_version: record.ir.since_version.clone(),
        version_behavior_changes: record.ir.version_behavior_changes.clone(),
        doc_page: record.ir.doc_page.clone(),
        inherited_from: None,
        requested_as: None,
        example: build_example(record),
    }
}

// ---------------------------------------------------------------------------
// Class cards
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct MemberSummary {
    pub id: String,
    #[serde(rename = "memberName")]
    pub member_name: String,
    pub kind: String,
    pub summary: String,
    /// True when this member is declared on an ancestor class and merely
    /// reachable here.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub inherited: bool,
    #[serde(rename = "declaredOn", skip_serializing_if = "Option::is_none")]
    pub declared_on: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ClassResult {
    #[serde(rename = "resultType")]
    pub result_type: &'static str,
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// What you write in a 4D type declaration: `4D.File`, `Collection`,
    /// `4D.Entity`. Not always `4D.<id>`.
    #[serde(rename = "typeName")]
    pub type_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<f32>,
    /// Why this card was returned instead of individual member rows.
    #[serde(rename = "cardReason", skip_serializing_if = "Option::is_none")]
    pub card_reason: Option<&'static str>,
    /// **How to obtain an instance** — raw IR recipes. This is the single most
    /// common thing an agent gets wrong about 4D OOP, so it leads the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instantiation: Option<Value>,
    /// A one-line, human-facing restatement of the recipes.
    #[serde(rename = "howToObtain")]
    pub how_to_obtain: Vec<String>,
    /// False for the 4 event-callback classes (`IncomingMessage`, `TCPEvent`,
    /// `UDPEvent`, `WebSocketConnection`) and the abstract bases: no
    /// expression in user code produces one.
    #[serde(rename = "constructibleByUserCode")]
    pub constructible_by_user_code: bool,
    #[serde(
        rename = "constructibilityNote",
        skip_serializing_if = "Option::is_none"
    )]
    pub constructibility_note: Option<&'static str>,
    #[serde(rename = "isAbstract", skip_serializing_if = "std::ops::Not::not")]
    pub is_abstract: bool,
    #[serde(rename = "isSingleton", skip_serializing_if = "std::ops::Not::not")]
    pub is_singleton: bool,
    #[serde(rename = "isStatic", skip_serializing_if = "std::ops::Not::not")]
    pub is_static: bool,
    /// True for the ORDA template classes (`DataClass`, `Entity`,
    /// `EntitySelection`, `cs` and the `cs.<DataClass>*` shapes), whose real
    /// per-project types are generated from the datastore's structure.
    #[serde(rename = "isTemplate", skip_serializing_if = "std::ops::Not::not")]
    pub is_template: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub superclass: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subclasses: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constructor: Option<String>,
    #[serde(rename = "sharedSemantics", skip_serializing_if = "Option::is_none")]
    pub shared_semantics: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(rename = "docPage", skip_serializing_if = "Option::is_none")]
    pub doc_page: Option<String>,
    #[serde(rename = "memberCounts")]
    pub member_counts: MemberCounts,
    pub members: Vec<MemberSummary>,
}

#[derive(Debug, Serialize, Default)]
pub struct MemberCounts {
    pub total: usize,
    pub declared: usize,
    pub inherited: usize,
    #[serde(rename = "oop_function")]
    pub functions: usize,
    #[serde(rename = "oop_property")]
    pub properties: usize,
    #[serde(rename = "oop_constructor")]
    pub constructors: usize,
}

const NOT_CONSTRUCTIBLE_CALLBACK: &str = "Instances of this class are created by 4D and passed into a callback; no expression in user code constructs one. Declare the callback parameter at this type and use the instance 4D hands you.";
const NOT_CONSTRUCTIBLE_ABSTRACT: &str = "This is an abstract base class: it is never instantiated directly. Obtain an instance of one of its concrete subclasses instead.";

fn recipe_kinds(cl: &ClassRecord) -> Vec<String> {
    cl.ir
        .instantiation
        .as_ref()
        .and_then(|i| i.get("recipes"))
        .and_then(Value::as_array)
        .map(|rs| {
            rs.iter()
                .filter_map(|r| r.get("kind").and_then(Value::as_str))
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn how_to_obtain(cl: &ClassRecord) -> Vec<String> {
    let Some(recipes) = cl
        .ir
        .instantiation
        .as_ref()
        .and_then(|i| i.get("recipes"))
        .and_then(Value::as_array)
    else {
        return Vec::new();
    };
    recipes
        .iter()
        .map(|r| {
            let expr = r.get("expression").and_then(Value::as_str).unwrap_or("");
            let kind = r.get("kind").and_then(Value::as_str).unwrap_or("");
            let mut line = format!("{expr}   [{kind}]");
            if let Some(reqs) = r.get("requires").and_then(Value::as_array) {
                let reqs: Vec<&str> = reqs.iter().filter_map(Value::as_str).collect();
                if !reqs.is_empty() {
                    line.push_str(&format!(" requires: {}", reqs.join(", ")));
                }
            }
            if let Some(note) = r.get("note").and_then(Value::as_str) {
                line.push_str(&format!(" -- {note}"));
            }
            line
        })
        .collect()
}

pub fn build_class_result(
    cl: &ClassRecord,
    score: Option<f32>,
    reason: Option<ClassCardReason>,
) -> ClassResult {
    let idx = index::get();
    let kinds = recipe_kinds(cl);
    let all_callback = !kinds.is_empty() && kinds.iter().all(|k| k == "event_callback");
    let all_not_instantiable =
        !kinds.is_empty() && kinds.iter().all(|k| k == "not_directly_instantiable");
    let constructible = !(all_callback || all_not_instantiable || cl.ir.is_abstract);
    let constructibility_note = if all_callback {
        Some(NOT_CONSTRUCTIBLE_CALLBACK)
    } else if all_not_instantiable || cl.ir.is_abstract {
        Some(NOT_CONSTRUCTIBLE_ABSTRACT)
    } else {
        None
    };

    let mut subclasses: Vec<String> = idx
        .classes
        .values()
        .filter(|c| c.ir.superclass.as_deref() == Some(cl.ir.id.as_str()))
        .map(|c| c.ir.id.clone())
        .collect();
    subclasses.sort();

    let mut counts = MemberCounts::default();
    let mut members = Vec::new();
    for (member_id, inherited) in cl
        .own_members
        .iter()
        .map(|m| (m, false))
        .chain(cl.inherited_members.iter().map(|m| (m, true)))
    {
        let Some(m) = idx.members.get(member_id) else {
            continue;
        };
        counts.total += 1;
        if inherited {
            counts.inherited += 1;
        } else {
            counts.declared += 1;
        }
        match m.ir.kind.as_str() {
            "oop_function" => counts.functions += 1,
            "oop_property" => counts.properties += 1,
            "oop_constructor" => counts.constructors += 1,
            _ => {}
        }
        members.push(MemberSummary {
            id: m.ir.id.clone(),
            member_name: m.ir.member_name.clone(),
            kind: m.ir.kind.clone(),
            summary: m.ir.summary.clone(),
            inherited,
            declared_on: inherited.then(|| m.class_id().to_string()),
        });
    }

    ClassResult {
        result_type: "class",
        id: cl.ir.id.clone(),
        display_name: cl.ir.display_name.clone(),
        type_name: cl.ir.type_name.clone(),
        score,
        card_reason: reason.map(|r| match r {
            ClassCardReason::ExactNameMatch => "query names this class",
            ClassCardReason::ManyMembersMatched => {
                "many members of this class matched; showing the class card instead of flooding the results"
            }
        }),
        instantiation: cl.ir.instantiation.clone(),
        how_to_obtain: how_to_obtain(cl),
        constructible_by_user_code: constructible,
        constructibility_note,
        is_abstract: cl.ir.is_abstract,
        is_singleton: cl.ir.is_singleton,
        is_static: cl.ir.is_static,
        is_template: cl.ir.is_template.is_some(),
        superclass: cl.ir.superclass.clone(),
        subclasses,
        constructor: cl.ir.constructor.clone(),
        shared_semantics: cl.ir.shared_semantics.clone(),
        note: cl.ir.note.clone(),
        doc_page: cl.ir.doc_page.clone(),
        member_counts: counts,
        members,
    }
}

// ---------------------------------------------------------------------------
// Unified lookup result
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum LookupResult {
    Member(Box<MemberResult>),
    Class(Box<ClassResult>),
}

impl LookupResult {
    pub fn id(&self) -> &str {
        match self {
            LookupResult::Member(m) => &m.id,
            LookupResult::Class(c) => &c.id,
        }
    }

    pub fn score(&self) -> f32 {
        match self {
            LookupResult::Member(m) => m.score.unwrap_or(0.0),
            LookupResult::Class(c) => c.score.unwrap_or(0.0),
        }
    }

    pub fn is_class(&self) -> bool {
        matches!(self, LookupResult::Class(_))
    }
}

pub fn lookup(query: &str, limit: usize) -> Vec<LookupResult> {
    index::search(query, limit)
        .into_iter()
        .map(|hit| match hit {
            Hit::Member(m, score) => {
                LookupResult::Member(Box::new(build_member_result(m, Some(score))))
            }
            Hit::Class(c, score, reason) => {
                LookupResult::Class(Box::new(build_class_result(c, Some(score), Some(reason))))
            }
        })
        .collect()
}

/// Exact class lookup by id or 4D type name.
pub fn class_card(name: &str) -> Option<ClassResult> {
    index::get()
        .resolve_class(name)
        .map(|c| build_class_result(c, None, None))
}

/// Exact member lookup, resolving inherited spellings. Asking for
/// `File.exists` returns `Document.exists` annotated with `inheritedFrom` and
/// `requestedAs`, rather than failing because the caller guessed the wrong
/// declaring class.
pub fn member_card(name: &str) -> Option<MemberResult> {
    let idx = index::get();
    let record = idx.resolve_member(name)?;
    let mut result = build_member_result(record, None);

    // Did the caller name a class that merely inherits this member?
    if let Some((class_part, _)) = name.rsplit_once('.') {
        if let Some(cl) = idx.resolve_class(class_part) {
            if idx.is_inherited_on(&cl.ir.id, &record.ir.id) {
                result.inherited_from = Some(record.class_id().to_string());
                result.requested_as = Some(name.to_string());
            }
        }
    }
    Some(result)
}

/// Members of a class, optionally filtered by kind. Includes inherited members.
#[derive(Debug, Serialize)]
pub struct MembersListing {
    #[serde(rename = "classId")]
    pub class_id: String,
    #[serde(rename = "typeName")]
    pub type_name: String,
    #[serde(rename = "isAbstract")]
    pub is_abstract: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub count: usize,
    pub members: Vec<MemberSummary>,
}

pub fn members_listing(class_name: &str, kind: Option<&str>) -> Option<MembersListing> {
    let card = class_card(class_name)?;
    let wanted = kind.map(normalize_kind);
    let members: Vec<MemberSummary> = card
        .members
        .into_iter()
        .filter(|m| wanted.as_deref().is_none_or(|k| m.kind == k))
        .collect();
    Some(MembersListing {
        class_id: card.id,
        type_name: card.type_name,
        is_abstract: card.is_abstract,
        kind: wanted,
        count: members.len(),
        members,
    })
}

/// Accepts `function`/`property`/`constructor` as well as the IR's own
/// `oop_*` spellings.
pub fn normalize_kind(kind: &str) -> String {
    let k = kind.trim().to_lowercase();
    if k.starts_with("oop_") {
        k
    } else {
        format!("oop_{}", k.trim_end_matches('s'))
    }
}

// ---------------------------------------------------------------------------
// "What produces an instance of this class"
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize)]
pub struct ProducerEntry {
    /// The relationship kinds that connect this producer to the class:
    /// `returns_instance_of` (its declared return type is this class) and/or
    /// `obtained_via` (an IR-curated "how to obtain one" edge).
    pub relationships: Vec<String>,
    /// Member id, or classic-language command name when `corpus` is `classic`.
    pub producer: String,
    /// `oop` (this corpus) or `classic` (lives in `4d-language-classic`).
    pub corpus: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(rename = "rawSyntax", skip_serializing_if = "Vec::is_empty")]
    pub raw_syntax: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub notes: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ReturnsResult {
    #[serde(rename = "classId")]
    pub class_id: String,
    #[serde(rename = "typeName")]
    pub type_name: String,
    #[serde(rename = "isAbstract")]
    pub is_abstract: bool,
    #[serde(rename = "constructibleByUserCode")]
    pub constructible_by_user_code: bool,
    #[serde(
        rename = "constructibilityNote",
        skip_serializing_if = "Option::is_none"
    )]
    pub constructibility_note: Option<&'static str>,
    #[serde(rename = "howToObtain")]
    pub how_to_obtain: Vec<String>,
    pub count: usize,
    pub producers: Vec<ProducerEntry>,
}

pub fn returns_result(class_name: &str) -> Option<ReturnsResult> {
    let idx = index::get();
    let cl = idx.resolve_class(class_name)?;
    let card = build_class_result(cl, None, None);

    // One row per producer, merging its relationship kinds: `File.open` is
    // both the `returns_instance_of` edge and the `obtained_via` recipe for
    // `4D.FileHandle`, and listing it twice is just noise.
    let mut order: Vec<String> = Vec::new();
    let mut merged: std::collections::HashMap<String, ProducerEntry> =
        std::collections::HashMap::new();
    for rel in idx.producers_of(&cl.ir.id) {
        let (endpoint, corpus_hint) = match rel.kind.as_str() {
            "returns_instance_of" => (&rel.from, rel.from.corpus.as_deref()),
            _ => (&rel.to, rel.to.corpus.as_deref()),
        };
        let Some(producer) = endpoint.command.clone() else {
            continue;
        };
        let corpus = if corpus_hint == Some("classic") {
            "classic"
        } else {
            "oop"
        };
        let entry = merged.entry(producer.clone()).or_insert_with(|| {
            order.push(producer.clone());
            let member = idx.members.get(&producer);
            ProducerEntry {
                relationships: Vec::new(),
                summary: member.map(|m| m.ir.summary.clone()),
                raw_syntax: member.map(raw_syntax).unwrap_or_default(),
                producer,
                corpus,
                notes: Vec::new(),
            }
        });
        if !entry.relationships.contains(&rel.kind) {
            entry.relationships.push(rel.kind.clone());
        }
        if let Some(note) = &rel.note {
            if !entry.notes.contains(note) {
                entry.notes.push(note.clone());
            }
        }
    }
    let mut producers: Vec<ProducerEntry> = order
        .into_iter()
        .filter_map(|k| merged.remove(&k))
        .collect();
    for p in &mut producers {
        p.relationships.sort();
    }
    producers.sort_by(|a, b| a.producer.cmp(&b.producer));

    Some(ReturnsResult {
        class_id: card.id,
        type_name: card.type_name,
        is_abstract: card.is_abstract,
        constructible_by_user_code: card.constructible_by_user_code,
        constructibility_note: card.constructibility_note,
        how_to_obtain: card.how_to_obtain,
        count: producers.len(),
        producers,
    })
}
