use std::sync::atomic::{AtomicU64, Ordering};

use abacas::monomial::Monomial;
use abacas::polynomial::Polynomial;
use divan::Bencher;
use fastrand::Rng;

fn main() {
	divan::main();
}

/// Helper to construct a random polynomial with the given degree.
fn random_poly(degree: usize) -> Polynomial {
	// Initial seed 42 chosen at random
	static SEED: AtomicU64 = AtomicU64::new(42);

	let mut poly = Vec::with_capacity(degree + 1);
	let mut rng = Rng::with_seed(SEED.fetch_add(1, Ordering::Relaxed));

	for degree in 0..=degree {
		let numer = rng.u16(1..);
		let denom = rng.u16(1..);

		poly.push(Monomial::new((numer, denom), degree));
	}

	Polynomial::new(poly)
}

#[divan::bench]
fn add(bencher: Bencher) {
	bencher
		.with_inputs(|| (random_poly(100), random_poly(50)))
		.bench_values(|(a, b)| a + b);
}

#[divan::bench]
fn div(bencher: Bencher) {
	bencher
		.with_inputs(|| (random_poly(100), random_poly(50)))
		.bench_values(|(a, b)| a / b);
}

#[divan::bench]
fn gcd(bencher: Bencher) {
	bencher
		.with_inputs(|| (random_poly(20), random_poly(10)))
		.bench_values(|(a, b)| a.gcd(b));
}

#[divan::bench]
fn gcd_ext(bencher: Bencher) {
	bencher
		.with_inputs(|| (random_poly(20), random_poly(10)))
		.bench_values(|(a, b)| a.gcd_ext(b));
}

#[divan::bench]
fn mul(bencher: Bencher) {
	bencher
		.with_inputs(|| (random_poly(100), random_poly(50)))
		.bench_values(|(a, b)| a * b);
}

#[divan::bench]
fn sub(bencher: Bencher) {
	bencher
		.with_inputs(|| (random_poly(100), random_poly(50)))
		.bench_values(|(a, b)| a - b);
}
