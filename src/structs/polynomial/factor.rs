use crate::structs::{Monomial, Polynomial};

/// Internal function that performs the euclidean algorithm.
fn gcd(mut a: f64, mut b: f64) -> f64 {
	while b != 0.0 {
		(a, b) = (b, a % b);
	}

	a.abs()
}

impl Polynomial {
	/// Extracts the common factor of all monomials.
	/// Returns [`None`] if the polynomial is zero or has coprime coefficients.
	///
	/// # Examples
	///
	/// ```rust
	/// # use abacas::structs::Polynomial;
	///
	/// let poly: Polynomial = "16x^2 + 8x + 4".parse().unwrap();
	/// let (factor, rest) = poly.factor().unwrap();
	///
	/// assert_eq!(factor, 4.0);
	/// assert_eq!(rest.to_string(), "4x^2 + 2x + 1");
	/// ```
	pub fn factor(mut self) -> Option<(f64, Self)> {
		self.factor_mut().map(|factor| (factor, self))
	}

	/// Extracts the common factor of all monomials in-place.
	/// Returns [`None`] if the polynomial is zero or has coprime coefficients.
	///
	/// # Examples
	///
	/// ```rust
	/// # use abacas::structs::Polynomial;
	///
	/// let mut poly: Polynomial = "16x^2 + 8x + 4".parse().unwrap();
	/// let factor = poly.factor_mut().unwrap();
	///
	/// assert_eq!(factor, 4.0);
	/// assert_eq!(poly.to_string(), "4x^2 + 2x + 1");
	/// ```
	pub fn factor_mut(&mut self) -> Option<f64> {
		let factor = self.0.iter().map(|mono| mono.coeff).reduce(gcd)?;

		if factor <= 1.0 {
			return None;
		}

		*self /= Monomial::constant(factor);

		Some(factor)
	}

	/// Creates a monic polynomial by dividing all monomials by the leading coefficient.
	/// Returns [`None`] if the polynomial is zero or already monic.
	///
	/// # Examples
	///
	/// ```rust
	/// # use abacas::structs::Polynomial;
	///
	/// let poly: Polynomial = "16x^9 + 4x^3 + 32".parse().unwrap();
	/// let (factor, monic) = poly.monic().unwrap();
	///
	/// assert_eq!(factor, 16.0);
	/// assert_eq!(monic.to_string(), "x^9 + 0.25x^3 + 2");
	/// ```
	pub fn monic(mut self) -> Option<(f64, Self)> {
		self.monic_mut().map(|factor| (factor, self))
	}

	/// Creates a monic polynomial in-place by dividing all monomials by the leading coefficient.
	/// Returns [`None`] if the polynomial is zero or already monic.
	///
	/// # Examples
	///
	/// ```rust
	/// # use abacas::structs::Polynomial;
	///
	/// let mut poly: Polynomial = "16x^9 + 4x^3 + 32".parse().unwrap();
	/// let factor = poly.monic_mut().unwrap();
	///
	/// assert_eq!(factor, 16.0);
	/// assert_eq!(poly.to_string(), "x^9 + 0.25x^3 + 2");
	/// ```
	pub fn monic_mut(&mut self) -> Option<f64> {
		let factor = self.0.first()?.coeff;

		if factor == 1.0 {
			return None;
		}

		*self /= Monomial::constant(factor);

		Some(factor)
	}
}
