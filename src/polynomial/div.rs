use std::ops::{Div, DivAssign, Rem};

use super::Polynomial;
use crate::monomial::Monomial;

impl Div<Monomial> for Polynomial {
	type Output = Polynomial;

	fn div(mut self, rhs: Monomial) -> Self::Output {
		self /= rhs;

		self
	}
}

impl DivAssign<Monomial> for Polynomial {
	fn div_assign(&mut self, rhs: Monomial) {
		for mono in self.0.iter_mut() {
			*mono = *mono / rhs;
		}
	}
}

impl Polynomial {
	pub fn div_rem(self, divisor: Polynomial) -> Result<(Polynomial, Polynomial), &'static str> {
		let mut dividend = self;

		// TODO: handle error of division by the zero polynomial
		let normalizer = *divisor.0.last().ok_or("Cannot divide by zero polynomial")?;

		let l1 = dividend.degree().unwrap();
		let l2 = divisor.degree().unwrap();

		let len = l1 - l2;

		for i in (0..=len).rev() {
			let term = dividend.get_mut(i).unwrap();
			term.coeff /= normalizer.coeff;
			let coeff = term.coeff;

			if coeff != 0.0 {
				for j in 1..=l2 {
					let var = dividend.get_mut_insert(i - j);
					var.coeff -= divisor.get_insert(l2 - j).coeff * coeff;
				}
			}
		}

		dividend /= Monomial::new(1.0, normalizer.degree);

		dividend.clean();

		let idx = match dividend.0.binary_search_by_key(&l2, |m| m.degree) {
			Ok(i) | Err(i) => i,
		};

		let rem = dividend.0.split_off((idx + 1).min(dividend.0.len()));
		let quo = dividend.0;

		Ok((Polynomial::new(quo), Polynomial::new(rem)))
	}
}

impl Div for Polynomial {
	type Output = Polynomial;

	fn div(self, divisor: Self) -> Self::Output {
		self.div_rem(divisor).unwrap().0
	}
}

impl Rem for Polynomial {
	type Output = Polynomial;

	fn rem(self, rhs: Self) -> Self::Output {
		self.div_rem(rhs).unwrap().1
	}
}
