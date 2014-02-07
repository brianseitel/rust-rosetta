fn main() {
	let search: ~[uint] = ~[2,3,6,13,236,632,231,13,78,43,321,6,7];

	let result = bsearch(search, 13);

	println!("Found: {:?}", result > -1);
}

fn bsearch(mut haystack: ~[uint], needle: uint) -> int {
	haystack.sort();
	let mut low: uint = 0;
	let mut high:uint = haystack.len() - 1;

	while low <= high {
		let middle: uint = low + (high - low) / 2;
		if haystack[middle] > needle {
			high = middle - 1;
		} else if haystack[middle] < needle {
			low = middle + 1;
		} else {
			return middle as int;
		}
	}

	-1
}

#[test]
fn test_not_found() {
	let search: ~[uint] = ~[2,3,6,13,236,632,231,13,78,43,321,6,7];

	if bsearch(search, 999) > -1 {
		fail!("Found 999 in list, but list does not contain 999!")
	}
}

#[test]
fn test_found() {
	let search: ~[uint] = ~[2,3,6,13,236,632,231,13,78,43,321,6,7];

	if bsearch(search, 236) == -1 {
		fail!("Did not find 236 in list, but list contains 236!");
	}
}