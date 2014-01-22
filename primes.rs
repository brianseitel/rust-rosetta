fn main() {
	for i in range(0, 100) {
		println!("{:d} => {:?}", i, is_prime(i));
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