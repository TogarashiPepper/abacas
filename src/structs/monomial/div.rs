use std::ops::{Div, DivAssign};

use crate::structs::Monomial;

impl<T: Into<Monomial>> Div<T> for Monomial {
	type Output = Self;

	fn div(mut self, rhs: T) -> Self::Output {
		self /= rhs;
		self
	}
}

impl<T: Into<Monomial>> DivAssign<T> for Monomial {
	fn div_assign(&mut self, rhs: T) {
		let rhs = rhs.into();

		self.coeff /= rhs.coeff;
		self.degree -= rhs.degree;
	}
}
