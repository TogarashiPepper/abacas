use std::collections::HashMap;

use abacas::polynomial::Polynomial;

use crate::expr::Expression;
use crate::token::Token::*;

pub struct Interpreter {
	variables: HashMap<String, Expression>,
}

impl Interpreter {
	pub fn new() -> Self {
		let variables = HashMap::new();

		// TODO: add constants like PI, E

		Self { variables }
	}
	pub fn execute_line(&mut self, ast: Expression) -> Expression {
		self.evaluate_expression(ast)
	}

	fn evaluate_expression(&mut self, expr: Expression) -> Expression {
		match expr {
			Expression::Number(number) => Expression::Number(number),
			Expression::Ident(ident) => self
				.variables
				.get(&ident)
				.unwrap_or(&Expression::Ident(ident))
				.clone(),
			Expression::Polynomial(polynomial) => Expression::Polynomial(polynomial),
			Expression::BinOp { lhs, op, rhs } => match op {
				Eq => {
					if let Expression::Ident(ident) = *lhs {
						let data = self.evaluate_expression(*rhs);

						self.variables.insert(ident, data.clone());

						data
					} else {
						panic!("expected identifier")
					}
				}
				Add => {
					let lhd = self.evaluate_expression(*lhs);
					let rhd = self.evaluate_expression(*rhs);

					match (lhd, rhd) {
						(Expression::Polynomial(p1), Expression::Polynomial(p2)) => {
							Expression::Polynomial(p1 + p2)
						}
						(Expression::Polynomial(p), Expression::Number(n))
						| (Expression::Number(n), Expression::Polynomial(p)) => Expression::Polynomial(p + n),
						(Expression::Number(n1), Expression::Number(n2)) => {
							Expression::Number(n1 + n2)
						}

						_ => unreachable!(),
					}
				}
				Sub => {
					let lhd = self.evaluate_expression(*lhs);
					let rhd = self.evaluate_expression(*rhs);

					match (lhd, rhd) {
						(Expression::Polynomial(p1), Expression::Polynomial(p2)) => {
							Expression::Polynomial(p1 - p2)
						}
						(Expression::Polynomial(p), Expression::Number(n))
						| (Expression::Number(n), Expression::Polynomial(p)) => Expression::Polynomial(p - n),
						(Expression::Number(n1), Expression::Number(n2)) => {
							Expression::Number(n1 - n2)
						}

						_ => unreachable!(),
					}
				}
				Mul => {
					let lhd = self.evaluate_expression(*lhs);
					let rhd = self.evaluate_expression(*rhs);

					match (lhd, rhd) {
						(Expression::Polynomial(p1), Expression::Polynomial(p2)) => {
							Expression::Polynomial(p1 * p2)
						}
						(Expression::Polynomial(p), Expression::Number(n))
						| (Expression::Number(n), Expression::Polynomial(p)) => Expression::Polynomial(p * n),
						(Expression::Number(n1), Expression::Number(n2)) => {
							Expression::Number(n1 * n2)
						}

						_ => unreachable!(),
					}
				}
				Div => {
					let lhd = self.evaluate_expression(*lhs);
					let rhd = self.evaluate_expression(*rhs);

					match (lhd, rhd) {
						(Expression::Polynomial(p1), Expression::Polynomial(p2)) => {
							Expression::Polynomial(p1 / p2)
						}
						(Expression::Polynomial(p), Expression::Number(n))
						| (Expression::Number(n), Expression::Polynomial(p)) => Expression::Polynomial(p / n),
						(Expression::Number(n1), Expression::Number(n2)) => {
							Expression::Number(n1 / n2)
						}

						_ => unreachable!(),
					}
				}
				Pow => {
					let lhd = self.evaluate_expression(*lhs);
					let rhd = self.evaluate_expression(*rhs);

					match (lhd, rhd) {
						(Expression::Polynomial(p1), Expression::Polynomial(p2)) => {
							Expression::BinOp {
								lhs: Box::new(Expression::Polynomial(p1)),
								op: Pow,
								rhs: Box::new(Expression::Polynomial(p2)),
							}
						}
						(Expression::Polynomial(_), Expression::Number(_)) => {
							// TODO: implement when polynomial has method for pow
							unimplemented!()
						}
						(Expression::Number(n), Expression::Polynomial(p)) => Expression::BinOp {
							lhs: Box::new(Expression::Number(n)),
							op: Pow,
							rhs: Box::new(Expression::Polynomial(p)),
						},
						(Expression::Number(_), Expression::Number(_)) => {
							// TODO: implement when number has method for pow
							unimplemented!()
						}

						_ => unreachable!(),
					}
				}
				Rem => {
					let lhd = self.evaluate_expression(*lhs);
					let rhd = self.evaluate_expression(*rhs);

					match (lhd, rhd) {
						(Expression::Polynomial(mut p1), Expression::Polynomial(p2)) => {
							Expression::Polynomial(p1.div_rem_mut(&p2).unwrap_or(Polynomial::ZERO))
						}
						(Expression::Polynomial(_), Expression::Number(_)) => {
							Expression::Polynomial(Polynomial::ZERO)
						}
						(Expression::Number(n), Expression::Polynomial(p)) => Expression::BinOp {
							lhs: Box::new(Expression::Number(n)),
							op: Rem,
							rhs: Box::new(Expression::Polynomial(p)),
						},
						(Expression::Number(n1), Expression::Number(n2)) => {
							Expression::Number(n1.rem(n2))
						}

						_ => unreachable!(),
					}
				}

				_ => unreachable!(),
			},
			Expression::PreOp { op, rhs } => match op {
				Sub => {
					let rhd = self.evaluate_expression(*rhs);

					match rhd {
						Expression::Number(number) => Expression::Number(-number),
						Expression::Ident(ident) => Expression::PreOp {
							op: Sub,
							rhs: Box::new(Expression::Ident(ident)),
						},
						Expression::Polynomial(polynomial) => Expression::Polynomial(-polynomial),
						Expression::BinOp { lhs, op, rhs } => Expression::PreOp {
							op: Sub,
							rhs: Box::new(Expression::BinOp { lhs, op, rhs }),
						},
						Expression::PreOp { op, rhs } => match op {
							Sub => *rhs,
							_ => Expression::PreOp { op: Sub, rhs },
						},
					}
				}

				_ => unreachable!(),
			},
		}
	}
}
