//! The number enum and its related operations.

use std::ops::{Add, Div, Mul, Sub};

use rug::{Integer, Rational};

/// Represents a number of any supported set.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Number {
	/// A number belonging to set of natural numbers (not including zero).
	Natural(Integer),
	/// A number belonging to the set of integers.
	Integer(Integer),
	/// A number belonging to the set of rational numbers.
	Rational(Rational),
}

impl From<Integer> for Number {
	fn from(value: Integer) -> Self {
		if value.is_positive() {
			Self::Natural(value)
		} else {
			Self::Integer(value)
		}
	}
}

impl From<Rational> for Number {
	fn from(value: Rational) -> Self {
		if value.is_integer() {
			Self::from(value.into_numer_denom().0)
		} else {
			Self::Rational(value)
		}
	}
}

impl Add for Number {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		match self {
			Self::Natural(a) | Self::Integer(a) => match rhs {
				Self::Natural(b) | Self::Integer(b) => Self::from(a + b),
				Self::Rational(b) => Self::from(a + b),
			},
			Self::Rational(a) => match rhs {
				Self::Natural(b) | Self::Integer(b) => Self::from(a + b),
				Self::Rational(b) => Self::from(a + b),
			},
		}
	}
}

impl Add<Integer> for Number {
	type Output = Self;

	fn add(self, rhs: Integer) -> Self::Output {
		self + Self::from(rhs)
	}
}

impl Add<Rational> for Number {
	type Output = Self;

	fn add(self, rhs: Rational) -> Self::Output {
		self + Self::from(rhs)
	}
}

impl Div for Number {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		match self {
			Self::Natural(a) | Self::Integer(a) => match rhs {
				Self::Natural(b) | Self::Integer(b) => Self::from(a / b),
				Self::Rational(b) => Self::from(a / b),
			},
			Self::Rational(a) => match rhs {
				Self::Natural(b) | Self::Integer(b) => Self::from(a / b),
				Self::Rational(b) => Self::from(a / b),
			},
		}
	}
}

impl Div<Integer> for Number {
	type Output = Self;

	fn div(self, rhs: Integer) -> Self::Output {
		self / Self::from(rhs)
	}
}

impl Div<Rational> for Number {
	type Output = Self;

	fn div(self, rhs: Rational) -> Self::Output {
		self / Self::from(rhs)
	}
}

impl Mul for Number {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		match self {
			Self::Natural(a) | Self::Integer(a) => match rhs {
				Self::Natural(b) | Self::Integer(b) => Self::from(a * b),
				Self::Rational(b) => Self::from(a * b),
			},
			Self::Rational(a) => match rhs {
				Self::Natural(b) | Self::Integer(b) => Self::from(a * b),
				Self::Rational(b) => Self::from(a * b),
			},
		}
	}
}

impl Mul<Integer> for Number {
	type Output = Self;

	fn mul(self, rhs: Integer) -> Self::Output {
		self * Self::from(rhs)
	}
}

impl Mul<Rational> for Number {
	type Output = Self;

	fn mul(self, rhs: Rational) -> Self::Output {
		self * Self::from(rhs)
	}
}

impl Sub for Number {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		match self {
			Self::Natural(a) | Self::Integer(a) => match rhs {
				Self::Natural(b) | Self::Integer(b) => Self::from(a - b),
				Self::Rational(b) => Self::from(a - b),
			},
			Self::Rational(a) => match rhs {
				Self::Natural(b) | Self::Integer(b) => Self::from(a - b),
				Self::Rational(b) => Self::from(a - b),
			},
		}
	}
}

impl Sub<Integer> for Number {
	type Output = Self;

	fn sub(self, rhs: Integer) -> Self::Output {
		self - Self::from(rhs)
	}
}

impl Sub<Rational> for Number {
	type Output = Self;

	fn sub(self, rhs: Rational) -> Self::Output {
		self - Self::from(rhs)
	}
}
