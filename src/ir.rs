//! Typed envelope over the embedded 4D OOP IR, the doc-example corpus and the
//! synthetic-example manifest.
//!
//! Only the fields needed to build the search index and render results are
//! given concrete types. Everything structurally rich — `overloads` (params,
//! union types, returns), `accessor.type` (which is a *list* of TypeRefs for
//! the 9 multi-variant properties), `returnsShape`, `instantiation.recipes` —
//! is kept as raw `serde_json::Value` and echoed back verbatim in responses.
//! This is the same deliberate choice classic makes: the whole point of this
//! service is to hand back *exactly* what the IR says, not a lossy
//! reinterpretation of it. Unknown fields are silently ignored by serde, which
//! keeps this module resilient to additive schema changes upstream.

use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

/// Root shape of `data/vendor/4d-oop-ir.json`.
#[derive(Debug, Deserialize)]
pub struct OopIrRoot {
    /// One entry per member (function, property or constructor).
    pub commands: Vec<MemberIr>,
    /// Class id -> class card.
    pub classes: HashMap<String, ClassIr>,
    /// Enum id -> constant list.
    #[serde(default)]
    pub enums: HashMap<String, Value>,
    /// Layer-3 edges: `member_of`, `returns_instance_of`, `obtained_via`,
    /// `same_mechanism_family`.
    #[serde(default)]
    pub relationships: Vec<Relationship>,
}

/// One entry from the IR's top-level `commands` list. Despite the key name
/// (kept identical to classic's IR for schema symmetry) these are *members*,
/// not standalone commands.
#[derive(Debug, Deserialize, Clone)]
pub struct MemberIr {
    /// e.g. `Collection.orderBy`, `4D.File.new`, `Document.exists`.
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// `oop_function` | `oop_property` | `oop_constructor`.
    pub kind: String,
    /// e.g. `.orderBy`, `.new`.
    #[serde(rename = "memberName")]
    pub member_name: String,
    pub receiver: Receiver,
    pub summary: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(rename = "docPage", default)]
    pub doc_page: Option<String>,
    #[serde(rename = "sinceVersion", default)]
    pub since_version: Option<String>,
    /// Present for callables (`oop_function`/`oop_constructor`). Raw, echoed
    /// verbatim — each overload carries a `rawSyntax` line that is
    /// byte-identical to the 4D docs.
    #[serde(default)]
    pub overloads: Vec<Value>,
    /// Present for `oop_property`. `accessor.type` may be a single TypeRef
    /// **or an array** of them (the 9 multi-variant properties); kept raw so
    /// both render, and so a caller can never see only the first variant.
    #[serde(default)]
    pub accessor: Option<Value>,
    #[serde(rename = "returnsShape", default)]
    pub returns_shape: Option<Value>,
    #[serde(rename = "errorModel", default)]
    pub error_model: Option<Value>,
    #[serde(rename = "versionBehaviorChanges", default)]
    pub version_behavior_changes: Vec<Value>,
    /// Present for the 7 dynamic pseudo-members (`.attributeName` and
    /// friends). These have no fixed name and no verbatim source line.
    #[serde(rename = "dynamicMember", default)]
    pub dynamic_member: Option<Value>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Receiver {
    #[serde(rename = "classId")]
    pub class_id: String,
    /// `instance` or `class`.
    pub kind: String,
    #[serde(rename = "typeRef", default)]
    pub type_ref: Option<Value>,
}

