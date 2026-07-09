//! The number structure and its related operations.

use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
use std::{fmt, str};

use num::integer::Integer;
use num::traits::{FromPrimitive, One, Pow, ToPrimitive};
use num::{BigRational, Signed, Zero};

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
		BigRational::is_integer(&self.0)
	}

	/// Whether this is the number negative one (`-1`).
	pub fn is_neg_one(&self) -> bool {
		self.0 == -BigRational::ONE
	}

	/// Whether this number is less than zero.
	pub fn is_negative(&self) -> bool {
		BigRational::is_negative(&self.0)
	}

	/// Whether this is the number one (`1`).
	pub fn is_one(&self) -> bool {
		BigRational::is_one(&self.0)
	}

	/// Whether this number is greater than zero.
	pub fn is_positive(&self) -> bool {
		BigRational::is_positive(&self.0)
	}

	/// Whether this is the number zero (`0`).
	pub fn is_zero(&self) -> bool {
		BigRational::is_zero(&self.0)
	}
}

// Operations
impl Number {
	/// Gets the denominator of this number.
	pub fn denom(self) -> Self {
		Self(self.0.denom().clone().into())
	}

	/// Performs division, rounding the quotient up.
	pub fn div_ceil(mut self, rhs: &Self) -> Self {
		self.div_ceil_assign(rhs);
		self
	}

	/// Performs division, rounding the quotient up and assigns it in-place.
	pub fn div_ceil_assign(&mut self, rhs: &Self) {
		self.0 /= &rhs.0;
		self.ceil_mut();
	}

	/// Performs Euclidean division, rounding the quotient so that the remainder cannot be negative.
	pub fn div_euc(mut self, rhs: &Self) -> Self {
		self.div_euc_assign(rhs);
		self
	}

	/// Performs Euclidean division, rounding the quotient so that the remainder cannot be negative and assigns it in-place.
	pub fn div_euc_assign(&mut self, rhs: &Self) {
		if rhs.is_positive() {
			self.div_floor_assign(rhs);
		} else {
			self.div_ceil_assign(rhs);
		}
	}

	/// Performs division, rounding the quotient down.
	pub fn div_floor(mut self, rhs: &Self) -> Self {
		self.div_floor_assign(rhs);
		self
	}

	/// Performs division, rounding the quotient down and assigns it in-place.
	pub fn div_floor_assign(&mut self, rhs: &Self) {
		self.0 /= &rhs.0;
		self.floor_mut();
	}

	/// Performs division, rounding the quotient towards zero.
	pub fn div_trunc(mut self, rhs: &Self) -> Self {
		self.div_trunc_assign(rhs);
		self
	}

	/// Performs division, rounding the quotient towards zero and assigns it in-place.
	pub fn div_trunc_assign(&mut self, rhs: &Self) {
		self.0 /= &rhs.0;
		self.trunc_mut();
	}

	/// Gets the greatest common divisor.
	pub fn gcd(mut self, rhs: &Self) -> Self {
		self.gcd_mut(rhs);
		self
	}

	/// Gets the greatest common divisor and assigns it in-place.
	pub fn gcd_mut(&mut self, rhs: &Self) {
		self.0 = BigRational::new(self.0.numer().gcd(rhs.0.numer()), self.0.denom().lcm(rhs.0.denom()))
	}

	/// Gets the least common multiple.
	pub fn lcm(mut self, rhs: &Self) -> Self {
		self.lcm_mut(rhs);
		self
	}

	/// Gets the least common multiple and assigns it in-place.
	pub fn lcm_mut(&mut self, rhs: &Self) {
		self.0 = BigRational::new(self.0.numer().lcm(rhs.0.numer()), self.0.denom().gcd(rhs.0.denom()))
	}

	/// Gets the numerator of this number.
	pub fn numer(self) -> Self {
		Self(self.0.numer().clone().into())
	}

	/// Gets the numerator and denominator of this number as a tuple.
	pub fn ratio(self) -> (Self, Self) {
		(self.clone().numer(), self.denom())
	}

	/// Finds the remainder when the quotient is rounded up.
	pub fn rem_ceil(mut self, rhs: &Self) -> Self {
		self.rem_ceil_assign(rhs);
		self
	}

	/// Finds the remainder when the quotient is rounded up and assigns it in-place.
	pub fn rem_ceil_assign(&mut self, rhs: &Self) {
		self.0 -= self.clone().div_ceil(rhs).0 * &rhs.0;
	}

