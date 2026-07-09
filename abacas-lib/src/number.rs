//! The number structure and its related operations.

use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
use std::{fmt, str};

use num::traits::{One, Pow, Signed, ToPrimitive, Zero};
use num::{BigRational, Integer};

use crate::error::Error;

/// Represents a specific number. Currently uses [`BigRational`] under the hood, however this should not be relied upon.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Number(BigRational);

// Constants
impl Number {
	/// The number negative one (`-1`).
	pub fn neg_one() -> Self {
		Self(-BigRational::ONE)
	}

	/// The number one (`1`).
	pub fn one() -> Self {
		Self(BigRational::ONE)
	}

	/// The number zero (`0`).
	pub fn zero() -> Self {
		Self(BigRational::ZERO)
	}
}

// Guards
impl Number {
	/// Whether this number is an integer.
	pub fn is_integer(&self) -> bool {
		self.0.is_integer()
	}

	/// Whether this is the number negative one (`-1`).
	pub fn is_neg_one(&self) -> bool {
		self.0 == -BigRational::ONE
	}

	/// Whether this number is less than zero.
	pub fn is_negative(&self) -> bool {
		self.0.is_negative()
	}

	/// Whether this is the number one (`1`).
	pub fn is_one(&self) -> bool {
		self.0.is_one()
	}

	/// Whether this number is greater than zero.
	pub fn is_positive(&self) -> bool {
		self.0.is_positive()
	}

	/// Whether this is the number zero (`0`).
	pub fn is_zero(&self) -> bool {
		self.0.is_zero()
	}
}

// Operations
impl Number {
	/// Gets the denominator of this number.
	pub fn denom(self) -> Self {
		Self(self.0.into_raw().1.into())
	}

	/// Gets the greatest common divisor.
	pub fn gcd(self, rhs: &Self) -> Self {
		Self(BigRational::new(
			self.0.numer().gcd(rhs.0.numer()),
			self.0.denom().lcm(rhs.0.denom()),
		))
	}

	/// Gets the least common multiple.
	pub fn lcm(self, rhs: &Self) -> Self {
		Self(BigRational::new(
			self.0.numer().lcm(rhs.0.numer()),
			self.0.denom().gcd(rhs.0.denom()),
		))
	}

	/// Gets the numerator of this number.
	pub fn numer(self) -> Self {
		Self(self.0.into_raw().0.into())
	}

	/// Gets the numerator and denominator of this number as a tuple.
	pub fn ratio(self) -> (Self, Self) {
		let (numer, denom) = self.0.into_raw();
		(Self(numer.into()), Self(denom.into()))
	}

	/// Converts this number into an [`f32`].
	pub fn to_f32(&self) -> f32 {
		self.0.to_f32().unwrap()
	}

	/// Converts this number into an [`f64`].
	pub fn to_f64(&self) -> f64 {
		self.0.to_f64().unwrap()
	}

	/// Internal method to write this number with specific configuration.
	pub(crate) fn write(&self, f: &mut fmt::Formatter<'_>, abs: bool) -> fmt::Result {
		if abs {
			write!(f, "{}", self.to_f64().abs())
		} else {
			write!(f, "{}", self.to_f64())
		}
	}
}

impl<T> Add<T> for Number
where
	Self: AddAssign<T>,
{
	type Output = Self;

	fn add(mut self, rhs: T) -> Self::Output {
		self += rhs;
		self
	}
}

impl AddAssign<&Self> for Number {
	fn add_assign(&mut self, rhs: &Self) {
		self.0 += &rhs.0;
	}
}

impl<T> Div<T> for Number
where
	Self: DivAssign<T>,
{
	type Output = Self;

	fn div(mut self, rhs: T) -> Self::Output {
		self /= rhs;
		self
	}
}

impl DivAssign<&Self> for Number {
	fn div_assign(&mut self, rhs: &Self) {
		self.0 /= &rhs.0;
	}
}

impl<T> Mul<T> for Number
where
	Self: MulAssign<T>,
{
	type Output = Self;

	fn mul(mut self, rhs: T) -> Self::Output {
		self *= rhs;
		self
	}
}

