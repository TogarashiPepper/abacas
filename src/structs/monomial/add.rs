use std::ops::Add;

use crate::structs::{Monomial, Polynomial};

impl<T: Into<Self>> Add<T> for Monomial {
	type Output = Polynomial;

	fn add(self, rhs: T) -> Self::Output {
		Polynomial::new([self, rhs.into()])
	}
}
