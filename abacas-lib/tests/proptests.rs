use abacas::monomial::Monomial;
use abacas::polynomial::Polynomial;

/// Helper to construct a random polynomial with the given degree.
fn random_poly(degree: usize) -> Polynomial {
	let mut poly = Vec::with_capacity(degree + 1);

	for degree in 0..=degree {
		let numer = fastrand::u16(1..);
		let denom = fastrand::u16(1..);

		poly.push(Monomial::new((numer, denom), degree));
	}

	Polynomial::new(poly)
}

#[test]
fn addsub() {
	for _ in 0..5000 {
		let a = random_poly(50);
		let b = random_poly(50);

		assert_eq!(a.clone() + b.clone() - b, a);
	}
}

#[test]
fn muldiv() {
	for _ in 0..5000 {
		let a = random_poly(20);
		let b = random_poly(20);

		assert_eq!(a.clone() * b.clone() / b, a);
	}
}

#[test]
fn polydiv() {
	for _ in 0..5000 {
		let a = random_poly(20);
		let b = random_poly(10);

		let (quotient, remainder) = a.clone().div_rem(&b).unwrap();

		assert!(remainder.degree() < b.degree());
		assert_eq!(quotient * b + remainder, a);
	}
}
