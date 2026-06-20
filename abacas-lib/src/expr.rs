//! The expression structure and its related items.

use std::cmp::Ordering;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

use itertools::Itertools;
use rug::ops::Pow;

use crate::number::Number;
use crate::polynomial::Polynomial;

/// Represents a symbol like `x` or `pi`.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Symbol(String);

impl Symbol {
	/// Gets the name of this symbol.
	pub fn name(&self) -> &str {
		&self.0
	}

	/// Creates a new symbol with the given name, if it matches the symbol criteria.
	pub fn new(name: String) -> Option<Self> {
		if name.is_empty() || name.chars().any(char::is_whitespace) {
			None
		} else {
			Some(Self(name))
		}
	}
}

impl fmt::Display for Symbol {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.0.fmt(f)
	}
}

/// Represents a general expression.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Expr {
	/// Represents the sum of multiple expressions.
	Add(Vec<Self>),
	/// Represents a function call.
	Fun(Symbol, Vec<Self>),
	/// Represents the product of multiple expressions.
	Mul(Vec<Self>),
	/// Represents a constant number.
	Num(Number),
	/// Represents a polynomial.
	Poly(Symbol, Polynomial),
	/// Represents the power of two expressions.
	Pow(Box<Self>, Box<Self>),
	/// Represents a variable number.
	Var(Symbol),
}

// Constants
impl Expr {
	/// The number negative one (`-1`).
	pub fn neg_one() -> Self {
		Self::Num(Number::neg_one())
	}

	/// The number one (`1`).
	pub fn one() -> Self {
		Self::Num(Number::one())
	}

	/// The number zero (`0`).
	pub fn zero() -> Self {
		Self::Num(Number::zero())
	}
}

// Guards
impl Expr {
	/// Whether this is the number negative one (`-1`).
	pub fn is_neg_one(&self) -> bool {
		matches!(self, Self::Num(num) if num.is_neg_one())
	}

	/// Whether this is a constant number.
	pub const fn is_num(&self) -> bool {
		matches!(self, Self::Num(_))
	}

	/// Whether this is the number one (`1`).
	pub fn is_one(&self) -> bool {
		matches!(self, Self::Num(num) if num.is_one())
	}

	/// Whether this is a polynomial.
	pub const fn is_poly(&self) -> bool {
		matches!(self, Self::Poly(_, _))
	}

	/// Whether this is the number zero (`0`).
	pub const fn is_zero(&self) -> bool {
		matches!(self, Self::Num(num) if num.is_zero())
	}
}

// Operations
impl Expr {
	/// Returns the inner value if this expression is [`Self::Num`], otherwise returns [`None`].
	pub fn into_num(self) -> Option<Number> {
		match self {
			Self::Num(num) => Some(num),
			_ => None,
		}
	}

	/// Returns the inner value if this expression is [`Self::Poly`], otherwise returns [`None`].
	pub fn into_poly(self) -> Option<(Symbol, Polynomial)> {
		match self {
			Self::Poly(sym, poly) => Some((sym, poly)),
			_ => None,
		}
	}

	/// Simplifies an expression on a best-effort basis.
	pub fn simplify(self) -> Self {
		match self {
			Self::Add(exprs) => Self::simplify_add(exprs),
			Self::Fun(sym, args) => Self::simplify_fun(sym, args),
			Self::Mul(exprs) => Self::simplify_mul(exprs),
			Self::Num(_) => self,
			Self::Poly(sym, poly) => Self::simplify_poly(sym, poly),
			Self::Pow(base, exp) => Self::simplify_pow(*base, *exp),
			Self::Var(_) => self,
		}
	}

	/// Simplifies a [`Self::Add`] expression.
	fn simplify_add(mut exprs: Vec<Self>) -> Self {
		// Simplify all elements individually and flatten inner sums
		exprs = exprs
			.into_iter()
			.flat_map(|expr| match expr.simplify() {
				Self::Add(exprs) => exprs,
				expr => vec![expr],
			})
			.collect();

		// Add all polynomials into one per symbol
		let mut polys = exprs
			.extract_if(.., |expr| expr.is_poly())
			.map(|expr| expr.into_poly().unwrap())
			.into_grouping_map()
			.reduce(|lhs, _, rhs| lhs + rhs);

		// Extract constant polynomials from the map
		let constants = polys
			.extract_if(|_, poly| poly.is_constant())
			.map(|(_, poly)| poly.into_constant().unwrap());

		// Add all numbers into one
		let num = exprs
			.extract_if(.., |expr| expr.is_num())
			.map(|expr| expr.into_num().unwrap())
			.chain(constants)
			.reduce(|lhs, rhs| lhs + &rhs);

		// For every other expression, count how often it appears
		let counts = exprs.into_iter().counts();

		// Convert into vec of muls and add extracted number and polynomials
		let mut result: Vec<_> = counts
			.into_iter()
			.map(|(expr, count)| expr * Self::Num(count.into()))
			.chain(num.filter(|num| !num.is_zero()).map(Self::Num))
			.chain(polys.into_iter().map(|(symbol, poly)| Self::Poly(symbol, poly)))
			.collect();

		// Sort the resulting array
		result.sort_by(Self::cmp);

		// Return the result as a new sum
		Self::Add(result)
	}

