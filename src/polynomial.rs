use std::{fmt::Debug, ops::Add};

use crate::monomial::Monomial;

/// Sorted by `degree`
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

impl Add for Polynomial {
    type Output = Polynomial;

    fn add(mut self, rhs: Self) -> Self::Output {
        for mono in rhs.0.into_iter() {
            self = self + mono;
        }

        self
    }
}

impl Debug for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.0.iter().rev()).finish()
    }
}
