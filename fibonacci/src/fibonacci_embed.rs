include!(concat!(env!("OUT_DIR"), "/fibonacci_data.rs"));

pub struct FibonacciEmbed {}

impl FibonacciEmbed {
	pub fn calc(input: &str) -> usize {
		let mut fibonacci_found = 0;
		let mut offset = 0;

		while offset < input.len() {
			let mut end = offset + 1;
			while end <= input.len() {
				let num = input[offset..end].parse::<u64>().unwrap();
				if FIBONACCI_NUMBERS.binary_search(&num).is_ok() {
					fibonacci_found += 1;
				}

				end += 1;
			}

			offset += 1;
		}

		fibonacci_found
	}
}
