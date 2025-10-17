use std::ops::Neg;

use rug::ops::NegAssign;

use crate::structs::Polynomial;

impl Neg for Polynomial {
	type Output = Polynomial;

	fn neg(mut self) -> Self::Output {
		self.neg_assign();
		self
	}
}

impl NegAssign for Polynomial {
	fn neg_assign(&mut self) {
		for monomial in self.0.iter_mut() {
			monomial.neg_assign();
		}
	}
}
