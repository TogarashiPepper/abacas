//! The Function struct and its related methods
use crate::expr::{Expr, Symbol};

/// User defined function
#[derive(Clone, Debug)]
pub struct Function {
	/// The parameters required by this function as arguments
	pub args: Vec<Symbol>,
	/// Expression to execute when this function is called
	pub execute: Expr,
	/// Name of the function
	pub name: Symbol,
}

impl Function {
	/// Create a new function
	pub fn new(args: Vec<Symbol>, execute: Expr, name: Symbol) -> Self {
		Self { args, execute, name }
	}
}
