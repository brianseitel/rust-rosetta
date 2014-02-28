fn main() {
	let message: ~str = ~"WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWBWWWWWWWWWWWWWW";
	let results = run_length(message);
	println!("{}", results);
}

fn run_length(input: ~str) -> ~str {
	let mut count: uint = 0;
	let mut output = ~"";
	let mut last: char = input.clone().chars().nth(0).unwrap();
	for c in input.chars() {
		if last != c {
			output = output + count.to_str() + last.to_str();
			count = 0;
			last = c;
		}

		count += 1;
	}

	if count > 0 {
		output = output + count.to_str() + last.to_str();
	}

	output
}

#[test]
fn test_run_length() {
	let message = ~"WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWBWWWWWWWWWWWWWW";
	let output = ~"12W1B12W3B24W1B14W";

	if run_length(message) != output {
		fail!("Output does not match message!");
	}
}