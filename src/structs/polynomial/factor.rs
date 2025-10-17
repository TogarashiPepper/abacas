use crate::structs::{Monomial, Polynomial};

fn gcd(mut a: f64, mut b: f64) -> f64 {
	if a == 0.0 {
		return b;
	}

	// Euclidean algorithm
	while b != 0.0 {
		(a, b) = (b, a % b);
	}

	a
}

impl Polynomial {
	/// Extracts the common factor of all monomials if there is one.
	pub fn factor(&self) -> Option<(f64, Polynomial)> {
		let factor = self.0.iter().map(|mono| mono.coeff).reduce(gcd)?;

		if factor <= 1.0 {
			return None;
		}

		let factored = self.clone() / Monomial::constant(factor);

		Some((factor, factored))
	}

	/// Creates a monic polynomial by dividing all monomials by the lead coefficient.
	pub fn monic(&self) -> Option<(f64, Polynomial)> {
		let factor = self.0.first()?.coeff;

		if factor == 1.0 {
			return None;
		}

		let monic = self.clone() / Monomial::constant(factor);

		Some((factor, monic))
	}
}
