use std::iter::Peekable;

use itertools::Itertools;

use crate::lexer::Token;

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
