#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

pub mod error;
pub mod structs;

#[cfg(test)]
mod tests {
	use crate::structs::{Monomial, Polynomial};

	const A: Monomial = Monomial::new(1.0, 0);
	const B: Monomial = Monomial::new(2.5, 0);
	const C: Monomial = Monomial::new(1.0, 1);
	const D: Monomial = Monomial::new(2.5, 1);
	const E: Monomial = Monomial::new(1.0, 4);
	const F: Monomial = Monomial::new(2.5, 4);

	/// Internal function to create a monomial from a string.
	fn m(s: &str) -> Monomial {
		s.parse().unwrap()
	}

	/// Internal function to create a polynomial from a string.
	fn p(s: &str) -> Polynomial {
		s.parse().unwrap()
	}

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

		let division = (F + E + D + C) / F;
		assert_eq!(division.to_string(), "1.4 + 1.4x^-3");

		let zero = B * D - D * B;
		assert_eq!(zero.to_string(), "0");
	}

	#[test]
	fn parse() {
		let expected = A - D - E - E;

		let mono = m("1") + m("-2.5x") + m("-2x^4");
		assert_eq!(mono, expected);

		let poly = p("-2x^4 - 2.5x + 1");
		assert_eq!(poly, expected);

		let same = p(expected.to_string().as_str());
		assert_eq!(same, expected);
	}

	#[test]
	fn zeros() {
		let dividend = p("6x^5 + 5x^2 - 7");
		let divisor = p("2x^2 - 1");

		let dividend_zero = Polynomial::ZERO.div_rem(&divisor);
		assert_eq!(dividend_zero, Some((Polynomial::ZERO, Polynomial::ZERO)));

		let divisor_zero = dividend.clone().div_rem(&Polynomial::ZERO);
		assert_eq!(divisor_zero, None);

		let both_zero = Polynomial::ZERO.div_rem(&Polynomial::ZERO);
		assert_eq!(both_zero, None);

		let dividend_smaller = divisor.clone().div_rem(&dividend);
		assert_eq!(dividend_smaller, Some((Polynomial::ZERO, divisor.clone())));
	}
}