	/// Simplifies a [`Self::Fun`] expression.
	fn simplify_fun(sym: Symbol, mut args: Vec<Self>) -> Self {
		// Simplify the inner arguments
		args = args.into_iter().map(Self::simplify).collect();

		// Return the result as a new function call
		Self::Fun(sym, args)
	}

	/// Simplifies a [`Self::Mul`] expression.
	fn simplify_mul(mut exprs: Vec<Self>) -> Self {
		// Simplify all elements individually and flatten inner products
		exprs = exprs
			.into_iter()
			.flat_map(|expr| match expr.simplify() {
				Self::Mul(exprs) => exprs,
				expr => vec![expr],
			})
			.collect();

		// Multiply all polynomials into one per symbol
		let mut polys = exprs
			.extract_if(.., |expr| expr.is_poly())
			.map(|expr| expr.into_poly().unwrap())
			.into_grouping_map()
			.reduce(|lhs, _, rhs| lhs * &rhs);

		// Extract constant polynomials from the map
		let constants = polys
			.extract_if(|_, poly| poly.is_constant())
			.map(|(_, poly)| poly.into_constant().unwrap());

		// Multiply all numbers into one
		let num = exprs
			.extract_if(.., |expr| expr.is_num())
			.map(|expr| expr.into_num().unwrap())
			.chain(constants)
			.reduce(|lhs, rhs| lhs * &rhs);

		// If the number is zero, the product will be zero
		if num.as_ref().is_some_and(Number::is_zero) {
			return Self::zero();
		}

		// For every other expression, count how often it appears
		let counts = exprs.into_iter().counts();

		// Convert into vec of pows and multiply extracted number and polynomials
		let mut result: Vec<_> = counts
			.into_iter()
			.map(|(expr, count)| expr.pow(Self::Num(count.into())))
			.chain(num.filter(|num| !num.is_one()).map(Self::Num))
			.chain(polys.into_iter().map(|(symbol, poly)| Self::Poly(symbol, poly)))
			.collect();

		// Sort the resulting array
		result.sort_by(Self::cmp);

		// Return the result as a new product
		Self::Mul(result)
	}

	/// Simplifies a [`Self::Poly`] expression.
	fn simplify_poly(sym: Symbol, poly: Polynomial) -> Self {
		// If the polynomial is constant, return it as a number
		if poly.is_constant() {
			return Self::Num(poly.into_constant().unwrap());
		}

		// Return the result as a new polynomial
		Self::Poly(sym, poly)
	}

	/// Simplifies a [`Self::Pow`] expression.
	fn simplify_pow(mut base: Self, mut exp: Self) -> Self {
		// First simplify the base and exponent separately
		base = base.simplify();
		exp = exp.simplify();

		// If exponent is zero, return one
		if exp.is_zero() {
			return Self::one();
		}

		// If exponent is one, return the base
		if exp.is_one() {
			return base;
		}

		// If base is another pow, multiply the exponents
		if let Self::Pow(base_base, base_exp) = base {
			return Self::Pow(base_base, Self::Mul(vec![*base_exp, exp]).into()).simplify();
		}

		// Return the result as a new power
		Self::Pow(base.into(), exp.into())
	}

