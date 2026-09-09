# 4d-language-oop

A deterministic, offline, natural-language lookup/reference service for the
**4D object (OOP) language** class reference — built for code agents that
aren't fluent in 4D. It is the sibling of
[`miyako/4d-language-classic`](https://github.com/miyako/4d-language-classic),
which covers the classic command set; the two share a structure, CLI surface
and response shape deliberately, so an agent only has to learn one interface.

Ask something like *"read a text file line by line"* or *"sort an entity
selection"* and get back:

1. the matched class member(s) — and, where the query is really about a
   class, that **class's card**,
2. their real syntax — the verbatim source line from the 4D docs, plus the
   full typed overload / accessor objects from a schema-validated IR,
3. **how to obtain an instance** of the receiver class, which is the single
   thing agents most often get wrong about 4D OOP, and
4. a compiler-verified example.

This is a lookup index over generated data, not a language model: matching is
pure deterministic keyword/alias/IDF scoring (see [How matching
works](#how-matching-works)), and the entire IR + example corpus is embedded
into the binary at compile time — the running program makes **zero** runtime
file or network accesses.

## Source data

All syntax rules and examples come from
[`miyako/4d-static-docs`](https://github.com/miyako/4d-static-docs), which
extracts, schema-validates, and compiler-verifies (via `tool4d`) a full IR of
the 4D object-language class reference. This repo vendors a **pinned
snapshot** of five of its generated artifacts under `data/vendor/`:

| File | Description |
|---|---|
| `4d-oop-ir.json` | The root IR: 502 members (258 functions, 225 properties, 19 constructors) across 50 classes, plus 37 enums and 685 relationships (`member_of`, `returns_instance_of`, `obtained_via`, `same_mechanism_family`). Each class carries `instantiation`, `superclass`, `isAbstract`/`isTemplate`/`isSingleton` and `sharedSemantics`. |
| `oop_doc_examples.json` | 473 hand-written documentation examples harvested from the 4D docs, each attributed to a member and carrying its own provenance + `tool4d` verification verdict. |
| `oop_synth_manifest.json` | Maps each **member id** to its generated example file and which overload/variant blocks it contains, with per-block line ranges. |
| `methods/SynthOOP_<Name>.4dm` | 502 files — one per member — each a real 4D method body compiled and verified error-free by `tool4d`. |
| `oop_corpus_stats.json` | Token document-frequencies over the corpus, used to build the IDF table at compile time. |

See `data/vendor/COMMIT` for exactly which `4d-static-docs` commit is
currently pinned.

**Join on `member_id`, never on filename.** The `SynthOOP_*` file names have
`.`/`:` replaced and are truncated to 4D's 31-character method-name limit
with a hash suffix (`SynthOOP_WebSocketServer_t_f011.4dm`), so they are
lossy. The manifest's `member_id` is the authoritative key, and a build-time
assertion in `build.rs` fails the build if any of the 502 fails to resolve.

### What the corpus guarantees

- **Every member has a compiler-verified example.** 218 members have a
  documentation example (141 of those compiler-verified), 284 are
  synthetic-only, and **0 have neither** — all 502 have a compiler-verified
  synthetic example. So a not-compiler-verified documentation example never
  has to stand alone.
- **Inherited members are modelled once, on the declaring class.** `.exists`
  lives on `Document`, not `File`. Lookups resolve through `superclass` (at
  most 2 levels), so `member File.exists` works and reports where the member
  is actually declared.
- **Every non-dynamic member carries a verbatim source line** that is
  byte-identical to the 4D docs. The tool prefers rendering that over
  reconstructing a signature.

## Build & run

Requires a stable Rust toolchain (tested with 1.91).

```sh
cargo build --release
./target/release/4d-language-oop query "read a text file line by line"
```

The resulting binary (`target/release/4d-language-oop`, ~3.1 MB stripped) is
fully self-contained — it can be copied anywhere and run with no other files
present.

## CLI modes

### `query` — natural-language lookup

```sh
4d-language-oop query "<natural language query>" [--limit N] [--json]
```

- Default output is human-readable text (ranked matches with their class,
  one-line summary, verbatim syntax, and the example).
- `--limit N` — how many ranked matches to return (default 5). Ranking is a
  pure function of the query; `--limit` only truncates, so raising it can
  only append rows, never reorder them.
- `--json` — print the same JSON shape the HTTP server returns.

Results are a mix of **member rows** and **class cards**; see
[class-flooding control](#class-flooding-control).

### `class` — class card

```sh
4d-language-oop class 4D.File [--json]
```

Prints the class's type name, superclass, **how to obtain an instance**, its
full member list (declared and inherited, attributed), and whether it can be
constructed by user code at all. Accepts the class id (`File`), the 4D type
name (`4D.File`), or the display name.

### `member` — one member

```sh
4d-language-oop member Collection.orderBy [--json]
```

Accepts `<Class>.<member>` using either the class id or its 4D type name, and
resolves inherited members: `member File.exists` returns `Document.exists`,
annotated with `inheritedFrom` and `requestedAs`. A bare member name works
too where it is unambiguous corpus-wide.

### `members` — list a class's members

```sh
4d-language-oop members --class Entity [--kind function|property|constructor] [--json]
```

### `returns` — what produces an instance of a class

```sh
4d-language-oop returns 4D.FileHandle [--json]
```

Inverts the `returns_instance_of` / `obtained_via` relationships to answer
"where does a `4D.FileHandle` come from?". Producers that are classic-language
commands rather than OOP members are tagged `corpus: classic`.

### `serve` — HTTP mode

```sh
4d-language-oop serve [--port 8080]
```

- `GET /lookup?q=<query>&limit=<n>` → JSON array of ranked results (same
  shape as `query --json`).
- `GET /class?name=<class>` → class card.
- `GET /member?name=<Class.member>` → member card.
- `GET /members?class=<class>&kind=<kind>` → member listing.
- `GET /returns?class=<class>` → producers.
- `GET /health` → `200 OK` plain text.

```sh
$ 4d-language-oop serve --port 8080 &
$ curl "http://localhost:8080/lookup?q=sort+an+entity+selection&limit=3"
```

## Response shape

Every result carries a `resultType` discriminator, `"member"` or `"class"`.

A **member** result includes the member's id/class/kind, its match score, the
verbatim `rawSyntax` line(s) from the docs, the IR's typed `overloads` (for
callables) or `accessor` (for properties) **verbatim** — this is the
authoritative syntax contract — plus `sinceVersion`, `errorModel`,
`versionBehaviorChanges`, `returnsShape` where present, and an `example`
block:

```json
{
  "resultType": "member",
  "id": "Collection.orderBy",
  "classId": "Collection",
  "classTypeName": "Collection",
  "receiverKind": "instance",
  "kind": "oop_function",
  "memberName": ".orderBy",
  "score": 26.46,
  "summary": "Returns a new collection containing all elements of the collection in the specified order",
  "rawSyntax": ["**.orderBy**() : Collection", "..."],
  "overloads": [ /* ...raw IR overload objects... */ ],
  "example": {
    "available": true,
    "primary": {
      "source": "documentation",
      "provenance": "documentation example, compiler-verified",
      "compilerVerified": true,
      "raw": " var $c; $c2 : Collection\n $c:=New collection\n ...",
      "docPage": "https://developer.4d.com/docs/API/CollectionClass",
      "title": "Example 1",
      "caption": "Ordering a collection of numbers ..."
    },
    "alternates": [ /* ...further examples, same shape... */ ]
  }
}
```

Every `docPage` -- on a member, on a class card, and on each example block --
is an official documentation permalink, and text mode prints it as a `docs:`
line. It is deliberately **version-less**: `https://developer.4d.com/docs/API/FileClass`
rather than `.../docs/21-R3/API/FileClass`, because a version-pinned URL stops
resolving once that release is superseded and would rot this binary on 4D's
release schedule. `4D.Transporter` has no API page and so carries none.

A **class** result carries `instantiation` (the raw IR recipes), a rendered
`howToObtain`, `constructibleByUserCode` + `constructibilityNote`,
`superclass`, `isAbstract`/`isTemplate`/`isSingleton`, `sharedSemantics`,
`memberCounts` and the attributed `members` list.

### Example selection precedence

Per member, in strict order:

1. `documentation example, compiler-verified`
2. documentation example wrapped in a host class and then compiler-verified
3. the compiler-verified **synthetic** example (all 502 members have one)
4. `documentation example, not compiler-verified` — only ever **alongside**
   the synthetic one, never instead of it, and clearly labelled

So `example.primary` is *always* compiler-verified. `provenance` is always
emitted.

**Documentation blocks that are not 4D source at all** (JSON payloads, query
grammar, console output) are dropped at index-build time and can never be
surfaced as code. Four members — `4D.HTTPRequest.new`, `Email.keywords`,
`IMAPTransporter.searchMails`, `SystemWorker.closeInput` — have *only* such
blocks, and fall through to their synthetic example.

**Synthetic examples carry a `placeholder_note`.** A synthetic body is
generated to exercise every parameter combination for `tool4d`'s compiler
check, not to be idiomatic. Its literal-looking tokens (`$result1`,
`$options3`, `[SynthTable]`, `SynthRelated`, `cs.SynthOOPHandler`,
`SynthOOPCallback`) are auto-generated placeholders — callers, including code
agents, must substitute their own receiver, values and names. This warning is
a hard-won lesson from the classic CLI, where agents copied placeholder
tokens verbatim; it is surfaced both as `example.primary.placeholder_note`
(machine-readable) and inline in the CLI's text output. Unverified
documentation examples carry their own `warning` field.

### Multi-variant properties

Nine properties declare an **array** of types rather than one — `Email.bcc`,
`.cc`, `.from`, `.replyTo`, `.sender`, `.to` are each `Text | Object |
Collection`; `Document.original` is `4D.File | 4D.Folder`;
`SystemWorker.response` is `Text | Blob`; `WebServer.characterSet` is `Number
| Text`. `accessorTypes` is **always** a list, and `multiVariant` is `true`
for these, so no variant can be silently dropped. Text output labels them
`MULTI-VARIANT -- all N alternatives are valid`.

### Dynamic pseudo-members

Seven members are name *patterns*, not fixed names —
`Entity.attributeName`, `DataClass.attributeName`,
`EntitySelection.attributeName`, `DataStore.dataclassName`,
`4D.classClassName`, `4D.classStoreName`, `WebForm.componentName`. They have
no verbatim source line, so `rawSyntax` is empty; instead they carry the raw
`dynamicMember` object with its `namePattern` and note, and are rendered from
that. They still have compiler-verified synthetic examples.

## How matching works

Fully deterministic — no embeddings, no ML, no external API calls, so the
same query always returns the same result on any machine:

1. **Tokenize** the query: split on non-alphanumeric runs, **then split
   camelCase humps**, lowercase, drop single characters. This is the
   tokenization the vendored corpus statistics were computed under, and it is
   what lets "entity selection" reach `EntitySelection` and "order by" reach
   `.orderBy`.
2. **Alias-expand** each token via a small hand-curated table
   (`data/aliases.json`) mapping colloquial terms to the vocabulary actually
   used in the IR (e.g. `"folder"` → also considers `"directory"`, `"save"` →
   also considers `"store"`/`"commit"`). Values are post-camel-split
   lowercase tokens. Extend this file as retrieval gaps are found — it is
   deliberately small and reviewable, not a generated model.
3. **Score** each candidate member as the sum, over matched tokens, of
   `field_weight × IDF(token)`:

   | Field | Weight |
   |---|---|
   | member name (`.orderBy`, `.exists`) | 3.0 |
   | class id (`Collection`, `EntitySelection`) | 2.5 |
   | summary | 1.0 |
   | param names | 0.6 |
   | constraint prose (`errorModel`, `versionBehaviorChanges`, `dynamicMember` notes) | 0.4 |

   Class name and member name are **both** strong signals in OOP — the main
   departure from classic, where the command id carried nearly everything.

4. **Rank** by `(score desc, class-card-before-member, id asc)`. The id
   tiebreak makes ordering fully reproducible even between members with an
   identical score.

### Why there is no stopword list

`IDF(token) = ln(N / df)` with `N = 502` and `df` from the corpus stats,
computed once at index-build time. Tokens with `idf <= 0` are dropped
entirely, which is exactly what a stopword list would do — `the` has `df =
502`, so its IDF is precisely `0.0`. IDF also handles the *domain* skew that
a hand-written list could never guess: `function` has `df = 293`, because the
docs phrase every summary as "The .x() function ...", while `imap` has `df =
36`.

More importantly, a classic-style stopword list would eat `at`, `by` and
`get` — which are real member-name tokens (`Collection.at`, `.orderBy`,
`Entity.getKey`). IDF discounts them without deleting them.

### Abstract classes are still nameable types

The four abstract bases — `Document`, `Directory`, `Function`, `Transporter`
— are flagged `isAbstract`, and their cards say so and point at the concrete
subclasses. That flag records the documented **instantiation** guidance; it
says nothing about whether the type may be *named*, which it may:
`var $x : 4D.Document` compiles, and every abstract class resolves under both
its bare id and its `4D.<Class>` form.

This was wrong in the upstream IR until `4d-static-docs` `fb0a69ac`, which
this snapshot postdates. `Document.typeName` was recorded as `4D.File` and
`Directory.typeName` as `4D.Folder` — the type of one of their *two* concrete
subclasses, which was both arbitrary (why `File` over `ZipFile`?) and false.
Rendering that would have told an agent that `Document` **is** `4D.File`.

The renderer now trusts the data, and a corpus-wide test
(`every_class_owns_its_type_name`) enforces the invariant that no class is
displayed under a type name belonging to another: a regression fails a test
naming the offending class rather than being quietly papered over.

```
$ 4d-language-oop class Document
4D.Document (class Document)
flags: abstract
subclasses: File, ZipFile
HOW TO OBTAIN AN INSTANCE:
  NOT CONSTRUCTIBLE BY USER CODE.
  This is an abstract base class: it is never instantiated directly. Obtain
  an instance of one of its concrete subclasses instead.
```

### Class-flooding control

`Collection` has 47 members, `WebServer` 41, `EntitySelection` 34. A naive
query for "web server" returns 41 near-identical rows and buries everything
else. Two rules prevent that, evaluated over a **fixed-size candidate pool**
(60) so that decisions never depend on `--limit`:

- **Cap**: any single class contributes at most **3 member rows** to the
  results.
- **Collapse**: when a query matches a class name exactly, or when 5+ members
  of one class would otherwise match, that class's **class card** is emitted
  as a result too. A card named outright by the query is ranked first, since
  the query was really asking about the class.

> **Deviation from the brief, deliberate — do not "fix" this back.** The
> specification says the class card is emitted "instead of" flooding with
> members. Emitting the card *in addition to* the capped member rows was
> measurably better: with pure replacement, "order a collection" returned the
> `Collection` card and hid `Collection.orderBy` entirely, which is the actual
> answer. Composing the two rules also keeps the 3-row cap from being dead
> code for exactly the classes that need it — under pure replacement the cap
> could never fire, leaving two rules that cannot both be live. `query "order
> a collection"` now returns the card followed by `.multiSort`, `.orderBy`,
> `.orderByMethod`. Reviewed and accepted by the pipeline owner.

This intentionally stays scoped to **single-member lookups** — no multi-step
task chaining/composition in this version.

## Refreshing the pinned snapshot

When `4d-static-docs`'s IR is updated upstream, re-vendor a specific commit
with the documented script:

```sh
scripts/refresh-vendor.sh <commit-sha> [path-to-local-4d-static-docs-clone]
```

This overwrites `data/vendor/{4d-oop-ir.json,oop_doc_examples.json,oop_synth_manifest.json,oop_corpus_stats.json,methods/*.4dm}`,
rewrites `data/vendor/COMMIT` with the new pin + fetch timestamp, and prints a
summary of members and classes added/removed versus the previous snapshot.
Always pin to an explicit commit sha (never "latest") so the embedded snapshot
is reproducible. After refreshing:

```sh
cargo test    # corpus-integrity + lookup regression tests
```

review the diff, then commit `data/vendor/` together with the updated
`data/vendor/COMMIT`.

The regression suite is deliberately frozen against measured expectations, so
a ranking shift always shows up as a test diff rather than silently changing
what agents are told.

## Releases

`.github/workflows/release.yml` builds a matrix of macOS arm64/x64, Linux
arm64/x64 (static musl) and Windows arm64/x64, and publishes one `.tar.xz`
per platform named:

```
4d-language-oop-macos-arm64.tar.xz
4d-language-oop-macos-x64.tar.xz
4d-language-oop-linux-arm64.tar.xz
4d-language-oop-linux-x64.tar.xz
4d-language-oop-windows-arm64.tar.xz
4d-language-oop-windows-x64.tar.xz
```

This matches the convention `miyako/skills` already uses for
`4d-language-classic`, whose `4dtools` provisioner resolves binaries by that
exact name — do not change it without updating the skills repo in the same
change. Each job also runs the binary from an empty directory to prove it is
genuinely self-contained.

## Project layout

```
Cargo.toml
build.rs                  # embeds data/vendor/methods/*.4dm and builds the IDF
                          # table from oop_corpus_stats.json at compile time
data/
  vendor/                 # pinned 4d-static-docs snapshot (see COMMIT)
  aliases.json            # hand-curated query-expansion table
scripts/
  refresh-vendor.sh       # re-pin to a new 4d-static-docs commit
src/
  lib.rs                  # module wiring (also exposed for tests/)
  main.rs                 # CLI arg dispatch (query | class | member | members |
                          #                    returns | serve)
  ir.rs                   # typed envelope over the embedded IR JSON
  data.rs                 # include_str!/include! of embedded data
  index.rs                # tokenizer, inverted index, IDF scoring,
                          # class/member resolution, flooding control
  model.rs                # response DTOs shared by CLI --json and HTTP
  cli.rs                  # one-shot subcommands and text rendering
  server.rs               # `serve` subcommand (tiny_http)
tests/
  lookup_regression.rs    # frozen query -> expected result, plus structural
                          # guards on flooding, inheritance, multi-variant
                          # properties, dynamic members and example precedence
```

## Dependencies

Kept deliberately minimal — no async runtime, no ML/embedding libraries:

| Crate | Why |
|---|---|
| `serde` / `serde_json` | Parse the embedded IR/examples/manifest JSON into typed structs at startup. |
| `tiny_http` | Minimal blocking HTTP server for `serve` mode — no `tokio`/`axum`. |

CLI argument parsing, HTTP query-string decoding, and tokenizing are all
hand-rolled to avoid pulling in `clap`/`url`/`regex`.
