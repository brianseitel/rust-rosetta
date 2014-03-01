fn main() {
	let message: ~str = ~"WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWBWWWWWWWWWWWWWW";
	let results = run_length(message);
	println!("{}", results);
}

fn run_length(input: ~str) -> ~str {
	let mut count: uint = 0;
	let mut output = ~"";

	// Get first character
	let mut last: char = input.char_at(0);

	for c in input.chars() {
		// If the current character is different from the last character,
		// append the count and the last character to the string.
		// Then reset the count and make the current character the "last"
		// character
		if last != c {
			output = output + count.to_str() + last.to_str();
			count = 0;
			last = c;
		}

		count += 1;
	}

	// If we have any counts left over, append them to the string
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