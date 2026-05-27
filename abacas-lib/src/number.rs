//! The number structure and its related operations.

use std::borrow::Borrow;
use std::cmp::Ordering;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
use std::{fmt, str};

use rug::ops::{DivRounding, DivRoundingAssign, NegAssign, Pow, PowAssign, RemRounding, RemRoundingAssign};
use rug::{Integer, Rational};

use crate::error::{NumberErrorKind, ParseNumberError};

/// Sealed trait for primitive floats.
trait PrimFloat: TryInto<Number> {}

/// Sealed trait for primitive integers.
trait PrimInt: Into<Integer> {}

/// Represents a specific number. Currently uses [`Rational`] under the hood, however this should not be relied upon.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Number(Rational);

// Constructors
impl Number {
	/// Creates a new number from a primitive integer.
	#[expect(private_bounds)]
	pub fn new(value: impl PrimInt) -> Self {
		Self(value.into().into())
	}

	/// Creates a new number from a primitive float. Returns [`None`] if the float is not finite.
	#[expect(private_bounds)]
	pub fn new_float(value: impl PrimFloat) -> Option<Self> {
		value.try_into().ok()
	}

	/// Creates a new number from numerator and denominator.
	#[expect(private_bounds)]
	pub fn new_ratio(numer: impl PrimInt, denom: impl PrimInt) -> Self {
		Self((numer.into(), denom.into()).into())
	}
}

// Constants
impl Number {
	/// The number e, finitely represented as `2.718281828459045`.
	pub fn e() -> Self {
		Self::new_ratio(2718281828459045u64, 1000000000000000u64)
	}

	/// The number negative one (`-1`).
	pub fn neg_one() -> Self {
		Self(Rational::NEG_ONE.clone())
	}

	/// The number one (`1`).
	pub fn one() -> Self {
		Self(Rational::ONE.clone())
	}

	/// The number pi, finitely represented as `3.141592653589793`.
	pub fn pi() -> Self {
		Self::new_ratio(3141592653589793u64, 1000000000000000u64)
	}

	/// The number zero (`0`).
	pub fn zero() -> Self {
		Self(Rational::new())
	}
}

// Guards
impl Number {
	/// Whether this number is an integer.
	pub const fn is_integer(&self) -> bool {
		self.0.is_integer()
	}

	/// Whether this is the number negative one (`-1`).
	pub fn is_neg_one(&self) -> bool {
		self.0 == *Rational::NEG_ONE
	}

	/// Whether this number is less than zero.
	pub const fn is_negative(&self) -> bool {
		self.0.is_negative()
	}

	/// Whether this is the number one (`1`).
	pub fn is_one(&self) -> bool {
		self.0 == *Rational::ONE
	}

	/// Whether this number is greater than zero.
	pub const fn is_positive(&self) -> bool {
		self.0.is_positive()
	}

	/// Whether this is the number zero (`0`).
	pub const fn is_zero(&self) -> bool {
		self.0.is_zero()
	}
}

// Operations
impl Number {
	/// Gets the absolute value of this number.
	pub fn abs(self) -> Self {
		Self(self.0.abs())
	}

	/// Gets the denominator of this number.
	pub fn denom(self) -> Self {
		self.ratio().1
	}

	/// Calculates the greatest common divisor.
	pub fn gcd(mut self, rhs: &Self) -> Self {
		self.gcd_mut(rhs);
		self
	}

	/// Calculates the greatest common divisor in-place.
	pub fn gcd_mut(&mut self, rhs: &Self) {
		self.0.mutate_numer_denom(|numer, denom| {
			numer.gcd_mut(rhs.0.numer());
			denom.lcm_mut(rhs.0.denom());
		});
	}

	/// Calculates the least common multiple.
	pub fn lcm(mut self, rhs: &Self) -> Self {
		self.lcm_mut(rhs);
		self
	}

	/// Calculates the least common multiple in-place.
	pub fn lcm_mut(&mut self, rhs: &Self) {
		self.0.mutate_numer_denom(|numer, denom| {
			numer.lcm_mut(rhs.0.numer());
			denom.gcd_mut(rhs.0.denom());
		});
	}

	/// Gets the numerator of this number.
	pub fn numer(self) -> Self {
		self.ratio().0
	}

	/// Gets the numerator and denominator of this number as a tuple.
	pub fn ratio(self) -> (Self, Self) {
		let (numer, denom) = self.0.into_numer_denom();
		(Self(numer.into()), Self(denom.into()))
	}

