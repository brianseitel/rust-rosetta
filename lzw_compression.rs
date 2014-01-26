use std::hashmap::HashMap;

fn main() {
	let uncompressed = ~"TOBEORNOTTOBEORTOBEORNOT";
	let compressed = compress(uncompressed.clone());
	println!("{:?}", compressed);
	let decompressed = decompress(compressed);
	println!("{:?}", decompressed);
}

fn compress(uncompressed: ~str) -> ~[u32] {
	let mut dict: HashMap<~str, u32> = HashMap::new();

	for i in range(0, 256) {
		let c = std::char::from_u32(i as u32).unwrap_or('?').to_str();
		dict.insert(c.to_str(), i as u32);
	}

	let mut w = ~"";
	let mut result: ~[u32] = ~[];
	for c in uncompressed.chars() {
		let wc = w + c.to_str();

		if dict.contains_key(&wc) {
			w = wc.clone();
		} else {
			let dictSize = dict.len();
			result.push(*dict.get(&w));
			dict.insert(wc.clone(), dictSize as u32);
			w = c.to_str();
		}
	}

	if w.len() > 0 {
		result.push(*dict.get(&w));
	}
	result
}

fn decompress(mut compressed: ~[u32]) -> ~str {
	let mut dict: HashMap<u32, ~str> = HashMap::new();
	for i in range(0, 256) {
		let c = std::char::from_u32(i as u32).unwrap_or('?').to_str();
		dict.insert(i as u32, c.to_str());
	}

	let mut dictSize: uint = dict.len();
	let mut w: ~str = dict.get(&compressed[0]).to_str();
	let mut result = w.clone();
	compressed.remove(0);
	for k in compressed.iter() {
		let mut entry: ~str = ~"";
		if (dict.contains_key(k)) {
			entry = dict.get(k).to_str();
		} else if *k == (dictSize - 1) as u32 {
			entry = w.clone() + w.slice_to(1);
		}

		result = result + entry.clone();
		dict.insert(dictSize as u32, w.clone() + entry.slice_to(1));
		dictSize += 1;

		w = entry;
	}

	result
}