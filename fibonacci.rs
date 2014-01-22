fn main() {
	let max = 25;
	let results = fibonacci(max, ~[]); // pass in max as arg1 and an empty, dynamically-sized vector as arg2

	println!("{:?}", results); // Print the results as a string. "{:?}" lets us print anything.
}

// @param uint - max - an unsigned integer, the number of Fibonacci numbers to generate
// @param vector - list - a mutable, empty, dynamically sized vector
// @return ~[int] - we're going to return a list of integers
fn fibonacci(max: uint, mut list: ~[int]) -> ~[int] {
	if list.len() == 0 {
		list = ~[0,1]; // If the list is empty, give it 0 and 1 as first to items (see rules above)
	} else if list.len() == max {
		return list;
	}

	// Get the last two items on the list
	let n1 = list.len() - 1; // Last item
	let n2 = list.len() - 2; // Second-to-last item

	let f1 = *list.iter().nth(n1).unwrap(); // Get the (n-1)th item
	let f2 = *list.iter().nth(n2).unwrap(); // get the (n-2)th item

	// Add them together and push the sum onto the list!
	list.push(f1 + f2);

	// Get the next number with the new list
	return fibonacci(max, list)
}