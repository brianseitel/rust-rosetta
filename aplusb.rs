use std::io::buffered::BufferedReader;
use std::io::stdin;

fn main() {
	let input = BufferedReader::new(stdin()).read_line().unwrap();

	// Drop the \n, split into words, collect into vector
	let words: ~[&str] = input.slice_to(input.len() - 1).words().collect();

	// Convert to integers, add them together
	let sum = from_str::<int>(words[0]).unwrap() + from_str::<int>(words[1]).unwrap();

	// Build response string
	let response = words[0] + " " + words[1] + " " + sum.to_str();
	println!("{}", response);
}