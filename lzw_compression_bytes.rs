use std::hashmap::HashMap;
use std::io::File;

fn main() {
	let path = Path::new("/Users/brianseitel/Downloads/paradise-lost2.txt");
	let uncompressed = File::open(&path).read_to_end();

	let compressed = compress(uncompressed.clone());
	println!("{:?}", compressed);
	let decompressed = decompress(compressed);
	println!("{:?}", decompressed);
}

fn compress(uncompressed: ~[u8]) -> ~[u32] {
	let maxdictsize = 1000;
	let mut dict = build_compression_dict();

	let mut w = ~"";
	let mut result: ~[u32] = ~[];
	for c in uncompressed.iter() {
		if dict.len() > maxdictsize {
			let mut dict = build_compression_dict();
		}
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

fn build_compression_dict() -> HashMap<~str, u32> {
	let mut dict: HashMap<~str, u32> = HashMap::new();

	for i in range(0, 256) {
		let c = std::char::from_u32(i as u32).unwrap_or('?').to_str();
		dict.insert(c.to_str(), i as u32);
	}

	dict
}

fn build_decompression_dict() -> HashMap<u32, ~str> {
	let mut dict: HashMap<u32, ~str> = HashMap::new();
	for i in range(0, 256) {
		let c = std::char::from_u32(i as u32).unwrap_or('?').to_str();
		dict.insert(i as u32, c.to_str());
	}

	dict
}

fn decompress(mut compressed: ~[u32]) -> ~str {
	let maxdictsize = 1000;
	let mut dict = build_decompression_dict();

	let mut dictSize: uint = dict.len();
	let mut w: ~str = dict.get(&compressed[0]).to_str();
	let mut result = w.clone();
	compressed.remove(0);
	for k in compressed.iter() {
		if dict.len() > maxdictsize {
			let mut dict = build_decompression_dict();
		}
		let mut entry: ~str = ~"";
		if (dict.contains_key(k)) {
			entry = dict.get(k).to_str();
		} else if *k == (dictSize - 1) as u32 {
			entry = w.clone() + w.slice_to(1);
		}

		result = result + entry;
		if (entry.len() > 0) {
			dict.insert(dictSize as u32, w + entry.slice_to(1));
			dictSize += 1;
		}
		w = entry;
	}

	result
}