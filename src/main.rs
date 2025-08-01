mod lexer;
mod parser;

use lexer::lex;
use parser::expr_bp;

fn main() {
    let st = "1 + 23 * 234 / 2345 ^ 10";
    let tks = lex(st).unwrap();
    let ast = expr_bp(&mut tks.into_iter().peekable(), 0).unwrap();

    println!("{ast:#?}");
}
