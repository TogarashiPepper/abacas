use itertools::Itertools;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Plus,
    Minus,
    Times,
    Slash,
    Power,
    LParen,
    RParen,
    Number(f64),
}

#[derive(Debug)]
pub enum LexError {
    NoDigitAfterDot,
    InvalidChar(char),
}

pub fn lex(input: &str) -> Result<Vec<Token>, LexError> {
    let mut it = input.char_indices();
    let mut tokens = vec![];

    while let Some((i, c)) = it.next() {
        let tok = match c {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Times,
            '/' => Token::Slash,
            '^' => Token::Power,

            '(' => Token::LParen,
            ')' => Token::RParen,

            '.' => {
                let (i2, _) = it
                    .peeking_take_while(|c| c.1.is_ascii_digit())
                    .last()
                    .ok_or(LexError::NoDigitAfterDot)?;

                Token::Number(input[i..=i2].parse::<f64>().unwrap())
            }
            '0'..='9' => {
                let mut seen_dot = false;
                let last = it
                    .peeking_take_while(|c| {
                        let cnd = c.1.is_ascii_digit() || (c.1 == '.' && !seen_dot);
                        if c.1 == '.' && !seen_dot {
                            seen_dot = true;
                        }
                        cnd
                    })
                    .last();

                if let Some((i2, _)) = last {
                    Token::Number(input[i..=i2].parse().unwrap())
                } else {
                    Token::Number(f64::from(c as u8 - b'0'))
                }
            }

            ' ' | '\t' | '\n' => continue,

            otherwise => {
                return Err(LexError::InvalidChar(otherwise));
            }
        };

        tokens.push(tok);
    }

    Ok(tokens)
}
