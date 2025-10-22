use crate::expr::Expression;
use crate::token::Token;

pub struct Parser {
	tokens: Vec<Vec<Token>>,
}

impl Parser {
	pub fn new(tokens: Vec<Vec<Token>>) -> Self {
		Self { tokens }
	}

	pub fn ast(&mut self) -> Vec<Expression> {
		vec![]
	}
}
