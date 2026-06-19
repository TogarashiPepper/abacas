//! Collection of error types used across the library.

use std::{error, fmt};

use crate::number::Number;

/// An error that can occur while parsing.
#[derive(Clone, Debug)]
pub enum ParseError {
	/// The parser encountered an invalid number.
	InvalidNumber(Number),
	/// The parser encountered an invalid string.
	InvalidString(String),
}

impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::InvalidNumber(number) => write!(f, "invalid number: {number}"),
			Self::InvalidString(string) => write!(f, "invalid string: {string}"),
		}
	}
}

impl error::Error for ParseError {}
