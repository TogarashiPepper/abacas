mod add;
mod div;
mod mul;
mod neg;
mod pow;
mod sub;

use std::{fmt, str};

use itertools::Itertools;
use rug::{Integer, Rational};

use crate::error::ParseError;

/// A monomial `ax^b` consisting of coefficient `a` and degree `b`.
///
/// # Examples
///
/// Creating a [`Monomial`]:
///
/// ```
/// use abacas::structs::Monomial;
///
/// let mono = Monomial::new(4.0, 10);
/// assert_eq!(mono.to_string(), "4x^10");
///
/// let mono: Monomial = "4x^10".parse().unwrap();
/// assert_eq!(mono.to_string(), "4x^10");
/// ```
///
/// Using arithmetic operations:
///
/// ```
/// use abacas::structs::Monomial;
///
/// let add = Monomial::new(4.0, 10) + Monomial::new(1.0, 20);
/// assert_eq!(add.to_string(), "x^20 + 4x^10");
///
/// let mul = Monomial::new(4.0, 10) * Monomial::linear(2.0);
/// assert_eq!(mul.to_string(), "8x^11");
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Monomial {
	/// The coefficient of the monomial
	pub coeff: Rational,
	/// The degree of the monomial
	pub degree: Integer,
}

impl Monomial {
	/// Creates a new monomial. Panics if `coeff` is zero.
	///
	/// # Examples
	///
	/// ```
	/// use abacas::structs::Monomial;
	///
	/// let mono = Monomial::new(4.0, 22);
	/// assert_eq!(mono.to_string(), "4x^22");
	/// ```
	pub fn new(coeff: Rational, degree: Integer) -> Self {
		if &coeff == Rational::ZERO {
			panic!("abacas: monomial coefficient must not be zero");
		}

		Self {
			coeff,
			degree: Integer::from(degree),
		}
	}

	/// Creates a constant monomial. Panics if `coeff` is zero.
	///
	/// # Examples
	///
	/// ```
	/// use abacas::structs::Monomial;
	///
	/// let mono = Monomial::constant(4.0);
	/// assert_eq!(mono.to_string(), "4");
	/// ```
	pub fn constant(coeff: Rational) -> Self {
		Self::new(coeff, Integer::ZERO)
	}

	/// Creates a linear monomial. Panics if `coeff` is zero.
	///
	/// # Examples
	///
	/// ```
	/// use abacas::structs::Monomial;
	///
	/// let mono = Monomial::linear(2.0);
	/// assert_eq!(mono.to_string(), "2x");
	/// ```
	pub fn linear(coeff: Rational) -> Self {
		Self::new(coeff, Integer::ONE.clone())
	}
}

impl<T: Into<Rational>> From<T> for Monomial {
	fn from(value: T) -> Self {
		Self::constant(value.into())
	}
}

impl fmt::Display for Monomial {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match (self.coeff.clone(), self.degree.clone()) {
			(a, b) if a == Integer::ONE.clone() && b == Rational::ONE.clone() => write!(f, "x"),
			(_, a) if a == Integer::ZERO.clone() => write!(f, "{}", self.coeff),
			(a, deg) if a == Rational::ONE.clone() => write!(f, "x^{deg}"),
			(_, a) if a == Integer::ONE.clone() => write!(f, "{}x", self.coeff),
			(_, _) => write!(f, "{}x^{}", self.coeff, self.degree),
		}
	}
}

impl str::FromStr for Monomial {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let mut chars = s.trim().chars();

		let init: String = chars.peeking_take_while(|&c| c != 'x').collect();
		let coeff = if init.is_empty() { 1.0 } else { init.parse()? };

		if coeff == 0.0 {
			return Err(ParseError::InvalidValue);
		}

		match chars.next() {
			None => return Ok(Self::constant(Rational::from_f64(coeff).unwrap())),
			Some('x') => (),
			Some(_) => return Err(ParseError::InvalidSyntax),
		}

		match chars.next() {
			None => return Ok(Self::linear(Rational::from_f64(coeff).unwrap())),
			Some('^') => (),
			Some(_) => return Err(ParseError::InvalidSyntax),
		}

		let tail: String = chars.collect();
		let degree: i64 = tail.parse()?;

		Ok(Self {
			coeff: Rational::from_f64(coeff).unwrap(),
			degree: Integer::from(degree),
		})
	}
}
