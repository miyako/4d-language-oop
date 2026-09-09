//! Regression guards for the deterministic lookup index.
//!
//! Three kinds of assertions:
//! - `assert_top1`: the query has one clearly-correct answer, which must rank #1.
//! - `assert_in_top`: the query is legitimately ambiguous between a small set
//!   of closely related members (e.g. `.orderBy` vs `.orderByMethod` vs
//!   `.multiSort`); any one of the expected ids appearing in the top N is a
//!   pass. This avoids over-fitting the suite to incidental scoring details
//!   while still catching real regressions (the expected member falling out of
//!   the results entirely).
//! - structural assertions on the class-flooding cap, inherited-member
//!   resolution, multi-variant properties, dynamic pseudo-members and the
//!   example-precedence rule.
//!
//! Every query/expectation below was verified against the actual embedded
//! index, not guessed. The suite is deliberately frozen: a ranking change is
//! supposed to show up here as a diff.

use fourd_language_oop::index::{self, CLASS_MEMBER_CAP};
use fourd_language_oop::ir;
use fourd_language_oop::model::{self, LookupResult};
use std::collections::HashMap;

const TOP_N: usize = 5;

fn ids(query: &str, limit: usize) -> Vec<String> {
    model::lookup(query, limit)
        .iter()
        .map(|r| r.id().to_string())
        .collect()
}

/// Ids of member rows only, ignoring class cards. Most "which member answers
/// this?" expectations want this view.
fn member_ids(query: &str, limit: usize) -> Vec<String> {
    model::lookup(query, limit)
        .iter()
        .filter(|r| !r.is_class())
        .map(|r| r.id().to_string())
        .collect()
}

fn assert_top1(query: &str, expected_id: &str) {
    let got = member_ids(query, TOP_N);
    assert!(
        !got.is_empty(),
        "query {query:?} returned no member results (expected top-1 {expected_id:?})"
    );
    assert_eq!(
        got[0],
        expected_id,
        "query {query:?}: expected top member {expected_id:?}, got {:?} (full top-{TOP_N}: {:?})",
        got[0],
        ids(query, TOP_N)
    );
}

fn assert_in_top(query: &str, expected_ids: &[&str], top_n: usize) {
    let got = ids(query, top_n);
    assert!(
        expected_ids.iter().any(|e| got.iter().any(|g| g == e)),
        "query {query:?}: expected one of {expected_ids:?} in top {top_n}, got {got:?}"
    );
}

/// The query must surface the named class's *card*, not just its members.
fn assert_class_card(query: &str, expected_class: &str, top_n: usize) {
    let results = model::lookup(query, top_n);
    let cards: Vec<&str> = results
        .iter()
        .filter(|r| r.is_class())
        .map(|r| r.id())
        .collect();
    assert!(
        cards.contains(&expected_class),
        "query {query:?}: expected a class card for {expected_class:?} in top {top_n}, got cards {cards:?} (all: {:?})",
        ids(query, top_n)
    );
}

// ---------------------------------------------------------------------------
// Files and folders
// ---------------------------------------------------------------------------

#[test]
fn read_a_file() {
    assert_top1("read a text file line by line", "FileHandle.readLine");
    assert_in_top(
        "get the contents of a file as text",
        &[
            "Document.getText",
            "Document.getContent",
            "FileHandle.readText",
        ],
        TOP_N,
    );
}

#[test]
fn write_a_file() {
    assert_in_top(
        "write text to a file",
        &["FileHandle.writeText", "File.setText", "File.setContent"],
        TOP_N,
    );
}

#[test]
fn file_existence_and_deletion() {
    // `.exists` is declared on the abstract `Document` base, not on `File`.
    assert_top1("check if a file exists", "Document.exists");
    assert_top1("delete a file", "File.delete");
}

#[test]
fn folders() {
    assert_top1("list the files in a folder", "Directory.files");
    assert_top1("create a folder", "Folder.create");
    assert_in_top(
        "copy a file to another folder",
        &["Document.copyTo", "Directory.copyTo"],
        TOP_N,
    );
}

#[test]
fn file_size() {
    assert_in_top(
        "get the size of a file",
        &["Document.size", "FileHandle.getSize"],
        TOP_N,
    );
}

// ---------------------------------------------------------------------------
// Collections
// ---------------------------------------------------------------------------

#[test]
fn order_a_collection() {
    assert_in_top(
        "order a collection",
        &[
            "Collection.orderBy",
            "Collection.orderByMethod",
            "Collection.multiSort",
        ],
        TOP_N,
    );
    assert_in_top(
        "sort a collection by a property",
        &[
            "Collection.orderBy",
            "Collection.orderByMethod",
            "Collection.multiSort",
        ],
        TOP_N,
    );
}

