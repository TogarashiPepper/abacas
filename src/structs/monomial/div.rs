use std::ops::{Div, DivAssign};

use crate::structs::Monomial;

impl Div for Monomial {
	type Output = Self;

	fn div(mut self, rhs: Self) -> Self::Output {
		self /= rhs;
		self
	}
}

impl DivAssign for Monomial {
	fn div_assign(&mut self, rhs: Self) {
		self.coeff /= rhs.coeff;
		self.degree -= rhs.degree;
	}
}
