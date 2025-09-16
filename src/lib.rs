pub mod monomial;
pub mod polynomial;

#[cfg(test)]
mod tests {
	use crate::monomial::Monomial;
	use crate::polynomial::Polynomial;

	const A: Monomial = Monomial::new(1.0, 0);
	const B: Monomial = Monomial::new(2.5, 0);
	const C: Monomial = Monomial::new(1.0, 1);
	const D: Monomial = Monomial::new(2.5, 1);
	const E: Monomial = Monomial::new(1.0, 4);
	const F: Monomial = Monomial::new(2.5, 4);

	#[test]
	fn construction() {
		let polynomial = Polynomial::new([A, D, F, D, A]);
		assert_eq!(polynomial.to_string(), "2.5x^4 + 5x + 2");

		let zero = Polynomial::new([]);
		assert_eq!(zero.to_string(), "0");
	}

	#[test]
	fn operators() {
		let addition = A + B;
		assert_eq!(addition.to_string(), "3.5");

		let subtraction = A - C + D + (-F);
		assert_eq!(subtraction.to_string(), "-2.5x^4 + 1.5x + 1");

		let multiplication = B * D + E * F;
		assert_eq!(multiplication.to_string(), "2.5x^8 + 6.25x");

		let zero = B * D - D * B;
		assert_eq!(zero.to_string(), "0");

        let division = (F + E + D + C) / F;
        assert_eq!(division.to_string(), "1.4 + 1.4x^-3")
	}
}
