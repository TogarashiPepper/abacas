use std::ops::{Div, DivAssign, Rem, RemAssign};

use crate::structs::{Monomial, Polynomial};

impl<T: Into<Monomial>> Div<T> for Polynomial {
	type Output = Self;

	fn div(mut self, rhs: T) -> Self::Output {
		self /= rhs;
		self
	}
}

impl<T: Into<Monomial>> DivAssign<T> for Polynomial {
	fn div_assign(&mut self, rhs: T) {
		let rhs = rhs.into();

		for monomial in self.0.iter_mut() {
			*monomial /= rhs;
		}
	}
}

impl Polynomial {
	/// Calculates division and remainder at the same time, returning [`None`] if the divisor is zero.
	///
	/// # Examples
	///
	/// ```
	/// use abacas::structs::Polynomial;
	///
	/// let dividend: Polynomial = "6x^5 + 5x^2 - 7".parse().unwrap();
	/// let divisor: Polynomial = "2x^2 - 1".parse().unwrap();
	///
	/// let (quotient, remainder) = dividend.clone().div_rem(&divisor).unwrap();
	///
	/// assert_eq!(quotient.to_string(), "3x^3 + 1.5x + 2.5");
	/// assert_eq!(remainder.to_string(), "1.5x - 4.5");
	/// assert_eq!(quotient * divisor + remainder, dividend);
	/// ```
	pub fn div_rem(mut self, divisor: &Self) -> Option<(Self, Self)> {
		self.div_rem_mut(divisor).map(|remainder| (self, remainder))
	}

	/// Calculates division in-place and returns the remainder, or [`None`] if the divisor is zero.
	///
	/// # Examples
	///
	/// ```
	/// use abacas::structs::Polynomial;
	///
	/// let mut dividend: Polynomial = "6x^5 + 5x^2 - 7".parse().unwrap();
	/// let divisor: Polynomial = "2x^2 - 1".parse().unwrap();
	///
	/// let remainder = dividend.div_rem_mut(&divisor).unwrap();
	///
	/// assert_eq!(dividend.to_string(), "3x^3 + 1.5x + 2.5");
	/// assert_eq!(remainder.to_string(), "1.5x - 4.5");
	/// ```
	pub fn div_rem_mut(&mut self, divisor: &Self) -> Option<Self> {
		let (normalizer, terms) = divisor.0.split_first()?;

		let Some(degree) = self.degree() else {
			return Some(Self::ZERO);
		};

		for degree in (normalizer.degree..=degree).rev() {
			let monomial = self.get_or_insert(degree);
			let coeff = monomial.coeff / normalizer.coeff;

			monomial.coeff = coeff;

			for term in terms {
				self.get_or_insert(degree + term.degree - normalizer.degree).coeff -= coeff * term.coeff;
			}
		}

		self.clean();

		let index = self
			.0
			.binary_search_by(|mono| normalizer.degree.cmp(&mono.degree))
			.map_or_else(|index| index, |index| index + 1);

		let remainder = Self::new(self.0.split_off(index));

		*self /= Monomial::new(1.0, normalizer.degree);

		Some(remainder)
	}
}

impl Div for Polynomial {
	type Output = Self;

	fn div(mut self, rhs: Self) -> Self::Output {
		self /= rhs;
		self
	}
}

impl DivAssign for Polynomial {
	fn div_assign(&mut self, rhs: Self) {
		self.div_rem_mut(&rhs).expect("abacas: cannot divide by zero");
	}
}

impl Rem for Polynomial {
	type Output = Self;

	fn rem(mut self, rhs: Self) -> Self::Output {
		self %= rhs;
		self
	}
}

impl RemAssign for Polynomial {
	fn rem_assign(&mut self, rhs: Self) {
		*self = self.div_rem_mut(&rhs).expect("abacas: cannot divide by zero");
	}
}
