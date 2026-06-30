//! The Function struct and its related methods.

use crate::expr::{Expr, Symbol};

/// User defined function.
#[derive(Clone, Debug)]
pub struct Function {
	/// Expression to execute when this function is called.
	pub execute: Expr,
	/// Name of the function.
	pub name: Symbol,
	/// The parameters required by this function as arguments.
	pub params: Vec<Symbol>,
}

impl Function {
	/// Create a new function.
	pub fn new(execute: Expr, name: Symbol, params: Vec<Symbol>) -> Self {
		Self { execute, name, params }
	}
}
