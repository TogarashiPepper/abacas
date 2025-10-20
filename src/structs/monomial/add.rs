use std::ops::Add;

use crate::structs::{Monomial, Polynomial};

impl Add for Monomial {
	type Output = Polynomial;

	fn add(self, rhs: Self) -> Self::Output {
		Polynomial::new([self, rhs])
	}
}
