use std::ops::Sub;

use crate::structs::{Monomial, Polynomial};

impl<T: Into<Self>> Sub<T> for Monomial {
	type Output = Polynomial;

	fn sub(self, rhs: T) -> Self::Output {
		Polynomial::new([self, -rhs.into()])
	}
}
