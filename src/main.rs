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

	for chr in id.bytes().rev() {
		if chr < b'a' || chr > b'z' {
			return Err("Identifier contains characters outside of valid range [a-z]");
		}

		num += (chr - b'a' + 1) as u32 * pow;
		pow *= 26u32;
	}

	Ok(num)
}

#[cfg(test)]
mod tests {
	use nyanoid;
	use std::str;

	#[test]
	fn empty() {
		assert_eq!(nyanoid(""), Ok(0));
	}

	#[test]
	fn fixed() {
		assert_eq!(nyanoid("a"), Ok(1));
		assert_eq!(nyanoid("z"), Ok(26));
		assert_eq!(nyanoid("aa"), Ok(26 + 1));
		assert_eq!(nyanoid("az"), Ok(26 + 26));
		assert_eq!(nyanoid("ngr"), Ok(9664));
	}

	#[test]
	fn invalid() {
		assert!(nyanoid("ü").is_err());
		assert!(nyanoid("🦄").is_err());
	}

	#[test]
	fn range() {
		for chr in b'a'..(b'z' + 1u8) {
			assert_eq!(nyanoid(str::from_utf8(&vec![chr]).unwrap()),
			           Ok((chr - b'a' + 1u8) as u32));
		}
	}

	#[test]
	fn invalid_range() {
		for chr in 0u8..b'a' {
			assert!(nyanoid(str::from_utf8(&vec![chr]).unwrap()).is_err());
		}

		for chr in (b'z' + 1u8)..128u8 {
			assert!(nyanoid(str::from_utf8(&vec![chr]).unwrap()).is_err());
		}
	}
}
