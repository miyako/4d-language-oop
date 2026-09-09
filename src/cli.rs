//! One-shot subcommands: `query`, `class`, `member`, `members`, `returns`.
//!
//! Every one of them prints human-readable text by default and the exact JSON
//! shape the HTTP server returns under `--json`.

use crate::index;
use crate::model::{
    self, ClassResult, ExampleBlock, ExampleSource, LookupResult, MemberResult, MembersListing,
    ReturnsResult,
};

// ---------------------------------------------------------------------------
// query
// ---------------------------------------------------------------------------

pub struct QueryArgs {
    pub query: String,
    pub limit: usize,
    pub json: bool,
}

pub fn parse_query_args(args: &[String]) -> Result<QueryArgs, String> {
    let mut query: Option<String> = None;
    let mut limit: usize = 5;
    let mut json = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--limit" => {
                i += 1;
                let val = args.get(i).ok_or("--limit requires a value")?;
                limit = val
                    .parse()
                    .map_err(|_| format!("invalid --limit value: {val}"))?;
            }
            "--json" => json = true,
            other => {
                if query.is_some() {
                    return Err(format!("unexpected extra argument: {other}"));
                }
                query = Some(other.to_string());
            }
        }
        i += 1;
    }

    let query = query.ok_or_else(|| "missing query text".to_string())?;
    if limit == 0 {
        return Err("--limit must be at least 1".to_string());
    }
    Ok(QueryArgs { query, limit, json })
}

pub fn run_query(args: QueryArgs) -> i32 {
    let results = model::lookup(&args.query, args.limit);

    if args.json {
        return print_json(&results);
    }
    if results.is_empty() {
        println!("No matches for query: {:?}", args.query);
        return 0;
    }

    for (rank, r) in results.iter().enumerate() {
        match r {
            LookupResult::Member(m) => {
                println!(
                    "{}. {} ({})  score={:.2}",
                    rank + 1,
                    m.display_name,
                    m.kind,
                    m.score.unwrap_or(0.0)
                );
                print_member_body(m, "   ");
            }
            LookupResult::Class(c) => {
                println!(
                    "{}. {} [class card]  score={:.2}",
                    rank + 1,
                    c.type_name,
                    c.score.unwrap_or(0.0)
                );
                if let Some(reason) = c.card_reason {
                    println!("   ({reason})");
                }
                print_class_body(c, "   ", 8);
            }
        }
        println!();
    }
    0
}

// ---------------------------------------------------------------------------
// class / member / members / returns
// ---------------------------------------------------------------------------

/// Shared shape for the four exact-lookup subcommands: one positional name
/// plus optional flags.
pub struct NameArgs {
    pub name: String,
    pub kind: Option<String>,
    pub json: bool,
}

/// `accept_class_flag` allows `--class X` in place of a positional name, which
/// is how `members` is spelled.
pub fn parse_name_args(
    args: &[String],
    accept_class_flag: bool,
    accept_kind: bool,
) -> Result<NameArgs, String> {
    let mut name: Option<String> = None;
    let mut kind: Option<String> = None;
    let mut json = false;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--json" => json = true,
            "--class" if accept_class_flag => {
                i += 1;
                let val = args.get(i).ok_or("--class requires a value")?;
                name = Some(val.clone());
            }
            "--kind" if accept_kind => {
                i += 1;
                let val = args.get(i).ok_or("--kind requires a value")?;
                let normalized = model::normalize_kind(val);
                if !matches!(
                    normalized.as_str(),
                    "oop_function" | "oop_property" | "oop_constructor"
                ) {
                    return Err(format!(
                        "invalid --kind value: {val} (expected function, property or constructor)"
                    ));
                }
                kind = Some(normalized);
            }
            other => {
                if other.starts_with("--") {
                    return Err(format!("unexpected option: {other}"));
                }
                if name.is_some() {
                    return Err(format!("unexpected extra argument: {other}"));
                }
                name = Some(other.to_string());
            }
        }
        i += 1;
    }

    let name = name.ok_or_else(|| "missing name".to_string())?;
    Ok(NameArgs { name, kind, json })
}

