//! The number enum and its related operations.

use std::cmp::Ordering;
use std::fmt::{self, Display};
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use std::str;

use rug::rational::ParseRationalError;
use rug::{Integer, Rational};

/// Represents a number of any supported set.
#[derive(Clone, Debug)]
pub enum Number {
	/// A number belonging to the set of integers.
	Integer(Integer),
	/// A number belonging to the set of natural numbers (not including zero).
	Natural(Integer),
	/// A number belonging to the set of rational numbers.
	Rational(Rational),
}

impl Number {
	/// The number zero.
	pub const ZERO: Self = Self::Integer(Integer::ZERO);

	/// The number pi.
	pub fn pi() -> Self {
		Self::Rational(Rational::from((3141592653589793u64, 1000000000000000u64)))
	}

	/// The eulers number.
	pub fn e() -> Self {
		Self::Rational(Rational::from((2718281828459045u64, 1000000000000000u64)))
	}
}

impl Default for Number {
	fn default() -> Self {
		Self::ZERO
	}
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

impl<T> PartialEq<T> for Number
where
	Integer: PartialEq<T>,
	Rational: PartialEq<T>,
{
	fn eq(&self, other: &T) -> bool {
		match self {
			Self::Integer(lhs) | Self::Natural(lhs) => lhs == other,
			Self::Rational(lhs) => lhs == other,
		}
	}
}

impl PartialEq for Number {
	fn eq(&self, other: &Self) -> bool {
		match other {
			Self::Integer(rhs) | Self::Natural(rhs) => self == rhs,
			Self::Rational(rhs) => self == rhs,
		}
	}
}

impl<T> PartialOrd<T> for Number
where
	Integer: PartialOrd<T>,
	Rational: PartialOrd<T>,
{
	fn partial_cmp(&self, other: &T) -> Option<Ordering> {
		match self {
			Self::Integer(lhs) | Self::Natural(lhs) => lhs.partial_cmp(other),
			Self::Rational(lhs) => lhs.partial_cmp(other),
		}
	}
}

impl PartialOrd for Number {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match other {
			Self::Integer(rhs) | Self::Natural(rhs) => self.partial_cmp(rhs),
			Self::Rational(rhs) => self.partial_cmp(rhs),
		}
	}
}

impl<T> Add<T> for Number
where
	Integer: Add<T, Output: Into<Self>>,
	Rational: Add<T, Output: Into<Self>>,
{
	type Output = Self;

	fn add(self, rhs: T) -> Self::Output {
		match self {
			Self::Integer(lhs) | Self::Natural(lhs) => (lhs + rhs).into(),
			Self::Rational(lhs) => (lhs + rhs).into(),
		}
	}
}

impl Add for Number {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		match rhs {
			Self::Integer(rhs) | Self::Natural(rhs) => self + rhs,
			Self::Rational(rhs) => self + rhs,
		}
	}
}

impl<T> Div<T> for Number
where
	Integer: Div<T, Output: Into<Self>>,
	Rational: Div<T, Output: Into<Self>>,
{
	type Output = Self;

	fn div(self, rhs: T) -> Self::Output {
		match self {
			Self::Integer(lhs) | Self::Natural(lhs) => (lhs / rhs).into(),
			Self::Rational(lhs) => (lhs / rhs).into(),
		}
	}
}

impl Div for Number {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		match rhs {
			Self::Integer(rhs) | Self::Natural(rhs) => self / rhs,
			Self::Rational(rhs) => self / rhs,
		}
	}
}

impl<T> Mul<T> for Number
where
	Integer: Mul<T, Output: Into<Self>>,
	Rational: Mul<T, Output: Into<Self>>,
{
	type Output = Self;

	fn mul(self, rhs: T) -> Self::Output {
		match self {
			Self::Integer(lhs) | Self::Natural(lhs) => (lhs * rhs).into(),
			Self::Rational(lhs) => (lhs * rhs).into(),
		}
	}
}

impl Mul for Number {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		match rhs {
			Self::Integer(rhs) | Self::Natural(rhs) => self * rhs,
			Self::Rational(rhs) => self * rhs,
		}
	}
}

impl Neg for Number {
	type Output = Self;

	fn neg(self) -> Self::Output {
		match self {
			Self::Integer(n) | Self::Natural(n) => (-n).into(),
			Self::Rational(n) => (-n).into(),
		}
	}
}

impl Rem for Number {
	type Output = Self;

	fn rem(self, rhs: Self) -> Self {
		match (self, rhs) {
			(Self::Integer(lhs), Self::Integer(rhs)) => (lhs % rhs).into(),
			(Self::Integer(lhs), Self::Natural(rhs)) => (lhs % rhs).into(),
			(Self::Natural(lhs), Self::Integer(rhs)) => (lhs % rhs).into(),
			(Self::Natural(lhs), Self::Natural(rhs)) => (lhs % rhs).into(),
			_ => unimplemented!(),
		}
	}
}

impl<T> Sub<T> for Number
where
	Integer: Sub<T, Output: Into<Self>>,
	Rational: Sub<T, Output: Into<Self>>,
{
	type Output = Self;

	fn sub(self, rhs: T) -> Self::Output {
		match self {
			Self::Integer(lhs) | Self::Natural(lhs) => (lhs - rhs).into(),
			Self::Rational(lhs) => (lhs - rhs).into(),
		}
	}
}

impl Sub for Number {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		match rhs {
			Self::Integer(rhs) | Self::Natural(rhs) => self - rhs,
			Self::Rational(rhs) => self - rhs,
		}
	}
}

impl Display for Number {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Integer(inner) | Self::Natural(inner) => inner.fmt(f),
			Self::Rational(inner) => inner.fmt(f),
		}
	}
}

impl str::FromStr for Number {
	type Err = ParseRationalError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Rational::from_str(s).map(Self::from)
	}
}
