use std::io::{self, Read};

fn main() {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input).unwrap();
	let input = input.trim();
	let mut output = String::new();

	for line in input.lines() {
		if check_parentheses(line) {
			output.push_str("yes");
		} else {
			output.push_str("no");
		}
		output.push('\n');
	}

	println!("{}", output.trim());
}

fn check_parentheses(input: &str) -> bool {
	let mut stack = Vec::new();

	for c in input.chars() {
		match c {
			'(' => stack.push(c),
			')' => {
				if stack.pop() != Some('(') {
					return false;
				}
			},
			_ => continue,
		}
	}

	stack.is_empty()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn check_parentheses_test() {
		assert_eq!(check_parentheses("(())"), true);
		assert_eq!(check_parentheses("()))"), false);
		assert_eq!(check_parentheses("(()((())))"), true);
		assert_eq!(check_parentheses("(()(()(()))"), false);
	}
}
