use rug::ops::{Pow, PowAssign};

use crate::structs::Monomial;

impl<T: Into<i32>> Pow<T> for Monomial {
	type Output = Self;

	fn pow(mut self, rhs: T) -> Self::Output {
		self.pow_assign(rhs);
		self
	}
}

impl<T: Into<i32>> PowAssign<T> for Monomial {
	fn pow_assign(&mut self, rhs: T) {
		let rhs = rhs.into();

		self.coeff.pow_assign(rhs);
		self.degree *= rhs;
	}
}
