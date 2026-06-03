use std::collections::HashMap;

use crate::expr::{Expr, Symbol};

#[derive(Clone, Debug)]
pub struct StandardLibrary(HashMap<Symbol, StandardLibraryFunction>);

impl StandardLibrary {
	pub fn new() -> Self {
		let mut functions = HashMap::new();

		functions.insert(
			Symbol::new("identity"),
			StandardLibraryFunction {
				name: Symbol::new("identity"),
				execute: identity,
			},
		);

		Self(functions)
	}
}

#[derive(Clone, Debug)]
pub struct StandardLibraryFunction {
	pub name: Symbol,
	pub execute: fn(args: Vec<Expr>) -> Option<Expr>,
}

fn identity(args: Vec<Expr>) -> Option<Expr> {
	if args.len() > 1 {
		// TODO: replace this with an error mechanism
		panic!("expected one argument")
	}
	args.get(0).cloned()
}
