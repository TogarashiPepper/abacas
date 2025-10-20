mod add;
mod div;
mod mul;
mod neg;
mod pow;
mod sub;

use std::{fmt, str};

use itertools::Itertools;

use crate::error::ParseError;

/// A monomial `ax^b` consisting of coefficient `a` and degree `b`.
///
/// # Examples
///
/// Creating a [`Monomial`]:
///
/// ```rust
/// # use abacas::structs::Monomial;
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
/// ```rust
/// # use abacas::structs::Monomial;
///
/// let add = Monomial::new(4.0, 10) + Monomial::new(1.0, 20);
/// assert_eq!(add.to_string(), "x^20 + 4x^10");
///
/// let mul = Monomial::new(4.0, 10) * Monomial::linear(2.0);
/// assert_eq!(mul.to_string(), "8x^11");
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Monomial {
	/// The coefficient of the monomial.
	pub coeff: f64,
	/// The degree of the monomial.
	pub degree: i64,
}

impl Monomial {
	/// Creates a new monomial. Panics if `coeff` is zero.
	///
	/// # Examples
	///
	/// ```rust
	/// # use abacas::structs::Monomial;
	///
	/// let mono = Monomial::new(4.0, 22);
	/// assert_eq!(mono.to_string(), "4x^22");
	/// ```
	pub const fn new(coeff: f64, degree: i64) -> Self {
		if coeff == 0.0 {
			panic!("abacas: monomial coefficient must not be zero");
		}

		Self { coeff, degree }
	}

	/// Creates a constant monomial. Panics if `coeff` is zero.
	///
	/// # Examples
	///
	/// ```rust
	/// # use abacas::structs::Monomial;
	///
	/// let mono = Monomial::constant(4.0);
	/// assert_eq!(mono.to_string(), "4");
	/// ```
	pub const fn constant(coeff: f64) -> Self {
		Self::new(coeff, 0)
	}

	/// Creates a linear monomial. Panics if `coeff` is zero.
	///
	/// # Examples
	///
	/// ```rust
	/// # use abacas::structs::Monomial;
	///
	/// let mono = Monomial::linear(2.0);
	/// assert_eq!(mono.to_string(), "2x");
	/// ```
	pub const fn linear(coeff: f64) -> Self {
		Self::new(coeff, 1)
	}
}

impl<T: Into<f64>> From<T> for Monomial {
	fn from(value: T) -> Self {
		Self::constant(value.into())
	}
}

impl fmt::Display for Monomial {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match (self.coeff, self.degree) {
			(1.0, 1) => write!(f, "x"),
			(_, 0) => write!(f, "{}", self.coeff),
			(1.0, deg) => write!(f, "x^{deg}"),
			(_, 1) => write!(f, "{}x", self.coeff),
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
			None => return Ok(Self::constant(coeff)),
			Some('x') => (),
			Some(_) => return Err(ParseError::InvalidSyntax),
		}

		match chars.next() {
			None => return Ok(Self::linear(coeff)),
			Some('^') => (),
			Some(_) => return Err(ParseError::InvalidSyntax),
		}

		let tail: String = chars.collect();
		let degree = tail.parse()?;

		Ok(Self { coeff, degree })
	}
}
