//! Module containing [`Expr`] and related structs, like [`Symbol`]
use std::cmp::Ordering;
use std::fmt::Display;
use std::ops;

use itertools::Itertools;
use rug::{Complete, Rational};

use crate::polynomial::Polynomial;

/// Struct representing a Symbol, i.e. `x`, `π`, or even something like `T_area`.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Symbol(String);

impl Symbol {
	/// Initialize a symbol with the given name
	pub fn new(name: String) -> Self {
		Self(name)
	}

	/// Gets the name of the symbol
	pub fn name(&self) -> &str {
		&self.0
	}
}

impl Display for Symbol {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.0)
	}
}

/// Represents a general expression
#[derive(PartialEq, Debug, Eq, Hash, Clone)]
pub enum Expr {
	/// Represents .0 to the power of .1
	Pow(Box<Expr>, Box<Expr>),
	/// Represents the sum of some [`Expr`]s
	Add(Vec<Expr>),
	/// Represents the product of some [`Expr`]s
	Mul(Vec<Expr>),
	/// Represents the additive inverse of an [`Expr`]
	Neg(Box<Expr>),
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

	fn zero() -> Expr {
		Expr::Number(Rational::ZERO.clone())
	}

	fn one() -> Expr {
		Expr::Number(Rational::ONE.clone())
	}

	fn is_one(&self) -> bool {
		match self {
			Number(n) => n == Rational::ONE,
			_ => false,
		}
	}

	fn is_zero(&self) -> bool {
		match self {
			Number(n) => n.is_zero(),
			_ => false,
		}
	}

	fn is_neg_one(&self) -> bool {
		match self {
			Number(n) => n == &-Rational::ONE.clone(),
			_ => false,
		}
	}

	fn inv(self) -> Self {
		match self {
			a @ (Add(_) | Neg(_) | Var(_) | Fun(..) | Poly(..)) => {
				Pow(Box::new(a), Box::new(Number(-Rational::ONE.clone())))
			}
			Mul(exprs) => Mul(exprs.into_iter().map(Self::inv).collect()),
			Pow(base, expr) if expr.is_neg_one() => *base,
			Pow(base, exp) => Pow(base, Box::new(-*exp)),
			Number(rational) => Number(rational.recip()),
		}
	}

	// TODO: make this return a Option so we can signal "remove this"
	// i.e for the case of x^0 in a mul or 0*x in an Add
	/// Simplify an expression (best-effort, may not fully simplify)
	pub fn simplify(self) -> Self {
		let is_add = matches!(self, Add(..));

		match self {
			Add(mut exprs) | Mul(mut exprs) if exprs.len() == 1 => exprs.remove(0).simplify(),
			Pow(base, exp) if exp.is_one() => *base,
			Add(exprs) | Mul(exprs) => {
				let mut simplified: Vec<Expr> = exprs.into_iter().map(Expr::simplify).collect();
				simplified.sort_by(Expr::cmp);

				if simplified.len() == 1 {
					simplified.remove(0)
				} else if is_add {
					Add(Self::simplify_add(simplified))
				} else {
					Mul(Self::simplify_mul(simplified))
				}
			}
			Neg(expr) => {
				if let Neg(inner_expr) = *expr {
					*inner_expr
				} else {
					Neg(expr)
				}
			}
			Pow(base, exp) if exp.is_neg_one() => match *base {
				Pow(inner_base, exp2) if exp2.is_neg_one() => *inner_base,

				base => Pow(Box::new(base), exp),
			},

			other => other,
		}
	}

