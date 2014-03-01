use std::iter::AdditiveIterator;

#[cfg(not(test))]
fn main() {
	let number = ~"49927398716";

	let result = luhn(number);

	println!("{:?}", result);
}

fn luhn(input: ~str) -> bool {
	let mut odd: ~[int] = ~[];
	let mut even: ~[int] = ~[];
	for (i, c) in input.chars().rev().enumerate() {
		if (i+1) % 2 == 0 {
			even.push(from_str::<int>(c.to_str()).unwrap());
		} else {
			odd.push(from_str::<int>(c.to_str()).unwrap());
		}
	}

	// Double each number, then add the digits together
	// e.g., 8 * 2 = 16, 1 + 6 = 7
	even = even.iter().map(|&x| 2 * x).collect();
	even = even.iter().map(|&x| (x / 10) + (x % 10)).collect();

	// Sum all the numbers in the lists
	let even: int = even.iter().map(|&x| x).sum();
	let odd: int = odd.iter().map(|&x| x).sum();

	// Add the two sums together
	let total = even + odd;

	// If it's divisible by 10, it passes.
	total % 10 == 0
}

#[test]
fn test_luhn_pass() {
	let number = ~"49927398716";

	let result = luhn(number);

	if !result {
		fail!("49927398716 should pass!");
	}
}

#[test]
fn test_luhn_fail(){
	let number = ~"49927398717";

	let result = luhn(number);

	if result {
		fail!("49927398717 should fail!");
	}
}