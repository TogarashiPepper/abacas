use crate::expr::{Expr, Product};
use crate::monomial::Monomial;
use crate::polynomial::Polynomial;

fn gcd(mut a: f64, mut b: f64) -> f64 {
	if a == 0.0 {
		return b;
	}
	if b == 0.0 {
		return a;
	}

	// Euclidean algorithm
	while b != 0.0 {
		let temp = a;
		a = b;
		b = temp % b;
	}

	a
}

impl Polynomial {
	pub fn factor(&self) -> Option<Product> {
		let g = self.0.iter().map(|m| m.coeff).reduce(gcd)?;
		if g == 1.0 {
			return None;
		}

		let factored = Polynomial::from_iter(self.0.iter().map(|m| Monomial {
			coeff: m.coeff / g,
			degree: m.degree,
		}));

		Some(Product(vec![Expr::Number(g), Expr::Polynomial(factored)]))
	}
}
