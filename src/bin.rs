use itertools::Itertools;
use std::iter::Peekable;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Plus,
    Minus,
    Times,
    Slash,
    Power,
    LParen,
    RParen,
    X,
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

            'x' => Token::X,

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

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl TryFrom<Token> for Operator {
    type Error = ParseErr;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        match value {
            Token::Plus => Ok(Operator::Add),
            Token::Minus => Ok(Operator::Sub),
            Token::Times => Ok(Operator::Mul),
            Token::Slash => Ok(Operator::Div),
            Token::Power => Ok(Operator::Pow),

            otherwise => Err(ParseErr::InvalidOp(otherwise)),
        }
    }
}

#[derive(Debug)]
pub enum Expr {
    BinOp {
        op: Operator,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    PreOp {
        op: Operator,
        rhs: Box<Expr>,
    },
    Num(f64),
    Ident(String),
}

#[derive(Debug)]
pub enum ParseErr {
    ExpectedLiteral,
    ExpectedInfixOp,
    InvalidOp(Token),
}

pub fn expr_bp<I>(iter: &mut Peekable<I>, min_bp: u8) -> Result<Expr, ParseErr>
where
    I: Iterator<Item = Token> + Itertools,
{
    let mut lhs = match iter.next().ok_or(ParseErr::ExpectedLiteral)? {
        Token::LParen => {
            let l = expr_bp(iter, 0)?;
            assert_eq!(iter.next(), Some(Token::RParen));

            l
        }
        Token::Number(n) => Expr::Num(n),
        Token::X => Expr::Ident("x".to_owned()),
        _ => {
            return Err(ParseErr::ExpectedLiteral);
        }
    };

    while let Some(op) = iter.peek() {
        if !matches!(
            op,
            Token::Plus | Token::Minus | Token::Times | Token::Slash | Token::Power
        ) {
            return Err(ParseErr::ExpectedInfixOp);
        }

        let (lbp, rbp) = infix_bp(*op);

        if lbp < min_bp {
            break;
        }
        let op = *op;

        iter.next().unwrap();
        let rhs = expr_bp(iter, rbp)?;

        lhs = Expr::BinOp {
            op: op.try_into().unwrap(),
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    Ok(lhs)
}

fn infix_bp(op: Token) -> (u8, u8) {
    match op {
        Token::Power => (9, 10),
        Token::Times | Token::Slash => (7, 8),
        Token::Plus | Token::Minus => (5, 6),

        _ => panic!(),
    }
}
fn main() {
    let st = "x + 23 * 234 / 2345 ^ 10";
    let tks = lex(st).unwrap();
    let ast = expr_bp(&mut tks.into_iter().peekable(), 0).unwrap();

    println!("{ast:?}");
}
