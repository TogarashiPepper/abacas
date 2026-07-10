#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

pub mod digit;
pub mod integer;
pub mod natural;
pub mod rational;
pub mod real;
pub mod symbol;

/// The library version currently in use.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
