//! Collection of error types used across the library.

use std::{error, fmt};

use crate::number::Number;

/// An error that can occur while parsing.
#[derive(Debug)]
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

/// An error that can occur while simplifying an expression.
#[derive(Debug)]
pub enum SimplifyError {
	/// The expression tried to divide by zero.
	DivisionByZero,
}

impl fmt::Display for SimplifyError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::DivisionByZero => write!(f, "division by zero"),
		}
	}
}

impl error::Error for SimplifyError {}
