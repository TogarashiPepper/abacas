use std::ops::{Add, AddAssign};

use crate::monomial::Monomial;

use super::Polynomial;

impl Add<Monomial> for Polynomial {
    type Output = Polynomial;

    fn add(mut self, rhs: Monomial) -> Self::Output {
        self += rhs;

        self
    }
}

impl AddAssign<Monomial> for Polynomial {
    fn add_assign(&mut self, rhs: Monomial) {
        self.deg_mut(rhs.degree).coeff += rhs.coeff;
    }
}

impl Add for Polynomial {
    type Output = Polynomial;

    fn add(mut self, rhs: Self) -> Self::Output {
        for mono in rhs.0 {
            self += mono;
        }

        self
    }
}
