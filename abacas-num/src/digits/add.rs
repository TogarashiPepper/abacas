//! Addition implementations for digits.

use std::mem;
use std::ops::AddAssign;

use crate::digits::Digits;

impl AddAssign for Digits {
	fn add_assign(&mut self, mut rhs: Self) {
		if self.capacity() < rhs.capacity() {
			mem::swap(self, &mut rhs);
		}

		self.add_assign(&rhs);
	}
}

impl AddAssign<&Self> for Digits {
	fn add_assign(&mut self, rhs: &Self) {
		let len = self.len().max(rhs.len());

		let mut carry = false;
		let mut sum;

		for index in 0..len {
			let lhs = self.get(index).copied().unwrap_or_default();
			let rhs = rhs.get(index).copied().unwrap_or_default();

			(sum, carry) = lhs.carrying_add(rhs, carry);

			if let Some(lhs) = self.get_mut(index) {
				*lhs = sum;
			} else {
				self.push(sum);
			}
		}

		if carry {
			self.push(1);
		}
	}
}
