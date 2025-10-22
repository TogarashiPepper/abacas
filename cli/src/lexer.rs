use crate::token::Token;

pub struct Lexer<'a> {
	contents: &'a str,
}

impl<'a> Lexer<'a> {
	pub fn new(contents: &'a str) -> Self {
		Self { contents }
	}

	pub fn tokens(&self) -> Vec<Vec<Token>> {
		let mut tokens = vec![];

		for line in self.contents.lines() {
			if line.starts_with("//") || line.is_empty() {
				continue;
			}

			let token = self.tokenize_line(line);

			tokens.push(token);
		}

		tokens
	}

	fn tokenize_line(&self, line: &str) -> Vec<Token> {
		let mut line = line.chars().peekable();
		let mut tokens = vec![];

		loop {
			let mut token = String::new();
			let char = line.next();

			if char.is_none() {
				break;
			}

			let char = char.unwrap();

			if char.is_whitespace() {
				continue;
			}

			if char.is_ascii_alphabetic() {
				token.push(char);

				loop {
					let char = line.peek();

					if char.is_none() || !char.unwrap().is_ascii_alphabetic() {
						break;
					}

					let char = line.next();

					token.push(char.unwrap());
				}

				tokens.push(Token::new(token));
			} else if char.is_ascii_digit() {
				token.push(char);
				let to_insert_mul;

				loop {
					let char = line.peek();

					if char.is_none() || (!char.unwrap().is_ascii_digit() && *char.unwrap() != '.') {
						to_insert_mul = char.is_some() && char.unwrap().is_ascii_alphabetic();
						break;
					}

					let char = line.next();

					token.push(char.unwrap());
				}

				tokens.push(Token::new(token));

				if to_insert_mul {
					tokens.push(Token::Mul);
				}
			} else {
				token.push(char);

				let punctuation = ['.', '(', ')'];

				loop {
					let char = line.peek();

					if char.is_none()
						|| char.unwrap().is_ascii_alphanumeric()
						|| char.unwrap().is_whitespace()
						|| punctuation.contains(char.unwrap())
						|| punctuation.map(|f| token.contains(f)).contains(&true)
					{
						break;
					}

					let char = line.next();

					token.push(char.unwrap());
				}

				tokens.push(Token::new(token));
			}
		}

		tokens
	}
}