/// One entry from the IR's top-level `classes` object.
#[derive(Debug, Deserialize, Clone)]
pub struct ClassIr {
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    /// The name you write in a 4D type declaration, e.g. `4D.File`,
    /// `Collection`, `4D.Entity`. Not always `4D.<id>`.
    #[serde(rename = "typeName")]
    pub type_name: String,
    #[serde(rename = "docPage", default)]
    pub doc_page: Option<String>,
    /// How to obtain an instance. Complete for all 50 classes — this is the
    /// single most valuable field on the class card.
    #[serde(default)]
    pub instantiation: Option<Value>,
    /// Ids of the members declared *on this class*. Inherited members are
    /// modelled once, on the declaring class, so this list does not repeat
    /// them; walk `superclass` to reach those.
    #[serde(default)]
    pub members: Vec<String>,
    #[serde(default)]
    pub superclass: Option<String>,
    /// Member id of the class's constructor, when it has one.
    #[serde(default)]
    pub constructor: Option<String>,
    #[serde(rename = "isAbstract", default)]
    pub is_abstract: bool,
    #[serde(rename = "isSingleton", default)]
    pub is_singleton: bool,
    #[serde(rename = "isStatic", default)]
    pub is_static: bool,
    /// ORDA template classes (`Entity`, `EntitySelection`, ...) — present as
    /// an empty object in the IR when true, absent otherwise.
    #[serde(rename = "isTemplate", default)]
    pub is_template: Option<Value>,
    #[serde(rename = "sharedSemantics", default)]
    pub shared_semantics: Option<Value>,
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Relationship {
    #[serde(rename = "type")]
    pub kind: String,
    pub from: RelEndpoint,
    pub to: RelEndpoint,
    #[serde(default)]
    pub note: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct RelEndpoint {
    #[serde(default)]
    pub command: Option<String>,
    #[serde(rename = "classId", default)]
    pub class_id: Option<String>,
    /// `classic` when the endpoint refers to a classic-language command that
    /// lives in the *other* corpus (`4d-language-classic`), absent otherwise.
    #[serde(default)]
    pub corpus: Option<String>,
}

// ---------------------------------------------------------------------------
// Documentation examples
// ---------------------------------------------------------------------------

/// Root shape of `data/vendor/oop_doc_examples.json`.
#[derive(Debug, Deserialize)]
pub struct DocExamplesRoot {
    pub examples: Vec<DocExample>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DocExample {
    #[serde(rename = "exampleId")]
    pub example_id: String,
    #[serde(rename = "memberId")]
    pub member_id: String,
    #[serde(rename = "docPage", default)]
    pub doc_page: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub caption: Option<String>,
    /// Fence language as published: `4d`, `text`, `json`, ...
    #[serde(default)]
    pub language: Option<String>,
    pub code: String,
    /// One of:
    ///   "documentation example, compiler-verified"
    ///   "documentation example, not compiler-verified"
    ///   "documentation example, not 4D source"
    pub provenance: String,
    #[serde(rename = "declaringClassId", default)]
    pub declaring_class_id: Option<String>,
    #[serde(default)]
    pub inherited: bool,
    #[serde(rename = "compilerVerification", default)]
    pub compiler_verification: Option<DocExampleVerification>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DocExampleVerification {
    /// 1 = clean under `tool4d check-syntax`; 3 = failed.
    #[serde(default)]
    pub bucket: Option<u32>,
    /// True when the snippet only compiled after being mechanically hosted in
    /// a class file (a class-body fragment). Still compiler-verified, but one
    /// tier down because the caller has to reproduce that hosting.
    #[serde(rename = "hostedAsClass", default)]
    pub hosted_as_class: Option<bool>,
}

impl DocExample {
    /// A block whose fence claimed 4D but whose content is JSON, a query
    /// grammar, or console output. These must never be surfaced as code.
    pub fn is_not_4d_source(&self) -> bool {
        self.provenance.contains("not 4D source")
    }

    pub fn is_compiler_verified(&self) -> bool {
        self.compiler_verification
            .as_ref()
            .and_then(|v| v.bucket)
            .is_some_and(|b| b == 1)
    }

    pub fn is_wrapped(&self) -> bool {
        self.compiler_verification
            .as_ref()
            .and_then(|v| v.hosted_as_class)
            .unwrap_or(false)
    }
}

// ---------------------------------------------------------------------------
// Synthetic example manifest
// ---------------------------------------------------------------------------

/// Root shape of `data/vendor/oop_synth_manifest.json`.
#[derive(Debug, Deserialize)]
pub struct SynthManifestRoot {
    /// Relative source path -> entry. **Join on `member_id`, never on the
    /// file name**: `SynthOOP_<id>` names replace `.`/`:` and are truncated to
    /// 4D's 31-character method-name limit with a hash suffix, so they are
    /// lossy and not reversible.
    pub files: HashMap<String, SynthFileEntry>,
    #[serde(rename = "target_ids", default)]
    pub target_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SynthFileEntry {
    pub member_id: String,
    #[serde(default)]
    pub class_id: Option<String>,
    #[serde(default)]
    pub blocks: Vec<SynthBlock>,
}

/// One synthesized block inside a `SynthOOP_*.4dm` file. `comment_line` is
/// 1-based and points at the block's leading `// ...` comment; `lines` counts
/// the block including that comment.
#[derive(Debug, Deserialize, Clone, serde::Serialize)]
pub struct SynthBlock {
    #[serde(rename = "overload_index", default)]
    pub overload_index: Option<i64>,
    #[serde(default)]
    pub variant: Option<String>,
    #[serde(rename = "comment_line")]
    pub comment_line: usize,
    pub lines: usize,
}

// ---------------------------------------------------------------------------
// Parsing
// ---------------------------------------------------------------------------

/// Parses the embedded IR JSON. Panics on failure — the embedded snapshot is
/// a build-time invariant, not user input, so a parse failure here means the
/// vendored data or this module's structs are out of sync and the binary
/// should not silently serve broken results.
pub fn parse_embedded() -> OopIrRoot {
    serde_json::from_str(crate::data::IR_JSON)
        .expect("embedded data/vendor/4d-oop-ir.json failed to parse against ir.rs structs")
}

pub fn parse_embedded_doc_examples() -> DocExamplesRoot {
    serde_json::from_str(crate::data::DOC_EXAMPLES_JSON)
        .expect("embedded data/vendor/oop_doc_examples.json failed to parse against ir.rs structs")
}

pub fn parse_embedded_synth_manifest() -> SynthManifestRoot {
    serde_json::from_str(crate::data::SYNTH_MANIFEST_JSON).expect(
        "embedded data/vendor/oop_synth_manifest.json failed to parse against ir.rs structs",
    )
}
