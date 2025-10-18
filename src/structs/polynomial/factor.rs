use crate::structs::{Monomial, Polynomial};

/// Internal function that performs the euclidean algorithm.
fn gcd(mut a: f64, mut b: f64) -> f64 {
	while b != 0.0 {
		(a, b) = (b, a % b);
	}

	a.abs()
}

impl Polynomial {
	/// Extracts the common factor of all monomials if there is one.
	pub fn factor(mut self) -> Option<(f64, Self)> {
		self.factor_mut().map(|factor| (factor, self))
	}

	/// Extracts the common factor of all monomials in-place if there is one.
	pub fn factor_mut(&mut self) -> Option<f64> {
		let factor = self.0.iter().map(|mono| mono.coeff).reduce(gcd)?;

		if factor <= 1.0 {
			return None;
		}

		*self /= Monomial::constant(factor);

		Some(factor)
	}

	/// Creates a monic polynomial by dividing all monomials by the lead coefficient.
	pub fn monic(mut self) -> Option<(f64, Self)> {
		self.monic_mut().map(|factor| (factor, self))
	}

	/// Creates a monic polynomial in-place by dividing all monomials by the lead coefficient.
	pub fn monic_mut(&mut self) -> Option<f64> {
		let factor = self.0.first()?.coeff;

		if factor == 1.0 {
			return None;
		}

		*self /= Monomial::constant(factor);

		Some(factor)
	}
}
