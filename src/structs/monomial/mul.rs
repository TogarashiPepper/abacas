use std::ops::{Mul, MulAssign};

use crate::structs::Monomial;

impl<T: Into<Self>> Mul<T> for Monomial {
	type Output = Self;

	fn mul(mut self, rhs: T) -> Self::Output {
		self *= rhs;
		self
	}
}

impl<T: Into<Self>> MulAssign<T> for Monomial {
	fn mul_assign(&mut self, rhs: T) {
		let rhs = rhs.into();

		self.coeff *= rhs.coeff;
		self.degree += rhs.degree;
	}
}
