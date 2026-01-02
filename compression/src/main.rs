use std::io::{self, Read};

fn main() {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input).unwrap();
	let input = input.trim();
	let mut output = String::new();

	for line in input.lines() {
		output.push_str(&compress(line));
		output.push('\n');
	}

	println!("{}", output.trim());
}

fn compress(input: &str) -> String {
	let mut output = String::new();
	let mut count = 1;

	let mut iter = input.chars().peekable();
	while let Some(c) = iter.next() {
		if let Some(next) = iter.peek()
			&& next == &c
		{
			count += 1;
		} else {
			if count > 2 {
				output.push_str(&format!("{count}{c}"));
			} else if count == 2 {
				output.push(c);
				output.push(c);
			} else {
				output.push(c);
			}
			count = 1;
		}
	}

	output
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn compress_test() {
		assert_eq!(compress("aaaaaiiiixqvsm"), String::from("5a4ixqvsm"));
		assert_eq!(compress("rrdkuuuuyyyrrrrgghc"), String::from("rrdk4u3y4rgghc"));
		assert_eq!(compress("xhzzzccccvvsssqppc"), String::from("xh3z4cvv3sqppc"));
		assert_eq!(compress("jbiiiulllllvvvvtttttxxxxxs"), String::from("jb3iu5l4v5t5xs"));
	}
}
