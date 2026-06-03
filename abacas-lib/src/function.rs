use crate::expr::{Expr, Symbol};

#[derive(Clone, Debug)]
pub struct Function {
	pub name: Symbol,
	pub execute: Expr,
}

impl Function {
	pub fn new(name: Symbol, execute: Expr) -> Self {
		Self { name, execute }
	}
}
