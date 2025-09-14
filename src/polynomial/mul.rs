use std::ops::{Mul, MulAssign};

use crate::monomial::Monomial;

use super::Polynomial;

impl Mul<Monomial> for Polynomial {
    type Output = Polynomial;

    fn mul(mut self, rhs: Monomial) -> Self::Output {
        self *= rhs;

        self
    }
}

impl MulAssign<Monomial> for Polynomial {
    fn mul_assign(&mut self, rhs: Monomial) {
        for mono in self.0.iter_mut() {
            *mono = *mono * rhs;
        }
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
