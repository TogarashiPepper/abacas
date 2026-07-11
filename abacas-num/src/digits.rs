//! The digit type and related items.

use std::num::NonZeroUsize;
use std::ops::{Deref, DerefMut};

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
	/// Checks whether this digit list is the number one (1).
	pub const fn is_one(&self) -> bool {
		matches!(self.as_slice(), [1])
	}

	/// Checks whether this digit list is the number zero (0).
	pub const fn is_zero(&self) -> bool {
		matches!(self.as_slice(), [])
	}
}

impl Digits {
	/// Converts this digit list into a mutable slice of digits.
	pub const fn as_mut_slice(&mut self) -> &mut [Digit] {
		match self {
			Self::Heap(heap) => heap.as_mut_slice(),
			Self::Stack(stack) => stack.as_mut_slice(),
		}
	}

	/// Converts this digit list into a slice of digits.
	pub const fn as_slice(&self) -> &[Digit] {
		match self {
			Self::Heap(heap) => heap.as_slice(),
			Self::Stack(stack) => stack.as_slice(),
		}
	}

	/// Gets the capacity of this digit list.
	pub const fn capacity(&self) -> NonZeroUsize {
		match self {
			Self::Heap(heap) => match NonZeroUsize::new(heap.capacity()) {
				None => NonZeroUsize::MIN,
				Some(capacity) => capacity,
			},
			Self::Stack(_) => NonZeroUsize::MIN,
		}
	}

	/// Adds a new digit to the list.
	pub fn push(&mut self, digit: Digit) {
		match self {
			Self::Heap(heap) => match heap.capacity() {
				0 => *self = Self::Stack(Some(digit)),
				_ => heap.push(digit),
			},
			Self::Stack(stack) => match stack.take() {
				None => *self = Self::Stack(Some(digit)),
				Some(stack) => *self = Self::Heap(vec![stack, digit]),
			},
		}
	}
}

impl Default for Digits {
	fn default() -> Self {
		Self::ZERO
	}
}

impl Deref for Digits {
	type Target = [Digit];

	fn deref(&self) -> &Self::Target {
		self.as_slice()
	}
}

impl DerefMut for Digits {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.as_mut_slice()
	}
}
