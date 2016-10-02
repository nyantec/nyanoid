use std::env;
use std::str;

fn main() {
	for arg in env::args().skip(1) {
		match nyanoid(arg.as_str()) {
			Ok(v) => println!("{}", v),
			Err(e) => panic!("Failed to parse argument ‘{}’: {}", arg, e),
		}
	}
}

fn nyanoid(id: &str) -> Result<u32, &'static str> {
	if id.len() > 6 {
		return Err("Identifier exceeds maximum length of 6 bytes");
	}

	let mut num = 0u32;
	let mut pow = 1u32;

	for chr in id.as_bytes() {
		if *chr < b'a' || *chr > b'z' {
			return Err("Identifier contains characters outside of valid range [a-z]");
		}

		num += (chr - b'a' + 1) as u32 * pow;
		pow *= 27u32;
	}

	Ok(num)
}
