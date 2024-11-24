#![forbid(unsafe_code)]

mod parser;
mod query;
mod versioned;

pub use {
    parser::load_rustdoc,
    versioned::{VersionedStorage, VersionedHandler, VersionedRustdocAdapter},
};
