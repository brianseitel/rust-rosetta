use std::rand::{task_rng, Rng};
use std::num::Real;
use std::num;

fn main() {
	let runs = 1_000;

	for n in range(1, 20) {
		let mut sum_of_runs = 0;
		for i in range(0, runs) {
			sum_of_runs += rand_until_repetition(n as uint);
		}
		let avg = (sum_of_runs as f32) / (runs as f32);
		let mut analytical = 0.0;
		for i in range(0, n) {
			analytical += (factorial(n) as f32 / (num::pow(n, i) as f32) / factorial(n - i) as f32) as f32;
		}

		let diff: f32 = ((avg as f32 / (analytical as f32) - 1.) * 100. as f32);
		println!("{} {} {} {}\t", n, avg, analytical, diff);
	}
}

fn factorial(n: int) -> int {
	if n == 0 {
		1
	} else {
		(n * factorial(n - 1))
	}
}

fn rand_until_repetition(n: uint) -> uint {
	let mut rands: ~[int] = ~[];

	loop {
		let mut rng = task_rng();
		let rand: int = rng.gen_range(0, n) as int;
		match rands.bsearch(|x| rand.cmp(x)) {
			Some(x) => return rands.len(),
			None => rands.push(rand)
		}
	}
	n
}