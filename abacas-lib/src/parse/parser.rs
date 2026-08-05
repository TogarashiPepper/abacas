use rug::ops::Pow;

use crate::context::Context;
use crate::expr::Expr;
use crate::monomial::Monomial;
use crate::number::Number;
use crate::parse::{ParseError, ParseResult, Token, Tokens};

/// A parser for [`Expr`] instances with built-in assignment support.
#[derive(Clone, Debug)]
pub struct Parser {
	ctx: Context,
}

impl Parser {
	/// Returns a reference to the inner context.
	pub fn ctx(&self) -> &Context {
		&self.ctx
	}

	/// Creates a new parser.
	pub fn new(ctx: Context) -> Self {
		Self { ctx }
	}

	/// Tries to parse an expression from a source string.
	pub fn parse(&mut self, source: &str) -> ParseResult<Expr> {
		self.pratt(0, &mut Tokens::new(source))
	}

	/// Internal method to recursively apply pratt parsing.
	fn pratt(&mut self, power: u8, tokens: &mut Tokens) -> ParseResult<Expr> {
		// Parse the first part of the expression
		let mut lhs = match tokens.pop()? {
			// Do nothing on unary plus
			Token::Add => self.pratt(Token::Add.prefix().unwrap(), tokens)?,

			// Parse expressions inside parentheses
			Token::Left => {
				let lhs = self.pratt(0, tokens)?;

				match tokens.pop()? {
					Token::Right => lhs,
					_ => return Err(ParseError::InvalidToken(tokens.span())),
				}
			}

			// A number is already a complete expression
			Token::Num(number) => Expr::Num(number),

			// Parse unary minus separately
			Token::Sub => -self.pratt(Token::Sub.prefix().unwrap(), tokens)?,

			// A symbol can either be a variable or the start of a function call
			Token::Sym(symbol) => match tokens.peek()? {
				// If next token is a left parenthesis, parse a function call
				Token::Left => {
					// Remove the left parenthesis
					tokens.pop()?;

					// Parse the argument list
					let mut args = Vec::new();

					loop {
						// Empty argument list or trailing comma
						if matches!(tokens.peek()?, Token::Right) {
							tokens.pop()?;
							break;
						}

						// Parse the argument until the next separator
						args.push(self.pratt(0, tokens)?);

						match tokens.pop()? {
							Token::Comma => continue,
							Token::Right => break,

							// Error on an invalid separator
							_ => return Err(ParseError::InvalidToken(tokens.span())),
						}
					}

					// Return the parsed function call
					Expr::Fun(symbol, args)
				}

				// Otherwise convert into a polynomial
				_ => Expr::Poly(symbol, Monomial::linear(Number::one()).into()),
			},

			// Otherwise return an error
			_ => return Err(ParseError::InvalidToken(tokens.span())),
		};

		// Continue parsing operators after the left hand side
		while !tokens.is_empty() {
			match tokens.peek()? {
				// These tokens belong to the caller
				Token::Comma | Token::Right => break,

				// Handle implicit multiplication
				Token::Left | Token::Sym(_) => {
					let (left, right) = Token::Mul.infix().unwrap();

					if left < power {
						break;
					}

					lhs = lhs * self.pratt(right, tokens)?;
				}

				// Parse any remaining infix operators
				token if token.infix().is_some() => {
					let (left, right) = token.infix().unwrap();

					if left < power {
						break;
					}

					match tokens.pop()? {
						Token::Add => lhs = lhs + self.pratt(right, tokens)?,
						Token::Div => lhs = lhs / self.pratt(right, tokens)?,
						Token::Eq => todo!(),
						Token::Mul => lhs = lhs * self.pratt(right, tokens)?,
						Token::Pow => lhs = lhs.pow(self.pratt(right, tokens)?),
						Token::Rem => todo!(),
						Token::Sub => lhs = lhs - self.pratt(right, tokens)?,
						_ => (),
					}
				}

				// Otherwise return an error
				_ => return Err(ParseError::InvalidToken(tokens.span())),
			}
		}

		Ok(lhs)
	}
}
