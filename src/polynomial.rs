use std::{
    fmt::{Debug, Display},
    ops::{Add, Mul, Sub},
};

use itertools::Itertools;

use crate::monomial::Monomial;

/// Sorted by `degree`
#[derive(Debug, Clone)]
pub struct Polynomial(Vec<Monomial>);

impl Polynomial {
    pub fn new(mut v: Vec<Monomial>) -> Self {
        v.sort_by_key(|m| m.degree);

        Polynomial(v)
    }
}

impl Add<Monomial> for Polynomial {
    type Output = Polynomial;

    fn add(mut self, rhs: Monomial) -> Self::Output {
        let searched = self.0.binary_search_by_key(&rhs.degree, |mono| mono.degree);

        match searched {
            Ok(idx) => {
                self.0[idx].coeff += rhs.coeff;
            }
            Err(would_be) => self.0.insert(would_be, rhs),
        }

        self
    }
}

impl Sub<Monomial> for Polynomial {
    type Output = Polynomial;

    fn sub(mut self, rhs: Monomial) -> Self::Output {
        let searched = self.0.binary_search_by_key(&rhs.degree, |mono| mono.degree);

        match searched {
            Ok(idx) => {
                self.0[idx].coeff -= rhs.coeff;
            }
            Err(would_be) => self.0.insert(would_be, -rhs),
        }

        self
    }
}

impl Mul<Monomial> for Polynomial {
    type Output = Polynomial;

    fn mul(mut self, rhs: Monomial) -> Self::Output {
        for mono in self.0.iter_mut() {
            *mono = *mono * rhs;
        }

        self
    }
}

impl Mul for Polynomial {
    type Output = Polynomial;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut acc = Polynomial::new(vec![]);

        for mono in self.0 {
            acc = acc + (rhs.clone() * mono);
        }

        acc
    }
}

impl Add for Polynomial {
    type Output = Polynomial;

    fn add(mut self, rhs: Self) -> Self::Output {
        for mono in rhs.0 {
            self = self + mono;
        }

        self
    }
}

impl Sub for Polynomial {
    type Output = Polynomial;

    fn sub(mut self, rhs: Self) -> Self::Output {
        for mono in rhs.0 {
            self = self - mono;
        }

        self
    }
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = self.0.iter().rev().map(|el| el.to_string()).join(" + ");

        write!(f, "{r}")
    }
}
