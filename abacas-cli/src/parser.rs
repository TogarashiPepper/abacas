use std::iter::Peekable;

use crate::expr::Expression;
use crate::token::Token::{self, *};

pub struct Parser {
	tokens: Vec<Vec<Token>>,
}

impl Parser {
	pub fn new(tokens: Vec<Vec<Token>>) -> Self {
		Self { tokens }
	}

	pub fn parse(self) -> Vec<Expression> {
		self.tokens.into_iter().map(Self::parse_line).collect()
	}

	fn parse_line(line: Vec<Token>) -> Expression {
		let mut it = line.into_iter().peekable();

		Self::expr_bp(0, &mut it).fold()
	}

	fn expr_bp<T>(min_bp: u8, tokens: &mut Peekable<T>) -> Expression
	where
		T: Iterator<Item = Token>,
	{
		let mut lhs = match tokens.next() {
			Some(Sub) => Expression::PreOp {
				op: Sub,
				rhs: Box::new(Self::expr_bp(prefix_bp(Sub), tokens)),
			},
			Some(Number(num)) => Expression::Number(num),
			Some(Ident(name)) => Expression::Ident(name),
			Some(LParen) => {
				let lhs = Self::expr_bp(0, tokens);
				assert_eq!(tokens.next(), Some(RParen));

				lhs
			}
			_ => panic!("Bad token"),
		};

		loop {
			match tokens.peek() {
				Some(t @ (Add | Sub | Mul | Div | Rem | Pow)) => {
					let (l_bp, r_bp) = infix_bp(t.clone());

					if l_bp < min_bp {
						break;
					}

					let op = tokens.next().unwrap();
					let rhs = Self::expr_bp(r_bp, tokens);

					lhs = Expression::BinOp {
						lhs: Box::new(lhs),
						op,
						rhs: Box::new(rhs),
					};
				}
				Some(Number(_) | Ident(_) | LParen) => {
					let (l_bp, r_bp) = infix_bp(Token::Mul);
					if l_bp < min_bp {
						break;
					}

					let rhs = Self::expr_bp(r_bp, tokens);

					lhs = Expression::BinOp {
						lhs: Box::new(lhs),
						op: Token::Mul,
						rhs: Box::new(rhs),
					};
				}
				None | Some(RParen) => break,

				Some(t) => panic!("bad token: {t:#?}"),
			}
		}

		lhs
	}
}

pub fn infix_bp(op: Token) -> (u8, u8) {
	match op {
		Sub | Add => (1, 2),
		Mul | Div | Rem => (3, 4),
		Pow => (5, 6),

		_ => unreachable!(),
	}
}

pub fn prefix_bp(op: Token) -> u8 {
	match op {
		Sub => 5,

		_ => unreachable!(),
	}
}
