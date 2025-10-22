use std::io;
use std::io::{Result, Write};

use abacas::Polynomial;

const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

fn main() -> Result<()> {
	let stdin = io::stdin();

	let mut buffer = String::new();
	let mut stdout = io::stdout();

	print!("Enter {GREEN}dividend{RESET}: ");

	stdout.flush()?;
	stdin.read_line(&mut buffer)?;

	let Ok(dividend) = buffer.parse::<Polynomial>() else {
		eprintln!("Could not parse dividend!");
		return Ok(());
	};

	buffer.clear();

	print!("Enter {RED}divisor{RESET}: ");

	stdout.flush()?;
	stdin.read_line(&mut buffer)?;

	let Ok(divisor) = buffer.parse::<Polynomial>() else {
		eprintln!("Could not parse divisor!");
		return Ok(());
	};

	let Some((quotient, remainder)) = dividend.clone().div_rem(&divisor) else {
		eprintln!("Cannot divide by zero!");
		return Ok(());
	};

	let fa = format_args!("{GREEN}{dividend}{RESET}");
	let fb = format_args!("{RED}{divisor}{RESET}");

	println!("\n({fa}) / ({fb}) = {quotient}");
	println!("({fa}) % ({fb}) = {remainder}");

	Ok(())
}
