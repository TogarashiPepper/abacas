//! The polynomial structure and its related algorithms.

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign};
use std::{fmt, mem, str};

use rug::ops::NegAssign;
use rug::{Integer, Rational};

use crate::error::ParseError;
use crate::monomial::Monomial;

/// Internal function that calculates the greatest common divisor.
fn gcd(a: Rational, b: &Rational) -> Rational {
	let (numer, denom) = a.into_numer_denom();

	// See: https://math.stackexchange.com/a/199905
	(numer.gcd(b.numer()), denom.lcm(b.denom())).into()
}

/// A polynomial with its monomials sorted by `degree` in descending order.
///
/// # Examples
///
/// Creating a [`Polynomial`]:
///
/// ```
/// use abacas::monomial::Monomial;
/// use abacas::polynomial::Polynomial;
///
/// let poly = Polynomial::new([Monomial::new(4, 2), Monomial::new(5, 3)]);
/// assert_eq!(poly.to_string(), "5x^3 + 4x^2");
///
/// let poly: Polynomial = "4x^2 + 5x^3".parse().unwrap();
/// assert_eq!(poly.to_string(), "5x^3 + 4x^2");
/// ```
///
/// Using arithmetic operations:
///
/// ```
/// use abacas::polynomial::Polynomial;
///
/// let a: Polynomial = "4x^4 + 3x^3 + 1".parse().unwrap();
/// let b: Polynomial = "2x^2 - 5".parse().unwrap();
///
/// let add = a.clone() + b.clone();
/// assert_eq!(add.to_string(), "4x^4 + 3x^3 + 2x^2 - 4");
///
/// let sub = a.clone() - b.clone() * 2;
/// assert_eq!(sub.to_string(), "4x^4 + 3x^3 - 4x^2 + 11");
///
/// let mul = a.clone() * b.clone();
/// assert_eq!(mul.to_string(), "8x^6 + 6x^5 - 20x^4 - 15x^3 + 2x^2 - 5");
/// ```
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Polynomial(Vec<Monomial>);

impl Polynomial {
	/// The zero polynomial.
	pub const ZERO: Self = Self(Vec::new());

	/// Internal method to clean up a polynomial after operating on it.
	fn clean(&mut self) {
		self.0.retain(|mono| !mono.coeff.is_zero());
	}

	/// Returns the degree of the polynomial, or [`None`] for the zero polynomial.
	///
	/// # Examples
	///
	/// ```
	/// use abacas::polynomial::Polynomial;
	///
	/// let poly: Polynomial = "4x^999 + 2x^3 + 1".parse().unwrap();
	/// assert_eq!(poly.degree(), Some(&999.into()));
	/// ```
	pub fn degree(&self) -> Option<&Integer> {
		self.0.first().map(|mono| &mono.degree)
	}

	/// Calculates division and remainder at the same time, returning [`None`] if the divisor is zero.
	///
	/// # Examples
	///
	/// ```
	/// use abacas::polynomial::Polynomial;
	///
	/// let dividend: Polynomial = "6x^5 + 5x^2 - 7".parse().unwrap();
	/// let divisor: Polynomial = "2x^2 - 1".parse().unwrap();
	///
	/// let (quotient, remainder) = dividend.clone().div_rem(&divisor).unwrap();
	///
	/// assert_eq!(quotient.to_string(), "3x^3 + 1.5x + 2.5");
	/// assert_eq!(remainder.to_string(), "1.5x - 4.5");
	/// assert_eq!(quotient * divisor + remainder, dividend);
	/// ```
	pub fn div_rem(mut self, divisor: &Self) -> Option<(Self, Self)> {
		self.div_rem_mut(divisor).map(|remainder| (self, remainder))
	}

	/// Calculates division in-place and returns the remainder, or [`None`] if the divisor is zero.
	///
	/// # Examples
	///
	/// ```
	/// use abacas::polynomial::Polynomial;
	///
	/// let mut dividend: Polynomial = "6x^5 + 5x^2 - 7".parse().unwrap();
	/// let divisor: Polynomial = "2x^2 - 1".parse().unwrap();
	///
	/// let remainder = dividend.div_rem_mut(&divisor).unwrap();
	///
	/// assert_eq!(dividend.to_string(), "3x^3 + 1.5x + 2.5");
	/// assert_eq!(remainder.to_string(), "1.5x - 4.5");
	/// ```
	pub fn div_rem_mut(&mut self, divisor: &Self) -> Option<Self> {
		let (normalizer, terms) = divisor.0.split_first()?;

		let Some(mut degree) = self.degree().cloned() else {
			return Some(Self::ZERO);
		};

		while degree >= normalizer.degree {
			let monomial = self.get_or_insert(&degree);
			let coeff = monomial.coeff.clone() / &normalizer.coeff;

			monomial.coeff = coeff.clone();

			for term in terms {
				let degree = degree.clone() + &term.degree - &normalizer.degree;
				let monomial = self.get_or_insert(&degree);

				monomial.coeff -= coeff.clone() * &term.coeff;
			}

			degree -= 1;
		}

		self.clean();

		let index = self
			.0
			.binary_search_by(|mono| normalizer.degree.cmp(&mono.degree))
			.map_or_else(|index| index, |index| index + 1);

		let remainder = Self::new(self.0.split_off(index));

		for monomial in self.0.iter_mut() {
			monomial.degree -= &normalizer.degree;
		}

		Some(remainder)
	}

