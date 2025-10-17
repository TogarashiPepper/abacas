use std::ops::{Sub, SubAssign};

use crate::structs::{Monomial, Polynomial};

impl Sub<Monomial> for Polynomial {
	type Output = Polynomial;

	fn sub(mut self, rhs: Monomial) -> Self::Output {
		self -= rhs;
		self
	}
}

impl SubAssign<Monomial> for Polynomial {
	fn sub_assign(&mut self, rhs: Monomial) {
		match self.0.binary_search_by(|mono| rhs.degree.cmp(&mono.degree)) {
			Ok(index) => self.0[index].coeff -= rhs.coeff,
			Err(index) => self.0.insert(index, -rhs),
		}

		self.clean();
	}
}

impl Sub for Polynomial {
	type Output = Polynomial;

	fn sub(mut self, rhs: Self) -> Self::Output {
		self -= rhs;
		self
	}
}

impl SubAssign for Polynomial {
	fn sub_assign(&mut self, rhs: Self) {
		for monomial in rhs.0 {
			*self -= monomial;
		}

		self.clean();
	}
}
