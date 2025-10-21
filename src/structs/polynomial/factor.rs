use rug::Rational;

use crate::structs::Polynomial;

/// Internal function that calculates the greatest common divisor.
fn gcd(a: Rational, b: &Rational) -> Rational {
	let (numer, denom) = a.into_numer_denom();

	// See: https://math.stackexchange.com/a/199905
	(numer.gcd(b.numer()), denom.lcm(b.denom())).into()
}

impl Polynomial {
	/// Returns the GCD of two polynomials in monic form.
	///
	/// # Examples
	///
	/// ```
	/// use abacas::structs::Polynomial;
	///
	/// let coeff = "x - 1".parse::<Polynomial>().unwrap();
	/// let a = coeff.clone() * "x - 21".parse::<Polynomial>().unwrap();
	/// let b = coeff.clone() * "4x - 9".parse::<Polynomial>().unwrap();
	///
	/// assert_eq!(a.gcd(b), coeff);
	/// ```
	pub fn gcd(mut self, mut b: Polynomial) -> Polynomial {
		while b != Polynomial::ZERO {
			(self, b) = (b.clone(), self % b);
		}

		self.monic_mut();
		self
	}

	/// Extracts the common factor of all monomials.
	/// Returns [`None`] if the polynomial is zero or has coprime coefficients.
	///
	/// # Examples
	///
	/// ```
	/// use abacas::structs::Polynomial;
	///
	/// let poly: Polynomial = "16x^2 + 8x + 4".parse().unwrap();
	/// let (factor, rest) = poly.factor().unwrap();
	///
	/// assert_eq!(factor, 4);
	/// assert_eq!(rest.to_string(), "4x^2 + 2x + 1");
	/// ```
	pub fn factor(mut self) -> Option<(Rational, Self)> {
		self.factor_mut().map(|factor| (factor, self))
	}

	/// Extracts the common factor of all monomials in-place.
	/// Returns [`None`] if the polynomial is zero or has coprime coefficients.
	///
	/// # Examples
	///
	/// ```
	/// use abacas::structs::Polynomial;
	///
	/// let mut poly: Polynomial = "16x^2 + 8x + 4".parse().unwrap();
	/// let factor = poly.factor_mut().unwrap();
	///
	/// assert_eq!(factor, 4);
	/// assert_eq!(poly.to_string(), "4x^2 + 2x + 1");
	/// ```
	pub fn factor_mut(&mut self) -> Option<Rational> {
		let factor = self.0.iter().map(|mono| &mono.coeff).fold(Rational::new(), gcd);

		if factor <= 1 {
			return None;
		}

		*self /= &factor;

		Some(factor)
	}

	/// Creates a monic polynomial by dividing all monomials by the leading coefficient.
	/// Returns [`None`] if the polynomial is zero or already monic.
	///
	/// # Examples
	///
	/// ```
	/// use abacas::structs::Polynomial;
	///
	/// let poly: Polynomial = "16x^9 + 4x^3 + 32".parse().unwrap();
	/// let (factor, monic) = poly.monic().unwrap();
	///
	/// assert_eq!(factor, 16);
	/// assert_eq!(monic.to_string(), "x^9 + 0.25x^3 + 2");
	/// ```
	pub fn monic(mut self) -> Option<(Rational, Self)> {
		self.monic_mut().map(|factor| (factor, self))
	}

	/// Creates a monic polynomial in-place by dividing all monomials by the leading coefficient.
	/// Returns [`None`] if the polynomial is zero or already monic.
	///
	/// # Examples
	///
	/// ```
	/// use abacas::structs::Polynomial;
	///
	/// let mut poly: Polynomial = "16x^9 + 4x^3 + 32".parse().unwrap();
	/// let factor = poly.monic_mut().unwrap();
	///
	/// assert_eq!(factor, 16);
	/// assert_eq!(poly.to_string(), "x^9 + 0.25x^3 + 2");
	/// ```
	pub fn monic_mut(&mut self) -> Option<Rational> {
		let factor = self.0.first()?.coeff.clone();

		if factor == 1 {
			return None;
		}

		*self /= &factor;

		Some(factor)
	}
}
