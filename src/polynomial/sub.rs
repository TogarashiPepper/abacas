use std::ops::{Sub, SubAssign};

use crate::monomial::Monomial;

use super::Polynomial;

impl Sub<Monomial> for Polynomial {
    type Output = Polynomial;

    fn sub(mut self, rhs: Monomial) -> Self::Output {
        self -= rhs;

        self
    }
}

impl SubAssign<Monomial> for Polynomial {
    fn sub_assign(&mut self, rhs: Monomial) {
        self.deg_mut(rhs.degree).coeff -= rhs.coeff;
    }
}

impl Sub for Polynomial {
    type Output = Polynomial;

    fn sub(mut self, rhs: Self) -> Self::Output {
        for mono in rhs.0 {
            self -= mono;
        }

        self
    }
}

