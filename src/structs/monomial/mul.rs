use std::ops::{Mul, MulAssign};

use crate::structs::Monomial;

impl Mul for Monomial {
	type Output = Self;

	fn mul(mut self, rhs: Self) -> Self::Output {
		self *= rhs;
		self
	}
}

impl MulAssign for Monomial {
	fn mul_assign(&mut self, rhs: Self) {
		self.coeff *= rhs.coeff;
		self.degree += rhs.degree;
	}
}
