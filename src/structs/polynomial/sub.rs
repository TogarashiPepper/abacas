use std::ops::{Sub, SubAssign};

use crate::structs::{Monomial, Polynomial};

impl<T: Into<Monomial>> Sub<T> for Polynomial {
	type Output = Self;

	fn sub(mut self, rhs: T) -> Self::Output {
		self -= rhs;
		self
	}
}

impl<T: Into<Monomial>> SubAssign<T> for Polynomial {
	fn sub_assign(&mut self, rhs: T) {
		let rhs = rhs.into();

		match self.0.binary_search_by(|mono| rhs.degree.cmp(&mono.degree)) {
			Ok(index) => self.0[index].coeff -= rhs.coeff,
			Err(index) => self.0.insert(index, -rhs),
		}

		self.clean();
	}
}

impl Sub for Polynomial {
	type Output = Self;

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
	}
}
