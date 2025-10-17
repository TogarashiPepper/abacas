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

	/// Helper to create a monomial from a string
	fn m(s: &str) -> Monomial {
		s.parse().unwrap()
	}

	/// Helper to create a polynomial from a string
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

		let zero = B * D - D * B;
		assert_eq!(zero.to_string(), "0");
	}

	#[test]
	fn parse() {
		let expected = A + D + E;

		let mono = m("1") + m("2.5x") + m("x^4");
		assert_eq!(mono, expected);

		let poly = p("x^4 + 2.5x + 1");
		assert_eq!(poly, expected);

		let same = p(expected.to_string().as_str());
		assert_eq!(same, expected);

		assert_eq!(p("-4x^2 - 2 + 2x + 5x^9").to_string(), "5x^9 - 4x^2 + 2x - 2");
	}

	#[test]
	fn div_rem() {
		let division = (F + E + D + C) / F;
		assert_eq!(division.to_string(), "1.4 + 1.4x^-3");

		let num = p("3x^3 + 4x^5 + x^2 + 1");
		let denom = p("x^3");
		let (quo, rem) = num.clone().div_rem(denom.clone()).unwrap();
		assert_eq!(quo.to_string(), "4x^2 + 3 + x^-1 + x^-3");
		assert_eq!(rem.to_string(), "0");

		assert_eq!(num, quo * denom + rem);
	}

	#[test]
	fn factor() {
		let poly = p("4x^3 + 2x^2 + 16");

		let factored = p("2x^3 + x^2 + 8");
		assert_eq!(poly.factor().unwrap(), (2.0, factored));

		let monic = p("x^3 + 0.5x^2 + 4");
		assert_eq!(poly.monic().unwrap(), (4.0, monic));
	}
}