	/// Extracts the common factor of all monomials.
	/// Returns [`None`] if the polynomial is zero or has coprime coefficients.
	///
	/// # Examples
	///
	/// ```
	/// use abacas::polynomial::Polynomial;
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
	/// use abacas::polynomial::Polynomial;
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

	/// Returns the GCD of two polynomials in monic form.
	///
	/// # Examples
	///
	/// ```
	/// use abacas::polynomial::Polynomial;
	///
	/// let coeff = "x - 1".parse::<Polynomial>().unwrap();
	/// let a = coeff.clone() * "x - 21".parse::<Polynomial>().unwrap();
	/// let b = coeff.clone() * "4x - 9".parse::<Polynomial>().unwrap();
	///
	/// assert_eq!(a.gcd(b), coeff);
	/// ```
	pub fn gcd(mut self, mut other: Polynomial) -> Polynomial {
		while other != Polynomial::ZERO {
			(other, self) = (self.div_rem_mut(&other).unwrap(), other);
		}

		self.monic_mut();
		self
	}

	/// Returns the monomial with the given degree, or [`None`] if the degree is not present.
	///
	/// # Examples
	///
	/// ```
	/// use abacas::monomial::Monomial;
	/// use abacas::polynomial::Polynomial;
	///
	/// let poly: Polynomial = "4x^9 + 2x^3 + x^2 + 100".parse().unwrap();
	/// assert_eq!(poly.get(&9.into()), Some(&Monomial::new(4, 9)));
	/// ```
	pub fn get(&self, degree: &Integer) -> Option<&Monomial> {
		self.0
			.binary_search_by(|mono| degree.cmp(&mono.degree))
			.ok()
			.and_then(|index| self.0.get(index))
	}

	/// Returns the monomial with the given degree, or [`None`] if the degree is not present.
	///
	/// # Examples
	///
	/// ```
	/// use abacas::monomial::Monomial;
	/// use abacas::polynomial::Polynomial;
	///
	/// let mut poly: Polynomial = "4x^9 + 2x^3 + x^2 + 100".parse().unwrap();
	/// assert_eq!(poly.get_mut(&9.into()), Some(&mut Monomial::new(4, 9)));
	/// ```
	pub fn get_mut(&mut self, degree: &Integer) -> Option<&mut Monomial> {
		self.0
			.binary_search_by(|mono| degree.cmp(&mono.degree))
			.ok()
			.and_then(|index| self.0.get_mut(index))
	}

	/// Internal method to get a monomial or insert it if it does not exist.
	fn get_or_insert(&mut self, degree: &Integer) -> &mut Monomial {
		let index = self
			.0
			.binary_search_by(|mono| degree.cmp(&mono.degree))
			.inspect_err(|&index| {
				let coeff = 0.into();
				let degree = degree.clone();

				self.0.insert(index, Monomial { coeff, degree });
			})
			.unwrap_or_else(|index| index);

		&mut self.0[index]
	}

	/// Creates a monic polynomial by dividing all monomials by the leading coefficient.
	/// Returns [`None`] if the polynomial is zero or already monic.
	///
	/// # Examples
	///
	/// ```
	/// use abacas::polynomial::Polynomial;
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
	/// use abacas::polynomial::Polynomial;
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

	/// Creates a new polynomial from the given monomials.
	///
	/// # Examples
	///
	/// ```
	/// use abacas::monomial::Monomial;
	/// use abacas::polynomial::Polynomial;
	///
	/// let poly = Polynomial::new([Monomial::new(4, 2), Monomial::new(9, 9)]);
	/// assert_eq!(poly.to_string(), "9x^9 + 4x^2");
	/// ```
	pub fn new(monomials: impl IntoIterator<Item = Monomial>) -> Self {
		Self::from_iter(monomials)
	}
}

impl<T: Into<Monomial>> From<T> for Polynomial {
	fn from(value: T) -> Self {
		Self::new([value.into()])
	}
}

impl FromIterator<Monomial> for Polynomial {
	fn from_iter<T: IntoIterator<Item = Monomial>>(iter: T) -> Self {
		iter.into_iter().fold(Self::ZERO, Self::add)
	}
}

impl<T: Into<Monomial>> Add<T> for Polynomial {
	type Output = Self;

	fn add(mut self, rhs: T) -> Self::Output {
		self += rhs;
		self
	}
}

impl Add for Polynomial {
	type Output = Self;

	fn add(mut self, rhs: Self) -> Self::Output {
		self += rhs;
		self
	}
}

