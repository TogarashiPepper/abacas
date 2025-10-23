use std::str::FromStr;

use abacas::number::Number;
use rug::{Integer, Rational};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Token {
	Number(Number),
	Ident(String),

	Eq,

	Add,
	Sub,
	Mul,
	Div,
	Pow,
	Rem,

	LParen,
	RParen,
}

impl Token {
	pub fn new(token: String) -> Self {
		match token.as_ref() {
			"=" => Token::Eq,

			"+" => Token::Add,
			"-" => Token::Sub,
			"*" => Token::Mul,
			"/" => Token::Div,
			"^" => Token::Pow,
			"%" => Token::Rem,

			"(" => Token::LParen,
			")" => Token::RParen,
			_ => {
				let split_string = token.split(".").collect::<Vec<&str>>();

				let mut rational_string = String::new();

				if split_string.len() > 1 {
					let decimal_index = split_string.get(1).map_or("", |v| v).len();

					rational_string = format!(
						"{} / 1{}",
						split_string[0].to_owned() + split_string[1],
						"0".repeat(decimal_index)
					);
				}

				let try_rational = Rational::from_str(&rational_string);
				let try_integer = Integer::from_str(&token);

				if try_integer.is_ok() {
					Token::Number(Number::from(try_integer.unwrap()))
				} else if try_rational.is_ok() {
					Token::Number(Number::from(try_rational.unwrap()))
				} else {
					Token::Ident(token)
				}
			}
		}
	}
}
