use std::vec::OwnedEqVector;

fn main() {
	let results = get_factors(12345);
}

fn get_factors(n: uint) -> ~[uint] {
	let mut factors: ~[uint] = ~[];
	factors.push(1); // errthang is a multiple of 1
	for i in range(2, std::num::pow(n, 2)) {
		if n % i == 0 {
			factors.push(i);
			if std::num::pow(i, 2) != n {
				factors.push(n / i);
			}
		}
	}

	factors.sort();
	factors.dedup();
	factors
}

#[test]
fn test_get_factors() {
	let results: ~[uint] = ~[1u, 3u, 5u, 15u, 823u, 2469u, 4115u, 12345u];

	if get_factors(12345) != results {
		fail!("Results do not match!");
	}
}