	/// Finds the positive remainder from Euclidean division.
	pub fn rem_euc(mut self, rhs: &Self) -> Self {
		self.rem_euc_assign(rhs);
		self
	}

	/// Finds the positive remainder from Euclidean division and assigns it in-place.
	pub fn rem_euc_assign(&mut self, rhs: &Self) {
		self.0 -= self.clone().div_euc(rhs).0 * &rhs.0;
	}

	/// Finds the remainder when the quotient is rounded down.
	pub fn rem_floor(mut self, rhs: &Self) -> Self {
		self.rem_floor_assign(rhs);
		self
	}

	/// Finds the remainder when the quotient is rounded down and assigns it in-place.
	pub fn rem_floor_assign(&mut self, rhs: &Self) {
		self.0 -= self.clone().div_floor(rhs).0 * &rhs.0;
	}

	/// Finds the remainder when the quotient is rounded towards zero.
	pub fn rem_trunc(mut self, rhs: &Self) -> Self {
		self.rem_trunc_assign(rhs);
		self
	}

	/// Finds the remainder when the quotient is rounded towards zero and assigns it in-place.
	pub fn rem_trunc_assign(&mut self, rhs: &Self) {
		self.0 -= self.clone().div_trunc(rhs).0 * &rhs.0;
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
			write!(f, "{}", self.0.to_f64().unwrap().abs())
		} else {
			write!(f, "{}", self.0.to_f64().unwrap())
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
		Self(self.0.neg())
	}
}

impl<T> Pow<T> for Number
where
	BigRational: Pow<T, Output = BigRational>,
{
	type Output = Self;

	fn pow(self, rhs: T) -> Self::Output {
		Self(self.0.pow(rhs))
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

impl<T> RemAssign<T> for Number
where
	Number: From<T>,
{
	fn rem_assign(&mut self, rhs: T) {
		self.rem_trunc_assign(&rhs.into());
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

macro_rules! impl_float {
	($(($float:ty, $fn:ident),)*) => {

		$(
			impl TryFrom<$float> for Number {
				type Error = $float;

				fn try_from(value: $float) -> Result<Self, Self::Error> {
					Ok(Self(BigRational::$fn(value.into()).unwrap()))
				}
			}
		)*
	};
}

macro_rules! impl_int {
	($(($int:ty, $fn:ident),)*) => {
		$(
			impl From<$int> for Number {
				fn from(value: $int) -> Self {
					Self(BigRational::$fn(value).unwrap())
				}
			}

			impl AddAssign<$int> for Number {
				fn add_assign(&mut self, rhs: $int) {
					self.0 += BigRational::$fn(rhs).unwrap();
				}
			}

			impl DivAssign<$int> for Number {
				fn div_assign(&mut self, rhs: $int) {
					self.0 /= BigRational::$fn(rhs).unwrap();
				}
			}

			impl MulAssign<$int> for Number {
				fn mul_assign(&mut self, rhs: $int) {
					self.0 *= BigRational::$fn(rhs).unwrap();
				}
			}

			impl PartialEq<$int> for Number {
				fn eq(&self, other: &$int) -> bool {
					self.0 == BigRational::$fn(*other).unwrap()
				}
			}

			impl PartialOrd<$int> for Number {
				fn partial_cmp(&self, other: &$int) -> Option<Ordering> {
					self.0.partial_cmp(&BigRational::$fn(*other).unwrap())
				}
			}

			impl SubAssign<$int> for Number {
				fn sub_assign(&mut self, rhs: $int) {
					self.0 -= BigRational::$fn(rhs).unwrap()
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
				pub fn $name(self) -> Self {
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

impl_float! {
	(f32, from_f32), (f64, from_f64),
}

impl_int! {
	(i8, from_i8), (i16, from_i16), (i32, from_i32), (i64, from_i64), (i128, from_i128), (isize, from_isize),
	(u8, from_u8), (u16, from_u16), (u32, from_u32), (u64, from_u64), (u128, from_u128), (usize, from_usize),
}

impl_rational! {
	abs, abs_mut, "absolute value";
	ceil, ceil_mut, "ceiled integer";
	floor, floor_mut, "floored integer";
	recip, recip_mut, "reciprocal value";
	round, round_mut, "rounded integer";
	trunc, trunc_mut, "truncated integer";
}
