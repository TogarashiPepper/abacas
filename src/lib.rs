pub mod monomial;
pub mod polynomial;

#[cfg(test)]
mod tests {
    use crate::{monomial::Monomial, polynomial::Polynomial};

    #[test]
    fn polynomial() {
        let m = Monomial::new(4.0, 2);
        let r = Monomial::new(5.0, 2);
        let res =
            (m + r + Monomial::new(1.0, 1000)) * (Monomial::new(10.0, 3) + Monomial::new(4.0, 21));

        assert_eq!(
            res.to_string(),
            Polynomial::new(vec![
                Monomial::new(4.0, 1021),
                Monomial::new(10.0, 1003),
                Monomial::new(36.0, 23),
                Monomial::new(90.0, 5)
            ])
            .to_string()
        );
    }
}
