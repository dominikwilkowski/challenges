use std::io::{self, Read};

fn convert_to_snake(input: &str) -> String {
	let mut output = String::new();

	let input = input.strip_prefix("u16").unwrap_or(input);
	let input = input.strip_prefix("u32").unwrap_or(input);
	let input = input.strip_prefix("u64").unwrap_or(input);
	let input = input.strip_prefix("i16").unwrap_or(input);
	let input = input.strip_prefix("i32").unwrap_or(input);
	let input = input.strip_prefix("i64").unwrap_or(input);
	let input = input.strip_prefix("f16").unwrap_or(input);
	let input = input.strip_prefix("f32").unwrap_or(input);
	let input = input.strip_prefix("f64").unwrap_or(input);
	let input = input.strip_prefix("dec").unwrap_or(input);
	let input = input.strip_prefix("dw").unwrap_or(input);
	let input = input.strip_prefix("ch").unwrap_or(input);
	let input = input.strip_prefix("fp").unwrap_or(input);
	let input = input.strip_prefix("dp").unwrap_or(input);
	let input = input.strip_prefix("rg").unwrap_or(input);
	let input = input.strip_prefix("sz").unwrap_or(input);
	let input = input.strip_prefix("st").unwrap_or(input);
	let input = input.strip_prefix("fn").unwrap_or(input);
	let input = input.strip_prefix("d").unwrap_or(input);
	let input = input.strip_prefix("b").unwrap_or(input);
	let input = input.strip_prefix("i").unwrap_or(input);
	let input = input.strip_prefix("f").unwrap_or(input);
	let input = input.strip_prefix("c").unwrap_or(input);
	let input = input.strip_prefix("f").unwrap_or(input);
	let input = input.strip_prefix("n").unwrap_or(input);
	let input = input.strip_prefix("p").unwrap_or(input);

	for c in input.chars() {
		if c.is_uppercase() && output.len() > 0 {
			output.push('_');
			output.push_str(&c.to_lowercase().to_string());
		} else if c.is_numeric() {
			output.push(c);
		} else if c.is_uppercase() && output.len() == 0 {
			output.push_str(&c.to_lowercase().to_string());
		} else if c.is_lowercase() {
			output.push_str(&c.to_lowercase().to_string());
		}
	}

	output
}

fn main() {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input).unwrap();
	let input = input.trim();
	let mut output = String::new();

	for line in input.lines() {
		output.push_str(&convert_to_snake(line));
		output.push('\n');
	}

	println!("{}", output.trim());
}
