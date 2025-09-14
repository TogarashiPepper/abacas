mod lexer;
mod monomial;
mod parser;
mod polynomial;

use monomial::Monomial;

fn main() {
    // let st = "x + 23 * 234 / 2345 ^ 10";
    // let tks = lex(st).unwrap();
    // let ast = expr_bp(&mut tks.into_iter().peekable(), 0).unwrap();

    // println!("{ast:#?}");

    let m = Monomial::new(4.0, 2);
    let r = Monomial::new(5.0, 2);
    let res =
        (m + r + Monomial::new(1.0, 1000)) * (Monomial::new(10.0, 3) + Monomial::new(4.0, 21));

    println!("{res}"); // prints 4x^1021 + 10x^1003 + 36x^23 + 90x^5
}
