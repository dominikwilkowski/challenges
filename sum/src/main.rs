use std::io::{self, Read};

fn main() {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input).unwrap();
	let input = input.trim();
	let mut output = String::new();

	for line in input.lines() {
		output.push_str(&format!("{}\n", parse(line)));
	}

	println!("{}", output.trim());
}

fn parse(input: &str) -> u64 {
	let mut nums = Vec::new();

	for item in input.split(" ") {
		if item.starts_with("0b") {
			// binary
			nums.push(u64::from_str_radix(&item[2..], 2).unwrap_or_else(|_| panic!("Invalid binary item: {}", item)));
		} else if item.starts_with("0o") {
			// octal
			nums.push(u64::from_str_radix(&item[2..], 8).unwrap_or_else(|_| panic!("Invalid octal item: {}", item)));
		} else if item.starts_with("0x") {
			// hex
			nums.push(u64::from_str_radix(&item[2..], 16).unwrap_or_else(|_| panic!("Invalid hex item: {}", item)));
		} else if item.bytes().all(|b| b.is_ascii_digit()) {
			// decimal
			nums.push(item.parse::<u64>().unwrap());
		} else {
			// ascii
			nums.push(item.chars().next().unwrap_or_else(|| panic!("Invalid ascii item: {}", item)) as u64);
		}
	}

	nums.into_iter().sum::<u64>()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn parse_test() {
		assert_eq!(parse("110 0x187 300 T / d"), 1032);
		assert_eq!(parse("180 A 0x10e 0x18c N 95"), 1084);
		assert_eq!(parse("423 0xac 417 0o20 q &"), 1179);
		assert_eq!(parse("0x14e 0b10000 247 284 0o447 268"), 1444);
	}
}
