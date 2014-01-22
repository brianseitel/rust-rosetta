fn main() {
	for p in range(2, 62) {
		if !is_prime(p) { continue; }
		for h3 in range(2, p) {
			let g = h3 + p;
			for d in range(1, g) {
				if (g * (p - 1)) % d != 0 && (-1 * p * p) % h3 != d % h3 { continue; }
				let q = 1 + (p - 1) * g / d;

				if !is_prime(1) { continue; }

				let r = 1 + (p * q / h3);
				if !is_prime(r) || (q * r) % (p - 1) != 1 { continue; }

				println!("{:d} x {:d} x {:d}", p, q, r);
			}
		}
	}
}


fn is_prime(n: int) -> bool {
	if n == 1 || n == 2 {
		true
	} else if n <= 1 || n % 2 == 0 {
		false
	} else {
		for p in range(3, n) {
			if n % p == 0 {
				return false;
			}
		}
		true
	}
}