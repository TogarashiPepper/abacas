//! Collection of error types used across the library.

use std::error::Error;
use std::fmt;
use std::num::{ParseFloatError, ParseIntError};

use crate::number::Number;

/// An error which can be returned when parsing a number.
///
/// This error is used as the error type for the [`FromStr`] implementation
/// for [`Number`].
///
/// # Example
///
/// ```
/// use std::str::FromStr;
/// use abacas::number::Number;
///
/// if let Err(e) = Number::from_str("1.2.3") {
///     println!("Failed conversion to Number: {e}");
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseNumberError {
	pub(super) kind: NumberErrorKind,
}

/// Enum to store the various types of errors that can cause parsing a [`Number`] to fail.
///
/// # Example
///
/// ```
/// use abacas::number::Number;
/// use std::str::FromStr;
///
/// # fn main() {
/// if let Err(e) = Number::from_str("1.2.3") {
///     println!("Failed conversion to Number: {:?}", e.kind());
/// }
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NumberErrorKind {
	/// Value being parsed is empty.
	///
	/// This variant will be constructed when parsing an empty string.
	Empty,
	/// Contains an invalid digit in its context.
	///
	/// Among other causes, this variant will be constructed when parsing a string that
	/// contains a non-ASCII char.
	///
	/// This variant is also constructed when a `+` or `-` is misplaced within a string
	/// either on its own or in the middle of a number.
	Invalid,
}

impl ParseNumberError {
	/// Outputs the detailed cause of parsing a [`Number`] failing.
	pub const fn kind(&self) -> &NumberErrorKind {
		&self.kind
	}
}

impl Error for ParseNumberError {
	#[allow(deprecated)]
	fn description(&self) -> &str {
		match self.kind {
			NumberErrorKind::Empty => "cannot parse number from empty string",
			NumberErrorKind::Invalid => "invalid number literal",
		}
	}
}

impl fmt::Display for ParseNumberError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		#[allow(deprecated)]
		self.description().fmt(f)
	}
}

/// An error that can occur while parsing.
#[derive(Clone, Debug)]
pub enum ParseError {
	/// The parser encountered an invalid value.
	InvalidValue(Number),
	/// A wrapper around a [`ParseFloatError`].
	ParseFloat(ParseFloatError),
	/// A wrapper around a [`ParseIntError`].
	ParseInt(ParseIntError),
	/// A wrapper around a [`ParseNumberError`].
	ParseNumber(ParseNumberError),
}

impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::InvalidValue(value) => write!(f, "invalid value: {value}"),
			Self::ParseFloat(error) => error.fmt(f),
			Self::ParseInt(error) => error.fmt(f),
			Self::ParseNumber(error) => error.fmt(f),
		}
	}
}

impl Error for ParseError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			Self::InvalidValue(_) => None,
			Self::ParseFloat(error) => Some(error),
			Self::ParseInt(error) => Some(error),
			Self::ParseNumber(error) => Some(error),
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

impl From<ParseNumberError> for ParseError {
	fn from(value: ParseNumberError) -> Self {
		Self::ParseNumber(value)
	}
}
