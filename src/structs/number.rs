use std::ops::{Add, Div, Mul, Sub};

use rug::{Integer, Rational};

/// Represents a Number
#[derive(Clone, Debug)]
pub enum Number {
	/// A number belonging to set of natural numbers (not including zero)
	Natural(Integer),
	/// A number belonging to the set of integers
	Integer(Integer),
	/// A real number
	Real(Rational),
}

impl Number {
	/// Convert rug::Integer to Number::Integer
	pub fn from_integer(n: Integer) -> Self {
		Self::Integer(n)
	}

	/// Convert rug::Rational to Number::Real
	pub fn from_rational(n: Rational) -> Self {
		Self::Real(n)
	}
}

impl Add for Number {
	type Output = Number;

	fn add(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Number::Natural(n1), Number::Natural(n2)) => Number::Natural(n1 + n2),
			(Number::Natural(n), Number::Integer(i)) | (Number::Integer(i), Number::Natural(n)) => {
				Number::from_integer(n + i)
			}
			(Number::Natural(n), Number::Real(r)) | (Number::Real(r), Number::Natural(n)) => {
				Number::from_rational(n + r)
			}
			(Number::Integer(i1), Number::Integer(i2)) => Number::from_integer(i1 + i2),
			(Number::Integer(i), Number::Real(r)) | (Number::Real(r), Number::Integer(i)) => {
				Number::from_rational(i + r)
			}
			(Number::Real(r1), Number::Real(r2)) => Number::from_rational(r1 + r2),
		}
	}
}

impl Div for Number {
	type Output = Number;

	fn div(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Number::Natural(n1), Number::Natural(n2)) => Number::Natural(n1 / n2),
			(Number::Natural(n), Number::Integer(i)) | (Number::Integer(i), Number::Natural(n)) => {
				Number::from_integer(n / i)
			}
			(Number::Natural(n), Number::Real(r)) | (Number::Real(r), Number::Natural(n)) => {
				Number::from_rational(n / r)
			}
			(Number::Integer(i1), Number::Integer(i2)) => Number::from_integer(i1 / i2),
			(Number::Integer(i), Number::Real(r)) | (Number::Real(r), Number::Integer(i)) => {
				Number::from_rational(i / r)
			}
			(Number::Real(r1), Number::Real(r2)) => Number::from_rational(r1 / r2),
		}
	}
}

impl Mul for Number {
	type Output = Number;

	fn mul(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Number::Natural(n1), Number::Natural(n2)) => Number::Natural(n1 * n2),
			(Number::Natural(n), Number::Integer(i)) | (Number::Integer(i), Number::Natural(n)) => {
				Number::from_integer(n * i)
			}
			(Number::Natural(n), Number::Real(r)) | (Number::Real(r), Number::Natural(n)) => {
				Number::from_rational(n * r)
			}
			(Number::Integer(i1), Number::Integer(i2)) => Number::from_integer(i1 * i2),
			(Number::Integer(i), Number::Real(r)) | (Number::Real(r), Number::Integer(i)) => {
				Number::from_rational(i * r)
			}
			(Number::Real(r1), Number::Real(r2)) => Number::from_rational(r1 * r2),
		}
	}
}

impl Sub for Number {
	type Output = Number;

	fn sub(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Number::Natural(n1), Number::Natural(n2)) => Number::Natural(n1 - n2),
			(Number::Natural(n), Number::Integer(i)) | (Number::Integer(i), Number::Natural(n)) => {
				Number::from_integer(n - i)
			}
			(Number::Natural(n), Number::Real(r)) | (Number::Real(r), Number::Natural(n)) => {
				Number::from_rational(n - r)
			}
			(Number::Integer(i1), Number::Integer(i2)) => Number::from_integer(i1 - i2),
			(Number::Integer(i), Number::Real(r)) | (Number::Real(r), Number::Integer(i)) => {
				Number::from_rational(i - r)
			}
			(Number::Real(r1), Number::Real(r2)) => Number::from_rational(r1 - r2),
		}
	}
}

impl Add<Integer> for Number {
	type Output = Number;

	fn add(self, rhs: Integer) -> Self::Output {
		self + Number::Integer(rhs)
	}
}

impl Div<Integer> for Number {
	type Output = Number;

	fn div(self, rhs: Integer) -> Self::Output {
		self / Number::Integer(rhs)
	}
}

impl Mul<Integer> for Number {
	type Output = Number;

	fn mul(self, rhs: Integer) -> Self::Output {
		self * Number::Integer(rhs)
	}
}

impl Sub<Integer> for Number {
	type Output = Number;

	fn sub(self, rhs: Integer) -> Self::Output {
		self - Number::Integer(rhs)
	}
}

impl Add<Rational> for Number {
	type Output = Number;

	fn add(self, rhs: Rational) -> Self::Output {
		self + Number::Real(rhs)
	}
}

impl Div<Rational> for Number {
	type Output = Number;

	fn div(self, rhs: Rational) -> Self::Output {
		self / Number::Real(rhs)
	}
}

impl Mul<Rational> for Number {
	type Output = Number;

	fn mul(self, rhs: Rational) -> Self::Output {
		self * Number::Real(rhs)
	}
}

impl Sub<Rational> for Number {
	type Output = Number;

	fn sub(self, rhs: Rational) -> Self::Output {
		self - Number::Real(rhs)
	}
}
