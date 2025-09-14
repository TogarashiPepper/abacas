use std::ops::{Div, DivAssign};

use crate::monomial::Monomial;

use super::Polynomial;

impl Div<Monomial> for Polynomial {
    type Output = Polynomial;

    fn div(mut self, rhs: Monomial) -> Self::Output {
        self /= rhs;

        self
    }
}

impl DivAssign<Monomial> for Polynomial {
    fn div_assign(&mut self, rhs: Monomial) {
        for mono in self.0.iter_mut() {
            *mono = *mono / rhs;
        }
    }
}

impl Div for Polynomial {
    type Output = Polynomial;

    fn div(self, rhs: Self) -> Self::Output {
        let mut dividend = self;

        let normalizer = *rhs.0.get(0).unwrap();

        for i in 0..(dividend.0.len() - rhs.0.len() + 1) {
            dividend.0[i] = dividend.0[i] / normalizer;
            let coeff = dividend.0[i];

            if coeff.coeff != 0.0 {
                for j in 1..rhs.0.len() {
                    dividend.0[i + j] = (dividend.0[i + j] - rhs.0[j] * coeff).0[0];
                }
            }
        }

        dividend
    }
}
