use std::{error, fmt};

use logos::Span;

/// The error type used for any parsing actions.
#[derive(Debug)]
pub enum ParseError {
	/// The parser encountered an invalid token.
	InvalidToken(Span),
	/// The parser encountered an unexpected end of file.
	UnexpectedEof,
	/// The parser encountered an unknown token.
	UnknownToken(Span),
}

impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::InvalidToken(span) => write!(f, "invalid token at {span:?}"),
			Self::UnexpectedEof => write!(f, "unexpected end of file"),
			Self::UnknownToken(span) => write!(f, "unknown token at {span:?}"),
		}
	}
}

impl error::Error for ParseError {}

/// The standard result type, but with the error set to [`ParseError`].
pub type ParseResult<T> = Result<T, ParseError>;
