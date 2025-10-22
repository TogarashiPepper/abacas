#![doc = include_str!("../../README.md")]
#![warn(missing_docs)]

/// The library version currently in use.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod error;
pub mod monomial;
pub mod number;
pub mod polynomial;
