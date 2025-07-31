use lexer::lex;

mod lexer;

fn main() {
    let st = "1 + 23 * 234 / 2345 ^ 10";
    let tks = lex(st).unwrap();

    println!("{tks:?}")
}
