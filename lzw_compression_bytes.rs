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
	let source: ~str = args.shift();

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

fn decompress(mut compressed: ~[u16]) -> ~[u8] {
	let mut dict = build_decompression_dict();
	let mut dictSize: u16 = dict.len() as u16;
	let mut w: ~[u8] = ~[compressed.shift() as u8];
	let mut result = w.clone();

	for k in compressed.iter() {
		let mut entry: ~[u8] = ~[];
		if dict.contains_key(k) {
			entry = dict.get(k).to_owned();
		} else if k == &dictSize {
			entry.push(w[0]);
		}

		let mut new_entry = w.clone();
		new_entry.push(entry[0]);
		dict.insert(dictSize, new_entry.clone());
		dictSize += 1;

		std::vec::bytes::push_bytes(&mut result, entry);
		w = entry.clone();

		if dict.len() as uint > maxdictsize {
			dict = build_decompression_dict();
			dictSize = dict.len() as u16;
		}
	}

	result
}

fn write_decompressed(data: ~[u8], filepath: ~str) {
	let path = Path::new(filepath + ".decompressed");
	let mut fp = File::create(&path);
	fp.write(data);
}

fn write_compressed(data: ~[u16], filepath: ~str) {
	let path = Path::new(filepath + ".compressed");
	let mut fp = File::create(&path);
	for b in data.iter() {
		fp.write_be_u16(*b);
	}
}

fn read_compressed(filepath: ~str) -> ~[u16] {
	let path = Path::new(filepath + ".compressed");
	let mut file = File::open(&path).unwrap();
	let mut compressed: ~[u16] = ~[];
	loop {
		if file.eof() { break; }
		compressed.push(file.read_be_u16());
	}

	compressed
}

fn read_file(filepath: ~str) -> ~[u8] {
	let path = Path::new(filepath);
	File::open(&path).read_to_end()
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
