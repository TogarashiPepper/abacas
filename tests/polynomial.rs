use abacas::structs::{Monomial, Polynomial};

fn a() -> Monomial {
	Monomial::new(1.into(), 0.into())
}

fn b() -> Monomial {
	Monomial::new((5, 2).into(), 0.into())
}

fn c() -> Monomial {
	Monomial::new(1.into(), 1.into())
}

fn d() -> Monomial {
	Monomial::new((5, 2).into(), 1.into())
}

fn e() -> Monomial {
	Monomial::new(1.into(), 4.into())
}

fn f() -> Monomial {
	Monomial::new((5, 2).into(), 4.into())
}

fn m(s: &str) -> Monomial {
	s.parse().unwrap()
}

fn p(s: &str) -> Polynomial {
	s.parse().unwrap()
}

#[test]
fn construction() {
	let polynomial = Polynomial::new([a(), d(), f(), d(), a()]);
	assert_eq!(polynomial.to_string(), "2.5x^4 + 5x + 2");

	let zero = Polynomial::new([]);
	assert_eq!(zero.to_string(), "0");
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
	let addition = a() + b();
	assert_eq!(addition.to_string(), "3.5");

	let subtraction = a() - c() + d() + (-f());
	assert_eq!(subtraction.to_string(), "-2.5x^4 + 1.5x + 1");

	let multiplication = b() * d() + e() * f();
	assert_eq!(multiplication.to_string(), "2.5x^8 + 6.25x");

	let division = (f() + e() + d() + c()) / f();
	assert_eq!(division.to_string(), "1.4 + 1.4x^-3");

	let zero = b() * d() - d() * b();
	assert_eq!(zero.to_string(), "0");
}

#[test]
fn parse() {
	let expected = a() - d() - e() - e();

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
	assert_eq!(dividend_smaller, Some((Polynomial::ZERO, divisor)));
}