impl<T: Into<Monomial>> AddAssign<T> for Polynomial {
	fn add_assign(&mut self, rhs: T) {
		let rhs = rhs.into();

		match self.0.binary_search_by(|mono| rhs.degree.cmp(&mono.degree)) {
			Ok(index) => self.0[index].coeff += rhs.coeff,
			Err(index) => self.0.insert(index, rhs),
		}

		self.clean();
	}
}

impl AddAssign for Polynomial {
	fn add_assign(&mut self, rhs: Self) {
		for monomial in rhs.0 {
			*self += monomial;
		}
	}
}

impl<T: Into<Monomial>> Div<T> for Polynomial {
	type Output = Self;

	fn div(mut self, rhs: T) -> Self::Output {
		self /= rhs;
		self
	}
}

impl Div for Polynomial {
	type Output = Self;

	fn div(mut self, rhs: Self) -> Self::Output {
		self /= rhs;
		self
	}
}

impl<T: Into<Monomial>> DivAssign<T> for Polynomial {
	fn div_assign(&mut self, rhs: T) {
		let rhs = rhs.into();

		for monomial in self.0.iter_mut() {
			*monomial /= rhs.clone();
		}
	}
}

impl DivAssign for Polynomial {
	fn div_assign(&mut self, rhs: Self) {
		self.div_rem_mut(&rhs).expect("abacas: cannot divide by zero");
	}
}

impl<T: Into<Monomial>> Mul<T> for Polynomial {
	type Output = Self;

	fn mul(mut self, rhs: T) -> Self::Output {
		self *= rhs;
		self
	}
}

impl Mul for Polynomial {
	type Output = Self;

	fn mul(mut self, rhs: Self) -> Self::Output {
		self *= rhs;
		self
	}
}

impl<T: Into<Monomial>> MulAssign<T> for Polynomial {
	fn mul_assign(&mut self, rhs: T) {
		let rhs = rhs.into();

		for monomial in self.0.iter_mut() {
			*monomial *= rhs.clone();
		}
	}
}

impl MulAssign for Polynomial {
	fn mul_assign(&mut self, rhs: Self) {
		let old = mem::take(self);

		for monomial in rhs.0 {
			*self += old.clone() * monomial;
		}
	}
}

impl Neg for Polynomial {
	type Output = Self;

	fn neg(mut self) -> Self::Output {
		self.neg_assign();
		self
	}
}

impl NegAssign for Polynomial {
	fn neg_assign(&mut self) {
		for monomial in self.0.iter_mut() {
			monomial.neg_assign();
		}
	}
}

impl Rem for Polynomial {
	type Output = Self;

	fn rem(mut self, rhs: Self) -> Self::Output {
		self %= rhs;
		self
	}
}

impl RemAssign for Polynomial {
	fn rem_assign(&mut self, rhs: Self) {
		*self = self.div_rem_mut(&rhs).expect("abacas: cannot divide by zero");
	}
}

impl<T: Into<Monomial>> Sub<T> for Polynomial {
	type Output = Self;

	fn sub(mut self, rhs: T) -> Self::Output {
		self -= rhs;
		self
	}
}

impl Sub for Polynomial {
	type Output = Self;

	fn sub(mut self, rhs: Self) -> Self::Output {
		self -= rhs;
		self
	}
}

impl<T: Into<Monomial>> SubAssign<T> for Polynomial {
	fn sub_assign(&mut self, rhs: T) {
		let rhs = rhs.into();

		match self.0.binary_search_by(|mono| rhs.degree.cmp(&mono.degree)) {
			Ok(index) => self.0[index].coeff -= rhs.coeff,
			Err(index) => self.0.insert(index, -rhs),
		}

		self.clean();
	}
}

impl SubAssign for Polynomial {
	fn sub_assign(&mut self, rhs: Self) {
		for monomial in rhs.0 {
			*self -= monomial;
		}
	}
}

impl fmt::Display for Polynomial {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self.0.first() {
			Some(first) => write!(f, "{first}")?,
			None => write!(f, "0")?,
		}

		for monomial in self.0.iter().skip(1) {
			if monomial.coeff.is_positive() {
				write!(f, " + {monomial}")?;
			} else {
				// TODO: Find an alternative without allocations
				write!(f, " - {}", -monomial.clone())?;
			}
		}

		Ok(())
	}
}

impl str::FromStr for Polynomial {
	type Err = ParseError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.trim() == "0" {
			Ok(Self::ZERO)
		} else {
			let s: String = s.chars().filter(|c| !c.is_ascii_whitespace()).collect();

			let (neg_first, s) = s.strip_prefix('-').map_or((false, s.as_str()), |rest| (true, rest));

			s.replace("-", "+-")
				.split('+')
				.enumerate()
				.map(|(i, m)| {
					m.parse::<Monomial>()
						.map(|mon| if i == 0 && neg_first { -mon } else { mon })
				})
				.collect()
		}
	}
}
