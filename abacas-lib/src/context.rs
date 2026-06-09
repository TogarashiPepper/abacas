use std::collections::HashMap;

use crate::expr::{Expr, Symbol};
use crate::function::Function;
use crate::standardlibrary::StandardLibrary;

#[derive(Debug)]
pub struct Context {
	pub variables: HashMap<Symbol, Expr>,
	pub functions: HashMap<Symbol, Function>,
	pub std: StandardLibrary,
}

impl Context {
	pub fn new() -> Self {
		Self {
			variables: HashMap::new(),
			functions: HashMap::new(),
			std: StandardLibrary::new(),
		}
	}
}