impl MulAssign<&Self> for Number {
	fn mul_assign(&mut self, rhs: &Self) {
		self.0 *= &rhs.0;
	}
}

impl Neg for Number {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Self(-self.0)
	}
}

impl Pow<&Self> for Number {
	type Output = Self;

	fn pow(self, rhs: &Self) -> Self::Output {
		if !rhs.is_integer() {
			panic!("exponent must be an integer");
		}

		Self(self.0.pow(rhs.0.numer()))
	}
}

impl<T> Rem<T> for Number
where
	Self: RemAssign<T>,
{
	type Output = Self;

	fn rem(mut self, rhs: T) -> Self::Output {
		self %= rhs;
		self
	}
}

impl RemAssign<&Self> for Number {
	fn rem_assign(&mut self, rhs: &Self) {
		self.0 %= &rhs.0;
	}
}

impl<T> Sub<T> for Number
where
	Self: SubAssign<T>,
{
	type Output = Self;

	fn sub(mut self, rhs: T) -> Self::Output {
		self -= rhs;
		self
	}
}

impl SubAssign<&Self> for Number {
	fn sub_assign(&mut self, rhs: &Self) {
		self.0 -= &rhs.0;
	}
}

impl fmt::Display for Number {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.write(f, false)
	}
}

impl str::FromStr for Number {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let full = match s.split_once('.') {
			Some((int, fract)) => format!("{}{}/1{}", int, fract, "0".repeat(fract.len())),
			None => s.into(),
		};

		full.parse().map(Self).map_err(|_| Error::InvalidString(full))
	}
}

macro_rules! impl_int {
	($($int:ty,)*) => {
		$(
			impl From<$int> for Number {
				fn from(value: $int) -> Self {
					Self(BigRational::from_integer(value.into()))
				}
			}

			impl AddAssign<$int> for Number {
				fn add_assign(&mut self, rhs: $int) {
					self.0 += BigRational::from_integer(rhs.into());
				}
			}

			impl DivAssign<$int> for Number {
				fn div_assign(&mut self, rhs: $int) {
					self.0 /= BigRational::from_integer(rhs.into());
				}
			}

			impl MulAssign<$int> for Number {
				fn mul_assign(&mut self, rhs: $int) {
					self.0 *= BigRational::from_integer(rhs.into());
				}
			}

			impl PartialEq<$int> for Number {
				fn eq(&self, other: &$int) -> bool {
					self.0 == BigRational::from_integer((*other).into())
				}
			}

			impl PartialOrd<$int> for Number {
				fn partial_cmp(&self, other: &$int) -> Option<Ordering> {
					self.0.partial_cmp(&BigRational::from_integer((*other).into()))
				}
			}

			impl Pow<$int> for Number {
				type Output = Self;

				fn pow(self, rhs: $int) -> Self::Output {
					Self(self.0.pow(rhs))
				}
			}

			impl RemAssign<$int> for Number {
				fn rem_assign(&mut self, rhs: $int) {
					self.0 %= BigRational::from_integer(rhs.into());
				}
			}

			impl SubAssign<$int> for Number {
				fn sub_assign(&mut self, rhs: $int) {
					self.0 -= BigRational::from_integer(rhs.into());
				}
			}
		)*
	};
}

macro_rules! impl_rational {
	($($name:ident, $name_mut:ident, $doc:literal;)*) => {
		impl Number {
			$(
				#[doc = concat!("Gets the ", $doc, " of this number.")]
				pub fn $name(&self) -> Self {
					Self(self.0.$name())
				}

				#[doc = concat!("Gets the ", $doc, " of this number and assigns it in-place.")]
				pub fn $name_mut(&mut self) {
					self.0 = self.0.$name();
				}
			)*
		}
	};
}

impl_int! {
	i8, i16, i32, i64, i128, isize,
	u8, u16, u32, u64, u128, usize,
}

impl_rational! {
	abs, abs_mut, "absolute value";
	ceil, ceil_mut, "ceiled integer";
	floor, floor_mut, "floored integer";
	recip, recip_mut, "reciprocal value";
	round, round_mut, "rounded integer";
	trunc, trunc_mut, "truncated integer";
}
