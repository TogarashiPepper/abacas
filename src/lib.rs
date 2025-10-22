#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

pub mod error;

mod monomial;
mod number;
mod polynomial;

pub use monomial::*;
pub use number::*;
pub use polynomial::*;