	/// Simplify a sum. Currently handles:
	/// - reducing 2 + 3 + x + 4 -> x + 9
	fn simplify_add(mut exprs: Vec<Expr>) -> Vec<Expr> {
		// Group all lone numbers into one
		let sum = exprs.extract_if(.., |e| e.is_number()).fold(Expr::zero(), |a, b| a + b);

		exprs = exprs
			.into_iter()
			.flat_map(|e| {
				let e = e.simplify();
				if let Add(inner) = e { inner } else { vec![e] }
			})
			.collect();

		let mut multiset: Vec<(Expr, Rational)> = vec![];

		for exp in exprs {
			let (coeff, mut core) = match exp {
				c @ (Add(_) | Var(_) | Fun(..) | Poly(..) | Pow(..)) => (Rational::ONE.clone(), c),
				Neg(exp) => (-Rational::ONE.clone(), *exp),
				Mul(exprs) => {
					let (x, y) = get_number(exprs);
					(x.unwrap_or_else(|| Rational::ONE.clone()), Mul(y))
				}
				Number(_) => unreachable!(),
			};

			core = core.simplify();

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
			.map(Expr::simplify)
			.collect();

		if !sum.is_zero() {
			end.push(sum);
		}

		end.sort_by(Expr::cmp);
		end
	}

	/// Simplify a product. Currently handles:
	/// - reducing 2 * 4 * x * y * 3 -> x * y * 24
	fn simplify_mul(mut exprs: Vec<Expr>) -> Vec<Expr> {
		let prod = exprs.extract_if(.., |e| e.is_number()).fold(Expr::one(), |a, b| a * b);

		exprs = exprs
			.into_iter()
			.flat_map(|e| {
				let e = e.simplify();
				if let Mul(inner) = e { inner } else { vec![e] }
			})
			.collect();

		if prod.is_zero() {
			return vec![];
		}

		// .0 is base, .1 is exponent
		let mut multiset: Vec<(Expr, Expr)> = vec![];

		for exp in exprs {
			let (mut base, mut exponent) = match exp {
				Pow(base, exp) => (*base, *exp),
				other => (other, Number(Rational::ONE.clone())),
			};

			base = base.simplify();
			exponent = exponent.simplify();

			match multiset.binary_search_by(|(e, _)| Expr::cmp(e, &base)) {
				// TODO: impl AddAssign for Expr
				Ok(idx) => multiset[idx].1 = multiset[idx].1.clone() + exponent,
				Err(idx) => multiset.insert(idx, (base, exponent)),
			}
		}

		let mut end: Vec<Expr> = multiset
			.into_iter()
			.flat_map(|(base, exp)| {
				if exp.is_one() {
					Some(base)
				} else if exp.is_zero() {
					None
				} else {
					Some(Pow(Box::new(base), Box::new(exp)))
				}
			})
			.map(Expr::simplify)
			.collect();

		if !prod.is_one() {
			end.push(prod);
		}

		end.sort_by(Expr::cmp);
		end
	}

	/// Raise self to the power of rhs
	pub fn pow(self, rhs: Self) -> Self {
		Pow(Box::new(self), Box::new(rhs))
	}

	fn cmp(a: &Expr, b: &Expr) -> Ordering {
		match (a, b) {
			(Add(p), Add(q)) => {
				let mut p = p.clone();
				let mut q = q.clone();

				p.sort_by(Expr::cmp);
				q.sort_by(Expr::cmp);

				let r = p.into_iter().zip(q).find(|(x, y)| Expr::cmp(x, y) != Ordering::Equal);

				match r {
					Some((a, b)) => Expr::cmp(&a, &b),
					None => Ordering::Equal,
				}
			}
			(Add(..), _) => Ordering::Greater,
			(_, Add(..)) => Ordering::Less,

			(Mul(p), Mul(q)) => {
				let mut p = p.clone();
				let mut q = q.clone();

				p.sort_by(Expr::cmp);
				q.sort_by(Expr::cmp);

				let r = p.into_iter().zip(q).find(|(x, y)| Expr::cmp(x, y) != Ordering::Equal);

				match r {
					Some((a, b)) => Expr::cmp(&a, &b),
					None => Ordering::Equal,
				}
			}
			(Mul(..), _) => Ordering::Greater,
			(_, Mul(..)) => Ordering::Less,

			(Neg(p), Neg(q)) => Expr::cmp(p, q),
			(Neg(..), _) => Ordering::Greater,
			(_, Neg(..)) => Ordering::Less,

			(Pow(b1, e1), Pow(b2, e2)) => {
				let cmp_base = Expr::cmp(b1, b2);

				match cmp_base {
					Ordering::Less | Ordering::Greater => cmp_base,
					Ordering::Equal => Expr::cmp(e1, e2),
				}
			}
			(Pow(..), _) => Ordering::Greater,
			(_, Pow(..)) => Ordering::Less,

			(Number(p), Number(q)) => p.cmp(q),
			(Number(..), _) => Ordering::Greater,
			(_, Number(..)) => Ordering::Less,

			(Var(p), Var(q)) => p.cmp(q),
			(Var(..), _) => Ordering::Greater,
			(_, Var(..)) => Ordering::Less,

			(Fun(r, p), Fun(s, q)) => {
				let ord = Expr::cmp(p, q);

				if ord == Ordering::Equal { r.cmp(s) } else { ord }
			}
			(Fun(..), _) => Ordering::Greater,
			(_, Fun(..)) => Ordering::Less,

			(Poly(r, p), Poly(s, q)) => {
				let mut ord = r.cmp(s);

				if ord == Ordering::Equal {
					ord = p.degree().cmp(&q.degree());
				}

				ord
			}
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

fn get_number(mut v: Vec<Expr>) -> (Option<Rational>, Vec<Expr>) {
	let idx = v.iter().position(|e| matches!(e, Number(..)));
	match idx {
		Some(idx) => {
			let num = match v.remove(idx) {
				Number(n) => n,
				_ => unreachable!(),
			};

			(Some(num), v)
		}
		None => (None, v),
	}
}

fn find_num(v: &mut [Expr]) -> Option<&mut Rational> {
	v.iter().position(|e| e.is_number()).map(|p| match &mut v[p] {
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
			e @ (Mul(_) | Var(_) | Fun(..) | Pow(..)) => Neg(Box::new(e)),
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

fn needs_parens(exp: &Expr) -> bool {
	matches!(exp, Add(_) | Mul(_) | Poly(..) | Pow(..))
}

impl Display for Expr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let st = match self {
			Add(exprs) => exprs.iter().map(|e| format!("{e}")).join(" + "),
			Mul(exprs) => exprs
				.iter()
				.map(|e| {
					if matches!(e, Add(..) | Poly(..)) {
						format!("({e})")
					} else {
						format!("{e}")
					}
				})
				.join("×"),
			Neg(expr) => match &**expr {
				Add(_) | Mul(_) | Poly(..) => format!("-({expr})"),
				_ => format!("-{expr}"),
			},
			Pow(base, exp) if exp.is_neg_one() => match &**base {
				Add(_) | Mul(_) | Poly(..) => format!("({base})^-1"),
				_ => format!("{base}^-1"),
			},
			Number(rational) => format!("{rational}"),
			Var(symbol) => format!("{symbol}"),
			Fun(symbol, expr) => format!("{symbol}({expr})"),
			Poly(symbol, polynomial) => format!("{polynomial}").replace("x", &symbol.0),
			Pow(base, exp) => match (needs_parens(base), needs_parens(exp)) {
				(false, false) => format!("{base}^{exp}"),
				(true, true) => format!("(){base})^({exp})"),
				(true, false) => format!("({base})^{exp}"),
				(false, true) => format!("{base}^({exp})"),
			},
		};

		write!(f, "{st}")?;

		Ok(())
	}
}
