use std::ops::{Div, DivAssign, Rem, RemAssign};

use crate::structs::{Monomial, Polynomial};

impl Div<Monomial> for Polynomial {
	type Output = Self;

	fn div(mut self, rhs: Monomial) -> Self::Output {
		self /= rhs;
		self
	}
}

impl DivAssign<Monomial> for Polynomial {
	fn div_assign(&mut self, rhs: Monomial) {
		for monomial in self.0.iter_mut() {
			*monomial /= rhs;
		}
	}
}

impl Polynomial {
	/// Calculates division and remainder at the same time, returning [`None`] if the divisor is zero.
	pub fn div_rem(mut self, divisor: &Self) -> Option<(Self, Self)> {
		self.div_rem_mut(divisor).map(|remainder| (self, remainder))
	}

	/// Calculates division in-place and returns the remainder, or [`None`] if the divisor is zero.
	pub fn div_rem_mut(&mut self, divisor: &Self) -> Option<Self> {
		let (normalizer, remaining) = divisor.0.split_first()?;

		let Some(degree) = self.degree() else {
			return Some(Self::ZERO);
		};

		for degree in (normalizer.degree..=degree).rev() {
			let monomial = self.get_or_insert(degree);
			let coeff = monomial.coeff / normalizer.coeff;

			monomial.coeff = coeff;

			for monomial in remaining {
				self.get_or_insert(degree + monomial.degree - normalizer.degree).coeff -= monomial.coeff * coeff;
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
