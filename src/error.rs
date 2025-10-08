use std::error::Error;
use std::fmt;
use std::num::{ParseFloatError, ParseIntError};

/// An error that can occur while parsing.
#[derive(Clone, Debug)]
pub enum ParseError {
	InvalidSyntax,
	InvalidValue,
	ParseFloat(ParseFloatError),
	ParseInt(ParseIntError),
}

impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::InvalidSyntax => f.write_str("invalid syntax"),
			Self::InvalidValue => f.write_str("invalid value"),
			Self::ParseFloat(error) => error.fmt(f),
			Self::ParseInt(error) => error.fmt(f),
		}
	}
}

impl Error for ParseError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			Self::InvalidSyntax => None,
			Self::InvalidValue => None,
			Self::ParseFloat(error) => Some(error),
			Self::ParseInt(error) => Some(error),
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
