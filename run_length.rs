fn main() {
	let message: ~str = ~"WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWBWWWWWWWWWWWWWW";
	let results = run_length(message);
	println!("{}", results);

	let results = decode(results);
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

fn decode(input: ~str) -> ~str {
	let mut output = ~"";
	let mut number = ~"";

	// Loop through each character
	for c in input.chars() {

		// If the character is a digit, append it to the number string.
		// Eventually, we'll wind up with "1" or "12"
		if c.is_digit() {
			number = number + c.to_str();
		} else {
			// If it's not a digit, then it's a letter. Cast to uint,
			// then repeat the character X times and append to output.
			let num = from_str::<uint>(number).unwrap_or(0);
			output = output + c.to_str().repeat(num);
			number = ~"";
		}
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

#[test]
fn test_run_length_decode() {
	let encoded = ~"12W1B12W3B24W1B14W";
	let output = ~"WWWWWWWWWWWWBWWWWWWWWWWWWBBBWWWWWWWWWWWWWWWWWWWWWWWWBWWWWWWWWWWWWWW";

	if decode(encoded) != output {
		fail!("Output does not match encoded message!");
	}
}