pub fn run_class(args: NameArgs) -> i32 {
    let Some(card) = model::class_card(&args.name) else {
        return not_found("class", &args.name, class_suggestions(&args.name));
    };
    if args.json {
        return print_json(&card);
    }
    println!("{} (class {})", card.type_name, card.id);
    print_class_body(&card, "", usize::MAX);
    0
}

pub fn run_member(args: NameArgs) -> i32 {
    let Some(card) = model::member_card(&args.name) else {
        return not_found("member", &args.name, member_suggestions(&args.name));
    };
    if args.json {
        return print_json(&card);
    }
    println!("{} ({})", card.display_name, card.kind);
    print_member_body(&card, "");
    0
}

pub fn run_members(args: NameArgs) -> i32 {
    let Some(listing) = model::members_listing(&args.name, args.kind.as_deref()) else {
        return not_found("class", &args.name, class_suggestions(&args.name));
    };
    if args.json {
        return print_json(&listing);
    }
    print_members_text(&listing);
    0
}

fn print_members_text(listing: &MembersListing) {
    println!(
        "{} ({}) -- {} member(s){}",
        listing.type_name,
        listing.class_id,
        listing.count,
        listing
            .kind
            .as_deref()
            .map(|k| format!(", kind={k}"))
            .unwrap_or_default()
    );
    for m in &listing.members {
        let marker = match m.declared_on.as_deref() {
            Some(owner) => format!(" (inherited from {owner})"),
            None => String::new(),
        };
        println!("  {:<28} {:<16}{}", m.member_name, short_kind(&m.kind), marker);
        println!("      {}", first_line(&m.summary));
    }
}

pub fn run_returns(args: NameArgs) -> i32 {
    let Some(result) = model::returns_result(&args.name) else {
        return not_found("class", &args.name, class_suggestions(&args.name));
    };
    if args.json {
        return print_json(&result);
    }
    print_returns_text(&result);
    0
}

fn print_returns_text(r: &ReturnsResult) {
    println!("What produces a {} ({})?", r.type_name, r.class_id);
    if !r.constructible_by_user_code {
        println!("  NOTE: not constructible by user code.");
        if let Some(note) = r.constructibility_note {
            println!("        {note}");
        }
    }
    println!("  how to obtain an instance:");
    for line in &r.how_to_obtain {
        println!("    {line}");
    }
    println!("  {} producer(s):", r.count);
    for p in &r.producers {
        let corpus = if p.corpus == "classic" {
            "  [classic language -- see 4d-language-classic]"
        } else {
            ""
        };
        println!("    {} [{}]{}", p.producer, p.relationships.join(", "), corpus);
        if let Some(summary) = &p.summary {
            println!("      {}", first_line(summary));
        }
        for s in &p.raw_syntax {
            println!("      {}", strip_markdown(s));
        }
        for note in &p.notes {
            println!("      {note}");
        }
    }
}

// ---------------------------------------------------------------------------
// Shared text rendering
// ---------------------------------------------------------------------------

