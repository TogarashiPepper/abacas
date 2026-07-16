//! The integer struct and related items.

use std::ops::Neg;

use crate::natural::Natural;
use crate::ops::NegAssign;
use crate::sign::Sign;

/// Represents an integer.
#[derive(Clone, Debug)]
pub struct Integer {
	/// The absolute value of this integer.
	abs: Natural,
	/// The sign of this integer.
	sign: Sign,
}

impl Integer {
	/// The number negative one (-1).
	pub const NEG_ONE: Self = Self::new(Natural::ONE, Sign::Minus);

	/// The number one (1).
	pub const ONE: Self = Self::new(Natural::ONE, Sign::Plus);

	/// The number zero (0).
	pub const ZERO: Self = Self::new(Natural::ZERO, Sign::Zero);
}

impl Integer {
	/// Creates a new integer.
	const fn new(abs: Natural, sign: Sign) -> Self {
		Self { abs, sign }
	}
}

impl Integer {
	/// Whether this integer is the number negative one (-1).
	pub const fn is_neg_one(&self) -> bool {
		self.abs.is_one() && self.sign.is_negative()
	}

	/// Whether this integer is negative.
	pub const fn is_negative(&self) -> bool {
		self.sign.is_negative()
	}

	/// Whether this integer is the number one (1).
	pub const fn is_one(&self) -> bool {
		self.abs.is_one() && self.sign.is_positive()
	}

	/// Whether this integer is positive.
	pub const fn is_positive(&self) -> bool {
		self.sign.is_positive()
	}

	/// Whether this integer is the number zero (0).
	pub const fn is_zero(&self) -> bool {
		self.sign.is_zero()
	}
}

impl Integer {
	/// Returns a reference to the absolute value.
	pub const fn abs(&self) -> &Natural {
		&self.abs
	}

	/// Returns the sign of this rational.
	pub const fn sign(&self) -> Sign {
		self.sign
	}
}

impl Default for Integer {
	fn default() -> Self {
		Self::ZERO
	}
}

impl Neg for Integer {
	type Output = Self;

	fn neg(mut self) -> Self::Output {
		self.neg_assign();
		self
	}
}

impl NegAssign for Integer {
	fn neg_assign(&mut self) {
		self.sign.neg_assign();
	}
}
