//! Iterator methods to convert into digits.

use std::iter::Copied;
use std::mem;
use std::slice::Iter;

use crate::digits::{Digit, Digits};

/// Types that can iterate over their contained digits.
pub trait IterDigits {
	/// The type of iterator that is being returned.
	type Iter<'a>: Iterator<Item = Digit>
	where
		Self: 'a;

	/// Iterates over the contained digits.
	fn iter_digits(&self) -> Self::Iter<'_>;
}

impl IterDigits for Digits {
	type Iter<'a> = Copied<Iter<'a, Digit>>;

	fn iter_digits(&self) -> Self::Iter<'_> {
		self.iter().copied()
	}
}

/// An iterator over the digits of an unsigned primitive.
pub struct IterDigitsPrim<T>(T);

/// Implements the relevant traits for unsigned primitives.
macro_rules! impl_unsigned {
	($target:ty) => {
		impl Iterator for IterDigitsPrim<$target> {
			type Item = Digit;

			fn next(&mut self) -> Option<Self::Item> {
				let shifted = self.0.unbounded_shr(Digit::BITS);

				match self.0 {
					0 => None,
					_ => Some(mem::replace(&mut self.0, shifted) as Digit),
				}
			}
		}

		impl IterDigits for $target {
			type Iter<'a> = IterDigitsPrim<Self>;

			fn iter_digits(&self) -> Self::Iter<'_> {
				IterDigitsPrim(*self)
			}
		}
	};

	($($target:ty)*) => {
		$(impl_unsigned! { $target })*
	};
}

impl_unsigned! {
	u8 u16 u32 u64 u128 usize
}
