use std::hashmap::HashMap;
use std::io::File;

fn main() {
	// let path = Path::new("/Users/brianseitel/Downloads/paradise-lost2.txt");
	// let uncompressed = File::open(&path).read_to_end();

	let uncompressed = (~"TOBEORNOTTOBEORTOBEORNOT").into_bytes();
	println!("Original: {:?}", uncompressed.len());
	println!("Compressing...");
	let compressed = compress(uncompressed);
	println!("Done compressing!");
	println!("Compressed: {:?}", compressed.len());
	println!("Writing to file...");
	let path2 = Path::new("test.txt.compressed");
	let mut dupe = File::create(&path2);
	dupe.write(compressed);
	println!("Done");

	// Decompress

	println!("Decompressing!");
	let decompressed = decompress(compressed);
	println!("Done decompressing!");
	println!("Decompressed: {:?}", decompressed.len());

	println!("Writing to file...");
	let path3 = Path::new("test.txt.decompressed");
	let mut dupe = File::create(&path3);
	dupe.write(decompressed);
	println!("Done!");

}

fn compress(uncompressed: ~[u8]) -> ~[u8] {
	let maxdictsize = 1000;
	let mut dict = build_compression_dict();

	let mut w: ~[u8] = ~[];
	let mut result: ~[u8] = ~[];
	for &c in uncompressed.iter() {
		if dict.len() > maxdictsize {
			let mut dict = build_compression_dict();
		}
		let mut wc = w.clone();
		wc.push(c);

		if dict.contains_key(&wc) {
			w = wc.clone();
		} else {
			println!("Adding {:?} => {:?}", wc, dict.get(&w));
			let dictSize = dict.len();
			result.push(*dict.get(&w));
			dict.insert(wc.clone(), dictSize as u8);
			w = ~[c];
		}
	}

	if w.len() > 0 {
		result.push(*dict.get(&w));
	}
	result
}

fn build_compression_dict() -> HashMap<~[u8], u8> {
	let mut dict: HashMap<~[u8], u8> = HashMap::new();

	for i in range(0, 256) {
		let c = std::char::from_u32(i as u32).unwrap_or('?');
		dict.insert(~[c as u8], i as u8);
	}

	dict
}

fn build_decompression_dict() -> HashMap<u8, ~[u8]> {
	let mut dict: HashMap<u8, ~[u8]> = HashMap::new();
	for i in range(0, 256) {
		let c = std::char::from_u32(i as u32).unwrap_or('?');
		dict.insert(i as u8, ~[c as u8]);
	}

	dict
}

fn decompress(mut compressed: ~[u8]) -> ~[u8] {
	let maxdictsize = 1000;
	let mut dict = build_decompression_dict();

	let mut dictSize: u8 = dict.len() as u8;
	let mut w: ~[u8] = dict.get(&compressed[0]).to_owned();
	let mut result = w.clone();

	compressed.remove(0);
	for k in compressed.iter() {
		if dict.len() > maxdictsize {
			let mut dict = build_decompression_dict();
		}
		let mut entry: ~[u8] = ~[];
		if dict.contains_key(k) {
			entry = dict.get(k).to_owned();
		} else if k == &((dictSize.clone() - 1) as u8) {
			entry.push(w[0]);
		}

		std::vec::bytes::push_bytes(&mut result, entry);

		let mut new_entry = entry.clone();
		new_entry.push(w[0]);
		dict.insert(dictSize as u8, new_entry.clone());
		dictSize += 1;

		w = new_entry;
	}

	result
}
