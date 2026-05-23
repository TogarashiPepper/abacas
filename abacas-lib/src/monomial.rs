//! The monomial structure and its related algorithms.

use std::ops::{Add, Div, DivAssign, Mul, MulAssign, Neg, Sub};
use std::{fmt, str};

use rug::ops::{NegAssign, Pow, PowAssign};

use crate::error::ParseError;
use crate::number::Number;
use crate::polynomial::Polynomial;

/// A monomial `ax^b` consisting of coefficient `a` and degree `b`.
///
/// # Examples
///
/// Creating a [`Monomial`]:
///
/// ```
/// use abacas::monomial::Monomial;
///
/// let mono = Monomial::new(4, 10);
/// assert_eq!(mono.to_string(), "4x^10");
///
/// let mono: Monomial = "4x^10".parse().unwrap();
/// assert_eq!(mono.to_string(), "4x^10");
/// ```
///
/// Using arithmetic operations:
///
/// ```
/// use abacas::monomial::Monomial;
///
/// let add = Monomial::new(4, 10) + Monomial::new(1, 20);
/// assert_eq!(add.to_string(), "x^20 + 4x^10");
///
/// let mul = Monomial::new(4, 10) * Monomial::linear(2);
/// assert_eq!(mul.to_string(), "8x^11");
/// ```
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Monomial {
	/// The coefficient of the monomial
	pub coeff: Number,
	/// The degree of the monomial
	pub degree: Number,
}

impl Monomial {
	/// Creates a constant monomial. Panics if `coeff` is zero.
	///
	/// # Examples
	///
	/// ```
	/// use abacas::monomial::Monomial;
	///
	/// let mono = Monomial::constant(4);
	/// assert_eq!(mono.to_string(), "4");
	/// ```
	pub fn constant(coeff: impl Into<Number>) -> Self {
		Self::new(coeff, 0)
	}

	/// Creates a linear monomial. Panics if `coeff` is zero.
	///
	/// # Examples
	///
	/// ```
	/// use abacas::monomial::Monomial;
	///
	/// let mono = Monomial::linear(2);
	/// assert_eq!(mono.to_string(), "2x");
	/// ```
	pub fn linear(coeff: impl Into<Number>) -> Self {
		Self::new(coeff, 1)
	}

	/// Creates a new monomial. Panics if `coeff` is zero.
	///
	/// # Examples
	///
	/// ```
	/// use abacas::monomial::Monomial;
	///
	/// let mono = Monomial::new(4, 22);
	/// assert_eq!(mono.to_string(), "4x^22");
	/// ```
	pub fn new(coeff: impl Into<Number>, degree: impl Into<Number>) -> Self {
		let coeff = coeff.into();
		let degree = degree.into();

		if coeff.is_zero() {
			panic!("abacas: monomial coefficient must not be zero");
		}

		Self { coeff, degree }
	}
}

impl From<Number> for Monomial {
	fn from(value: Number) -> Self {
		Self::constant(value)
	}
}

impl<T> Add<T> for Monomial
where
	Polynomial: Add<T, Output = Polynomial>,
{
	type Output = Polynomial;

	fn add(self, rhs: T) -> Self::Output {
		Polynomial::from(self) + rhs
	}
}

impl<T> Div<T> for Monomial
where
	Self: DivAssign<T>,
{
	type Output = Self;

	fn div(mut self, rhs: T) -> Self::Output {
		self /= rhs;
		self
	}
}

impl<T: Into<Self>> DivAssign<T> for Monomial {
	fn div_assign(&mut self, rhs: T) {
		let rhs = rhs.into();

		self.coeff /= rhs.coeff;
		self.degree -= rhs.degree;
	}
}

impl<T> Mul<T> for Monomial
where
	Self: MulAssign<T>,
{
	type Output = Self;

	fn mul(mut self, rhs: T) -> Self::Output {
		self *= rhs;
		self
	}
}

impl<T: Into<Self>> MulAssign<T> for Monomial {
	fn mul_assign(&mut self, rhs: T) {
		let rhs = rhs.into();

		self.coeff *= rhs.coeff;
		self.degree += rhs.degree;
	}
}

impl Neg for Monomial {
	type Output = Self;

	fn neg(mut self) -> Self::Output {
		self.neg_assign();
		self
	}
}

impl NegAssign for Monomial {
	fn neg_assign(&mut self) {
		self.coeff.neg_assign();
	}
}

impl<T> Pow<T> for Monomial
where
	Self: PowAssign<T>,
{
	type Output = Self;

	fn pow(mut self, rhs: T) -> Self::Output {
		self.pow_assign(rhs);
		self
	}
}

impl<T: Into<i32>> PowAssign<T> for Monomial {
	fn pow_assign(&mut self, rhs: T) {
		let rhs = rhs.into();

		self.coeff.pow_assign(rhs.into());
		self.degree *= rhs.into();
	}
}

impl<T> Sub<T> for Monomial
where
	Polynomial: Sub<T, Output = Polynomial>,
{
	type Output = Polynomial;

	fn sub(self, rhs: T) -> Self::Output {
		Polynomial::from(self) - rhs
	}
}

impl fmt::Display for Monomial {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.degree.is_zero() {
			write!(f, "{}", self.coeff)
		} else if self.degree.is_one() {
			if self.coeff == -Number::one() {
				write!(f, "-x")
			} else if self.coeff == Number::one() {
				write!(f, "x")
			} else {
				write!(f, "{}x", self.coeff)
			}
		} else if self.coeff == -Number::one() {
			write!(f, "-x^{}", self.degree)
		} else if self.coeff == Number::one() {
			write!(f, "x^{}", self.degree)
		} else {
			write!(f, "{}x^{}", self.coeff, self.degree)
		}
	}
}

impl str::FromStr for Monomial {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let (init, degree) = if let Some((init, tail)) = s.split_once("x^") {
			(init, tail.parse()?)
		} else if let Some(init) = s.strip_suffix('x') {
			(init, 1)
		} else {
			(s, 0)
		};

		let coeff = match init {
			"" | "+" if degree != 0 => 1.0,
			"-" if degree != 0 => -1.0,
			_ => init.parse()?,
		};

		if coeff == 0.0 {
			return Err(ParseError::InvalidValue(coeff));
		}

		let Some(coeff) = Number::from_f64(coeff) else {
			return Err(ParseError::InvalidValue(coeff));
		};

		Ok(Self::new(coeff, degree))
	}
}
