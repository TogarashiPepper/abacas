//! The Function struct and its related methods
use crate::expr::{Expr, Symbol};

/// User defined function
#[derive(Clone, Debug)]
pub struct Function {
	/// Name of the function
	pub name: Symbol,
	/// Expression to execute when this function is called
	pub execute: Expr,
}

impl Function {
	/// Create a new function
	pub fn new(name: Symbol, execute: Expr) -> Self {
		Self { name, execute }
	}
}
