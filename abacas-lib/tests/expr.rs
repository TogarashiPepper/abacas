use abacas::expr::{Expr, Symbol};
use abacas::number::Number;

const ADD: fn(Vec<Expr>) -> Expr = |exprs| Expr::Add(exprs).simplify().unwrap();
const MUL: fn(Vec<Expr>) -> Expr = |exprs| Expr::Mul(exprs).simplify().unwrap();

const FUN: fn(&str, Vec<Expr>) -> Expr = |name, args| Expr::Fun(Symbol::new(name.into()).unwrap(), args);
const NUM: fn(Number) -> Expr = |num| Expr::Num(num);

const X: fn(&str) -> Expr = |poly| Expr::Poly(Symbol::new("x".into()).unwrap(), poly.parse().unwrap());
const Y: fn(&str) -> Expr = |poly| Expr::Poly(Symbol::new("y".into()).unwrap(), poly.parse().unwrap());

#[test]
fn add() {
	let expr = ADD(vec![]);
	assert_eq!(expr.to_string(), "0");

	let expr = ADD(vec![NUM(2.into()), X("x"), X("x + 2"), Y("x"), Y("-x + 2")]);
	assert_eq!(expr.to_string(), "2x + 6");

	let expr = ADD(vec![NUM(2.into()), X("x"), X("x + 2"), Y("x"), Y("-2x + 1")]);
	assert_eq!(expr.to_string(), "5 + 2x - y");

	let expr = ADD(vec![NUM(2.into()), FUN("cos", vec![NUM(0.into())]), NUM((-3).into())]);
	assert_eq!(expr.to_string(), "cos(0) - 1");
}

#[test]
fn mul() {
	let expr = MUL(vec![]);
	assert_eq!(expr.to_string(), "1");

	let expr = MUL(vec![NUM(2.into()), X("x"), X("x + 2"), Y("x"), Y("2x^-1")]);
	assert_eq!(expr.to_string(), "4x^2 + 8x");

	let expr = MUL(vec![NUM(2.into()), X("x"), X("x + 2"), Y("x"), Y("x^-1 + 2")]);
	assert_eq!(expr.to_string(), "4 * (x^2 + 2x) * (y + 0.5)");

	let expr = MUL(vec![NUM(2.into()), FUN("cos", vec![NUM(0.into())]), NUM((-3).into())]);
	assert_eq!(expr.to_string(), "cos(0) * -6");
}
