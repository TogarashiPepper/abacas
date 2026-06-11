//! The Standard Library of abacas
use std::collections::HashMap;

use crate::expr::{Expr, Symbol};

/// StandardLibrary struct containing all the global data
#[derive(Clone, Debug)]
pub struct StandardLibrary {
	/// Global functions in the standard library
	pub functions: HashMap<Symbol, StandardLibraryFunction>,
}

impl StandardLibrary {
	/// Create a new reference to the StandardLibrary
	pub fn new() -> Self {
		let mut functions = HashMap::new();

		functions.insert(
			Symbol::new("identity"),
			StandardLibraryFunction {
				name: Symbol::new("identity"),
				execute: identity,
			},
		);

		Self { functions }
	}
}

impl Default for StandardLibrary {
	fn default() -> Self {
		Self::new()
	}
}

/// A Standard Library Function
#[derive(Clone, Debug)]
pub struct StandardLibraryFunction {
	/// Name of the function
	pub name: Symbol,
	/// Pointer to function in Rust
	pub execute: fn(args: Vec<Expr>) -> Option<Expr>,
}

fn identity(args: Vec<Expr>) -> Option<Expr> {
	if args.len() > 1 {
		// TODO: replace this with an error mechanism
		panic!("expected one argument")
	}
	args.first().cloned()
}
