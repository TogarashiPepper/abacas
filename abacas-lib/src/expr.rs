//! Module containing [`Expr`] and related structs, like [`Symbol`]
use std::cmp::Ordering;
use std::ops;

use rug::Rational;

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

	/// Simplify a sum. Currently handles:
	/// - reducing 2 + 3 + x + 4 -> x + 9
	fn simplify_add(mut exprs: Vec<Expr>) -> Vec<Expr> {
		let sum = exprs
			.extract_if(.., |e| e.is_number())
			.fold(Expr::zero(), |a, b| a + b);

		push(exprs, sum)
	}

	/// Simplify a product. Currently handles:
	/// - reducing 2 * 4 * x * y * 3 -> x * y * 24
	fn simplify_mul(mut exprs: Vec<Expr>) -> Vec<Expr> {
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

				p.sort_by(|a, b| Expr::cmp(a, b));
				q.sort_by(|a, b| Expr::cmp(a, b));

				let (a, b) = p
					.into_iter()
					.zip(q.into_iter())
					.find(|(x, y)| Expr::cmp(x, y) != Ordering::Equal)
					.unwrap();

				Expr::cmp(&a, &b)
			}
			(Add(..), _) => Ordering::Greater,
			(Mul(..), Add(..)) => Ordering::Less,
			(Mul(p), Mul(q)) => {
				let mut p = p.clone();
				let mut q = q.clone();

				p.sort_by(|a, b| Expr::cmp(a, b));
				q.sort_by(|a, b| Expr::cmp(a, b));

				let (a, b) = p
					.into_iter()
					.zip(q.into_iter())
					.find(|(x, y)| Expr::cmp(x, y) != Ordering::Equal)
					.unwrap();

				Expr::cmp(&a, &b)
			}
			(Mul(..), _) => Ordering::Greater,
			(Neg(..), Add(..)) | (Neg(..), Mul(..)) => Ordering::Less,
			(Neg(p), Neg(q)) => Expr::cmp(p, q),
			(Neg(..), _) => Ordering::Greater,
			(Inv(..), Add(..)) | (Inv(..), Mul(..)) | (Inv(..), Neg(..)) => Ordering::Less,
			(Inv(p), Inv(q)) => Expr::cmp(p, q),
			(Inv(..), _) => Ordering::Greater,
			(Number(..), Add(..))
			| (Number(..), Mul(..))
			| (Number(..), Neg(..))
			| (Number(..), Inv(..)) => Ordering::Less,
			(Number(p), Number(q)) => p.cmp(&q),
			(Number(..), _) => Ordering::Greater,
			(Var(p), Var(q)) => p.cmp(&q),
			(Var(..), Fun(..)) | (Var(..), Poly(..)) => Ordering::Greater,
			(Var(..), _) => Ordering::Less,
			(Fun(r, p), Fun(s, q)) => {
				let ord = Expr::cmp(p, q);

				if ord == Ordering::Equal {
					r.cmp(&s)
				} else {
					ord
				}
			}
			(Fun(..), Poly(..)) => Ordering::Greater,
			(Fun(..), _) => Ordering::Less,
			(Poly(r, p), Poly(s, q)) => {
				let mut ord = r.cmp(&s);

				if ord == Ordering::Equal {
					ord = p.degree().cmp(&q.degree());
				}

				ord
			}
			(Poly(..), _) => Ordering::Less,
		}
	}
}

fn concat<T>(mut l: Vec<T>, r: Vec<T>) -> Vec<T> {
	l.extend(r);
	l
}

fn push<T>(mut l: Vec<T>, r: T) -> Vec<T> {
	l.push(r);
	l
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
