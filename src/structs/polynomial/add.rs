use std::ops::{Add, AddAssign};

use crate::structs::{Monomial, Polynomial};

impl<T: Into<Monomial>> Add<T> for Polynomial {
	type Output = Self;

	fn add(mut self, rhs: T) -> Self::Output {
		self += rhs;
		self
	}
}

impl<T: Into<Monomial>> AddAssign<T> for Polynomial {
	fn add_assign(&mut self, rhs: T) {
		let rhs = rhs.into();

		match self.0.binary_search_by(|mono| rhs.degree.cmp(&mono.degree)) {
			Ok(index) => self.0[index].coeff += rhs.coeff,
			Err(index) => self.0.insert(index, rhs),
		}

		self.clean();
	}
}

impl Add for Polynomial {
	type Output = Self;

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