#[test]
fn collection_basics() {
    assert_top1("add an element to a collection", "Collection.push");
    assert_top1("count elements in a collection", "Collection.count");
    assert_in_top(
        "sum the values in a collection",
        &["Collection.sum", "Collection.average"],
        TOP_N,
    );
    assert_in_top(
        "find an element in a collection",
        &[
            "Collection.findIndex",
            "Collection.indexOf",
            "Collection.query",
        ],
        TOP_N,
    );
}

// ---------------------------------------------------------------------------
// ORDA
// ---------------------------------------------------------------------------

#[test]
fn orda_queries() {
    assert_top1("query a dataclass", "DataClass.query");
    assert_top1("save an entity", "Entity.save");
    assert_in_top(
        "sort an entity selection",
        &["EntitySelection.orderBy", "EntitySelection.orderByFormula"],
        TOP_N,
    );
    assert_in_top(
        "convert an entity selection to a collection",
        &["EntitySelection.toCollection"],
        TOP_N,
    );
}

#[test]
fn creating_an_entity_surfaces_the_class_card() {
    // No member is called "create an entity" -- `ds.MyClass.new()` is. The
    // Entity class card is what carries that recipe, so it must be present.
    assert_class_card("create a new entity", "Entity", TOP_N);
}

// ---------------------------------------------------------------------------
// Mail, HTTP, web, workers
// ---------------------------------------------------------------------------

#[test]
fn send_an_email() {
    assert_top1("send an email with an attachment", "SMTPTransporter.send");
}

#[test]
fn connect_to_imap_mailbox() {
    assert_class_card("connect to an imap mailbox", "IMAPTransporter", TOP_N);
}

#[test]
fn http_requests() {
    assert_class_card("make an http request", "HTTPRequest", TOP_N);
    assert_in_top(
        "download a file over http",
        &[
            "4D.HTTPRequest.new",
            "HTTPRequest.agent",
            "4D.HTTPAgent.new",
        ],
        TOP_N,
    );
}

#[test]
fn web_and_sockets() {
    assert_top1("start the web server", "WebServer.start");
    assert_class_card(
        "handle a websocket connection",
        "WebSocketConnection",
        TOP_N,
    );
    assert_class_card("open a tcp connection", "TCPConnection", TOP_N);
}

#[test]
fn system_worker() {
    assert_in_top(
        "run a shell command",
        &["4D.SystemWorker.new", "SystemWorker.commandLine"],
        TOP_N,
    );
}

#[test]
fn crypto_and_session() {
    assert_top1("encrypt data with a key", "CryptoKey.encrypt");
    assert_top1(
        "get the current session privileges",
        "Session.getPrivileges",
    );
}

// ---------------------------------------------------------------------------
// Class-name queries resolve to class cards
// ---------------------------------------------------------------------------

#[test]
fn naming_a_class_returns_its_card_first() {
    for (query, class_id) in [
        ("collection", "Collection"),
        ("4D.File", "File"),
        ("4D.Folder", "Folder"),
        ("entity selection", "EntitySelection"),
        ("web server", "WebServer"),
        ("blob", "Blob"),
        ("4D.CryptoKey", "CryptoKey"),
    ] {
        let results = model::lookup(query, TOP_N);
        assert!(
            !results.is_empty(),
            "query {query:?} returned nothing (expected the {class_id} class card)"
        );
        assert!(
            results[0].is_class() && results[0].id() == class_id,
            "query {query:?}: expected the {class_id} class card ranked #1, got {:?}",
            ids(query, TOP_N)
        );
    }
}

// ---------------------------------------------------------------------------
// Class-flooding control
// ---------------------------------------------------------------------------

/// `WebServer` has 41 members, `Collection` 47, `EntitySelection` 34. Without
/// the cap, a query naming one of them returns nothing else.
#[test]
fn no_single_class_floods_the_results() {
    for query in [
        "web server",
        "collection",
        "entity selection",
        "order a collection",
        "http request options",
        "imap mailbox",
    ] {
        for limit in [5usize, 10, 20] {
            let results = model::lookup(query, limit);
            let mut per_class: HashMap<String, usize> = HashMap::new();
            for r in &results {
                if let LookupResult::Member(m) = r {
                    *per_class.entry(m.class_id.clone()).or_insert(0) += 1;
                }
            }
            for (class_id, count) in &per_class {
                assert!(
                    *count <= CLASS_MEMBER_CAP,
                    "query {query:?} (limit {limit}): class {class_id} contributed {count} member rows, cap is {CLASS_MEMBER_CAP}"
                );
            }
        }
    }
}

