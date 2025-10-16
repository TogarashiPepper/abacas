use std::ops::{Add, Div, Mul, Sub};

use rug::{Integer, Rational};

#[derive(Clone, Debug)]
pub enum Number {
	Natural(Integer),
	Integer(Integer),
	Real(Rational),
}

impl Number {
	pub fn from_rug_integer(n: Integer) -> Self {
		Self::Integer(n)
	}

	pub fn from_rug_rational(n: Rational) -> Self {
		Self::Real(n)
	}
}

impl Add for Number {
	type Output = Number;

	fn add(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Number::Natural(n1), Number::Natural(n2)) => Number::Natural(n1 + n2),
			(Number::Natural(n), Number::Integer(i)) | (Number::Integer(i), Number::Natural(n)) => {
				Number::from_rug_integer(n + i)
			}
			(Number::Natural(n), Number::Real(r)) | (Number::Real(r), Number::Natural(n)) => {
				Number::from_rug_rational(n + r)
			}
			(Number::Integer(i1), Number::Integer(i2)) => Number::from_rug_integer(i1 + i2),
			(Number::Integer(i), Number::Real(r)) | (Number::Real(r), Number::Integer(i)) => {
				Number::from_rug_rational(i + r)
			}
			(Number::Real(r1), Number::Real(r2)) => Number::from_rug_rational(r1 + r2),
		}
	}
}

impl Sub for Number {
	type Output = Number;

	fn sub(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Number::Natural(n1), Number::Natural(n2)) => Number::Natural(n1 - n2),
			(Number::Natural(n), Number::Integer(i)) | (Number::Integer(i), Number::Natural(n)) => {
				Number::from_rug_integer(n - i)
			}
			(Number::Natural(n), Number::Real(r)) | (Number::Real(r), Number::Natural(n)) => {
				Number::from_rug_rational(n - r)
			}
			(Number::Integer(i1), Number::Integer(i2)) => Number::from_rug_integer(i1 - i2),
			(Number::Integer(i), Number::Real(r)) | (Number::Real(r), Number::Integer(i)) => {
				Number::from_rug_rational(i - r)
			}
			(Number::Real(r1), Number::Real(r2)) => Number::from_rug_rational(r1 - r2),
		}
	}
}

impl Mul for Number {
	type Output = Number;

	fn mul(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Number::Natural(n1), Number::Natural(n2)) => Number::Natural(n1 * n2),
			(Number::Natural(n), Number::Integer(i)) | (Number::Integer(i), Number::Natural(n)) => {
				Number::from_rug_integer(n * i)
			}
			(Number::Natural(n), Number::Real(r)) | (Number::Real(r), Number::Natural(n)) => {
				Number::from_rug_rational(n * r)
			}
			(Number::Integer(i1), Number::Integer(i2)) => Number::from_rug_integer(i1 * i2),
			(Number::Integer(i), Number::Real(r)) | (Number::Real(r), Number::Integer(i)) => {
				Number::from_rug_rational(i * r)
			}
			(Number::Real(r1), Number::Real(r2)) => Number::from_rug_rational(r1 * r2),
		}
	}
}

impl Div for Number {
	type Output = Number;

	fn div(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Number::Natural(n1), Number::Natural(n2)) => Number::Natural(n1 / n2),
			(Number::Natural(n), Number::Integer(i)) | (Number::Integer(i), Number::Natural(n)) => {
				Number::from_rug_integer(n / i)
			}
			(Number::Natural(n), Number::Real(r)) | (Number::Real(r), Number::Natural(n)) => {
				Number::from_rug_rational(n / r)
			}
			(Number::Integer(i1), Number::Integer(i2)) => Number::from_rug_integer(i1 / i2),
			(Number::Integer(i), Number::Real(r)) | (Number::Real(r), Number::Integer(i)) => {
				Number::from_rug_rational(i / r)
			}
			(Number::Real(r1), Number::Real(r2)) => Number::from_rug_rational(r1 / r2),
		}
	}
}
