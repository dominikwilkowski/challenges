use std::io::{self, Read};

fn main() {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input).unwrap();
	let input = input.trim();
	let mut output = String::new();

	for line in input.lines() {
		output.push_str(&format!("{}\n", base64_decode(line).unwrap()));
	}

	println!("{}", output.trim());
}

#[derive(Debug, PartialEq)]
pub enum Base64Error {
	InvalidLength,
	InvalidByte(u8),
	InvalidPadding,
}

fn b64_val(b: u8) -> Result<u8, Base64Error> {
	match b {
		b'A'..=b'Z' => Ok(b - b'A'),
		b'a'..=b'z' => Ok(b - b'a' + 26),
		b'0'..=b'9' => Ok(b - b'0' + 52),
		b'+' => Ok(62),
		b'/' => Ok(63),
		b'=' => Ok(0),
		_ => Err(Base64Error::InvalidByte(b)),
	}
}

pub fn base64_decode(input: &str) -> Result<String, Base64Error> {
	let bytes: Vec<u8> = input.bytes().filter(|b| !b" \n\r\t".contains(b)).collect();

	if !bytes.len().is_multiple_of(4) {
		return Err(Base64Error::InvalidLength);
	}

	let mut out = Vec::with_capacity(bytes.len() / 4 * 3);

	for chunk in bytes.chunks_exact(4) {
		let a = chunk[0];
		let b = chunk[1];
		let c = chunk[2];
		let d = chunk[3];

		// Validate padding rules
		let pad2 = c == b'=';
		let pad3 = d == b'=';
		if pad2 && !pad3 {
			// "xx=Y" is invalid
			return Err(Base64Error::InvalidPadding);
		}

		let v0 = b64_val(a)? as u32;
		let v1 = b64_val(b)? as u32;
		let v2 = b64_val(c)? as u32;
		let v3 = b64_val(d)? as u32;

		let n = (v0 << 18) | (v1 << 12) | (v2 << 6) | v3;

		out.push(((n >> 16) & 0xFF) as u8);
		if !pad2 {
			out.push(((n >> 8) & 0xFF) as u8);
		}
		if !pad3 {
			out.push((n & 0xFF) as u8);
		}
	}

	Ok(String::from_utf8_lossy(&out).to_string())
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn base64_decode_test() {
		assert_eq!(base64_decode("bGF0ZS1hdC1uaWdodA=="), Ok(String::from("late-at-night")));
		assert_eq!(base64_decode("d2l0aC10aGUtcmlzaW5nLWFwZQ=="), Ok(String::from("with-the-rising-ape")));
		assert_eq!(base64_decode("dGhlLXJ1dGhsZXNzLXNldmVu"), Ok(String::from("the-ruthless-seven")));
	}
}
