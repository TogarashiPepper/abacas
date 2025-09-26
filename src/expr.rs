use crate::{monomial::Monomial, polynomial::Polynomial};

#[derive(Debug, PartialEq)]
pub enum Expr {
	Product(Product),
	Polynomial(Polynomial),
	Monomial(Monomial),
	Number(f64),
}

#[derive(Debug, PartialEq)]
pub struct Product(pub Vec<Expr>);

impl From<Product> for Expr {
	fn from(value: Product) -> Self {
		Expr::Product(value)
	}
}
