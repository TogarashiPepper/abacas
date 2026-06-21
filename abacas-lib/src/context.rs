//! The Context structure and its related methods
use std::collections::HashMap;

use crate::expr::{Expr, Symbol};
use crate::function::Function;

/// Context struct owns and manages the core "global" data
#[derive(Debug)]
pub struct Context {
	/// Variables stored in this context
	pub variables: HashMap<Symbol, Expr>,
	/// Functions declared in this context
	pub functions: HashMap<Symbol, Function>,
}

impl Default for Context {
	fn default() -> Self {
		Self::new()
	}
}

impl Context {
	/// Create a new context
	pub fn new() -> Self {
		Self {
			variables: HashMap::new(),
			functions: HashMap::new(),
		}
	}
}
