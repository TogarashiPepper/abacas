use abacas::number::Number;
use rug::Rational;

#[test]
fn eq() {
	let int = Number::Integer(1.into());
	let nat = Number::Natural(1.into());
	let rat = Number::Rational(1.into());

	assert_eq!(int, nat);
	assert_eq!(int, rat);
	assert_eq!(nat, rat);

	assert_ne!(int + 1, nat);
	assert_ne!(nat + 1, rat);
}

#[test]
fn from() {
	let int = Number::from(Rational::from(0));
	let nat = Number::from(Rational::from(2));
	let rat = Number::from(Rational::from((5, 2)));

	assert!(matches!(int, Number::Integer(_)));
	assert!(matches!(nat, Number::Natural(_)));
	assert!(matches!(rat, Number::Rational(_)));
}

#[test]
fn from_str() {
	let int = "0".parse();
	let nat = "2".parse();
	let rat = "5/2".parse();

	assert!(matches!(int, Ok(Number::Integer(_))));
	assert!(matches!(nat, Ok(Number::Natural(_))));
	assert!(matches!(rat, Ok(Number::Rational(_))));
}

#[test]
fn ord() {
	let int = Number::Integer(0.into());
	let nat = Number::Natural(2.into());
	let rat = Number::Rational((5, 2).into());

	assert!(int < nat);
	assert!(int < rat);
	assert!(nat < rat);
}
