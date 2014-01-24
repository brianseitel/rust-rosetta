use std::hashmap::HashMap;

fn main() {
	let (encode_dict, decode_dict) = build_dict();

	let uncompressed = ~"I like hot dogs";

	let compressed = compress(uncompressed, encode_dict);
	println!("{:?}", compressed);
	let decompressed = decompress(compressed, decode_dict);
	println!("{:?}", decompressed);
}

fn compress(uncompressed: ~str, mut dict: HashMap<~str, u32>) -> ~[u32] {
	let mut result: ~[u32] = ~[];
	let mut dictSize = 256;
	let mut w = ~"";
	for c in uncompressed.chars() {
		let wc = w + c.to_str();

		if dict.contains_key(&wc) {
			w = wc;
		} else {
			result.push(*dict.find(&w).unwrap() as u32);
			dictSize += 1;
			dict.insert(wc, dictSize);
			w = c.to_str();
		}
	}
	if !w.eq(&~"") {
		result.push(*dict.get(&w));
	}
	result
}

fn decompress(compressed: ~[u32], mut dict: HashMap<u32, ~str>) -> ~str {
	let mut result = ~"";
	let mut dictSize = 256;
	let mut w = ~"";

	for k in compressed.iter() {
		let mut entry = ~"";
		if dict.contains_key(k) {
			entry = dict.get(k).to_str();
		} else if k.to_int().unwrap() == dictSize {
			entry = w + dict.get(k).to_str();
		} else {
			fail!("Bad compressed k: {:?}", k);
		}
		let we = w  + entry.clone();
		dictSize += 1;
		dict.insert(dictSize as u32, entry.clone());
		result = result + entry.clone();

		w = we;
	}

	result
}

fn build_dict() -> (HashMap<~str, u32>, HashMap<u32, ~str>) {
	let mut compress_list: HashMap<~str, u32> = HashMap::new();
	let mut decompress_list: HashMap<u32, ~str> = HashMap::new();

	for i in range(0, 256) {
		let c = std::char::from_u32(i as u32).unwrap_or('?').to_str();
		let c2 = c.clone();
		compress_list.insert(c, i as u32);
		decompress_list.insert(i as u32, c2);
	}

	(compress_list, decompress_list)
}