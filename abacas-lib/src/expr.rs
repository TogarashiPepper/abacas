//! Module containing [`Expr`] and related structs, like [`Symbol`]
use std::cmp::Ordering;
use std::ops;

use rug::{Complete, Rational};

use crate::polynomial::Polynomial;

/// Struct representing a Symbol, i.e. `x`, `π`, or even something like `T_area`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol(String);

/// Represents a general expression
#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Expr {
	/// Represents the sum of some [`Expr`]s
	Add(Vec<Expr>),
	/// Represents the product of some [`Expr`]s
	Mul(Vec<Expr>),
	/// Represents the additive inverse of an [`Expr`]
	Neg(Box<Expr>),
	/// Represents the multiplicative inverse of an [`Expr`]
	Inv(Box<Expr>),
	/// A constant number
	Number(Rational),
	/// A variable
	Var(Symbol),
	/// A function call
	Fun(Symbol, Box<Expr>),
	/// Special form of [`Expr`] that permits extra operations
	Poly(Symbol, Polynomial),
}

impl Expr {
	/// Returns `true` if the expr is [`Number`].
	///
	/// [`Number`]: Expr::Number
	#[must_use]
	fn is_number(&self) -> bool {
		matches!(self, Self::Number(..))
	}

	fn as_var(&self) -> Option<&Symbol> {
		if let Self::Var(v) = self {
			Some(v)
		} else {
			None
		}
	}

	fn zero() -> Expr {
		Expr::Number(Rational::ZERO.clone())
	}

	fn one() -> Expr {
		Expr::Number(Rational::ONE.clone())
	}

	fn inv(self) -> Self {
		match self {
			a @ (Add(_) | Neg(_) | Var(_) | Fun(..) | Poly(..)) => Inv(Box::new(a)),
			Mul(exprs) => Mul(exprs.into_iter().map(Self::inv).collect()),
			Inv(expr) => *expr,
			Number(rational) => Number(rational.recip()),
		}
	}

	// Normalize the representation
	// At the moment:
	// - turns Mul[x] -> x
	// - turns Add[x] -> x
	fn normalize(&mut self) {
		match self {
			Add(exprs) | Mul(exprs) if exprs.len() == 1 => *self = exprs.remove(0),
			_ => {}
		}
	}

	/// Simplify a sum. Currently handles:
	/// - reducing 2 + 3 + x + 4 -> x + 9
	fn simplify_add(mut exprs: Vec<Expr>) -> Vec<Expr> {
		for exp in &mut exprs {
			exp.normalize();
		}
		exprs.sort_by(Expr::cmp);

		// Group all lone numbers into one
		let sum = exprs
			.extract_if(.., |e| e.is_number())
			.fold(Expr::zero(), |a, b| a + b);

		let mut multiset: Vec<(Expr, Rational)> = vec![];

		for mut exp in exprs {
			match exp {
				Mul(es) => exp = Mul(Expr::simplify_mul(es)),
				Add(es) => exp = Add(Expr::simplify_add(es)),
				_ => {}
			}

			let (coeff, core) = match exp {
				c @ (Add(_) | Inv(_) | Var(_) | Fun(..) | Poly(..)) => (Rational::ONE.clone(), c),
				Neg(exp) => (-Rational::ONE.clone(), *exp),
				Mul(exprs) => {
					let (x, y) = get_number(exprs);
					(x, Mul(y))
				}
				Number(_) => unreachable!(),
			};

			match multiset.binary_search_by(|(e, _)| Expr::cmp(e, &core)) {
				Ok(idx) => multiset[idx].1 += coeff,
				Err(idx) => multiset.insert(idx, (core, coeff)),
			}
		}

		let mut end: Vec<Expr> = multiset
			.into_iter()
			.flat_map(|(exp, coeff)| {
				if coeff.is_zero() {
					None
				} else if &coeff == Rational::ONE {
					Some(exp)
				} else if coeff == (-Rational::ONE).complete() {
					Some(Neg(Box::new(exp)))
				} else {
					Some(Mul(vec![Number(coeff), exp]))
				}
			})
			.collect();
		end.push(sum);
		end.sort_by(Expr::cmp);

		end
	}

	/// Simplify a product. Currently handles:
	/// - reducing 2 * 4 * x * y * 3 -> x * y * 24
	fn simplify_mul(mut exprs: Vec<Expr>) -> Vec<Expr> {
		for exp in &mut exprs {
			exp.normalize();
		}
		exprs.sort_by(Expr::cmp);

		let prod = exprs
			.extract_if(.., |e| e.is_number())
			.fold(Expr::one(), |a, b| a * b);

		push(exprs, prod)
	}

