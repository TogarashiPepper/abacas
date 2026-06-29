//! The Standard Library of abacas

use std::collections::HashMap;

use crate::context::Context;
use crate::expr::{Expr, Symbol};

/// StdLib struct containing all the global functions
#[derive(Clone, Debug, Default)]
pub struct StdLib(pub HashMap<Symbol, StdLibFunction>);

impl StdLib {
	/// Creates a new copy of the StdLib
	pub fn new() -> Self {
		let mut functions = HashMap::new();

		functions.insert(
			Symbol::new("echo").unwrap(),
			StdLibFunction {
				name: Symbol::new("echo").unwrap(),
				execute: echo,
			},
		);

		functions.insert(
			Symbol::new("round").unwrap(),
			StdLibFunction {
				name: Symbol::new("round").unwrap(),
				execute: round,
			},
		);

		functions.insert(
			Symbol::new("ceil").unwrap(),
			StdLibFunction {
				name: Symbol::new("ceil").unwrap(),
				execute: ceil,
			},
		);

		functions.insert(
			Symbol::new("floor").unwrap(),
			StdLibFunction {
				name: Symbol::new("floor").unwrap(),
				execute: floor,
			},
		);

		Self(functions)
	}
}

/// A StdLib Function
#[derive(Clone, Debug)]
pub struct StdLibFunction {
	/// Name of the function
	pub name: Symbol,
	/// The implementation of the function
	pub execute: fn(args: Vec<Expr>, ctx: &mut Context) -> Expr,
}

//TODO: Add proper error mechanism
//TODO: Add `abs`, `exp`, `ln`, `log10`, `log`, `sin`, `cos`, `tan`, `sqrt`, `nrt` methods

/// echo(a) -> a
///
/// Returns the argument provided to it
pub fn echo(args: Vec<Expr>, _: &mut Context) -> Expr {
	if args.len() != 1 {
		panic!("expected one argument")
	}
	args.into_iter().next().unwrap()
}

/// round(n) -> n
///
/// Returns the rounded number following Banker's Rounding
pub fn round(args: Vec<Expr>, ctx: &mut Context) -> Expr {
	if args.len() != 1 {
		panic!("expected one argument")
	}

	let Ok(Expr::Num(n)) = args.into_iter().next().unwrap().simplify(ctx) else {
		unimplemented!()
	};

	Expr::Num(n.round())
}

/// ceil(n) -> n
///
/// Returns the smallest integer more than or equal to n
pub fn ceil(args: Vec<Expr>, ctx: &mut Context) -> Expr {
	if args.len() != 1 {
		panic!("expected one argument")
	}

	let Ok(Expr::Num(n)) = args.into_iter().next().unwrap().simplify(ctx) else {
		unimplemented!()
	};

	Expr::Num(n.ceil())
}

/// floor(n) -> n
///
/// Returns the smallest integer less than or equal to n
pub fn floor(args: Vec<Expr>, ctx: &mut Context) -> Expr {
	if args.len() != 1 {
		panic!("expected one argument")
	}

	let Ok(Expr::Num(n)) = args.into_iter().next().unwrap().simplify(ctx) else {
		unimplemented!()
	};

	Expr::Num(n.floor())
}
