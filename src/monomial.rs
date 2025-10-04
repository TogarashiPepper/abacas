use std::fmt;
use std::ops::{Add, Div, DivAssign, Mul, MulAssign, Neg, Sub};

use crate::polynomial::Polynomial;

/// A monomial `ax^b` consisting of coefficient `a` and degree `b`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Monomial {
	pub coeff: f64,
	pub degree: i64,
}

impl Monomial {
	/// Creates a new monomial. Panics if `coeff` is zero.
	pub const fn new(coeff: f64, degree: i64) -> Self {
		if coeff == 0.0 {
			panic!("abacas: monomial coefficient must not be zero");
		}

		Monomial { coeff, degree }
	}

    pub const fn constant(coeff: f64) -> Self {
        Monomial { coeff, degree: 1 }
    }
}

impl Add for Monomial {
	type Output = Polynomial;

	fn add(self, rhs: Self) -> Self::Output {
		Polynomial::new([self, rhs])
	}
}

impl Div for Monomial {
	type Output = Monomial;

	fn div(mut self, rhs: Self) -> Self::Output {
		self /= rhs;
		self
	}
}

impl DivAssign for Monomial {
	fn div_assign(&mut self, rhs: Self) {
		self.coeff /= rhs.coeff;
		self.degree -= rhs.degree;
	}
}

impl Mul for Monomial {
	type Output = Monomial;

	fn mul(mut self, rhs: Self) -> Self::Output {
		self *= rhs;
		self
	}
}

impl MulAssign for Monomial {
	fn mul_assign(&mut self, rhs: Self) {
		self.coeff *= rhs.coeff;
		self.degree += rhs.degree;
	}
}

impl Neg for Monomial {
	type Output = Monomial;

	fn neg(mut self) -> Self::Output {
		self.coeff = -self.coeff;
		self
	}
}

impl Sub for Monomial {
	type Output = Polynomial;

	fn sub(self, rhs: Self) -> Self::Output {
		Polynomial::new([self, -rhs])
	}
}

impl fmt::Display for Monomial {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.coeff != 1.0 || self.degree == 0 {
			write!(f, "{}", self.coeff)?;
		}

		if self.degree == 1 {
			write!(f, "x")?;
		} else if self.degree != 0 {
			write!(f, "x^{}", self.degree)?;
		}

		Ok(())
	}
}
