//! The real enum and related items.

use crate::rational::Rational;
use crate::symbol::Symbol;

/// Represents any real number.
pub enum Real {
	/// The sum of multiple reals.
	Add(Vec<Self>),
	/// A function call.
	Fun(Symbol, Vec<Self>),
	/// The product of multiple reals.
	Mul(Vec<Self>),
	/// The power of two reals.
	Pow(Box<Self>, Box<Self>),
	/// A rational number.
	Rat(Rational),
}
