#![doc = include_str!("../../README.md")]
#![warn(missing_docs)]

pub mod context;
pub mod error;
pub mod expr;
pub mod function;
pub mod monomial;
pub mod number;
pub mod polynomial;
pub mod standardlibrary;

/// The library version currently in use.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
