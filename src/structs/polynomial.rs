mod add;
mod div;
mod factor;
mod mul;
mod neg;
mod sub;

use std::ops::Add;
use std::{fmt, str};

use crate::error::ParseError;
use crate::structs::Monomial;

// Required to let docstr link to Polynomial::from_str
#[cfg(doc)]
use std::str::FromStr;

/// A polynomial with its monomials sorted by `degree` in descending order.
/// # Examples
/// ## Creating a [`Polynomial`]
/// ### Using An Iterable of [`Monomial`]s
/// ```rust
/// use abacas::structs::{Polynomial, Monomial};
/// let poly = Polynomial::new([Monomial::new(4.0, 2), Monomial::new(5.0, 3)]);
///
/// assert_eq!(poly.to_string(), "5x^3 + 4x^2");
/// ```
///
/// # Using the [`Polynomial::from_str`] impl 
/// ```rust
/// use abacas::structs::{Polynomial, Monomial};
/// let poly = "4x^2 + 5x^3".parse::<Polynomial>().unwrap();
///
/// assert_eq!(poly.to_string(), "5x^3 + 4x^2");
/// ```
///
///
/// ## Arithmetic Operations
/// ```rust
/// use abacas::structs::{Polynomial, Monomial};
///
/// let mut a = "5x^5 + 4x^4 + 3x^3 + 1".parse::<Polynomial>().unwrap();
/// let b = "2x^2".parse::<Polynomial>().unwrap();
///
/// a += b.clone();
///
/// assert_eq!(a.to_string(), "5x^5 + 4x^4 + 3x^3 + 2x^2 + 1");
///
/// a -= b * Monomial::constant(2.0);
///
/// assert_eq!(a.to_string(), "5x^5 + 4x^4 + 3x^3 - 2x^2 + 1");
///
/// let fac: Polynomial = "x^2 + 2".parse().unwrap(); 
/// assert_eq!((a * fac).to_string(), "5x^7 + 4x^6 + 13x^5 + 6x^4 + 6x^3 - 3x^2 + 2");
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Polynomial(Vec<Monomial>);

impl Polynomial {
	/// The zero polynomial.
	pub const ZERO: Self = Self(Vec::new());

	/// Internal method to clean up a polynomial after operating on it.
	fn clean(&mut self) {
		self.0.retain(|mono| mono.coeff != 0.0);
	}

	/// The degree of the polynomial. Returns [`None`] for the zero polynomial.
    /// # Example
    /// ```rust
    /// # use abacas::structs::Polynomial;
    /// let poly: Polynomial = "4x^999 + 2x^3 + 1".parse().unwrap();
    /// assert_eq!(poly.degree(), Some(999));
    /// ```
	pub fn degree(&self) -> Option<i64> {
		self.0.first().map(|mono| mono.degree)
	}

	/// Gets the monomial with the given degree.
    /// # Example
    /// ```rust
    /// # use abacas::structs::{Polynomial, Monomial};
    /// let poly: Polynomial = "4x^9 + 2x^3 + x^2 + 100".parse().unwrap();
    /// assert_eq!(poly.get(9).unwrap(), &Monomial::new(4.0, 9));
    /// ```
	pub fn get(&self, degree: i64) -> Option<&Monomial> {
		self.0
			.binary_search_by(|mono| degree.cmp(&mono.degree))
			.ok()
			.and_then(|index| self.0.get(index))
	}

	/// Gets the monomial with the given degree.
    /// # Example
    /// ```rust
    /// # use abacas::structs::{Polynomial, Monomial};
    /// let mut poly: Polynomial = "4x^9 + 2x^3 + x^2 + 100".parse().unwrap();
    /// assert_eq!(poly.get_mut(9).unwrap(), &mut Monomial::new(4.0, 9));
    /// ```
	pub fn get_mut(&mut self, degree: i64) -> Option<&mut Monomial> {
		self.0
			.binary_search_by(|mono| degree.cmp(&mono.degree))
			.ok()
			.and_then(|index| self.0.get_mut(index))
	}

	/// Internal method to get a monomial or insert it if it does not exist.
	fn get_or_insert(&mut self, degree: i64) -> &mut Monomial {
		let index = self
			.0
			.binary_search_by(|mono| degree.cmp(&mono.degree))
			.inspect_err(|&index| self.0.insert(index, Monomial { coeff: 0.0, degree }))
			.unwrap_or_else(|index| index);

		&mut self.0[index]
	}

	/// Creates a new polynomial from the given monomials.
    /// # Example
    /// ```rust
    /// # use abacas::structs::{Polynomial, Monomial};
    /// let poly = Polynomial::new([Monomial::new(4.0, 2), Monomial::new(9.0, 9)]);
    /// assert_eq!(poly.to_string(), "9x^9 + 4x^2");
    /// ```
	pub fn new(monomials: impl IntoIterator<Item = Monomial>) -> Self {
		Self::from_iter(monomials)
	}
}

impl FromIterator<Monomial> for Polynomial {
	fn from_iter<T: IntoIterator<Item = Monomial>>(iter: T) -> Self {
		iter.into_iter().fold(Self::ZERO, Self::add)
	}
}

impl fmt::Display for Polynomial {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.0.is_empty() {
			write!(f, "0")?;
		} else {
			write!(f, "{}", self.0.first().unwrap())?;
			for mono in self.0[1..].iter() {
				let mut mono = *mono;
				let is_neg = mono.coeff.is_sign_negative();

				mono.coeff = mono.coeff.abs();

				write!(f, " {} {mono}", if is_neg { '-' } else { '+' })?;
			}
		}

		Ok(())
	}
}

impl str::FromStr for Polynomial {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.trim() == "0" {
			Ok(Self::ZERO)
		} else {
			let s: String = s.chars().filter(|c| !c.is_ascii_whitespace()).collect();

			let (neg_first, s) = s.strip_prefix('-').map_or((false, s.as_str()), |rest| (true, rest));

			s.replace("-", "+-")
				.split('+')
				.enumerate()
				.map(|(i, m)| {
					m.parse::<Monomial>()
						.map(|mon| if i == 0 && neg_first { -mon } else { mon })
				})
				.collect()
		}
	}
}
