mod add;
mod mul;
mod sub;

use std::fmt::Display;

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

    /// Gets the `Monomial` with `monomial.degree == degree`
    /// # Panics
    /// Panics if there's no existing `Monomial` with `degree == index`
    fn deg(&self, degree: i64) -> &Monomial {
        let true_idx = self.0.binary_search_by_key(&degree, |m| m.degree).unwrap();
        &self.0[true_idx]
    }

    /// Gets the `Monomial` with `monomial.degree == degree`.
    /// Creates the value if doesn't already exist
    fn deg_mut(&mut self, degree: i64) -> &mut Monomial {
        let true_idx = match self.0.binary_search_by_key(&degree, |m| m.degree) {
            Ok(idx) => idx,
            Err(idx) => {
                self.0.insert(idx, Monomial::new(0.0, degree));
                idx
            }
        };

        &mut self.0[true_idx]
    }

    fn lead_coeff(&self) -> f64 {
        self.0.last().unwrap().coeff
    }
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = self.0.iter().rev().map(|el| el.to_string()).join(" + ");

        write!(f, "{r}")
    }
}
