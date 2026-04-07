use std::io::{self, Read};

fn convert_to_snake(input: &str) -> String {
	let mut output = String::new();

	let prefixes = [
		"u16", "u32", "u64", "i16", "i32", "i64", "f16", "f32", "f64", "dec", "dw", "ch", "fp", "dp", "rg", "sz", "st",
		"fn", "d", "b", "i", "f", "c", "f", "n", "p",
	];

	let input = prefixes.iter().find_map(|prefix| input.strip_prefix(prefix)).unwrap_or(input);

	for c in input.chars() {
		if c.is_uppercase() && !output.is_empty() {
			output.push('_');
			output.push_str(&c.to_lowercase().to_string());
		} else if c.is_numeric() {
			output.push(c);
		} else {
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

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn compress_test() {
		assert_eq!(convert_to_snake("szWindowContents"), String::from("window_contents"));
		assert_eq!(convert_to_snake("iAirflowParameter"), String::from("airflow_parameter"));
		assert_eq!(convert_to_snake("fMixtureRatio"), String::from("mixture_ratio"));
	}
}
