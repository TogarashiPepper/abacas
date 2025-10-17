use std::ops::{Div, DivAssign, Rem};

use crate::structs::{Monomial, Polynomial};

impl Div<Monomial> for Polynomial {
	type Output = Polynomial;

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
	pub fn div_rem(mut self, divisor: &Polynomial) -> Option<(Polynomial, Polynomial)> {
		let (normalizer, remaining) = divisor.0.split_first()?;

		let Some(degree) = self.degree() else {
			return Some((Polynomial::ZERO, Polynomial::ZERO));
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

		let remainder = Polynomial::new(self.0.split_off(self.0.len().min(index)));
		let quotient = self / Monomial::new(1.0, normalizer.degree);

		Some((quotient, remainder))
	}
}

impl Div for Polynomial {
	type Output = Polynomial;

	fn div(self, rhs: Self) -> Self::Output {
		self.div_rem(&rhs).expect("abacas: cannot divide by zero").0
	}
}

impl Rem for Polynomial {
	type Output = Polynomial;

	fn rem(self, rhs: Self) -> Self::Output {
		self.div_rem(&rhs).expect("abacas: cannot divide by zero").1
	}
}
