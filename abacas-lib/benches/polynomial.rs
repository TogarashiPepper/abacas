use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use abacas::monomial::Monomial;
use abacas::polynomial::Polynomial;
use divan::Bencher;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

fn main() {
	divan::main();
}

fn random_poly(degree: usize) -> Polynomial {
    // Initial seed 42 chosen at random
	static SEED: AtomicU64 = AtomicU64::new(42);

	let mut poly = Vec::with_capacity(degree + 1);
	let mut rng = Xoshiro256PlusPlus::seed_from_u64(SEED.fetch_add(1, Ordering::Relaxed));

	for degree in 0..=degree {
		let numer = rng.random_range(1..1000);
		let denom = rng.random_range(1..1000);

		poly.push(Monomial::new((numer, denom), degree));
	}

	Polynomial::new(poly)
}

#[divan::bench]
fn poly_add(bencher: Bencher) {
	bencher
		.with_inputs(|| (random_poly(100), random_poly(100)))
		.bench_local_values(|(a, b)| {
			let _ = a + b;
		});
}

#[divan::bench]
fn poly_sub(bencher: Bencher) {
	bencher
		.with_inputs(|| (random_poly(100), random_poly(100)))
		.bench_local_values(|(a, b)| {
			let _ = a - b;
		});
}

#[divan::bench]
fn poly_mul(bencher: Bencher) {
	bencher
		.with_inputs(|| (random_poly(100), random_poly(100)))
		.bench_local_values(|(a, b)| {
			let _ = a * b;
		});
}

#[divan::bench]
fn poly_div(bencher: Bencher) {
	bencher
		.with_inputs(|| (random_poly(100), random_poly(25)))
		.bench_local_values(|(a, b)| {
			let _ = a / b;
		});
}

#[divan::bench(max_time = Duration::from_secs(15))]
fn poly_gcd(bencher: Bencher) {
	bencher
		.with_inputs(|| (random_poly(20), random_poly(20)))
		.bench_local_values(|(a, b)| {
			let _ = a.gcd(b);
		});
}

#[divan::bench(max_time = Duration::from_secs(15))]
fn poly_gcd_ext(bencher: Bencher) {
	bencher
		.with_inputs(|| (random_poly(20), random_poly(20)))
		.bench_local_values(|(a, b)| {
			let _ = a.gcd_ext(b);
		});
}
