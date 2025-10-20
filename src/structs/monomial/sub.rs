use std::ops::Sub;

use crate::structs::{Monomial, Polynomial};

impl Sub for Monomial {
	type Output = Polynomial;

	fn sub(self, rhs: Self) -> Self::Output {
		Polynomial::new([self, -rhs])
	}
}
