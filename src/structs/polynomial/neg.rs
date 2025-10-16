use std::ops::Neg;

use crate::structs::Polynomial;

impl Neg for Polynomial {
	type Output = Polynomial;

	fn neg(mut self) -> Self::Output {
		for monomial in self.0.iter_mut() {
			*monomial = -*monomial;
		}

		self
	}
}
