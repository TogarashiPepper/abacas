pub mod expr;
pub mod monomial;
pub mod polynomial;

#[cfg(test)]
mod tests {
	use crate::{
		expr::{Expr, Product},
		monomial::Monomial,
		polynomial::Polynomial,
	};

	const A: Monomial = Monomial::new(1.0, 0);
	const B: Monomial = Monomial::new(2.5, 0);
	const C: Monomial = Monomial::new(1.0, 1);
	const D: Monomial = Monomial::new(2.5, 1);
	const E: Monomial = Monomial::new(1.0, 4);
	const F: Monomial = Monomial::new(2.5, 4);

	/// Helper to create a monomial from a string {n}x^{k}
	fn m(s: &str) -> Monomial {
		let mut neg = false;
		let mut chrs = s.chars().peekable();

		if let Some('-') = chrs.peek() {
			neg = true;
			chrs.next();
		}

		let mut seen_dot = false;
		let mut coeff = (&mut chrs)
			.take_while(|e| {
				if *e == '.' && !seen_dot {
					seen_dot = true;
					true
				} else {
					e.is_ascii_digit()
				}
			})
			.collect::<String>()
			.parse::<f64>()
			.unwrap();

		assert_eq!(chrs.next(), Some('^'));

		let degree = chrs
			.take_while(|c| c.is_ascii_digit() || *c == '-')
			.collect::<String>()
			.parse::<i64>()
			.unwrap();

		if neg {
			coeff *= -1.0;
		}

		Monomial { coeff, degree }
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
	fn division() {
		let division = (F + E + D + C) / F;
		assert_eq!(division.to_string(), "1.4 + 1.4x^-3");
	}

	#[test]
	fn factor() {
		let poly = m("4x^3") + m("2x^2") + m("16x^0");
		let factored = m("2x^3") + m("1x^2") + m("8x^0");

		assert_eq!(
			poly.factor().unwrap(),
			Product(vec![Expr::Number(2.0), Expr::Polynomial(factored)])
		)
	}

	#[test]
	fn mon_helper() {
		let pol1 = Monomial::new(4.0, 2) + Monomial::new(2.3, 3) + Monomial::new(1.0, 1);
		let pol2 = m("4x^2") + m("2.3x^3") + m("1x^1");

		assert_eq!(pol1, pol2);
	}
}
