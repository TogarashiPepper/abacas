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

		Self::expr_bp(0, &mut it)
	}

	fn expr_bp<T>(min_bp: u8, tokens: &mut Peekable<T>) -> Expression
	where
		T: Iterator<Item = Token>,
	{
		let mut lhs = match tokens.next() {
			Some(Token::Number(num)) => Expression::Number(num),
			Some(Token::Ident(name)) => Expression::Ident(name),
			_ => panic!("Bad token"),
		};

		loop {
			let op = match tokens.peek() {
				Some(t @ (Add | Sub | Mul | Div | Rem | Pow)) => t,
				Some(t) => panic!("bad token: {t:#?}"),
				None => break,
			};

			let (l_bp, r_bp) = infix_binding_power(op.clone());

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

		lhs
	}
}

fn infix_binding_power(op: Token) -> (u8, u8) {
	match op {
		Token::Sub | Token::Add => (1, 2),
		Token::Mul | Token::Div | Token::Rem => (3, 4),
		Token::Pow => (5, 6),

		_ => unreachable!(),
	}
}
