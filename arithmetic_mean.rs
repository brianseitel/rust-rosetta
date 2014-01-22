fn main() {
	let v: [f64, ..5] = [1.0, 2.0, 2.718, 3.0, 3.142];
	println!("{}", mean(v).to_str());
}

fn mean(vec: &[f64]) -> f64 {
	let mut sum: f64 = 0.0;

	for &i in vec.iter() {
		sum += i;
	}

	sum / vec.len() as f64
}