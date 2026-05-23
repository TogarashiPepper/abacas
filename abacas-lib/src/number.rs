//! The number enum and its related operations.

use std::fmt::{self, Display};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};
use std::str;

use rug::Rational;
use rug::ops::{NegAssign, Pow, PowAssign};
use rug::rational::ParseRationalError;

/// Represents a number of any supported set.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Number(Rational);

impl Number {
	/// The number zero.
	pub fn zero() -> Self {
		Self::new(0u64, 1u64)
	}

	/// The number one.
	pub fn one() -> Self {
		Self::new(1u64, 1u64)
	}

	/// The number pi.
	pub fn pi() -> Self {
		Self::new(3141592653589793u64, 1000000000000000u64)
	}

	/// The eulers number.
	pub fn e() -> Self {
		Self::new(2718281828459045u64, 1000000000000000u64)
	}

	/// Create a new Number from numerator and denominator.
	pub fn new(num: u64, denom: u64) -> Self {
		Self::from_rational(Rational::from(((num), (denom))))
	}

	/// Create a new Number from f64.
	pub fn from_f64(coeff: f64) -> Option<Self> {
		Some(Self::from_rational(Rational::from_f64(coeff)?))
	}

	/// Create a new Number from rug Rational type.
	pub fn from_rational(number: Rational) -> Self {
		Self(number)
	}

	/// Whether this Number is 0.
	pub fn is_zero(&self) -> bool {
		self.0.is_zero()
	}

	/// Whether this Number is 1.
	pub fn is_one(&self) -> bool {
		self == &Self::one()
	}

	/// Whether this Number is Greater than 0.
	pub fn is_positive(&self) -> bool {
		self > &Self::zero()
	}

	/// Whether this Number is Lesser than 0.
	pub fn is_negative(&self) -> bool {
		self < &Self::zero()
	}

	/// Whether this Number is Greater than or Equal to 0.
	pub fn is_non_negative(&self) -> bool {
		self >= &Self::zero()
	}

	/// Whether this Number is Lesser than or Equal to 0.
	pub fn is_non_positive(&self) -> bool {
		self <= &Self::zero()
	}

	/// Get the numerator of this Number.
	pub fn numer(&self) -> Number {
		Self::from_rational(self.0.numer().into())
	}

	/// Get the denominator of this Number.
	pub fn denom(&self) -> Number {
		Self::from_rational(self.0.denom().into())
	}

	/// Get the numerator and denominator of this Number as a tuple.
	pub fn numer_denom(&self) -> (Number, Number) {
		(self.numer(), self.denom())
	}

	/// Function that calculates the greatest common divisor.
	pub fn gcd(lhs: Self, rhs: Self) -> Self {
		let (numer, denom) = lhs.0.into_numer_denom();

		// See: https://math.stackexchange.com/a/199905
		(numer.gcd(rhs.0.numer()), denom.lcm(rhs.0.denom())).into()
	}

	/// Reciprocal of this Number.
	pub fn recip(&self) -> Self {
		Self(self.0.clone().recip())
	}
}

impl Default for Number {
	fn default() -> Self {
		Self::zero()
	}
}

impl<T: Into<Rational>> From<T> for Number {
	fn from(number: T) -> Self {
		Self(number.into())
	}
}

impl AddAssign for Number {
	fn add_assign(&mut self, rhs: Self) {
		self.0 = self.0.clone() + rhs.0;
	}
}

impl Add for Number {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self::from_rational(self.0 + rhs.0)
	}
}

impl DivAssign for Number {
	fn div_assign(&mut self, rhs: Self) {
		self.0 = self.0.clone() / rhs.0;
	}
}

impl Div for Number {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		Self::from_rational(self.0 / rhs.0)
	}
}

impl MulAssign for Number {
	fn mul_assign(&mut self, rhs: Self) {
		self.0 = self.0.clone() * rhs.0;
	}
}

impl Mul for Number {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Self::from_rational(self.0 * rhs.0)
	}
}

impl NegAssign for Number {
	fn neg_assign(&mut self) {
		self.0 = -self.0.clone()
	}
}

impl Neg for Number {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Self::from_rational(-self.0)
	}
}

impl PowAssign<Number> for Number {
	fn pow_assign(&mut self, rhs: Self) {
		todo!()
	}
}

impl Pow<Number> for Number {
	type Output = Self;

	fn pow(self, rhs: Self) -> Self::Output {
		todo!()
	}
}

impl Rem for Number {
	type Output = Self;

	fn rem(self, rhs: Self) -> Self::Output {
		Self::from_rational(self.0.clone() - rhs.0.clone() * (self.0 / rhs.0).trunc())
	}
}

impl SubAssign for Number {
	fn sub_assign(&mut self, rhs: Self) {
		self.0 = self.0.clone() - rhs.0;
	}
}

impl Sub for Number {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self::from_rational(self.0 - rhs.0)
	}
}

impl Display for Number {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}

impl str::FromStr for Number {
	type Err = ParseRationalError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Rational::from_str(s).map(Self::from_rational)
	}
}
