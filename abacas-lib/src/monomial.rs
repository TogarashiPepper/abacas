//! The monomial structure and its related algorithms.

use std::ops::{Add, Div, DivAssign, Mul, MulAssign, Neg, Sub};
use std::{fmt, str};

use rug::ops::{NegAssign, Pow, PowAssign};
use rug::{Integer, Rational};

use crate::error::ParseError;
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Monomial {
	/// The coefficient of the monomial
	pub coeff: Rational,
	/// The degree of the monomial
	pub degree: Integer,
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
	pub fn constant(coeff: impl Into<Rational>) -> Self {
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
	pub fn linear(coeff: impl Into<Rational>) -> Self {
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
	pub fn new(coeff: impl Into<Rational>, degree: impl Into<Integer>) -> Self {
		let coeff = coeff.into();
		let degree = degree.into();

		if coeff.is_zero() {
			panic!("abacas: monomial coefficient must not be zero");
		}

		Self { coeff, degree }
	}
}

impl<T: Into<Rational>> From<T> for Monomial {
	fn from(value: T) -> Self {
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

impl<T: Into<Rational>> DivAssign<T> for Monomial {
	fn div_assign(&mut self, rhs: T) {
		let rhs = rhs.into();

		if rhs.is_zero() {
			panic!("abacas: cannot divide by zero");
		} else {
			*self /= Self::from(rhs);
		}
	}
}

impl DivAssign for Monomial {
	fn div_assign(&mut self, rhs: Self) {
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

impl<T: Into<Rational>> MulAssign<T> for Monomial {
	fn mul_assign(&mut self, rhs: T) {
		let rhs = rhs.into();

		if rhs.is_zero() {
			panic!("abacas: cannot multiply by zero");
		} else {
			*self *= Self::from(rhs);
		}
	}
}

impl MulAssign for Monomial {
	fn mul_assign(&mut self, rhs: Self) {
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

		self.coeff.pow_assign(rhs);
		self.degree *= rhs;
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
		if self.degree == 0 {
			write!(f, "{}", self.coeff.to_f64())
		} else if self.degree == 1 {
			match self.coeff.to_f64() {
				-1.0 => write!(f, "-x"),
				1.0 => write!(f, "x"),
				coeff => write!(f, "{coeff}x"),
			}
		} else {
			match self.coeff.to_f64() {
				-1.0 => write!(f, "-x^{}", self.degree),
				1.0 => write!(f, "x^{}", self.degree),
				coeff => write!(f, "{coeff}x^{}", self.degree),
			}
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

		let Some(coeff) = Rational::from_f64(coeff) else {
			return Err(ParseError::InvalidValue(coeff));
		};

		Ok(Self::new(coeff, degree))
	}
}