fn print_member_body(m: &MemberResult, pad: &str) {
    if m.class_id == m.class_type_name {
        println!("{pad}class: {} ({})", m.class_id, m.receiver_kind);
    } else {
        println!(
            "{pad}class: {} (declared type {}, {} member)",
            m.class_id, m.class_type_name, m.receiver_kind
        );
    }
    if let Some(from) = &m.inherited_from {
        let asked = m.requested_as.as_deref().unwrap_or(&m.id);
        println!("{pad}inherited: {asked} resolves to {}, declared on {from}", m.id);
    }
    println!("{pad}summary: {}", first_line(&m.summary));

    if let Some(dynamic) = &m.dynamic_member {
        // The 7 dynamic pseudo-members have no fixed name and no source line.
        let pattern = dynamic
            .get("namePattern")
            .and_then(|v| v.as_str())
            .unwrap_or("<dynamic>");
        let note = dynamic.get("note").and_then(|v| v.as_str()).unwrap_or("");
        println!("{pad}DYNAMIC MEMBER: the name is a pattern, not a literal member name.");
        println!("{pad}  pattern: {pattern}");
        if !note.is_empty() {
            println!("{pad}  {note}");
        }
    }

    if m.raw_syntax.is_empty() {
        if m.dynamic_member.is_none() {
            println!("{pad}syntax: (none published)");
        }
    } else {
        println!("{pad}syntax (verbatim from the 4D docs):");
        for s in &m.raw_syntax {
            for line in s.split("<br/>") {
                println!("{pad}  {}", strip_markdown(line));
            }
        }
    }

    if !m.accessor_types.is_empty() {
        let names: Vec<String> = m.accessor_types.iter().map(type_ref_name).collect();
        if m.multi_variant {
            // Showing only the first variant is a bug this project has already
            // been bitten by once. All of them, always.
            println!(
                "{pad}property type: MULTI-VARIANT -- all {} alternatives are valid: {}",
                names.len(),
                names.join(" | ")
            );
        } else {
            println!("{pad}property type: {}", names.join(" | "));
        }
        if let Some(acc) = &m.accessor {
            let readable = acc.get("readable").and_then(|v| v.as_bool()).unwrap_or(true);
            let writable = acc.get("writable").and_then(|v| v.as_bool()).unwrap_or(false);
            let computed = acc.get("computed").and_then(|v| v.as_bool()).unwrap_or(false);
            println!(
                "{pad}  access: {}{}{}",
                if readable { "read" } else { "" },
                if writable { "/write" } else { " only" },
                if computed { " (computed)" } else { "" }
            );
        }
    }

    print_example(&m.example, pad);
}

fn print_example(example: &model::ExampleInfo, pad: &str) {
    let Some(primary) = &example.primary else {
        println!("{pad}example: none available");
        return;
    };
    print_example_block(primary, pad);
    // Alternates are summarized, not dumped: text mode stays readable, and
    // `--json` (or the HTTP API) carries every block in full.
    for alt in &example.alternates {
        let source = source_label(alt.source);
        let verified = if alt.compiler_verified {
            "compiler-verified"
        } else {
            "NOT compiler-verified"
        };
        let lines = alt.raw.lines().count();
        println!(
            "{pad}also available: {source} example, {verified}, {lines} line(s) -- see --json"
        );
    }
}

fn source_label(source: ExampleSource) -> &'static str {
    match source {
        ExampleSource::Documentation => "documentation",
        ExampleSource::Synthetic => "synthetic",
    }
}

fn print_example_block(block: &ExampleBlock, pad: &str) {
    let verified = if block.compiler_verified {
        "compiler-verified"
    } else {
        "NOT compiler-verified"
    };
    println!("{pad}example ({}, {verified}):", source_label(block.source));
    if let Some(caption) = &block.caption {
        println!("{pad}  {}", first_line(caption));
    }
    if block.source == ExampleSource::Synthetic {
        println!("{pad}  NOTE: $result1/$options2, [SynthTable], cs.SynthOOPHandler etc. below are auto-generated placeholders -- substitute your own receiver, values and names, do not copy verbatim.");
    }
    if let Some(warning) = block.warning {
        println!("{pad}  WARNING: {warning}");
    }
    for line in block.raw.lines() {
        println!("{pad}    {line}");
    }
}

