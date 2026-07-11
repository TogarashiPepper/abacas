//! The rational struct and related items.

use crate::integer::Integer;
use crate::natural::Natural;

/// Represents a rational number.
#[derive(Debug)]
pub struct Rational {
	/// The denominator of this rational.
	denom: Natural,
	/// The numerator of this rational.
	numer: Integer,
}

impl Rational {
	/// The number negative one (-1).
	pub const NEG_ONE: Self = Self::new(Natural::ONE, Integer::NEG_ONE);

	/// The number one (1).
	pub const ONE: Self = Self::new(Natural::ONE, Integer::ONE);

	/// The number zero (0).
	pub const ZERO: Self = Self::new(Natural::ONE, Integer::ZERO);
}

impl Rational {
	/// Creates a new rational.
	const fn new(denom: Natural, numer: Integer) -> Self {
		Self { denom, numer }
	}
}

impl Rational {
	/// Whether this rational is an integer.
	pub const fn is_integer(&self) -> bool {
		self.denom.is_one()
	}

	/// Whether this rational is the number negative one (-1).
	pub const fn is_neg_one(&self) -> bool {
		self.denom.is_one() && self.numer.is_neg_one()
	}

	/// Whether this rational is negative.
	pub const fn is_negative(&self) -> bool {
		self.numer.is_negative()
	}

	/// Whether this rational is the number one (1).
	pub const fn is_one(&self) -> bool {
		self.denom.is_one() && self.numer.is_one()
	}

	/// Whether this rational is positive.
	pub const fn is_positive(&self) -> bool {
		self.numer.is_positive()
	}

	/// Whether this rational is the number zero (0).
	pub const fn is_zero(&self) -> bool {
		self.numer.is_zero()
	}
}

impl Rational {
	/// Returns a reference to the denominator.
	pub const fn denom(&self) -> &Natural {
		&self.denom
	}

	/// Returns a reference to the numerator.
	pub const fn numer(&self) -> &Integer {
		&self.numer
	}
}

impl Default for Rational {
	fn default() -> Self {
		Self::ZERO
	}
}
