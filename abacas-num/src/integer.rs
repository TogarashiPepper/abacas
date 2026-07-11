//! The integer struct and related items.

use crate::natural::Natural;

/// Represents an integer.
#[derive(Debug, Default)]
pub struct Integer {
	/// The absolute value of this integer.
	natural: Natural,
	/// Whether this integer is negative.
	negative: bool,
}

impl Integer {
	/// The number negative one (-1).
	pub const NEG_ONE: Self = Self::new(Natural::ONE, true);

	/// The number one (1).
	pub const ONE: Self = Self::new(Natural::ONE, false);

	/// The number zero (0).
	pub const ZERO: Self = Self::new(Natural::ZERO, false);
}

impl Integer {
	/// Creates a new integer.
	const fn new(natural: Natural, negative: bool) -> Self {
		Self { natural, negative }
	}
}

impl Integer {
	/// Whether this integer is the number negative one (-1).
	pub const fn is_neg_one(&self) -> bool {
		self.natural.is_one() && self.negative
	}

	/// Whether this integer is negative.
	pub const fn is_negative(&self) -> bool {
		self.negative
	}

	/// Whether this integer is the number one (1).
	pub const fn is_one(&self) -> bool {
		self.natural.is_one() && !self.negative
	}

	/// Whether this integer is positive.
	pub const fn is_positive(&self) -> bool {
		!self.natural.is_zero() && !self.negative
	}

	/// Whether this integer is the number zero (0).
	pub const fn is_zero(&self) -> bool {
		self.natural.is_zero()
	}
}
