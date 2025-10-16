use std::ops::{Add, Div, Mul, Sub};

use rug::{Integer as RugInteger, Rational as RugRational};

#[derive(Clone, Debug)]
pub enum Number {
	Natural(Natural),
	Integer(Integer),
	Real(Real),
}

impl Number {
	pub fn from_rug_integer(n: RugInteger) -> Self {
		Self::Integer(Integer(n))
	}

	pub fn from_rug_rational(n: RugRational) -> Self {
		Self::Real(Real(n))
	}
}

impl Add for Number {
	type Output = Number;

	fn add(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Number::Natural(n1), Number::Natural(n2)) => Number::Natural(Natural(n1.0 + n2.0)),
			(Number::Natural(n), Number::Integer(i)) | (Number::Integer(i), Number::Natural(n)) => {
				Number::from_rug_integer(n.0 + i.0)
			}
			(Number::Natural(n), Number::Real(r)) | (Number::Real(r), Number::Natural(n)) => {
				Number::from_rug_rational(n.0 + r.0)
			}
			(Number::Integer(i1), Number::Integer(i2)) => Number::from_rug_integer(i1.0 + i2.0),
			(Number::Integer(i), Number::Real(r)) | (Number::Real(r), Number::Integer(i)) => {
				Number::from_rug_rational(i.0 + r.0)
			}
			(Number::Real(r1), Number::Real(r2)) => Number::from_rug_rational(r1.0 + r2.0),
		}
	}
}

impl Sub for Number {
	type Output = Number;

	fn sub(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Number::Natural(n1), Number::Natural(n2)) => Number::Natural(Natural(n1.0 - n2.0)),
			(Number::Natural(n), Number::Integer(i)) | (Number::Integer(i), Number::Natural(n)) => {
				Number::from_rug_integer(n.0 - i.0)
			}
			(Number::Natural(n), Number::Real(r)) | (Number::Real(r), Number::Natural(n)) => {
				Number::from_rug_rational(n.0 - r.0)
			}
			(Number::Integer(i1), Number::Integer(i2)) => Number::from_rug_integer(i1.0 - i2.0),
			(Number::Integer(i), Number::Real(r)) | (Number::Real(r), Number::Integer(i)) => {
				Number::from_rug_rational(i.0 - r.0)
			}
			(Number::Real(r1), Number::Real(r2)) => Number::from_rug_rational(r1.0 - r2.0),
		}
	}
}

impl Mul for Number {
	type Output = Number;

	fn mul(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Number::Natural(n1), Number::Natural(n2)) => Number::Natural(Natural(n1.0 * n2.0)),
			(Number::Natural(n), Number::Integer(i)) | (Number::Integer(i), Number::Natural(n)) => {
				Number::from_rug_integer(n.0 * i.0)
			}
			(Number::Natural(n), Number::Real(r)) | (Number::Real(r), Number::Natural(n)) => {
				Number::from_rug_rational(n.0 * r.0)
			}
			(Number::Integer(i1), Number::Integer(i2)) => Number::from_rug_integer(i1.0 * i2.0),
			(Number::Integer(i), Number::Real(r)) | (Number::Real(r), Number::Integer(i)) => {
				Number::from_rug_rational(i.0 * r.0)
			}
			(Number::Real(r1), Number::Real(r2)) => Number::from_rug_rational(r1.0 * r2.0),
		}
	}
}

impl Div for Number {
	type Output = Number;

	fn div(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Number::Natural(n1), Number::Natural(n2)) => Number::Natural(Natural(n1.0 / n2.0)),
			(Number::Natural(n), Number::Integer(i)) | (Number::Integer(i), Number::Natural(n)) => {
				Number::from_rug_integer(n.0 / i.0)
			}
			(Number::Natural(n), Number::Real(r)) | (Number::Real(r), Number::Natural(n)) => {
				Number::from_rug_rational(n.0 / r.0)
			}
			(Number::Integer(i1), Number::Integer(i2)) => Number::from_rug_integer(i1.0 / i2.0),
			(Number::Integer(i), Number::Real(r)) | (Number::Real(r), Number::Integer(i)) => {
				Number::from_rug_rational(i.0 / r.0)
			}
			(Number::Real(r1), Number::Real(r2)) => Number::from_rug_rational(r1.0 / r2.0),
		}
	}
}

#[derive(Clone, Debug)]
pub struct Natural(RugInteger);

impl Natural {
	pub fn new(n: usize) -> Self {
		Self(RugInteger::from(n))
	}
}

#[derive(Clone, Debug)]
pub struct Integer(RugInteger);

impl Integer {
	pub fn new(n: isize) -> Self {
		Self(RugInteger::from(n))
	}
}

#[derive(Clone, Debug)]
pub struct Real(RugRational);

impl Real {
	pub fn new(n: f64) -> Self {
		Self(RugRational::from_f64(n).unwrap())
	}
}
