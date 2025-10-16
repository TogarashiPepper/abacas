use std::ops::{Add, Div, DivAssign, Mul, MulAssign, Neg, Sub};
use std::{fmt, str};

use itertools::Itertools;

use crate::error::ParseError;
use crate::structs::polynomial::Polynomial;

/// A monomial `ax^b` consisting of coefficient `a` and degree `b`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Monomial {
	pub coeff: f64,
	pub degree: i64,
}

impl Monomial {
	/// Creates a new monomial. Panics if `coeff` is zero.
	pub const fn new(coeff: f64, degree: i64) -> Self {
		if coeff == 0.0 {
			panic!("abacas: monomial coefficient must not be zero");
		}

		Self { coeff, degree }
	}

	/// Creates a constant monomial. Panics if `coeff` is zero.
	pub const fn constant(coeff: f64) -> Self {
		Self::new(coeff, 0)
	}

	/// Creates a linear monomial. Panics if `coeff` is zero.
	pub const fn linear(coeff: f64) -> Self {
		Self::new(coeff, 1)
	}
}

impl Add for Monomial {
	type Output = Polynomial;

	fn add(self, rhs: Self) -> Self::Output {
		Polynomial::new([self, rhs])
	}
}

impl Div for Monomial {
	type Output = Monomial;

	fn div(mut self, rhs: Self) -> Self::Output {
		self /= rhs;
		self
	}
}

impl DivAssign for Monomial {
	fn div_assign(&mut self, rhs: Self) {
		self.coeff /= rhs.coeff;
		self.degree -= rhs.degree;
	}
}

impl Mul for Monomial {
	type Output = Monomial;

	fn mul(mut self, rhs: Self) -> Self::Output {
		self *= rhs;
		self
	}
}

impl MulAssign for Monomial {
	fn mul_assign(&mut self, rhs: Self) {
		self.coeff *= rhs.coeff;
		self.degree += rhs.degree;
	}
}

impl Neg for Monomial {
	type Output = Monomial;

	fn neg(mut self) -> Self::Output {
		self.coeff = -self.coeff;
		self
	}
}

impl Sub for Monomial {
	type Output = Polynomial;

	fn sub(self, rhs: Self) -> Self::Output {
		Polynomial::new([self, -rhs])
	}
}

impl fmt::Display for Monomial {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match (self.coeff, self.degree) {
			(1.0, 1) => write!(f, "x"),
			(_, 0) => write!(f, "{}", self.coeff),
			(1.0, deg) => write!(f, "x^{deg}"),
			(_, 1) => write!(f, "{}x", self.coeff),
			(_, _) => write!(f, "{}x^{}", self.coeff, self.degree),
		}
	}
}

impl str::FromStr for Monomial {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut chars = s.trim().chars().peekable();

		let init: String = chars.peeking_take_while(|&c| c != 'x').collect();
		let coeff = if init.is_empty() { 1.0 } else { init.parse()? };

		if coeff == 0.0 {
			return Err(ParseError::InvalidValue);
		}

		match chars.next() {
			None => return Ok(Self::constant(coeff)),
			Some('x') => (),
			Some(_) => return Err(ParseError::InvalidSyntax),
		}

		match chars.next() {
			None => return Ok(Self::linear(coeff)),
			Some('^') => (),
			Some(_) => return Err(ParseError::InvalidSyntax),
		}

		let tail: String = chars.collect();
		let degree = tail.parse()?;

		Ok(Self { coeff, degree })
	}
}
