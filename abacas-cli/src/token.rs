use std::fmt::Display;
use std::str::FromStr;

use logos::Logos;
use rug::Rational;

#[derive(Logos, Debug, Clone, PartialEq, PartialOrd)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
	#[regex(r"\d+(\.\d+)?", |lex| parse_num(lex.slice()))]
	Number(Rational),
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

impl Display for Token {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Token::Number(number) => write!(f, "{number}"),
			Token::Ident(name) => write!(f, "{name}"),
			Token::Eq => write!(f, "="),
			Token::Add => write!(f, "+"),
			Token::Sub => write!(f, "-"),
			Token::Mul => write!(f, "*"),
			Token::Div => write!(f, "/"),
			Token::Pow => write!(f, "^"),
			Token::Rem => write!(f, "%"),
			Token::LParen => write!(f, "("),
			Token::RParen => write!(f, ")"),
		}
	}
}

fn parse_num(st: &str) -> Rational {
	let (dec, int) = st.split_once(".").unwrap_or((st, ""));

	let formatted = if !int.is_empty() {
		format!("{} / 1{}", dec.to_owned() + int, "0".repeat(int.len()))
	} else {
		dec.to_owned()
	};

	if let Ok(rational) = Rational::from_str(&formatted) {
		rational
	} else {
		unreachable!()
	}
}
