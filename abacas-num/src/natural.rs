//! The natural struct and related items.

use crate::digits::Digits;

/// Represents a natural number.
#[derive(Clone, Debug)]
pub struct Natural {
	/// The digits that constitute this natural.
	digits: Digits,
}

impl Natural {
	/// The number one (1).
	pub const ONE: Self = Self::new(Digits::ONE);

	/// The number zero (0).
	pub const ZERO: Self = Self::new(Digits::ZERO);
}

impl Natural {
	/// Creates a new natural.
	const fn new(digits: Digits) -> Self {
		Self { digits }
	}
}

impl Natural {
	/// Checks whether this natural is the number one (1).
	pub const fn is_one(&self) -> bool {
		self.digits.is_one()
	}

	/// Checks whether this natural is the number zero (0).
	pub const fn is_zero(&self) -> bool {
		self.digits.is_zero()
	}
}

impl Default for Natural {
	fn default() -> Self {
		Self::ZERO
	}
}
