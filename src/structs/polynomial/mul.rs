use std::mem;
use std::ops::{Mul, MulAssign};

use crate::structs::{Monomial, Polynomial};

impl<T: Into<Monomial>> Mul<T> for Polynomial {
	type Output = Self;

	fn mul(mut self, rhs: T) -> Self::Output {
		self *= rhs;
		self
	}
}

impl<T: Into<Monomial>> MulAssign<T> for Polynomial {
	fn mul_assign(&mut self, rhs: T) {
		let rhs = rhs.into();

		for monomial in self.0.iter_mut() {
			*monomial *= rhs;
		}

		self.clean();
	}
}

impl Mul for Polynomial {
	type Output = Self;

	fn mul(mut self, rhs: Self) -> Self::Output {
		self *= rhs;
		self
	}
}

impl MulAssign for Polynomial {
	fn mul_assign(&mut self, rhs: Self) {
		let old = mem::take(self);

		for monomial in rhs.0 {
			*self += old.clone() * monomial;
		}

		self.clean();
	}
}
