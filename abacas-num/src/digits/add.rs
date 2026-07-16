//! Addition implementations for digits.

use std::mem;
use std::ops::AddAssign;

use crate::digits::Digits;
use crate::digits::iter::IterDigits;

impl AddAssign for Digits {
	fn add_assign(&mut self, mut rhs: Self) {
		if self.capacity() < rhs.capacity() {
			mem::swap(self, &mut rhs);
		}

		self.add_assign(&rhs);
	}
}

/// Implements addition with a common algorithm.
macro_rules! impl_add {
	($target:ty) => {
		impl AddAssign<$target> for Digits {
			fn add_assign(&mut self, rhs: $target) {
				let mut carry = false;
				let mut sum;

				for (index, rhs) in rhs.iter_digits().enumerate() {
					(sum, carry) = self.digit(index).carrying_add(rhs, carry);

					match self.get_mut(index) {
						None => self.push(sum),
						Some(digit) => *digit = sum,
					}
				}

				if carry {
					self.push(1);
				}
			}
		}
	};

	($($target:ty)*) => {
		$(impl_add! { $target })*
	};
}

impl_add! {
	u8 u16 u32 u64 u128 usize
	&u8 &u16 &u32 &u64 &u128 &usize
	&Digits
}
