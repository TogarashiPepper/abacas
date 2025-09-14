use std::{
    cmp::Ordering,
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::polynomial::Polynomial;

#[derive(Clone, Copy, Debug)]
pub struct Monomial {
    pub coeff: f64,
    pub degree: i64,
}

impl Monomial {
    pub fn new(coeff: f64, degree: i64) -> Self {
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

impl Display for Monomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.coeff == 0.0 {
            write!(f, "0")
        } else if self.degree == 0 {
            write!(f, "{}", self.coeff)
        } else if self.degree == 1 {
            if self.coeff == 1.0 {
                write!(f, "x")
            } else {
                write!(f, "{}x", self.coeff)
            }
        } else {
            write!(f, "{}x^{}", self.coeff, self.degree)
        }
    }
}
