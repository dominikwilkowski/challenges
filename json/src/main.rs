use std::io::{self, Read};

fn main() {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input).unwrap();
	let input = input.trim();
	let mut output = Vec::new();

	for line in input.lines() {
		output.push(parse(line));
	}

	output.sort_by_key(|a| a.balance);
	output.iter().for_each(|Line { name, balance }| println!("{name}: {}", format_i64_commas(*balance)));
}

#[derive(Debug, PartialEq)]
struct Line {
	name: String,
	balance: i64,
}

fn parse(input: &str) -> Line {
	let (name_end, _) = input.match_indices("\"").nth(1).expect("No second quote found");
	let name = input[2..name_end].to_string();

	let balance_start = input.rfind("\"balance\": ").expect("No balance start found") + 11;
	let balance_end = if input.find("\"extra\"").is_some() {
		input.rfind("}}").expect("No balance end found in extra branch")
	} else {
		input.rfind(", \"account_number").expect("No balance end found")
	};
	let balance = input[balance_start..balance_end]
		.parse()
		.unwrap_or_else(|_| panic!("Could not parse balance \"{}\"", &input[balance_start..balance_end]));

	Line { name, balance }
}

fn format_i64_commas(n: i64) -> String {
	let sign = if n < 0 { "-" } else { "" };
	let s = n.unsigned_abs().to_string();

	let mut out = String::with_capacity(sign.len() + s.len() + s.len() / 3);
	out.push_str(sign);

	let bytes = s.as_bytes();
	let len = bytes.len();

	for (i, &b) in bytes.iter().enumerate() {
		out.push(b as char);
		let left = len - i - 1;
		if left > 0 && left.is_multiple_of(3) {
			out.push(',');
		}
	}
	out
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn parse_test() {
		assert_eq!(
			parse(r#"{"Bentley.G": {"balance": 2134, "account_number": 233831255"}}"#),
			Line {
				name: String::from("Bentley.G"),
				balance: 2134
			}
		);

		assert_eq!(
			parse(r#"{"Barclay.E": {"balance": 1123, "account_number": 312333321}}"#),
			Line {
				name: String::from("Barclay.E"),
				balance: 1123
			}
		);

		assert_eq!(
			parse(r#"{"Alton.K": {"balance": 9315, "account_number": 203123613}, "extra": {"balance": 131}}"#),
			Line {
				name: String::from("Alton.K"),
				balance: 131
			}
		);

		assert_eq!(
			parse(r#"{"Bancroft.M": {"balance": 233,"account_number":287655771101}, "extra": {"balance": 98}}"#),
			Line {
				name: String::from("Bancroft.M"),
				balance: 98
			}
		);
	}

	#[test]
	fn format_i64_commas_test() {
		assert_eq!(format_i64_commas(2134), String::from("2,134"));
		assert_eq!(format_i64_commas(1123), String::from("1,123"));
		assert_eq!(format_i64_commas(1507378), String::from("1,507,378"));
		assert_eq!(format_i64_commas(-1004810), String::from("-1,004,810"));
	}
}
