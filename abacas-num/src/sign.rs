//! The sign enum and related items.

use std::ops::Neg;

use crate::ops::NegAssign;

/// Represents the sign of a number.
#[derive(Clone, Copy, Debug)]
pub enum Sign {
	/// A negative number.
	Minus,
	/// A positive number.
	Plus,
	/// The number zero.
	Zero,
}

impl Sign {
	/// Whether this sign is negative.
	pub const fn is_negative(self) -> bool {
		matches!(self, Self::Minus)
	}

	/// Whether this sign is positive.
	pub const fn is_positive(self) -> bool {
		matches!(self, Self::Plus)
	}

	/// Whether this sign is zero.
	pub const fn is_zero(self) -> bool {
		matches!(self, Self::Zero)
	}
}

impl Neg for Sign {
	type Output = Self;

	fn neg(mut self) -> Self::Output {
		self.neg_assign();
		self
	}
}

impl NegAssign for Sign {
	fn neg_assign(&mut self) {
		match self {
			Self::Minus => *self = Self::Plus,
			Self::Plus => *self = Self::Minus,
			Self::Zero => (),
		}
	}
}
