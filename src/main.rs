mod lexer;
mod monomial;
mod parser;
mod polynomial;

use lexer::lex;
use monomial::Monomial;
use parser::expr_bp;

fn main() {
    // let st = "x + 23 * 234 / 2345 ^ 10";
    // let tks = lex(st).unwrap();
    // let ast = expr_bp(&mut tks.into_iter().peekable(), 0).unwrap();

    // println!("{ast:#?}");

    let m = Monomial::new(4.0, 2);
    let r = Monomial::new(5.0, 2);
    let res = m + r + Monomial::new(1.0, 1000);
    let res = res + (Monomial::new(10.0, 10) + Monomial::new(9.0, 9));

    println!("{res:?}"); // prints [x^1000, 10x^10, 9x^9, 9x^2]
}
