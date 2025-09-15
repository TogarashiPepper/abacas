use std::mem;
use std::ops::{Mul, MulAssign};

use super::Polynomial;
use crate::monomial::Monomial;

impl Mul<Monomial> for Polynomial {
	type Output = Polynomial;

	fn mul(mut self, rhs: Monomial) -> Self::Output {
		self *= rhs;
		self
	}
}

impl MulAssign<Monomial> for Polynomial {
	fn mul_assign(&mut self, rhs: Monomial) {
		for monomial in self.0.iter_mut() {
			*monomial *= rhs;
		}

		self.clean();
	}
}

impl Mul for Polynomial {
	type Output = Polynomial;

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
