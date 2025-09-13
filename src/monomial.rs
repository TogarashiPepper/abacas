use std::{
    cmp::Ordering,
    fmt::Debug,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::polynomial::Polynomial;

#[derive(Clone, Copy)]
pub struct Monomial {
    pub coeff: f64,
    pub degree: u64,
}

impl Monomial {
    pub fn new(coeff: f64, degree: u64) -> Self {
        Monomial { coeff, degree }
    }
}

impl Neg for Monomial {
    type Output = Monomial;

    fn neg(self) -> Self::Output {
        Monomial {
            coeff: -self.coeff,
            degree: self.degree,
        }
    }
}

impl Add for Monomial {
    type Output = Polynomial;

    fn add(self, rhs: Self) -> Self::Output {
        let mut poly = vec![];

        match self.degree.cmp(&rhs.degree) {
            Ordering::Equal => poly.push(Monomial {
                coeff: self.coeff + rhs.coeff,
                degree: self.degree,
            }),
            Ordering::Greater => poly.extend([self, rhs]),
            Ordering::Less => poly.extend([rhs, self]),
        }

        Polynomial::new(poly)
    }
}

impl Sub for Monomial {
    type Output = Polynomial;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut poly = vec![];

        match self.degree.cmp(&rhs.degree) {
            Ordering::Equal => poly.push(Monomial {
                coeff: self.coeff - rhs.coeff,
                degree: self.degree,
            }),
            Ordering::Greater => poly.extend([self, -rhs]),
            Ordering::Less => poly.extend([-rhs, self]),
        }

        Polynomial::new(poly)
    }
}

impl Mul for Monomial {
    type Output = Monomial;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self.degree += rhs.degree;
        self.coeff *= rhs.coeff;

        self
    }
}

impl Div for Monomial {
    type Output = Monomial;

    fn div(mut self, rhs: Self) -> Self::Output {
        self.degree -= rhs.degree;
        self.coeff /= rhs.coeff;

        self
    }
}

impl Debug for Monomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.coeff == 1.0 {
            write!(f, "x^{}", self.degree)
        } else {
            write!(f, "{}x^{}", self.coeff, self.degree)
        }
    }
}
