use abacas::polynomial::Polynomial;

const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

fn main() {
	let a: Polynomial = "x^3 + 2x^2 + 3x + 4".parse().unwrap();
	let b: Polynomial = "9x^4 + 3x^3 + 2x + 1".parse().unwrap();

	let fa = format_args!("{GREEN}a{RESET}");
	let fb = format_args!("{RED}b{RESET}");

	println!("let {fa} = {GREEN}{a}{RESET}");
	println!("let {fb} = {RED}{b}{RESET}");

	println!("\n{fa} + {fb} = {}", a.clone() + b.clone());
	println!("{fa} - {fb} = {}", a.clone() - b.clone());
	println!("{fa} * {fb} = {}", a.clone() * b.clone());
}
