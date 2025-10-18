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

/// A polynomial with its monomials sorted by `degree` in descending order.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Polynomial(Vec<Monomial>);

impl Polynomial {
	/// The zero polynomial.
	pub const ZERO: Self = Self(Vec::new());

	/// Internal method to clean up a polynomial after operating on it.
	fn clean(&mut self) {
		self.0.retain(|mono| mono.coeff != 0.0);
	}

	/// The degree of the polynomial.
	pub fn degree(&self) -> Option<i64> {
		self.0.first().map(|mono| mono.degree)
	}

	/// Gets the monomial with the given degree.
	pub fn get(&self, degree: i64) -> Option<&Monomial> {
		self.0
			.binary_search_by(|mono| degree.cmp(&mono.degree))
			.ok()
			.and_then(|index| self.0.get(index))
	}

	/// Gets the monomial with the given degree.
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
