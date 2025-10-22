use abacas::number::Number;
use abacas::polynomial::Polynomial;

#[derive(Debug, Clone)]
pub enum Expression {
	Number(Number),
	Polynomial(Polynomial),
}
