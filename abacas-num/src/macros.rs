//! Collection of macros used across the crate.

use std::iter::{Product, Sum};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

use crate::integer::Integer;
use crate::natural::Natural;
use crate::ops::{Pow, PowAssign};
use crate::rational::Rational;

/// Forwards trait implementations that only depend on other traits.
macro_rules! forward_impls {
	($target:ty) => {
		impl<T> Add<T> for $target
		where
			Self: AddAssign<T>,
		{
			type Output = Self;

			fn add(mut self, rhs: T) -> Self::Output {
				self.add_assign(rhs);
				self
			}
		}

		impl<T> Div<T> for $target
		where
			Self: DivAssign<T>,
		{
			type Output = Self;

			fn div(mut self, rhs: T) -> Self::Output {
				self.div_assign(rhs);
				self
			}
		}

		impl<T> Mul<T> for $target
		where
			Self: MulAssign<T>,
		{
			type Output = Self;

			fn mul(mut self, rhs: T) -> Self::Output {
				self.mul_assign(rhs);
				self
			}
		}

		impl<T> Pow<T> for $target
		where
			Self: PowAssign<T>,
		{
			type Output = Self;

			fn pow(mut self, rhs: T) -> Self::Output {
				self.pow_assign(rhs);
				self
			}
		}

		impl<T> Product<T> for $target
		where
			Self: Mul<T, Output = Self>,
		{
			fn product<I>(iter: I) -> Self
			where
				I: Iterator<Item = T>,
			{
				iter.fold(Self::ONE, Self::mul)
			}
		}

		impl<T> Rem<T> for $target
		where
			Self: RemAssign<T>,
		{
			type Output = Self;

			fn rem(mut self, rhs: T) -> Self::Output {
				self.rem_assign(rhs);
				self
			}
		}

		impl<T> Sub<T> for $target
		where
			Self: SubAssign<T>,
		{
			type Output = Self;

			fn sub(mut self, rhs: T) -> Self::Output {
				self.sub_assign(rhs);
				self
			}
		}

		impl<T> Sum<T> for $target
		where
			Self: Add<T, Output = Self>,
		{
			fn sum<I>(iter: I) -> Self
			where
				I: Iterator<Item = T>,
			{
				iter.fold(Self::ZERO, Self::add)
			}
		}
	};
}

forward_impls!(Integer);
forward_impls!(Natural);
forward_impls!(Rational);
