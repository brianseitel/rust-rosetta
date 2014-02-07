use std::hashmap::HashMap;
use std::io::File;

static initialdictsize: uint = 256;
static maxdictsize: uint = 65535;

fn main() {
	let mut args = std::os::args();

	if args.len() < 2 {
		fail!("Syntax: ./lzw_compression_bytes (source)");
	}
	args.shift(); // lose first arg, which is command
	let source: ~str = args.shift().unwrap();

	if source.len() == 0 {
		fail!("Syntax: ./lzw_compression_bytes (source)");
	}

	let uncompressed = read_file(source.clone());

	let compressed = compress(uncompressed);
	write_compressed(compressed.clone(), source.clone());
	
	println!("------");

	let compressed = read_compressed(source.clone());

	let decompressed = decompress(compressed);
	write_decompressed(decompressed, source.clone());
}

fn compress(uncompressed: ~[u8]) -> ~[u16] {
	let mut dict = build_compression_dict();
	let mut w: ~[u8] = ~[];
	let mut result: ~[u16] = ~[];
	for &c in uncompressed.iter() {
		let mut wc = w.clone();
		wc.push(c);

		if dict.contains_key(&wc) {
			w = wc.clone();
		} else {
			let dictSize = dict.len();
			result.push(*dict.get(&w));
			dict.insert(wc.clone(), dictSize as u16);
			w = ~[c];
		}
		if dict.len() as uint > maxdictsize {
			dict = build_compression_dict();
		}
	}

	if w.len() > 0 {
		result.push(*dict.get(&w));
	}

	result
}

fn decompress(compressed: ~[u16]) -> ~[u8] {
	let mut dict = build_decompression_dict();
	let mut result: ~[u8] = ~[];

	for i in range(0, compressed.len()) {
		let k = compressed[i];

		let mut entry = dict.get(&k).to_owned();
		let new_key = dict.len() as u16;

		std::vec::bytes::push_bytes(&mut result, entry);

		if i < compressed.len() - 1 {
			let kn = compressed[i+1];
			if dict.contains_key(&kn) {
				entry.push(dict.get(&kn)[0]);
			} else {
				entry.push(entry[0]);
			}
			dict.insert(new_key, entry);
		}

		if dict.len() as uint > maxdictsize {
			dict = build_decompression_dict();
		}
	}

	result
}

fn write_decompressed(data: ~[u8], filepath: ~str) {
	let path = Path::new(filepath + ".decompressed");
	let mut fp = File::create(&path);
	match fp.write(data) {
		Err(x) => println!("Error writing decompressed file: {:?}", x),
		Ok(_) => {}
	}
}

fn write_compressed(data: ~[u16], filepath: ~str) {
	let path = Path::new(filepath + ".compressed");
	let mut fp = File::create(&path);
	for b in data.iter() {
		match fp.write_be_u16(*b) {
			Err(x) => println!("Error writing compressed file: {:?}", x),
			Ok(_) => {}
		}
	}
}

fn read_compressed(filepath: ~str) -> ~[u16] {
	let path = Path::new(filepath + ".compressed");
	let mut file = File::open(&path).unwrap();
	let mut compressed: ~[u16] = ~[];
	loop {
		let b = file.read_be_u16();
		match b {
			Ok(x) => compressed.push(x),
			Err(_) => break // End of the file, get outta here.
		};

	}

	compressed
}

fn read_file(filepath: ~str) -> ~[u8] {
	let path = Path::new(filepath);
	File::open(&path).read_to_end().unwrap()
}

fn build_compression_dict() -> HashMap<~[u8], u16> {
	let mut dict: HashMap<~[u8], u16> = HashMap::new();

	for i in range(0, initialdictsize) {
		let c = std::char::from_u32(i as u32).unwrap_or('?');
		dict.insert(~[c as u8], i as u16);
	}

	dict
}

fn build_decompression_dict() -> HashMap<u16, ~[u8]> {
	let mut dict: HashMap<u16, ~[u8]> = HashMap::new();
	for i in range(0, initialdictsize) {
		let c = std::char::from_u32(i as u32).unwrap_or('?');
		dict.insert(i as u16, ~[c as u8]);
	}

	dict
}
