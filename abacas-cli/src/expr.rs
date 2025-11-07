use std::ops::{Add, Mul, Sub};

use abacas::monomial::Monomial;
use abacas::number::Number;
use abacas::polynomial::Polynomial;

use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expression {
	Number(Number),
	Ident(String),
	Polynomial(Polynomial),
	BinOp {
		lhs: Box<Expression>,
		op: Token,
		rhs: Box<Expression>,
	},
}

impl Expression {
	pub fn fold(self) -> Self {
		use Expression as Exp;

		match self {
			// Numbers and idents cant be reduced further, polynomials handle their own reducing
			x @ (Exp::Number(_) | Exp::Ident(_) | Exp::Polynomial(_)) => x,

			Exp::BinOp { lhs, op, rhs } => match (lhs.fold(), op.clone(), rhs.fold()) {
				// Fold x * number into Poly
				(Exp::Number(n), Token::Mul, Exp::Ident(i))
				| (Exp::Ident(i), Token::Mul, Exp::Number(n))
					if i == "x" =>
				{
					let mono = Monomial::linear(1);

					Exp::Polynomial((mono * n).into())
				}

				// Fold (x ^ number) into a polynomial (x^n)
				(
					Exp::Ident(name),
					Token::Pow,
					Exp::Number(Number::Integer(i) | Number::Natural(i)),
				) if name == "x" => Expression::Polynomial(Monomial::new(1, i).into()),

				// Fold the number * poly into poly
				(Exp::Number(n), Token::Mul, Exp::Polynomial(m))
				| (Exp::Polynomial(m), Token::Mul, Exp::Number(n)) => Exp::Polynomial(m * n),

				// Fold Poly {+, -, *} Poly into Poly
				(Exp::Polynomial(m), Token::Add | Token::Sub | Token::Mul, Exp::Polynomial(m2)) => {
					let op = match op {
						Token::Add => Add::add,
						Token::Sub => Sub::sub,
						Token::Mul => Mul::mul,

						_ => unreachable!(),
					};

					Exp::Polynomial(op(m, m2))
				}

				// Fold Poly + num into Poly
				(Exp::Polynomial(p), Token::Add, Exp::Number(n))
				| (Exp::Number(n), Token::Add, Exp::Polynomial(p)) => Exp::Polynomial(p + n),

				// Fold poly - num into Poly
				(Exp::Polynomial(p), Token::Sub, Exp::Number(n)) => Exp::Polynomial(p - n),
				(Exp::Number(n), Token::Sub, Exp::Polynomial(p)) => Exp::Polynomial(-p + n),

				// Fold Poly / Poly into Poly if they share a factor
				(Exp::Polynomial(m), Token::Div, Exp::Polynomial(m2))
					if m.clone().gcd(m2.clone()) != Polynomial::from(1) =>
				{
					let (div, rem) = m.div_rem(&m2).unwrap();

					assert_eq!(rem, Polynomial::ZERO);

					Exp::Polynomial(div)
				}

				(lhs, _, rhs) => Exp::BinOp {
					lhs: Box::new(lhs),
					op,
					rhs: Box::new(rhs),
				},
			},
		}
	}
}
