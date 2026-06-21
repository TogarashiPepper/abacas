//! The Standard Library of abacas
use rug::ops::Pow;

use crate::context::Context;
use crate::expr::Expr;
use crate::number::Number;

//TODO: Add proper error mechanism
//TODO: Add `abs` method

pub fn identity(args: Vec<Expr>) -> Option<Expr> {
	if args.len() != 1 {
		panic!("expected one argument")
	}
	args.first().cloned()
}

pub fn round(args: Vec<Expr>, ctx: &mut Context) -> Expr {
	if args.len() != 1 {
		panic!("expected one argument")
	}

	let Expr::Number(n) = args.into_iter().next().unwrap().simplify(ctx) else {
		unimplemented!()
	};

	Expr::Number(n.round())
}

pub fn ceil(args: Vec<Expr>, ctx: &mut Context) -> Expr {
	if args.len() != 1 {
		panic!("expected one argument")
	}

	let Expr::Number(n) = args.into_iter().next().unwrap().simplify(ctx) else {
		unimplemented!()
	};

	Expr::Number(n.ceil())
}

pub fn floor(args: Vec<Expr>, ctx: &mut Context) -> Expr {
	if args.len() != 1 {
		panic!("expected one argument")
	}

	let Expr::Number(n) = args.into_iter().next().unwrap().simplify(ctx) else {
		unimplemented!()
	};

	Expr::Number(n.floor())
}

pub fn exp(args: Vec<Expr>, ctx: &mut Context) -> Expr {
	if args.len() != 1 {
		panic!("expected one argument")
	}

	let Expr::Number(n) = args.into_iter().next().unwrap().simplify(ctx) else {
		unimplemented!()
	};

	Expr::Number(Number::e().pow(&n))
}

pub fn ln(args: Vec<Expr>, ctx: &mut Context) -> Expr {
	if args.len() != 1 {
		panic!("expected one argument")
	}

	let Expr::Number(n) = args.into_iter().next().unwrap().simplify(ctx) else {
		unimplemented!()
	};

	// Expr::Number(n.ln())
	todo!()
}

// pub fn log10(args: Vec<Expr>) -> Expr {
// 	match a {
// 		Data::Number(..) => div(&ln(a), &ln(&Data::new_real(Decimal::TEN))),
// 		_ => unimplemented!(),
// 	}
// }

// pub fn log(args: Vec<Expr>, b: &Data) -> Expr {
// 	match a {
// 		Data::Number(..) => div(&ln(a), &ln(b)),
// 		_ => unimplemented!(),
// 	}
// }

// pub fn sin(args: Vec<Expr>) -> Expr {
// 	let x = a.to_real();
// 	let y = a.to_img();

// 	if y == Decimal::ZERO {
// 		return Data::new_real((dec!(3.0) * Decimal::PI).sin());
// 	}

// 	let p = &mul(&Data::new_real(x.sin()), &cosh(&Data::new_real(y)));
// 	let q = &mul(&Data::new_real(x.cos()), &sinh(&Data::new_real(y)));

// 	Data::Number(p.to_real(), q.to_real())
// }

// pub fn sinh(args: Vec<Expr>) -> Expr {
// 	div(
// 		&sub(&exp(a), &exp(&mul(a, &Data::new_real(Decimal::NEGATIVE_ONE)))),
// 		&Data::new_real(Decimal::TWO),
// 	)
// }

// pub fn cos(args: Vec<Expr>) -> Expr {
// 	let x = a.to_real();
// 	let y = a.to_img();

// 	if y == Decimal::ZERO {
// 		return Data::new_real(x.cos());
// 	}

// 	let p = &mul(&Data::new_real(x.cos()), &cosh(&Data::new_real(y)));
// 	let q = &mul(&Data::new_real(-x.sin()), &sinh(&Data::new_real(y)));

// 	Data::Number(p.to_real(), q.to_real())
// }

// pub fn cosh(args: Vec<Expr>) -> Expr {
// 	div(
// 		&add(&exp(a), &exp(&mul(a, &Data::new_real(Decimal::NEGATIVE_ONE)))),
// 		&Data::new_real(Decimal::TWO),
// 	)
// }

// pub fn tan(args: Vec<Expr>) -> Expr {
// 	div(&sin(a), &cos(a))
// }

// pub fn sqrt(args: Vec<Expr>) -> Expr {
// 	let b = a.to_img();
// 	let a = a.to_real();

// 	let r = (a * a + b * b).sqrt().unwrap();

// 	let zr = ((a + r) * (a + r) + b * b).sqrt().unwrap();

// 	Data::Number(r.sqrt().unwrap() * (a + r) / zr, r.sqrt().unwrap() * b / zr)
// }

// pub fn nrt(args: Vec<Expr>, b: &Data) -> Expr {
// 	if let Data::Number(x, y) = a
// 		&& let Data::Number(r, _) = abs(a)
// 		&& let Data::Number(b, _) = b
// 	{
// 		let z = r.powd(Decimal::ONE / b);

// 		let Data::Number(theta, _) = atan2(&Data::Number(*x, Decimal::ZERO), &Data::Number(*y, Decimal::ZERO)) else {
// 			unreachable!()
// 		};

// 		let theta = theta / b;

// 		return Data::Number(z * theta.cos(), z * theta.sin());
// 	}

// 	unimplemented!()
// }
