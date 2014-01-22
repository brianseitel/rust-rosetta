fn main() {
	let a = 12;
	let b = 8;

	println!("Add\t = {:d}", a + b);
	println!("Subtract\t = {:d}", a - b);
	println!("Multiply\t = {:d}", a * b);
	println!("Divide\t = {:d}", a / b);
	println!("Remainder\t = {:d}", a % b);
	println!("Exponent\t = {:d}", std::num::pow(a as int, b as uint));
}