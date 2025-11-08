use std::f64::consts::E;
use std::ops::{Add, Mul, Sub};

use abacas::monomial::Monomial;
use abacas::number::Number;
use abacas::polynomial::Polynomial;
use rug::Integer;

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
	PreOp {
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

			Exp::PreOp { op, rhs } => match (op, rhs.fold()) {
				(Token::Sub, Exp::Polynomial(p)) => Exp::Polynomial(-p),
				(Token::Sub, Exp::Number(n)) => match n {
					Number::Integer(int) => Exp::Number((-int).into()),
					Number::Natural(nat) => Exp::Number((-nat).into()),
					Number::Rational(rat) => Exp::Number((-rat).into()),
				},

				(op, rhs) => Exp::PreOp {
					op,
					rhs: Box::new(rhs),
				},
			},

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

				// Fold Poly {+, *} Ident into Poly (given same ident)
				(Exp::Polynomial(p), Token::Add | Token::Mul, Expression::Ident(n))
				| (Exp::Ident(n), Token::Add | Token::Mul, Exp::Polynomial(p))
					if n == "x" =>
				{
					let mon = Monomial::linear(1);
					let op = match op {
						Token::Add => Add::add,
						Token::Mul => Mul::mul,

						_ => unreachable!(),
					};

					Exp::Polynomial(op(p, mon))
				}

				// Fold Poly - Ident into Poly (given same ident)
				(Exp::Polynomial(p), Token::Sub, Expression::Ident(n)) if n == "x" => {
					Exp::Polynomial(p - Monomial::linear(1))
				}
				(Exp::Ident(n), Token::Sub, Expression::Polynomial(p)) if n == "x" => {
					Exp::Polynomial(Monomial::linear(1) - p)
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

				// Fold x {+, -} num into Poly
				(Exp::Ident(i), Token::Add, Exp::Number(num))
				| (Exp::Number(num), Token::Add, Exp::Ident(i))
					if i == "x" =>
				{
					Exp::Polynomial(Monomial::linear(1) + num)
				}
				(Exp::Ident(i), Token::Sub, Exp::Number(n)) if i == "x" => {
					Exp::Polynomial(Monomial::linear(1) - n)
				}
				(Exp::Number(n), Token::Sub, Exp::Ident(i)) if i == "x" => {
					Exp::Polynomial(Polynomial::new([Monomial::from(n), Monomial::linear(-1)]))
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
