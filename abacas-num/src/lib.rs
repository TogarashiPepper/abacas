#![doc = include_str!("../README.md")]
#![warn(missing_docs, clippy::missing_docs_in_private_items)]

pub mod integer;
pub mod list;
pub mod natural;
pub mod ops;
pub mod rational;
pub mod sign;

/// The library version currently in use.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
