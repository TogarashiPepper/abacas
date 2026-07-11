//! The natural struct and related items.

use std::mem;
use std::ops::AddAssign;

use crate::digits::Digits;

/// Represents a natural number.
#[derive(Debug, Default)]
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

impl AddAssign for Natural {
	fn add_assign(&mut self, mut rhs: Self) {
		if self.digits.capacity() < rhs.digits.capacity() {
			mem::swap(self, &mut rhs);
		}

		self.add_assign(&rhs);
	}
}

impl AddAssign<&Self> for Natural {
	fn add_assign(&mut self, rhs: &Self) {
		let len = self.digits.len().max(rhs.digits.len());

		let mut carry = false;
		let mut sum;

		for index in 0..len {
			let lhs = self.digits.get(index).copied().unwrap_or_default();
			let rhs = rhs.digits.get(index).copied().unwrap_or_default();

			(sum, carry) = lhs.carrying_add(rhs, carry);

			if let Some(lhs) = self.digits.get_mut(index) {
				*lhs = sum;
			} else {
				self.digits.push(sum);
			}
		}

		if carry {
			self.digits.push(1);
		}
	}
}
