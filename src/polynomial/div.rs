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
        let divisor = rhs;

        let normalizer = *divisor.0.last().unwrap();

        let l1 = dividend.0.last().unwrap().degree as usize;
        let l2 = divisor.0.last().unwrap().degree as usize;

        let len = l1 - l2;

        for i in 0..=len {
            let term = dividend.deg_mut((l1 - i) as i64);
            term.coeff = term.coeff / normalizer.coeff;
            let coeff = term.coeff;

            if coeff != 0.0 {
                for j in 1..=l2 as usize {
                    let var = dividend.deg_mut((l1 - i - j) as i64);
                    var.coeff = var.coeff - divisor.0[l2 - j].coeff * coeff;
                }
            }
        }

        dividend = dividend / Monomial::new(1.0, divisor.0.last().unwrap().degree);

        dividend.clean();

        dividend
    }
}
