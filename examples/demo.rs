use abacas::structs::Polynomial;

const GREEN: &str = "\x1b[38;5;10m";
const RED: &str = "\x1b[38;5;1m";
const PURPLE: &str = "\x1b[38;5;33m";
const RESET: &str = "\x1b[0m";

fn main() {
	let a = "x^3 + 2x^2 + 3x + 4".parse::<Polynomial>().unwrap();
	let b = "9x^4 + 3x^3 + 2x + 1".parse::<Polynomial>().unwrap();

	let fa = format!("{GREEN}a{RESET}");
	let fb = format!("{RED}b{RESET}");

	println!("let {fa} = {GREEN}{a}{RESET};");
	println!("let {fb} = {RED}{b}{RESET};");
	println!("{fa} + {fb} → {PURPLE}{}{RESET}", a.clone() + b.clone());
	println!("{fa} - {fb} → {PURPLE}{}{RESET}", a.clone() - b.clone());
	println!("{fa} * {fb} → {PURPLE}{}{RESET}", a.clone() * b.clone());
}
