//! The digit type and related items.

use std::slice;

/// Represents a single digit.
type Digit = u64;

/// Represents a list of digits with more optimal memory usage.
#[derive(Debug)]
pub enum Digits {
	/// A list of values on the heap.
	Heap(Vec<Digit>),
	/// A single value on the stack.
	Stack(Option<Digit>),
}

impl Digits {
	/// The number one (1).
	pub const ONE: Self = Self::Stack(Some(1));

	/// The number zero (0).
	pub const ZERO: Self = Self::Stack(None);
}

impl Digits {
	/// Checks whether these digits are the number one (1).
	pub const fn is_one(&self) -> bool {
		matches!(self.as_slice(), [1])
	}

	/// Checks whether this natural is the number zero (0).
	pub const fn is_zero(&self) -> bool {
		matches!(self.as_slice(), [])
	}
}

impl Digits {
	/// Converts these digits into a slice of digits.
	pub const fn as_slice(&self) -> &[Digit] {
		match self {
			Self::Heap(heap) => heap.as_slice(),
			Self::Stack(None) => &[],
			Self::Stack(Some(stack)) => slice::from_ref(stack),
		}
	}
}

impl Default for Digits {
	fn default() -> Self {
		Self::ZERO
	}
}
