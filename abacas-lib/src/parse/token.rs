use logos::{Lexer, Logos, Span};

use crate::context::Symbol;
use crate::number::Number;
use crate::parse::{ParseError, ParseResult};

/// Represents a syntax token of the input string.
#[derive(Clone, Debug, Logos)]
#[logos(skip r"\s+")]
pub enum Token {
	/// The token addition (`+`).
	#[token("+")]
	Add,

	/// The token comma (`,`).
	#[token(",")]
	Comma,

	/// The token division (`/`).
	#[token("/")]
	Div,

	/// The token equals (`=`).
	#[token("=")]
	Eq,

	/// The token left parenthesis (`(`).
	#[token("(")]
	Left,

	/// The token multiplication (`*`).
	#[token("*")]
	Mul,

	/// A non-negative number literal.
	#[regex(r"[0-9]+", |lex| lex.slice().parse().ok())]
	Num(Number),

	/// The token power (`^`).
	#[token("^")]
	Pow,

	/// The token remainder (`%`).
	#[token("%")]
	Rem,

	/// The token right parenthesis (`)`).
	#[token(")")]
	Right,

	/// The token subtraction (`-`).
	#[token("-")]
	Sub,

	/// A symbol used for names.
	#[regex(r"[a-z]+", |lex| Symbol::new(lex.slice().into()))]
	Sym(Symbol),
}

impl Token {
	/// Returns the infix binding power for the current token.
	pub fn infix(&self) -> Option<(u8, u8)> {
		match self {
			Self::Add => Some((3, 4)),
			Self::Div => Some((5, 6)),
			Self::Eq => Some((2, 1)),
			Self::Mul => Some((5, 6)),
			Self::Pow => Some((8, 7)),
			Self::Rem => Some((5, 6)),
			Self::Sub => Some((3, 4)),
			_ => None,
		}
	}

	/// Returns the prefix binding power for the current token.
	pub fn prefix(&self) -> Option<u8> {
		match self {
			Self::Add => Some(7),
			Self::Sub => Some(7),
			_ => None,
		}
	}
}

/// A pseudo-iterator over the tokens with peeking capabilities.
#[derive(Clone, Debug)]
pub struct Tokens<'a> {
	inner: Lexer<'a, Token>,
	stack: Option<Option<Result<Token, ()>>>,
}

impl<'a> Tokens<'a> {
	/// Returns whether there are no more tokens to consume.
	pub fn is_empty(&mut self) -> bool {
		matches!(self.peek(), Err(ParseError::UnexpectedEof))
	}

	/// Creates a new token stream from a source string.
	pub fn new(source: &'a str) -> Self {
		Self {
			inner: Token::lexer(source),
			stack: None,
		}
	}

	/// Peeks the next token without consuming it.
	pub fn peek(&mut self) -> ParseResult<&Token> {
		self.stack
			.get_or_insert_with(|| self.inner.next())
			.as_ref()
			.ok_or(ParseError::UnexpectedEof)?
			.as_ref()
			.map_err(|_| ParseError::UnknownToken(self.inner.span()))
	}

	/// Pops the next token and consumes it.
	pub fn pop(&mut self) -> ParseResult<Token> {
		self.stack
			.take()
			.unwrap_or_else(|| self.inner.next())
			.ok_or(ParseError::UnexpectedEof)?
			.map_err(|_| ParseError::UnknownToken(self.inner.span()))
	}

	/// Returns the span of the most recent token, used to construct errors while parsing.
	pub fn span(&self) -> Span {
		self.inner.span()
	}
}
