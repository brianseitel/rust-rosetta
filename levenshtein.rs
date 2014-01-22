fn main() {
	let distance = levenshtein("brian", "seitel");
	println!("{}", distance);
}

/*
	Calculate the levenshtein distance between two strings.
	@param first - 
 */
fn levenshtein(first: &str, second: &str) -> uint {
	let len1 = first.len() + 1;
	let len2 = second.len() + 1;

	// Build distance matrix
	let mut distance = ~[~[0]];

	for i in range(1, len1) { distance[0].push(i); }
	for j in range(1, len2) { distance.push(~[j]); }

	// For each letter in each word, compare each possible change (deletion, insertion, substitution)
	// and determine cheapest move. Save that as the distance for that position.
	for j in range(1, len2) {
		for i in range(1, len1) {
			let x: uint = if first[i - 1] == second[j - 1] {
				distance[j-1][i-1]
			} else {
				let min = [distance[j][i-1],distance[j-1][i],distance[j-1][i-1]];
				*min.iter().min().unwrap() + 1
			};

			distance[j].push(x);
		}
	}

	distance[len2-1][len1-1]
}