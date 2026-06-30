//! The Context structure and its related methods.

use std::collections::HashMap;

use crate::expr::{Expr, Symbol};
use crate::function::Function;

/// Context struct owns and manages the core "global" data.
#[derive(Clone, Debug, Default)]
pub struct Context {
	/// Variables stored in this context.
	pub variables: HashMap<Symbol, Expr>,
	/// Functions declared in this context
	pub functions: HashMap<Symbol, Function>,
}

impl Context {
	/// Create a new context.
	pub fn new() -> Self {
		Self::default()
	}
}
