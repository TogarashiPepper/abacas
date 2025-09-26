pub mod add;
pub mod div;
pub mod mul;
pub mod sub;
pub mod factor;

use std::fmt;
use std::ops::{Add, Neg};

use itertools::Itertools;

use crate::monomial::Monomial;

/// A polynomial with its monomials sorted by `degree`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Polynomial(Vec<Monomial>);

impl Polynomial {
	/// The zero polynomial.
	pub const ZERO: Self = Self(Vec::new());

	/// Internal method to clean up a polynomial after operating on it.
	fn clean(&mut self) {
		self.0.retain(|mono| mono.coeff != 0.0);
		self.0.sort_by_key(|mono| mono.degree);
	}

	/// The degree of the polynomial.
	pub fn degree(&self) -> Option<i64> {
		self.0.last().map(|mono| mono.degree)
	}

	/// Gets the monomial with the given degree.
	pub fn get(&self, degree: i64) -> Option<Monomial> {
		self.0
			.binary_search_by_key(&degree, |mono| mono.degree)
			.ok()
			.map(|index| self.0[index])
	}

	/// Gets the monomial with the given degree.
	pub fn get_mut(&mut self, degree: i64) -> Option<&mut Monomial> {
		self.0
			.binary_search_by_key(&degree, |mono| mono.degree)
			.ok()
			.map(|index| &mut self.0[index])
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

impl Neg for Polynomial {
	type Output = Polynomial;

	fn neg(mut self) -> Self::Output {
		for monomial in self.0.iter_mut() {
			*monomial = -*monomial;
		}

		self
	}
}

impl fmt::Display for Polynomial {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.0.is_empty() {
			write!(f, "0")
		} else {
			write!(f, "{}", self.0.iter().rev().join(" + "))
		}
	}
}
