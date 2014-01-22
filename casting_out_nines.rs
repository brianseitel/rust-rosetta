fn main() {
	let base = 10;
	co9(base);
}

fn co9(base: int) {
	let mut c1 = 0;
	let mut c2 = 0;
	for k in range(1, std::num::pow(base, 2)) {
		c1 += 1;
		if k % (base - 1) == (k*k) % (base - 1) {
			c2 += 1;
			print!("{:d} ", k);
		}
	}

	println!("Trying {:d} numbers instead of {:d} numbers saves {:f}%.", c2, c1, 100. - (c2 as f32 / c1 as f32) * 100. as f32);
}