//! Collection of error types used across the library.

use std::error::Error;
use std::fmt;
use std::num::{ParseFloatError, ParseIntError};

use rug::rational::ParseRationalError;

use crate::number::Number;

/// An error that can occur while parsing.
#[derive(Clone, Debug)]
pub enum ParseError {
	/// The parser encountered an invalid value.
	InvalidValue(Number),
	/// A wrapper around a [`ParseFloatError`].
	ParseFloat(ParseFloatError),
	/// A wrapper around a [`ParseIntError`].
	ParseInt(ParseIntError),
	/// A wrapper around a [`ParseRationalError`].
	ParseRational(ParseRationalError),
}

impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::InvalidValue(value) => write!(f, "invalid value: {value}"),
			Self::ParseFloat(error) => error.fmt(f),
			Self::ParseInt(error) => error.fmt(f),
			Self::ParseRational(error) => error.fmt(f),
		}
	}
}

impl Error for ParseError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			Self::InvalidValue(_) => None,
			Self::ParseFloat(error) => Some(error),
			Self::ParseInt(error) => Some(error),
			Self::ParseRational(error) => Some(error),
		}
	}
}

impl From<ParseFloatError> for ParseError {
	fn from(value: ParseFloatError) -> Self {
		Self::ParseFloat(value)
	}
}

impl From<ParseIntError> for ParseError {
	fn from(value: ParseIntError) -> Self {
		Self::ParseInt(value)
	}
}

impl From<ParseRationalError> for ParseError {
	fn from(value: ParseRationalError) -> Self {
		Self::ParseRational(value)
	}
}
