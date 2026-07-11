//! The real enum and related items.

use std::iter::{Product, Sum};
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::ops::{Inv, Pow};
use crate::rational::Rational;
use crate::symbol::Symbol;

/// Represents any real number.
#[derive(Debug)]
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

impl Real {
	/// The number negative one (-1).
	pub const NEG_ONE: Self = Self::Rat(Rational::NEG_ONE);

	/// The number one (1).
	pub const ONE: Self = Self::Rat(Rational::ONE);

	/// The number zero (0).
	pub const ZERO: Self = Self::Rat(Rational::ZERO);
}

impl Real {
	/// Returns the same number, but simplified on a best-effort basis.
	pub fn simplify(self) -> Self {
		match self {
			Self::Add(reals) => Self::simplify_add(reals),
			Self::Fun(name, args) => Self::simplify_fun(name, args),
			Self::Mul(reals) => Self::simplify_mul(reals),
			Self::Pow(base, exp) => Self::simplify_pow(base, exp),
			Self::Rat(rational) => Self::Rat(rational),
		}
	}

	/// Simplifies the [`Self::Add`] branch.
	fn simplify_add(reals: Vec<Self>) -> Self {
		Self::Add(reals)
	}

	/// Simplifies the [`Self::Fun`] branch.
	fn simplify_fun(name: Symbol, args: Vec<Self>) -> Self {
		Self::Fun(name, args)
	}

	/// Simplifies the [`Self::Mul`] branch.
	fn simplify_mul(reals: Vec<Real>) -> Self {
		Self::Mul(reals)
	}

	/// Simplifies the [`Self::Pow`] branch.
	fn simplify_pow(base: Box<Self>, exp: Box<Self>) -> Self {
		Self::Pow(base, exp)
	}
}

impl Add for Real {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self::Add(vec![self, rhs]).simplify()
	}
}

impl Default for Real {
	fn default() -> Self {
		Self::ZERO
	}
}

impl Div for Real {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		self.mul(rhs.inv())
	}
}

impl<T> From<T> for Real
where
	T: Into<Rational>,
{
	fn from(value: T) -> Self {
		Self::Rat(value.into()).simplify()
	}
}

impl Inv for Real {
	type Output = Self;

	fn inv(self) -> Self::Output {
		self.pow(Self::NEG_ONE)
	}
}

impl Mul for Real {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Self::Mul(vec![self, rhs]).simplify()
	}
}

impl Neg for Real {
	type Output = Self;

	fn neg(self) -> Self::Output {
		self.mul(Self::NEG_ONE)
	}
}

impl Pow for Real {
	type Output = Self;

	fn pow(self, rhs: Self) -> Self::Output {
		Self::Pow(self.into(), rhs.into()).simplify()
	}
}

impl Product for Real {
	fn product<I>(iter: I) -> Self
	where
		I: Iterator<Item = Self>,
	{
		Self::Mul(iter.collect()).simplify()
	}
}

impl Sub for Real {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		self.add(rhs.neg())
	}
}

impl Sum for Real {
	fn sum<I>(iter: I) -> Self
	where
		I: Iterator<Item = Self>,
	{
		Self::Add(iter.collect()).simplify()
	}
}