/// A class whose members flood the candidate pool must also offer its card, so
/// the caller gets `instantiation` rather than 41 near-identical rows.
#[test]
fn flooding_classes_emit_a_class_card() {
    assert_class_card("web server", "WebServer", TOP_N);
    assert_class_card("order a collection", "Collection", TOP_N);
    assert_class_card("sort an entity selection", "EntitySelection", TOP_N);
}

/// Collapse and cap decisions are made over a fixed-size candidate pool, so
/// raising `--limit` may only *append* rows -- it must never reorder or drop
/// the ones already shown.
#[test]
fn results_are_stable_as_the_limit_grows() {
    for query in [
        "order a collection",
        "read a text file line by line",
        "create a new entity",
        "web server",
    ] {
        let small = ids(query, 3);
        let large = ids(query, 12);
        assert!(
            large.starts_with(&small),
            "query {query:?}: top-3 {small:?} is not a prefix of top-12 {large:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// Inherited members
// ---------------------------------------------------------------------------

/// Inherited members are modelled once, on the declaring class: `.exists`
/// lives on `Document`, not `File`. A caller must not have to know that.
#[test]
fn inherited_members_resolve_from_the_subclass() {
    for (asked, declared_on) in [
        ("File.exists", "Document.exists"),
        ("4D.File.exists", "Document.exists"),
        ("ZipFile.getText", "Document.getText"),
        ("Folder.exists", "Directory.exists"),
        ("4D.Folder.files", "Directory.files"),
        (
            "SMTPTransporter.authenticationMode",
            "Transporter.authenticationMode",
        ),
        ("IMAPTransporter.port", "Transporter.port"),
        ("Formula.call", "Function.call"),
    ] {
        let card = model::member_card(asked)
            .unwrap_or_else(|| panic!("expected {asked:?} to resolve (declared on {declared_on})"));
        assert_eq!(
            card.id, declared_on,
            "expected {asked:?} to resolve to {declared_on:?}"
        );
        assert_eq!(
            card.inherited_from.as_deref(),
            Some(declared_on.split('.').next().expect("id has a class part")),
            "expected {asked:?} to be reported as inherited"
        );
        assert_eq!(card.requested_as.as_deref(), Some(asked));
    }
}

/// A member declared on the class itself must not be flagged as inherited.
#[test]
fn declared_members_are_not_flagged_inherited() {
    let card = model::member_card("File.delete").expect("File.delete is declared on File");
    assert_eq!(card.id, "File.delete");
    assert!(card.inherited_from.is_none());
}

/// The class card lists inherited members and attributes them correctly.
#[test]
fn class_card_lists_inherited_members() {
    let card = model::class_card("4D.File").expect("4D.File resolves");
    assert_eq!(card.id, "File");
    assert_eq!(card.superclass.as_deref(), Some("Document"));
    let exists = card
        .members
        .iter()
        .find(|m| m.id == "Document.exists")
        .expect("File must expose the inherited Document.exists");
    assert!(exists.inherited);
    assert_eq!(exists.declared_on.as_deref(), Some("Document"));
    assert!(card.member_counts.inherited > 0);
    assert_eq!(
        card.member_counts.total,
        card.member_counts.declared + card.member_counts.inherited
    );
}

// ---------------------------------------------------------------------------
// Class cards: instantiation and constructibility
// ---------------------------------------------------------------------------

/// `instantiation` is complete for all 50 classes and is the single most
/// common thing an agent gets wrong about 4D OOP.
#[test]
fn every_class_says_how_to_obtain_an_instance() {
    let idx = index::get();
    assert_eq!(idx.classes.len(), 50, "expected the full 50-class corpus");
    for id in idx.classes.keys() {
        let card = model::class_card(id).unwrap_or_else(|| panic!("class {id} must resolve"));
        assert!(
            !card.how_to_obtain.is_empty(),
            "class {id} has no instantiation recipe"
        );
        assert!(!card.type_name.is_empty(), "class {id} has no type name");
    }
}

/// The 4 event-callback classes have no constructor at all: instances only
/// ever arrive as callback arguments. The card must say so rather than
/// implying a constructor exists.
#[test]
fn event_callback_classes_are_not_constructible() {
    for id in [
        "IncomingMessage",
        "TCPEvent",
        "UDPEvent",
        "WebSocketConnection",
    ] {
        let card = model::class_card(id).unwrap_or_else(|| panic!("class {id} must resolve"));
        assert!(
            !card.constructible_by_user_code,
            "{id} must not be reported as constructible by user code"
        );
        assert!(
            card.constructibility_note
                .is_some_and(|n| n.contains("callback")),
            "{id} must carry a callback-only explanation"
        );
        assert!(card.constructor.is_none(), "{id} must have no constructor");
    }
}

/// An abstract base is still a real, nameable 4D type. This was once wrong in
/// the upstream IR: `Document.typeName` was recorded as `4D.File` and
/// `Directory.typeName` as `4D.Folder` — the type of one of their *two*
/// concrete subclasses, which was arbitrary and false. `tool4d` accepts
/// `var $x : 4D.Document` and rejects `4D.DocumentX`. Fixed in
/// `4d-static-docs` `fb0a69ac`; pinned here so it cannot regress.
///
/// All four abstract classes are covered, not just the two that were broken:
/// `Function` and `Transporter` were correct only by luck.
#[test]
fn abstract_classes_have_their_own_type_name() {
    for (id, type_name) in [
        ("Document", "4D.Document"),
        ("Directory", "4D.Directory"),
        ("Function", "4D.Function"),
        ("Transporter", "4D.Transporter"),
    ] {
        let card = model::class_card(id).unwrap_or_else(|| panic!("class {id} must resolve"));
        assert_eq!(card.type_name, type_name, "{id} has the wrong type name");
        assert_eq!(
            model::class_heading(&card.id, &card.type_name),
            format!("{type_name} (class {id})"),
            "wrong heading for abstract class {id}"
        );

        // The 4D.<Class> spelling must resolve to the same card...
        let by_type =
            model::class_card(type_name).unwrap_or_else(|| panic!("{type_name} must resolve"));
        assert_eq!(by_type.id, id, "{type_name} resolved to the wrong class");

        // ...while a fabricated name must not. A resolution test proves
        // nothing unless something in it is required to fail.
        let bogus = format!("{type_name}X");
        assert!(
            model::class_card(&bogus).is_none(),
            "{bogus} is not a real type and must not resolve"
        );
    }
}

/// Concrete class headings, for contrast — including `Collection`, whose type
/// name is bare rather than `4D.<id>`.
#[test]
fn concrete_class_headings() {
    for (id, want) in [
        ("File", "4D.File (class File)"),
        ("Folder", "4D.Folder (class Folder)"),
        ("Entity", "4D.Entity (class Entity)"),
        ("Collection", "Collection"),
    ] {
        let card = model::class_card(id).unwrap_or_else(|| panic!("class {id} must resolve"));
        assert_eq!(
            model::class_heading(&card.id, &card.type_name),
            want,
            "wrong heading for {id}"
        );
    }
}

/// Corpus-wide invariant: no class may be displayed under a type name that
/// belongs to a different class. Enforced as a test rather than handled in
/// the renderer, so a data regression names the offending class instead of
/// being quietly papered over.
#[test]
fn every_class_owns_its_type_name() {
    let idx = index::get();
    let borrowed: Vec<(&str, &str)> = idx
        .classes
        .iter()
        .filter(|(id, cl)| !model::owns_its_type_name(id, &cl.ir.type_name))
        .map(|(id, cl)| (id.as_str(), cl.ir.type_name.as_str()))
        .collect();
    assert!(
        borrowed.is_empty(),
        "these classes are recorded under a type name that is not theirs: {borrowed:?}"
    );
}

/// The 4 abstract bases are never instantiated directly. `isAbstract` records
/// the documented instantiation guidance — it says nothing about whether the
/// type may be *named*, which it may.
#[test]
fn abstract_classes_are_flagged() {
    for id in ["Directory", "Document", "Function", "Transporter"] {
        let card = model::class_card(id).unwrap_or_else(|| panic!("class {id} must resolve"));
        assert!(card.is_abstract, "{id} must be flagged abstract");
        assert!(!card.constructible_by_user_code);
        assert!(
            card.constructibility_note
                .is_some_and(|n| n.contains("concrete subclass")),
            "{id} must point the caller at its concrete subclasses"
        );
    }
}

/// The ORDA template classes are flagged, since their real per-project types
/// are generated from the datastore's structure.
#[test]
fn orda_template_classes_are_flagged() {
    for id in ["DataClass", "Entity", "EntitySelection"] {
        let card = model::class_card(id).unwrap_or_else(|| panic!("class {id} must resolve"));
        assert!(card.is_template, "{id} must be flagged as an ORDA template");
    }
}

// ---------------------------------------------------------------------------
// returns / members
// ---------------------------------------------------------------------------

#[test]
fn returns_reports_producers() {
    let r = model::returns_result("4D.FileHandle").expect("4D.FileHandle resolves");
    assert_eq!(r.class_id, "FileHandle");
    assert!(
        r.producers.iter().any(|p| p.producer == "File.open"),
        "File.open must be listed as producing a 4D.FileHandle, got {:?}",
        r.producers.iter().map(|p| &p.producer).collect::<Vec<_>>()
    );
    // File.open is both the `returns_instance_of` edge and the `obtained_via`
    // recipe; it must be merged into one row, not listed twice.
    let opens = r
        .producers
        .iter()
        .filter(|p| p.producer == "File.open")
        .count();
    assert_eq!(opens, 1, "File.open must appear exactly once");
    assert!(!r.how_to_obtain.is_empty());

    let e = model::returns_result("4D.Entity").expect("4D.Entity resolves");
    for expected in ["DataClass.new", "DataClass.get", "EntitySelection.first"] {
        assert!(
            e.producers.iter().any(|p| p.producer == expected),
            "{expected} must be listed as producing a 4D.Entity"
        );
    }
}

#[test]
fn members_listing_filters_by_kind() {
    let all = model::members_listing("Collection", None).expect("Collection resolves");
    let functions =
        model::members_listing("Collection", Some("oop_function")).expect("Collection resolves");
    let properties =
        model::members_listing("Collection", Some("oop_property")).expect("Collection resolves");
    assert!(all.count > 40, "Collection should have 40+ members");
    assert_eq!(all.count, {
        let constructors = model::members_listing("Collection", Some("oop_constructor"))
            .expect("Collection resolves");
        functions.count + properties.count + constructors.count
    });
    assert!(functions.members.iter().all(|m| m.kind == "oop_function"));
    assert!(properties.members.iter().all(|m| m.kind == "oop_property"));
}

// ---------------------------------------------------------------------------
// Multi-variant properties and dynamic pseudo-members
// ---------------------------------------------------------------------------

/// 9 properties have an *array* of TypeRefs. Rendering only the first variant
/// is a bug this project has already been bitten by once.
#[test]
fn multi_variant_properties_expose_every_variant() {
    let expected: &[(&str, &[&str])] = &[
        ("Email.bcc", &["Text", "Object", "Collection"]),
        ("Email.cc", &["Text", "Object", "Collection"]),
        ("Email.from", &["Text", "Object", "Collection"]),
        ("Email.replyTo", &["Text", "Object", "Collection"]),
        ("Email.sender", &["Text", "Object", "Collection"]),
        ("Email.to", &["Text", "Object", "Collection"]),
        ("Document.original", &["4D.File", "4D.Folder"]),
        ("SystemWorker.response", &["Text", "Blob"]),
        ("WebServer.characterSet", &["Number", "Text"]),
    ];
    for (id, variants) in expected {
        let card = model::member_card(id).unwrap_or_else(|| panic!("{id} must resolve"));
        assert!(card.multi_variant, "{id} must be flagged multi-variant");
        let got: Vec<&str> = card
            .accessor_types
            .iter()
            .filter_map(|t| t.get("name").and_then(|v| v.as_str()))
            .collect();
        assert_eq!(&got, variants, "{id}: wrong variant list");
    }
}

/// Exactly 9, corpus-wide: a tenth appearing (or one disappearing) means the
/// upstream IR changed and the rendering needs re-checking.
#[test]
fn there_are_exactly_nine_multi_variant_properties() {
    let idx = index::get();
    let multi: Vec<&str> = idx
        .members
        .values()
        .filter(|m| {
            m.ir.accessor
                .as_ref()
                .and_then(|a| a.get("type"))
                .is_some_and(|t| t.is_array())
        })
        .map(|m| m.ir.id.as_str())
        .collect();
    assert_eq!(
        multi.len(),
        9,
        "expected 9 multi-variant properties, got {multi:?}"
    );
}

/// The 7 dynamic pseudo-members are name *patterns* with no verbatim source
/// line. They must resolve and render without panicking.
#[test]
fn dynamic_pseudo_members_render() {
    let expected = [
        "4D.classClassName",
        "4D.classStoreName",
        "DataClass.attributeName",
        "DataStore.dataclassName",
        "Entity.attributeName",
        "EntitySelection.attributeName",
        "WebForm.componentName",
    ];
    let idx = index::get();
    let found: Vec<&str> = idx
        .members
        .values()
        .filter(|m| m.ir.dynamic_member.is_some())
        .map(|m| m.ir.id.as_str())
        .collect();
    assert_eq!(found, expected, "the dynamic pseudo-member set changed");

    for id in expected {
        let card = model::member_card(id).unwrap_or_else(|| panic!("{id} must resolve"));
        let dynamic = card
            .dynamic_member
            .as_ref()
            .unwrap_or_else(|| panic!("{id} must carry dynamicMember"));
        assert!(
            dynamic
                .get("namePattern")
                .and_then(|v| v.as_str())
                .is_some(),
            "{id} must carry a namePattern"
        );
        // No verbatim source line exists for these -- but a compiler-verified
        // synthetic example does, and it must still be served.
        assert!(
            card.example.primary.is_some(),
            "{id} must still have an example"
        );
    }
}

// ---------------------------------------------------------------------------
// Example precedence
// ---------------------------------------------------------------------------

/// All 502 members have a compiler-verified synthetic example, and 0 have
/// neither kind, so every result must carry a compiler-verified primary.
#[test]
fn every_member_has_a_compiler_verified_primary_example() {
    let idx = index::get();
    assert_eq!(
        idx.members.len(),
        502,
        "expected the full 502-member corpus"
    );
    for (id, record) in &idx.members {
        let example = model::build_example(record);
        assert!(example.available, "{id} has no example at all");
        let primary = example
            .primary
            .as_ref()
            .unwrap_or_else(|| panic!("{id} has no primary example"));
        assert!(
            primary.compiler_verified,
            "{id}'s primary example is not compiler-verified (provenance: {})",
            primary.provenance
        );
        assert!(!primary.raw.trim().is_empty(), "{id}'s example is empty");
    }
}

/// A synthetic example must always carry the placeholder warning, and a
/// not-compiler-verified documentation example must always carry its own.
#[test]
fn examples_carry_the_right_warnings() {
    let idx = index::get();
    for (id, record) in &idx.members {
        let example = model::build_example(record);
        for block in example.primary.iter().chain(example.alternates.iter()) {
            match block.source {
                model::ExampleSource::Synthetic => assert!(
                    block.placeholder_note.is_some(),
                    "{id}: synthetic example is missing placeholder_note"
                ),
                model::ExampleSource::Documentation => {
                    assert!(
                        block.placeholder_note.is_none(),
                        "{id}: documentation example must not claim synthetic placeholders"
                    );
                    if !block.compiler_verified {
                        assert!(
                            block.warning.is_some(),
                            "{id}: unverified documentation example is missing its warning"
                        );
                    }
                }
            }
        }
    }
}

/// A not-compiler-verified documentation example may never stand alone: the
/// compiler-verified synthetic example must always accompany it.
#[test]
fn unverified_doc_examples_never_stand_alone() {
    let idx = index::get();
    for (id, record) in &idx.members {
        let example = model::build_example(record);
        let has_unverified = example
            .primary
            .iter()
            .chain(example.alternates.iter())
            .any(|b| !b.compiler_verified);
        if !has_unverified {
            continue;
        }
        let has_verified_synth = example
            .primary
            .iter()
            .chain(example.alternates.iter())
            .any(|b| b.source == model::ExampleSource::Synthetic && b.compiler_verified);
        assert!(
            has_verified_synth,
            "{id} surfaces an unverified documentation example without the compiler-verified synthetic one"
        );
    }
}

/// The 4 members whose only documentation blocks are not 4D source at all
/// (JSON, query grammar, console output) must never surface them as code.
#[test]
fn non_4d_doc_blocks_are_never_surfaced() {
    let idx = index::get();
    for id in [
        "4D.HTTPRequest.new",
        "Email.keywords",
        "IMAPTransporter.searchMails",
        "SystemWorker.closeInput",
    ] {
        let record = idx
            .members
            .get(id)
            .unwrap_or_else(|| panic!("{id} must exist"));
        assert!(
            record.doc_examples.is_empty(),
            "{id} must have no usable documentation examples, got {:?}",
            record
                .doc_examples
                .iter()
                .map(|e| &e.example_id)
                .collect::<Vec<_>>()
        );
        let example = model::build_example(record);
        let primary = example.primary.as_ref().expect("synthetic example exists");
        assert_eq!(primary.source, model::ExampleSource::Synthetic);
    }
    // Corpus-wide: no `not 4D source` block reached the index at all.
    for record in idx.members.values() {
        assert!(
            record.doc_examples.iter().all(|e| !e.is_not_4d_source()),
            "a `not 4D source` block leaked into the served examples"
        );
    }
}

/// When a compiler-verified documentation example exists it leads, and the
/// synthetic one still follows as an alternate.
#[test]
fn verified_doc_examples_take_precedence() {
    let card = model::member_card("Collection.orderBy").expect("Collection.orderBy resolves");
    let primary = card.example.primary.as_ref().expect("has an example");
    assert_eq!(primary.source, model::ExampleSource::Documentation);
    assert!(primary.compiler_verified);
    assert!(
        card.example
            .alternates
            .iter()
            .any(|b| b.source == model::ExampleSource::Synthetic),
        "the synthetic example must still be offered as an alternate"
    );
}

// ---------------------------------------------------------------------------
// Corpus integrity
// ---------------------------------------------------------------------------

/// Guards against the embedded IR snapshot drifting out of sync with the
/// `ir.rs` structs: every member in the real, full corpus must deserialize.
#[test]
fn full_ir_corpus_deserializes() {
    let root = ir::parse_embedded();
    assert_eq!(root.commands.len(), 502);
    assert_eq!(root.classes.len(), 50);
    assert_eq!(root.enums.len(), 37);
    assert!(root.relationships.len() >= 674);

    let mut by_kind: HashMap<&str, usize> = HashMap::new();
    for m in &root.commands {
        assert!(!m.id.is_empty());
        assert!(!m.display_name.is_empty());
        assert!(!m.summary.is_empty());
        assert!(!m.receiver.class_id.is_empty());
        *by_kind.entry(m.kind.as_str()).or_insert(0) += 1;
    }
    assert_eq!(by_kind.get("oop_function"), Some(&258));
    assert_eq!(by_kind.get("oop_property"), Some(&225));
    assert_eq!(by_kind.get("oop_constructor"), Some(&19));
}

/// The synthetic corpus is joined to members by `member_id`, never by file
/// name (`SynthOOP_<id>` names are truncated to 31 chars with a hash suffix
/// and are therefore lossy). All 502 must resolve.
#[test]
fn every_member_has_a_synthetic_example_joined_by_id() {
    let idx = index::get();
    for (id, record) in &idx.members {
        assert!(
            record.synth_raw.is_some_and(|s| !s.trim().is_empty()),
            "{id} has no synthetic example text"
        );
        assert!(
            !record.synth_blocks.is_empty(),
            "{id} has no manifest block ranges"
        );
    }
}

/// Every non-dynamic member carries a verbatim source line from the 4D docs.
#[test]
fn non_dynamic_members_have_a_verbatim_source_line() {
    let idx = index::get();
    for (id, record) in &idx.members {
        if record.ir.dynamic_member.is_some() {
            continue;
        }
        let card = model::build_member_result(record, None);
        assert!(
            !card.raw_syntax.is_empty(),
            "{id} has no verbatim rawSyntax line"
        );
    }
}

/// camelCase splitting is what lets "order by" reach `.orderBy` and "entity
/// selection" reach `EntitySelection`, and it is the tokenization the vendored
/// corpus statistics (and therefore the IDF table) were computed under.
#[test]
fn tokenizer_splits_camel_case_humps() {
    assert_eq!(index::tokenize(".orderBy"), vec!["order", "by"]);
    assert_eq!(
        index::tokenize("EntitySelection"),
        vec!["entity", "selection"]
    );
    // The corpus tokenizer splits digit->upper too, so "4D." decomposes into
    // the single characters "4" and "D"; single-character tokens are dropped as
    // noise (every OOP class name is prefixed "4D.").
    assert_eq!(
        index::tokenize("4D.HTTPRequest.new"),
        vec!["http", "request", "new"]
    );
    assert_eq!(index::tokenize("searchMails"), vec!["search", "mails"]);
    // All-lowercase and acronym-only runs stay intact.
    assert_eq!(index::tokenize("imap"), vec!["imap"]);
    assert_eq!(index::tokenize("HTTP"), vec!["http"]);
}

/// `ln(N / df)` replaces a hand-written stopword list: a token in every member
/// scores exactly 0, a rare one scores high.
#[test]
fn idf_discounts_ubiquitous_tokens() {
    use fourd_language_oop::data;
    assert_eq!(data::idf("the"), Some(0.0), "df == N must give an IDF of 0");
    let function = data::idf("function").expect("'function' is in the corpus");
    let imap = data::idf("imap").expect("'imap' is in the corpus");
    assert!(
        imap > function,
        "a rare token ({imap}) must outweigh a ubiquitous one ({function})"
    );
    assert!(data::CORPUS_MEMBERS as usize == 502);
}

/// `docPage` is an official permalink, not a path into the docs mirror the
/// pipeline parsed. The mirror is not shipped, so a mirror path served here
/// would be a dead reference for every consumer.
///
/// The URL is deliberately version-less: a `/docs/21-R3/API/...` URL stops
/// resolving once 21-R3 is superseded, so this asserts the absence of a
/// version segment rather than merely asserting the prefix -- a pinned URL
/// would satisfy a prefix check while rotting on 4D's release schedule.
#[test]
fn doc_pages_are_version_less_official_permalinks() {
    const BASE: &str = "https://developer.4d.com/docs/API/";
    let idx = index::get();
    let mut seen = 0;

    for (id, record) in &idx.members {
        let card = model::build_member_result(record, None);
        let page = card
            .doc_page
            .unwrap_or_else(|| panic!("{id} has no docPage"));
        assert!(
            page.starts_with(BASE),
            "{id} docPage is not an official API permalink: {page}"
        );
        let slug = &page[BASE.len()..];
        assert!(
            !slug.contains('/'),
            "{id} docPage carries a version or extra path segment: {page}"
        );
        // Members deep-link to their own section; the anchor is what makes
        // the link land on the member rather than the top of a page listing
        // dozens of them.
        let (_, anchor) = slug
            .split_once('#')
            .unwrap_or_else(|| panic!("{id} docPage has no member anchor: {page}"));
        assert!(
            !anchor.is_empty(),
            "{id} docPage has an empty anchor: {page}"
        );
        assert!(
            !slug.ends_with(".html"),
            "{id} docPage still looks like a mirror file: {page}"
        );
        seen += 1;
    }
    assert_eq!(seen, 502, "expected every member to carry a docPage");

    // Classes agree with their members, and the one class with no API page
    // stays null rather than being given a fabricated URL.
    // Constructors anchor on the fully-qualified name, everything else on the
    // bare member name. Pinned explicitly because bare-name anchoring would
    // collapse all 19 constructors to `#new` and still look plausible.
    for (id, expected) in [
        (
            "Document.exists",
            "https://developer.4d.com/docs/API/Document#exists",
        ),
        (
            "Collection.orderBy",
            "https://developer.4d.com/docs/API/CollectionClass#orderby",
        ),
        (
            "4D.IMAPNotifier.new",
            "https://developer.4d.com/docs/API/IMAPNotifierClass#4dimapnotifiernew",
        ),
    ] {
        let record = idx
            .members
            .get(id)
            .unwrap_or_else(|| panic!("{id} missing"));
        let card = model::build_member_result(record, None);
        assert_eq!(card.doc_page.as_deref(), Some(expected), "{id}");
    }

    // Class cards stay page-level: the page is already the right target, so an
    // anchor here would be noise.
    let file = model::class_card("File").expect("File class");
    assert_eq!(
        file.doc_page.as_deref(),
        Some("https://developer.4d.com/docs/API/FileClass")
    );
    let transporter = model::class_card("Transporter").expect("Transporter class");
    assert_eq!(
        transporter.doc_page, None,
        "Transporter has no API page; a URL here would be invented"
    );

    // Doc examples carry their own docPage, and it was missed by the first
    // pass at this change -- the IR was rewritten while `example.primary`
    // still served a mirror path. Cover it here so the two cannot diverge
    // again, since it is the citation an agent is most likely to surface.
    let mut examples_seen = 0;
    for (id, record) in &idx.members {
        let card = model::build_member_result(record, None);
        let blocks = card
            .example
            .primary
            .iter()
            .chain(card.example.alternates.iter());
        for block in blocks {
            if let Some(page) = &block.doc_page {
                assert!(
                    page.starts_with(BASE) && !page.ends_with(".html"),
                    "{id} example cites a non-permalink docPage: {page}"
                );
                examples_seen += 1;
            }
        }
    }
    assert!(
        examples_seen > 400,
        "expected the doc-example corpus to be exercised, saw {examples_seen}"
    );

    // Control: the assertions above pass trivially if the predicate is weak,
    // so pin that the same predicate rejects the two shapes this test exists
    // to keep out -- a pinned-version URL and a mirror path.
    let rejects = |page: &str| {
        !page.starts_with(BASE) || page[BASE.len()..].contains('/') || page.ends_with(".html")
    };
    assert!(rejects("https://developer.4d.com/docs/21-R3/API/FileClass"));
    assert!(rejects("mirror/docs/21-R3/API/FileClass.html"));
    assert!(!rejects("https://developer.4d.com/docs/API/FileClass"));
}