	/// Compares two expressions for a consistent ordering.
	fn cmp(&self, other: &Self) -> Ordering {
		match (self, other) {
			// If both are sums, compare the vecs
			(Self::Add(lhs), Self::Add(rhs)) => Self::cmp_vecs(lhs, rhs),

			// If both are function calls, compare symbol first, then arguments
			(Self::Fun(lhs_sym, lhs_args), Self::Fun(rhs_sym, rhs_args)) => {
				lhs_sym.cmp(rhs_sym).then_with(|| Self::cmp_vecs(lhs_args, rhs_args))
			}

			// If both are products, compare the vecs
			(Self::Mul(lhs), Self::Mul(rhs)) => Self::cmp_vecs(lhs, rhs),

			// If both are numbers, compare them directly
			(Self::Num(lhs), Self::Num(rhs)) => lhs.cmp(rhs),

			// If both are polynomials, compare symbol first, then monomials
			(Self::Poly(lhs_sym, lhs_poly), Self::Poly(rhs_sym, rhs_poly)) => {
				lhs_sym.cmp(rhs_sym).then_with(|| Self::cmp_polys(lhs_poly, rhs_poly))
			}

			// If both are powers, compare base first, then exponent
			(Self::Pow(lhs_base, lhs_exp), Self::Pow(rhs_base, rhs_exp)) => {
				lhs_base.cmp(rhs_base).then_with(|| lhs_exp.cmp(rhs_exp))
			}

			// If both are variables, compare them directly
			(Self::Var(lhs), Self::Var(rhs)) => lhs.cmp(rhs),

			// Otherwise, compare the discriminants
			(Self::Add(_), _) => Ordering::Less,
			(_, Self::Add(_)) => Ordering::Greater,
			(Self::Fun(_, _), _) => Ordering::Less,
			(_, Self::Fun(_, _)) => Ordering::Greater,
			(Self::Mul(_), _) => Ordering::Less,
			(_, Self::Mul(_)) => Ordering::Greater,
			(Self::Num(_), _) => Ordering::Less,
			(_, Self::Num(_)) => Ordering::Greater,
			(Self::Poly(_, _), _) => Ordering::Less,
			(_, Self::Poly(_, _)) => Ordering::Greater,
			(Self::Pow(_, _), _) => Ordering::Less,
			(_, Self::Pow(_, _)) => Ordering::Greater,
		}
	}

	/// Compares two polynomials for a consistent ordering.
	fn cmp_polys(lhs: &Polynomial, rhs: &Polynomial) -> Ordering {
		lhs.monomials()
			.zip(rhs.monomials())
			.map(|(lhs, rhs)| lhs.coeff.cmp(&rhs.coeff).then_with(|| lhs.degree.cmp(&rhs.degree)))
			.find(|ord| ord.is_ne())
			.unwrap_or_else(|| lhs.monomials().len().cmp(&rhs.monomials().len()))
	}

	/// Compares two expression vecs for a consistent ordering.
	fn cmp_vecs(lhs: &[Self], rhs: &[Self]) -> Ordering {
		lhs.iter()
			.zip(rhs)
			.map(|(lhs, rhs)| lhs.cmp(rhs))
			.find(|ord| ord.is_ne())
			.unwrap_or_else(|| lhs.len().cmp(&rhs.len()))
	}

	/// Formats this expression with parentheses if necessary.
	fn with_parens(&self) -> impl fmt::Display {
		struct WithParens<'a>(&'a Expr);

		impl fmt::Display for WithParens<'_> {
			fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
				match self.0 {
					// If the expr has more than one term, use parentheses
					Expr::Add(exprs) if exprs.len() > 1 => write!(f, "({})", self.0),
					Expr::Mul(exprs) if exprs.len() > 1 => write!(f, "({})", self.0),
					Expr::Poly(_, poly) if poly.monomials().len() > 1 => write!(f, "({})", self.0),

					// No wildcard so new variants do not get silenced
					Expr::Add(_) => write!(f, "{}", self.0),
					Expr::Fun(_, _) => write!(f, "{}", self.0),
					Expr::Mul(_) => write!(f, "{}", self.0),
					Expr::Num(_) => write!(f, "{}", self.0),
					Expr::Poly(_, _) => write!(f, "{}", self.0),
					Expr::Pow(_, _) => write!(f, "{}", self.0),
					Expr::Var(_) => write!(f, "{}", self.0),
				}
			}
		}

		WithParens(self)
	}
}

impl Add<Self> for Expr {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		Self::Add(vec![self, rhs]).simplify()
	}
}

impl Div<Self> for Expr {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		Self::Mul(vec![self, Self::Pow(rhs.into(), Self::neg_one().into())]).simplify()
	}
}

impl Mul<Self> for Expr {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		Self::Mul(vec![self, rhs]).simplify()
	}
}

impl Neg for Expr {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Self::Mul(vec![self, Self::neg_one()]).simplify()
	}
}

impl Pow<Self> for Expr {
	type Output = Self;

	fn pow(self, rhs: Self) -> Self::Output {
		Self::Pow(self.into(), rhs.into()).simplify()
	}
}

impl Sub<Self> for Expr {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		Self::Add(vec![self, Self::Mul(vec![rhs, Self::neg_one()])]).simplify()
	}
}

impl fmt::Display for Expr {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Add(exprs) => write!(f, "{}", exprs.iter().format(" + ")),
			Self::Fun(sym, args) => write!(f, "{sym}({})", args.iter().format(", ")),
			Self::Mul(exprs) => write!(f, "{}", exprs.iter().map(Self::with_parens).format(" * ")),
			Self::Num(num) => write!(f, "{num}"),
			Self::Poly(sym, poly) => write!(f, "{}", poly.to_string().replace('x', sym.name())),
			Self::Pow(base, exp) => write!(f, "{}^{}", base.with_parens(), exp.with_parens()),
			Self::Var(sym) => write!(f, "{sym}"),
		}
	}
}