	fn cmp(a: &Expr, b: &Expr) -> Ordering {
		match (a, b) {
			(Add(p), Add(q)) => {
				let mut p = p.clone();
				let mut q = q.clone();

				p.sort_by(Expr::cmp);
				q.sort_by(Expr::cmp);

				let (a, b) = p
					.into_iter()
					.zip(q)
					.find(|(x, y)| Expr::cmp(x, y) != Ordering::Equal)
					.unwrap();

				Expr::cmp(&a, &b)
			}
			(Add(..), _) => Ordering::Greater,
			(_, Add(..)) => Ordering::Less,

			(Mul(p), Mul(q)) => {
				let mut p = p.clone();
				let mut q = q.clone();

				p.sort_by(Expr::cmp);
				q.sort_by(Expr::cmp);

				let (a, b) = p
					.into_iter()
					.zip(q)
					.find(|(x, y)| Expr::cmp(x, y) != Ordering::Equal)
					.unwrap();

				Expr::cmp(&a, &b)
			}
			(Mul(..), _) => Ordering::Greater,
			(_, Mul(..)) => Ordering::Less,

			(Neg(p), Neg(q)) => Expr::cmp(p, q),
			(Neg(..), _) => Ordering::Greater,
			(_, Neg(..)) => Ordering::Less,

			(Inv(p), Inv(q)) => Expr::cmp(p, q),
			(Inv(..), _) => Ordering::Greater,
			(_, Inv(..)) => Ordering::Less,

			(Number(p), Number(q)) => p.cmp(q),
			(Number(..), _) => Ordering::Greater,
			(_, Number(..)) => Ordering::Less,

			(Var(p), Var(q)) => p.cmp(q),
			(Var(..), _) => Ordering::Greater,
			(_, Var(..)) => Ordering::Less,

			(Fun(r, p), Fun(s, q)) => {
				let ord = Expr::cmp(p, q);

				if ord == Ordering::Equal {
					r.cmp(s)
				} else {
					ord
				}
			}
			(Fun(..), _) => Ordering::Greater,
			(_, Fun(..)) => Ordering::Less,

			(Poly(r, p), Poly(s, q)) => {
				let mut ord = r.cmp(s);

				if ord == Ordering::Equal {
					ord = p.degree().cmp(&q.degree());
				}

				ord
			} // Poly catchalls aren't needed since theres nothing after poly
		}
	}
}

fn concat(mut l: Vec<Expr>, r: Vec<Expr>) -> Vec<Expr> {
	l.extend(r);
	l
}

fn push(mut l: Vec<Expr>, r: Expr) -> Vec<Expr> {
	let (Ok(idx) | Err(idx)) = l.binary_search_by(|x| Expr::cmp(x, &r));
	l.insert(idx, r);
	l
}

fn get_number(mut v: Vec<Expr>) -> (Rational, Vec<Expr>) {
	let idx = v.iter().position(|e| matches!(e, Number(..))).unwrap();
	let num = match v.remove(idx) {
		Number(n) => n,
		_ => unreachable!(),
	};

	(num, v)
}

fn find_num(v: &mut [Expr]) -> Option<&mut Rational> {
	v.iter()
		.position(|e| e.is_number())
		.map(|p| match &mut v[p] {
			Expr::Number(n) => n,
			_ => unreachable!(),
		})
}

use Expr::*;

impl ops::Add for Expr {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Add(l), Add(r)) => Add(concat(l, r)),

			(Add(mut a), Number(num)) | (Number(num), Add(mut a)) => {
				if let Some(existing_num) = find_num(&mut a) {
					*existing_num += num;

					Expr::Add(a)
				} else {
					Expr::Add(push(a, Expr::Number(num)))
				}
			}

			// y + 2x - y => 2x
			(Add(mut a), Neg(neg)) | (Neg(neg), Add(mut a)) => {
				if let Some(pos) = a.iter().position(|e| *e == *neg) {
					a.remove(pos);
				}

				Expr::Add(push(a, Neg(neg)))
			}
			(Add(a), other) | (other, Add(a)) => Add(push(a, other)),

			(Number(l), Number(r)) => Number(l + r),

			(Poly(s1, p1), Poly(s2, p2)) if s1 == s2 => Poly(s1, p1 + p2),

			(l, r) => Add(vec![l, r]),
		}
	}
}

impl ops::Neg for Expr {
	type Output = Self;

	fn neg(self) -> Self::Output {
		match self {
			e @ (Mul(_) | Inv(_) | Var(_) | Fun(..)) => Neg(Box::new(e)),
			Add(exprs) => Add(exprs.into_iter().map(|e| -e).collect()),
			Poly(sym, inner) => Poly(sym, -inner),
			Neg(expr) => *expr,
			Number(rational) => Number(-rational),
		}
	}
}

impl ops::Sub for Expr {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		self + -rhs
	}
}

impl ops::Mul for Expr {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(Mul(l), Mul(r)) => Mul(concat(l, r)),

			(Mul(mut a), Number(num)) | (Number(num), Mul(mut a)) => {
				if let Some(existing_num) = find_num(&mut a) {
					*existing_num *= num;

					Expr::Mul(a)
				} else {
					Expr::Mul(push(a, Expr::Number(num)))
				}
			}

			// (2x / y) * y => 2x
			(Mul(mut a), Inv(inv)) | (Inv(inv), Mul(mut a)) => {
				if let Some(pos) = a.iter().position(|e| *e == *inv) {
					a.remove(pos);
				}

				Expr::Mul(push(a, Inv(inv)))
			}
			(Mul(m), other) | (other, Mul(m)) => Mul(push(m, other)),
			(Number(l), Number(r)) => Number(l * r),

			(l, r) => Mul(vec![l, r]),
		}
	}
}

impl ops::Div for Expr {
	type Output = Self;

	#[allow(clippy::suspicious_arithmetic_impl)]
	fn div(self, rhs: Self) -> Self::Output {
		self * rhs.inv()
	}
}
