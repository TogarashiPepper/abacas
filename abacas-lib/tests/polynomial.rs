use abacas::monomial::Monomial;
use abacas::polynomial::Polynomial;
use rug::Rational;

const A: fn() -> Monomial = || Monomial::new(1, 0);
const B: fn() -> Monomial = || Monomial::new((5, 2), 0);
const C: fn() -> Monomial = || Monomial::new(1, 1);
const D: fn() -> Monomial = || Monomial::new((5, 2), 1);
const E: fn() -> Monomial = || Monomial::new(1, 4);
const F: fn() -> Monomial = || Monomial::new((5, 2), 4);

/// Helper to construct a monomial without type inference required.
fn m(input: &str) -> Monomial {
	input.parse().unwrap()
}

/// Helper to construct a polynomial without type inference required.
fn p(input: &str) -> Polynomial {
	input.parse().unwrap()
}

fn random_poly(deg: u32) -> Polynomial {
	let mut poly = Vec::with_capacity(deg as usize);
	for deg in 0..=deg {
		let numer: u32 = rand::random();
		let coeff = Rational::from(numer) / rand::random_range(1..=numer);
		poly.push(Monomial::new(coeff, deg));
	}

	Polynomial::new(poly)
}

#[test]
fn construction() {
	let polynomial = Polynomial::new([A(), D(), F(), D(), A()]);
	assert_eq!(polynomial.to_string(), "2.5x^4 + 5x + 2");

	let zero = Polynomial::new([]);
	assert_eq!(zero.to_string(), "0");
}

#[test]
fn gcd() {
	let a = p("x - 1") * p("x + 6");
	let b = p("x - 1") * p("x - 20");

	assert_eq!(a.clone().gcd(b.clone()), p("x - 1"));
	assert_eq!(b.gcd(a), p("x - 1"));

	let a = p("2x - 1") * p("x + 6") * p("41x + 2");
	let b = p("2x - 1") * p("x - 20") * p("99999x^2 + 7");

	assert_eq!(a.clone().gcd(b.clone()), p("x - 0.5"));
	assert_eq!(b.gcd(a), p("x - 0.5"));
}

#[test]
fn gcd_ext() {
	let a = p("2x - 1") * p("x + 6") * p("41x + 2");
	let b = p("2x - 1") * p("x - 20") * p("99999x^2 + 7");

	let (s, t, gcd) = a.clone().gcd_ext(b.clone());
	let bezout = s * a + t * b;

	assert_eq!(bezout, gcd);
}

#[test]
fn impls() {
	use rug::ops::Pow;

	let mono = m("5x^4").pow(3);
	assert_eq!(mono.to_string(), "125x^12");

	let poly = p("2x^2 + 5x + 3") * 4 - (11, 2);
	assert_eq!(poly.to_string(), "8x^2 + 20x + 6.5");
}

#[test]
fn operators() {
	let addition = A() + B();
	assert_eq!(addition.to_string(), "3.5");

	let subtraction = A() - C() + D() + (-F());
	assert_eq!(subtraction.to_string(), "-2.5x^4 + 1.5x + 1");

	let multiplication = B() * D() + E() * F();
	assert_eq!(multiplication.to_string(), "2.5x^8 + 6.25x");

	let division = (F() + E() + D() + C()) / F();
	assert_eq!(division.to_string(), "1.4 + 1.4x^-3");

	let zero = B() * D() - D() * B();
	assert_eq!(zero.to_string(), "0");
}

#[test]
fn parse() {
	let expected = A() - D() - E() - E();

	let mono = m("1") + m("-2.5x") + m("-2x^4");
	assert_eq!(mono, expected);

	let poly = p("-2x^4 - 2.5x + 1");
	assert_eq!(poly, expected);

	let same = p(expected.to_string().as_str());
	assert_eq!(same, expected);
}

#[test]
fn polydiv() {
	let dividend = p("6x^5 + 5x^2 - 7");
	let divisor = p("2x^2 - 1");

	let dividend_zero = Polynomial::ZERO.div_rem(&divisor);
	assert_eq!(dividend_zero, Some((Polynomial::ZERO, Polynomial::ZERO)));

	let divisor_zero = dividend.clone().div_rem(&Polynomial::ZERO);
	assert_eq!(divisor_zero, None);

	let both_zero = Polynomial::ZERO.div_rem(&Polynomial::ZERO);
	assert_eq!(both_zero, None);

	let dividend_smaller = divisor.clone().div_rem(&dividend);
	assert_eq!(dividend_smaller, Some((Polynomial::ZERO, divisor)));

	let div_self = dividend.clone().div_rem(&dividend).unwrap();
	assert_eq!(div_self, (Polynomial::from(1), Polynomial::ZERO));
}

#[test]
fn prop_polydiv() {
	for _ in 0..5000 {
		let poly_a = random_poly(10);
		let poly_b = random_poly(5);

		let (q, r) = poly_a.clone().div_rem(&poly_b).unwrap();

		let recon_polya = q * poly_b + r;

		if poly_a != recon_polya {
			eprintln!("Assertion Failed:");
			eprintln!("{poly_a:#}\n!=\n{recon_polya:#}");
			panic!("");
		}
	}
}

#[test]
fn prop_addsub() {
	for _ in 0..5000 {
		let poly_a = random_poly(50);
		let poly_b = random_poly(50);

		assert_eq!(poly_a, poly_a.clone() + poly_b.clone() - poly_b);
	}
}

#[test]
fn prop_muldiv() {
	let poly_a = random_poly(25);
	let poly_b = random_poly(25);

	assert_eq!(poly_a, poly_a.clone() * poly_b.clone() / poly_b);
}
