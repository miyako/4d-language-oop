//! Library surface for `4d-language-oop`, split out so integration tests (in
//! `tests/`) can exercise the index/model/tokenizer directly, and so the thin
//! `main.rs` binary is just an argument-dispatch shim over this crate.

pub mod cli;
pub mod data;
pub mod index;
pub mod ir;
pub mod model;
pub mod server;
