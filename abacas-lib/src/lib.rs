#![doc = include_str!("../../README.md")]
#![warn(missing_docs)]

pub mod error;
pub mod monomial;
pub mod polynomial;
pub mod expr;

/// The library version currently in use.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
