use std::io::{self, Read};

fn main() {
	let mut input = String::new();
	io::stdin().read_to_string(&mut input).unwrap();
	let input = input.trim();
	let mut output = String::new();

	for line in input.lines() {
		output.push_str(&format!("{}", convert(line)));
		output.push('\n');
	}

	println!("{}", output.trim());
}

fn convert(input: &str) -> u16 {
	let binary = input.replace("#", "1").replace(".", "0");
	u16::from_str_radix(&binary, 2).unwrap()
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn compress_test() {
		assert_eq!(convert("#.#.#.###.#.##.#"), 43949);
		assert_eq!(convert("##.##.......#..#"), 55305);
		assert_eq!(convert("#..#####..#.#..."), 40744);
		assert_eq!(convert("###..#....###.##"), 58427);
		assert_eq!(convert("#..#..#.#....##"), 18755);
		assert_eq!(convert("#############.#."), 65530);
	}
}
