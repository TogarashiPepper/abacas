use std::str::FromStr;

use abacas::number::Number;
use logos::Logos;
use rug::{Integer, Rational};

#[derive(Logos, Debug, Clone, PartialEq, PartialOrd)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
	#[regex(r"-?\d+(\.\d+)?", |lex| parse_num(lex.slice()))]
	Number(Number),
	#[regex(r"[a-zA-Z]+", |lex| lex.slice().to_owned())]
	Ident(String),

	#[token("=")]
	Eq,

	#[token("+")]
	Add,
	#[token("-")]
	Sub,
	#[token("*")]
	Mul,
	#[token("/")]
	Div,
	#[token("^")]
	Pow,
	#[token("%")]
	Rem,

	#[token("(")]
	LParen,
	#[token(")")]
	RParen,
}

fn parse_num(st: &str) -> Number {
	let (dec, int) = st.split_once(".").unwrap_or((st, ""));

	let formatted = if !int.is_empty() {
		format!("{} / 1{}", dec.to_owned() + int, "0".repeat(int.len()))
	} else {
		dec.to_owned()
	};

	if let Ok(integer) = Integer::from_str(&formatted) {
		Number::from(integer)
	} else if let Ok(rational) = Rational::from_str(&formatted) {
		Number::from(rational)
	} else {
		unreachable!()
	}
}