fn print_class_body(c: &ClassResult, pad: &str, max_members: usize) {
    let mut flags = Vec::new();
    if c.is_abstract {
        flags.push("abstract");
    }
    if c.is_template {
        flags.push("ORDA template");
    }
    if c.is_singleton {
        flags.push("singleton");
    }
    if c.is_static {
        flags.push("static/namespace");
    }
    if !flags.is_empty() {
        println!("{pad}flags: {}", flags.join(", "));
    }
    if let Some(sup) = &c.superclass {
        println!("{pad}superclass: {sup}");
    }
    if !c.subclasses.is_empty() {
        println!("{pad}subclasses: {}", c.subclasses.join(", "));
    }

    // The single most common thing an agent gets wrong about 4D OOP, so it
    // leads the card.
    println!("{pad}HOW TO OBTAIN AN INSTANCE:");
    if !c.constructible_by_user_code {
        println!("{pad}  NOT CONSTRUCTIBLE BY USER CODE.");
        if let Some(note) = c.constructibility_note {
            println!("{pad}  {note}");
        }
    }
    for line in &c.how_to_obtain {
        println!("{pad}  {line}");
    }
    if let Some(shared) = &c.shared_semantics {
        if let Some(note) = shared.get("note").and_then(|v| v.as_str()) {
            println!("{pad}shared semantics: {note}");
        }
    }
    if let Some(note) = &c.note {
        println!("{pad}note: {note}");
    }

    let counts = &c.member_counts;
    println!(
        "{pad}members: {} total ({} declared, {} inherited; {} functions, {} properties, {} constructors)",
        counts.total,
        counts.declared,
        counts.inherited,
        counts.functions,
        counts.properties,
        counts.constructors
    );
    for m in c.members.iter().take(max_members) {
        let marker = match m.declared_on.as_deref() {
            Some(owner) => format!(" (inherited from {owner})"),
            None => String::new(),
        };
        println!("{pad}  {:<26} {:<14}{}", m.member_name, short_kind(&m.kind), marker);
    }
    if c.members.len() > max_members {
        println!(
            "{pad}  ... and {} more -- run `4d-language-oop members --class {}`",
            c.members.len() - max_members,
            c.id
        );
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn print_json<T: serde::Serialize>(value: &T) -> i32 {
    match serde_json::to_string_pretty(value) {
        Ok(s) => {
            println!("{s}");
            0
        }
        Err(e) => {
            eprintln!("error: failed to serialize results: {e}");
            1
        }
    }
}

fn not_found(what: &str, name: &str, suggestions: Vec<String>) -> i32 {
    eprintln!("error: no such {what}: {name:?}");
    if !suggestions.is_empty() {
        eprintln!("did you mean: {}", suggestions.join(", "));
    }
    1
}

/// Cheap deterministic suggestions: prefix/substring matches on the class
/// name, capped and alphabetical. No fuzzy distance metric — deliberately.
fn class_suggestions(name: &str) -> Vec<String> {
    let needle = name.to_lowercase();
    let idx = index::get();
    let mut out: Vec<String> = idx
        .classes
        .values()
        .filter(|c| {
            c.ir.id.to_lowercase().contains(&needle) || c.ir.type_name.to_lowercase().contains(&needle)
        })
        .map(|c| c.ir.type_name.clone())
        .collect();
    out.sort();
    out.truncate(5);
    out
}

fn member_suggestions(name: &str) -> Vec<String> {
    let needle = name.to_lowercase().replace('.', "");
    let idx = index::get();
    let mut out: Vec<String> = idx
        .members
        .keys()
        .filter(|id| id.to_lowercase().replace('.', "").contains(&needle))
        .cloned()
        .collect();
    out.sort();
    out.truncate(5);
    out
}

fn short_kind(kind: &str) -> &str {
    kind.strip_prefix("oop_").unwrap_or(kind)
}

fn first_line(text: &str) -> String {
    let line = text.lines().next().unwrap_or("").trim();
    strip_markdown(line)
}

/// The IR's summaries and `rawSyntax` lines carry the docs' own markdown
/// emphasis (`**.slice**`, `*start*`). Strip it for plain-text output only —
/// `--json` and the HTTP API always return the verbatim string.
fn strip_markdown(text: &str) -> String {
    text.replace("**", "").replace('*', "").trim().to_string()
}

fn type_ref_name(value: &serde_json::Value) -> String {
    value
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string()
}
