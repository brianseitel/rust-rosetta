use std::hashmap::HashMap;

fn main() {
	// let (encode_dict, decode_dict) = build_dict();

	let uncompressed = ~"I like hot dogs and hot dogs are my favorite. Once I wrote a book about how bored I was, and I found out that my book was boring. I don't know how boring it was, but boredom is boring, I know.";

	let compressed = compress(uncompressed.clone());
	println!("{:?}", compressed.clone());
	println!("Length of uncompressed: {}", uncompressed.len().to_str());
	println!("Length of compressed: {}", compressed.len().to_str());
	let decompressed = decompress(compressed);
	println!("{:?}", decompressed);
}

fn compress(uncompressed: ~str) -> ~[u32] {
	let mut dict: HashMap<~str, u32> = HashMap::new();

	for i in range(0, 256) {
		let c = std::char::from_u32(i as u32).unwrap_or('?').to_str();
		dict.insert(c, i as u32);
	}

	let mut result: ~[u32] = ~[];
	let mut dictSize = 256;
	let mut w = ~"";
	for c in uncompressed.chars() {
		let wc = w + c.to_str();

		if dict.contains_key(&wc) {
			println!("Found {}", wc);
			w = wc;
		} else {
			result.push(*dict.find(&w).unwrap() as u32);
			dict.insert(wc.clone(), dictSize as u32);
			dictSize += 1;
			w = c.to_str();

			println!("{:?} => {:?}", wc, dictSize as u32);
		}
	}

	if !w.eq(&~"") {
		result.push(*dict.get(&w));
	}

	result
}

fn decompress(compressed: ~[u32]) -> ~str {
	let mut dict: HashMap<u32, ~str> = HashMap::new();

	for i in range(0, 256) {
		let c = std::char::from_u32(i as u32).unwrap_or('?').to_str();
		dict.insert(i as u32, c);
	}

	let mut result = ~"";
	let mut dictSize = 256;
	let mut w = ~"";

	result = w.clone();
	for i in range(0, compressed.len()) {
		let k = compressed.iter().nth(i).unwrap();
		let mut entry = ~"";
		if dict.contains_key(k) {
			println!("Contains {}", k.to_str());
			entry = dict.get(k).to_str();
		} else {
			if i == dictSize {
				println!("i == dictSize == {}", i.to_str());
				entry = w.clone() + dict.get(k).to_str();
			} else {
				fail!("Bad compressed k: {:?} {:?}", k, i);
			}
		}
		result = result + entry.clone();
		dictSize += 1;
		dict.insert(dictSize as u32, w.clone() + entry.clone());

		w = entry;
	}

	result
}

// fn build_dict() -> (HashMap<~str, u32>, HashMap<u32, ~str>) {
// 	let mut compress_list: HashMap<~str, u32> = HashMap::new();
// 	let mut decompress_list: HashMap<u32, ~str> = HashMap::new();

// 	for i in range(0, 256) {
// 		let c = std::char::from_u32(i as u32).unwrap_or('?').to_str();
// 		let c2 = c.clone();
// 		compress_list.insert(c, i as u32);
// 		decompress_list.insert(i as u32, c2);
// 	}

// 	(compress_list, decompress_list)
// }