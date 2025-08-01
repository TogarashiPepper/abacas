use std::{fmt::Debug, ops::Add};

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

impl Add for Monomial {
    type Output = Polynomial;

    fn add(self, rhs: Self) -> Self::Output {
        let mut poly = vec![];
        if self.degree == rhs.degree {
            poly.push(Monomial {
                coeff: self.coeff + rhs.coeff,
                degree: self.degree,
            })
        } else if self.degree > rhs.degree {
            poly.extend([self, rhs]);
        } else {
            poly.extend([rhs, self]);
        }

        Polynomial::new(poly)
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
