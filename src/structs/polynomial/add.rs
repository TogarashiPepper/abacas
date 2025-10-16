use std::ops::{Add, AddAssign};

use crate::structs::Monomial;
use crate::structs::Polynomial;

impl Add<Monomial> for Polynomial {
	type Output = Polynomial;

	fn add(mut self, rhs: Monomial) -> Self::Output {
		self += rhs;
		self
	}
}

impl AddAssign<Monomial> for Polynomial {
	fn add_assign(&mut self, rhs: Monomial) {
		match self.0.binary_search_by_key(&rhs.degree, |mono| mono.degree) {
			Ok(index) => self.0[index].coeff += rhs.coeff,
			Err(index) => self.0.insert(index, rhs),
		}

		self.clean();
	}
}

impl Add for Polynomial {
	type Output = Polynomial;

	fn add(mut self, rhs: Self) -> Self::Output {
		self += rhs;
		self
	}
}

impl AddAssign for Polynomial {
	fn add_assign(&mut self, rhs: Self) {
		for monomial in rhs.0 {
			*self += monomial;
		}

		self.clean();
	}
}
