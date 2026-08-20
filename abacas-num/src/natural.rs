//! The natural struct and related items.

use crate::list::List;

/// Represents a natural number.
#[derive(Clone, Debug)]
pub struct Natural {
	/// The digits that constitute this natural.
	digits: List<usize>,
}

impl Natural {
	/// The number one (1).
	pub const ONE: Self = Self::new(List::singleton(1));

	/// The number zero (0).
	pub const ZERO: Self = Self::new(List::new());
}

impl Natural {
	/// Creates a new natural.
	const fn new(digits: List<usize>) -> Self {
		Self { digits }
	}
}

impl Natural {
	/// Checks whether this natural is the number one (1).
	pub const fn is_one(&self) -> bool {
		matches!(self.digits.as_slice(), [1])
	}

	/// Checks whether this natural is the number zero (0).
	pub const fn is_zero(&self) -> bool {
		matches!(self.digits.as_slice(), [])
	}
}

impl Default for Natural {
	fn default() -> Self {
		Self::ZERO
	}
}