	/// Calculates the reciprocal of this number.
	pub fn recip(mut self) -> Self {
		self.recip_mut();
		self
	}

	/// Calculates the reciprocal of this number in-place.
	pub fn recip_mut(&mut self) {
		self.0.recip_mut();
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

impl<T: Borrow<Self>> AddAssign<T> for Number {
	fn add_assign(&mut self, rhs: T) {
		self.0 += &rhs.borrow().0;
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

impl<T: Borrow<Self>> DivAssign<T> for Number {
	fn div_assign(&mut self, rhs: T) {
		self.0 /= &rhs.borrow().0;
	}
}

impl<T> DivRounding<T> for Number
where
	Self: DivRoundingAssign<T>,
{
	type Output = Self;

	fn div_ceil(mut self, rhs: T) -> Self::Output {
		self.div_ceil_assign(rhs);
		self
	}

	fn div_euc(mut self, rhs: T) -> Self::Output {
		self.div_euc_assign(rhs);
		self
	}

	fn div_floor(mut self, rhs: T) -> Self::Output {
		self.div_floor_assign(rhs);
		self
	}

	fn div_trunc(mut self, rhs: T) -> Self::Output {
		self.div_trunc_assign(rhs);
		self
	}
}

impl<T: Borrow<Self>> DivRoundingAssign<T> for Number {
	fn div_ceil_assign(&mut self, rhs: T) {
		self.0 /= &rhs.borrow().0;
		self.0.ceil_mut();
	}

	fn div_euc_assign(&mut self, rhs: T) {
		if rhs.borrow().is_positive() {
			self.div_floor_assign(rhs);
		} else {
			self.div_ceil_assign(rhs);
		}
	}

	fn div_floor_assign(&mut self, rhs: T) {
		self.0 /= &rhs.borrow().0;
		self.0.floor_mut();
	}

	fn div_trunc_assign(&mut self, rhs: T) {
		self.0 /= &rhs.borrow().0;
		self.0.trunc_mut();
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

impl<T: Borrow<Self>> MulAssign<T> for Number {
	fn mul_assign(&mut self, rhs: T) {
		self.0 *= &rhs.borrow().0;
	}
}

impl Neg for Number {
	type Output = Self;

	fn neg(mut self) -> Self::Output {
		self.neg_assign();
		self
	}
}

impl NegAssign for Number {
	fn neg_assign(&mut self) {
		self.0.neg_assign();
	}
}

impl<T> Pow<T> for Number
where
	Self: PowAssign<T>,
{
	type Output = Self;

	fn pow(mut self, rhs: T) -> Self::Output {
		self.pow_assign(rhs);
		self
	}
}

impl<T: Borrow<Self>> PowAssign<T> for Number {
	fn pow_assign(&mut self, rhs: T) {
		if !rhs.borrow().is_integer() || rhs.borrow().0.numer().clone().abs() > i32::MAX {
			panic!("exponent with power greater than 2^31-1 is not supported")
		}

		self.0 = Pow::<i32>::pow(&self.0, rhs.borrow().0.numer().try_into().unwrap()).into()
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
	Self: RemRoundingAssign<T>,
{
	fn rem_assign(&mut self, rhs: T) {
		self.rem_trunc_assign(rhs);
	}
}

impl<T> RemRounding<T> for Number
where
	Self: RemRoundingAssign<T>,
{
	type Output = Self;

	fn rem_ceil(mut self, rhs: T) -> Self::Output {
		self.rem_ceil_assign(rhs);
		self
	}

	fn rem_euc(mut self, rhs: T) -> Self::Output {
		self.rem_euc_assign(rhs);
		self
	}

	fn rem_floor(mut self, rhs: T) -> Self::Output {
		self.rem_floor_assign(rhs);
		self
	}

	fn rem_trunc(mut self, rhs: T) -> Self::Output {
		self.rem_trunc_assign(rhs);
		self
	}
}

impl<T: Borrow<Self>> RemRoundingAssign<T> for Number {
	fn rem_ceil_assign(&mut self, rhs: T) {
		self.0 -= self.clone().div_ceil(rhs.borrow()).0 * &rhs.borrow().0;
	}

	fn rem_euc_assign(&mut self, rhs: T) {
		self.0 -= self.clone().div_euc(rhs.borrow()).0 * &rhs.borrow().0;
	}

	fn rem_floor_assign(&mut self, rhs: T) {
		self.0 -= self.clone().div_floor(rhs.borrow()).0 * &rhs.borrow().0;
	}

	fn rem_trunc_assign(&mut self, rhs: T) {
		self.0 -= self.clone().div_trunc(rhs.borrow()).0 * &rhs.borrow().0;
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

impl<T: Borrow<Self>> SubAssign<T> for Number {
	fn sub_assign(&mut self, rhs: T) {
		self.0 -= &rhs.borrow().0;
	}
}

impl fmt::Display for Number {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.to_f64().fmt(f)
	}
}

impl str::FromStr for Number {
	type Err = ParseNumberError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (dec, int) = s.split_once(".").unwrap_or((s, ""));

		let formatted = if !int.is_empty() {
			format!("{} / 1{}", dec.to_owned() + int, "0".repeat(int.len()))
		} else {
			dec.to_owned()
		};

		formatted.parse().map(Self).map_err(|_| ParseNumberError {
			kind: NumberErrorKind::Invalid,
		})
	}
}

macro_rules! impl_float {
	($($float:ty,)*) => {
		$(
			impl TryFrom<$float> for Number {
				type Error = ();

				fn try_from(value: $float) -> Result<Self, Self::Error> {
					value.try_into().map(Self).map_err(|_| ())
				}
			}

			impl PrimFloat for $float {}
		)*
	};
}

macro_rules! impl_int {
	($($int:ty,)*) => {
		$(
			impl From<$int> for Number {
				fn from(value: $int) -> Self {
					Self(value.into())
				}
			}

			impl AddAssign<$int> for Number {
				fn add_assign(&mut self, rhs: $int) {
					self.0 += rhs;
				}
			}

			impl DivAssign<$int> for Number {
				fn div_assign(&mut self, rhs: $int) {
					self.0 /= rhs;
				}
			}

			impl DivRoundingAssign<$int> for Number {
				fn div_ceil_assign(&mut self, rhs: $int) {
					self.0 /= rhs;
					self.0.ceil_mut();
				}

				fn div_euc_assign(&mut self, rhs: $int) {
					if rhs > 0 {
						self.div_floor_assign(rhs);
					} else {
						self.div_ceil_assign(rhs);
					}
				}

				fn div_floor_assign(&mut self, rhs: $int) {
					self.0 /= rhs;
					self.0.floor_mut();
				}

				fn div_trunc_assign(&mut self, rhs: $int) {
					self.0 /= rhs;
					self.0.trunc_mut();
				}
			}

			impl MulAssign<$int> for Number {
				fn mul_assign(&mut self, rhs: $int) {
					self.0 *= rhs;
				}
			}

			impl PartialEq<$int> for Number {
				fn eq(&self, other: &$int) -> bool {
					self.0 == *other
				}
			}

			impl PartialOrd<$int> for Number {
				fn partial_cmp(&self, other: &$int) -> Option<Ordering> {
					self.0.partial_cmp(other)
				}
			}


			impl PowAssign<$int> for Number {

				fn pow_assign(&mut self, rhs: $int) {

					if rhs > i32::MAX as $int {
						panic!("exponent with power greater than 2^31-1 is not supported")
					}

					self.0 = (Pow::<i32>::pow(
						&self.0,
						rhs as i32,
					)).into();
				}
			}

			impl RemRoundingAssign<$int> for Number {
				fn rem_ceil_assign(&mut self, rhs: $int) {
					self.0 -= self.clone().div_ceil(rhs).0 * rhs;
				}

				fn rem_euc_assign(&mut self, rhs: $int) {
					self.0 -= self.clone().div_euc(rhs).0 * rhs;
				}

				fn rem_floor_assign(&mut self, rhs: $int) {
					self.0 -= self.clone().div_floor(rhs).0 * rhs;
				}

				fn rem_trunc_assign(&mut self, rhs: $int) {
					self.0 -= self.clone().div_trunc(rhs).0 * rhs;
				}
			}

			impl SubAssign<$int> for Number {
				fn sub_assign(&mut self, rhs: $int) {
					self.0 -= rhs;
				}
			}

			impl PrimInt for $int {}
		)*
	};
}

impl_float! {
	f32, f64,
}

impl_int! {
	i8, i16, i32, i64, i128, isize,
	u8, u16, u32, u64, u128, usize,
}
