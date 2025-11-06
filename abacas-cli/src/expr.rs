use abacas::number::Number;
use abacas::polynomial::Polynomial;

use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expression {
	Number(Number),
	Ident(String),
	Polynomial(Polynomial),
	BinOp {
		lhs: Box<Expression>,
		op: Token,
		rhs: Box<Expression>,
	},
}
