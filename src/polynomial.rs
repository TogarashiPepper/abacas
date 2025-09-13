use std::{
    fmt::Debug,
    ops::{Add, Mul, Sub},
};

use crate::monomial::Monomial;

/// Sorted by `degree`
#[derive(Clone)]
pub struct Polynomial(Vec<Monomial>);

impl Polynomial {
    pub fn new(v: Vec<Monomial>) -> Self {
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

        self.0.sort_by_key(|m| m.degree);
        
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
        for mono in rhs.0.into_iter() {
            self = self + mono;
        }

        self
    }
}

impl Sub for Polynomial {
    type Output = Polynomial;

    fn sub(mut self, rhs: Self) -> Self::Output {
        for mono in rhs.0.into_iter() {
            self = self - mono;
        }

        self
    }
}

impl Debug for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.0.iter().rev()).finish()
    }
}
