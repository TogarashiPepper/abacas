mod add;
mod div;
mod factor;
mod mul;
mod neg;
mod sub;

use std::ops::Add;
use std::{fmt, str};

use rug::Integer;

use crate::error::ParseError;
use crate::structs::Monomial;

/// A polynomial with its monomials sorted by `degree` in descending order.
///
/// # Examples
///
/// Creating a [`Polynomial`]:
///
/// ```
/// use abacas::structs::{Monomial, Polynomial};
///
/// let poly = Polynomial::new([Monomial::new(4, 2), Monomial::new(5, 3)]);
/// assert_eq!(poly.to_string(), "5x^3 + 4x^2");
///
/// let poly: Polynomial = "4x^2 + 5x^3".parse().unwrap();
/// assert_eq!(poly.to_string(), "5x^3 + 4x^2");
/// ```
///
/// Using arithmetic operations:
///
/// ```
/// use abacas::structs::Polynomial;
///
/// let a: Polynomial = "4x^4 + 3x^3 + 1".parse().unwrap();
/// let b: Polynomial = "2x^2 - 5".parse().unwrap();
///
/// let add = a.clone() + b.clone();
/// assert_eq!(add.to_string(), "4x^4 + 3x^3 + 2x^2 - 4");
///
/// let sub = a.clone() - b.clone() * 2;
/// assert_eq!(sub.to_string(), "4x^4 + 3x^3 - 4x^2 + 11");
///
/// let mul = a.clone() * b.clone();
/// assert_eq!(mul.to_string(), "8x^6 + 6x^5 - 20x^4 - 15x^3 + 2x^2 - 5");
/// ```
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Polynomial(Vec<Monomial>);

impl Polynomial {
	/// The zero polynomial.
	pub const ZERO: Self = Self(Vec::new());

	/// Internal method to clean up a polynomial after operating on it.
	fn clean(&mut self) {
		self.0.retain(|mono| !mono.coeff.is_zero());
	}

	/// Returns the degree of the polynomial, or [`None`] for the zero polynomial.
	///
	/// # Examples
	///
	/// ```
	/// use abacas::structs::Polynomial;
	///
	/// let poly: Polynomial = "4x^999 + 2x^3 + 1".parse().unwrap();
	/// assert_eq!(poly.degree(), Some(&999.into()));
	/// ```
	pub fn degree(&self) -> Option<&Integer> {
		self.0.first().map(|mono| &mono.degree)
	}

	/// Returns the monomial with the given degree, or [`None`] if the degree is not present.
	///
	/// # Examples
	///
	/// ```
	/// use abacas::structs::{Monomial, Polynomial};
	///
	/// let poly: Polynomial = "4x^9 + 2x^3 + x^2 + 100".parse().unwrap();
	/// assert_eq!(poly.get(&9.into()), Some(&Monomial::new(4, 9)));
	/// ```
	pub fn get(&self, degree: &Integer) -> Option<&Monomial> {
		self.0
			.binary_search_by(|mono| degree.cmp(&mono.degree))
			.ok()
			.and_then(|index| self.0.get(index))
	}

	/// Returns the monomial with the given degree, or [`None`] if the degree is not present.
	///
	/// # Examples
	///
	/// ```
	/// use abacas::structs::{Monomial, Polynomial};
	///
	/// let mut poly: Polynomial = "4x^9 + 2x^3 + x^2 + 100".parse().unwrap();
	/// assert_eq!(poly.get_mut(&9.into()), Some(&mut Monomial::new(4, 9)));
	/// ```
	pub fn get_mut(&mut self, degree: &Integer) -> Option<&mut Monomial> {
		self.0
			.binary_search_by(|mono| degree.cmp(&mono.degree))
			.ok()
			.and_then(|index| self.0.get_mut(index))
	}

	/// Internal method to get a monomial or insert it if it does not exist.
	fn get_or_insert(&mut self, degree: &Integer) -> &mut Monomial {
		let index = self
			.0
			.binary_search_by(|mono| degree.cmp(&mono.degree))
			.inspect_err(|&index| {
				let coeff = 0.into();
				let degree = degree.clone();

				self.0.insert(index, Monomial { coeff, degree });
			})
			.unwrap_or_else(|index| index);

		&mut self.0[index]
	}

	/// Creates a new polynomial from the given monomials.
	///
	/// # Examples
	///
	/// ```
	/// use abacas::structs::{Monomial, Polynomial};
	///
	/// let poly = Polynomial::new([Monomial::new(4, 2), Monomial::new(9, 9)]);
	/// assert_eq!(poly.to_string(), "9x^9 + 4x^2");
	/// ```
	pub fn new(monomials: impl IntoIterator<Item = Monomial>) -> Self {
		Self::from_iter(monomials)
	}
}

impl<T: Into<Monomial>> From<T> for Polynomial {
	fn from(value: T) -> Self {
		Self::new([value.into()])
	}
}

impl FromIterator<Monomial> for Polynomial {
	fn from_iter<T: IntoIterator<Item = Monomial>>(iter: T) -> Self {
		iter.into_iter().fold(Self::ZERO, Self::add)
	}
}

impl fmt::Display for Polynomial {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self.0.first() {
			Some(first) => write!(f, "{first}")?,
			None => write!(f, "0")?,
		}

		for monomial in self.0.iter().skip(1) {
			if monomial.coeff.is_positive() {
				write!(f, " + {monomial}")?;
			} else {
				// TODO: Find an alternative without allocations
				write!(f, " - {}", -monomial.clone())?;
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
