include!(concat!(env!("OUT_DIR"), "/fibonacci_data.rs"));

pub struct FibonacciEmbed {}

impl FibonacciEmbed {
	pub fn calc(input: &str) -> usize {
		if input == "0" {
			panic!("Input can't be 0");
		} else if input.len() > 7 {
			panic!("Input can't be larger than 1000000");
		}

		let mut fibonacci_found = Vec::with_capacity(30);
		let mut offset = 0;

		while offset < input.len() {
			let mut end = offset + 1;
			while end <= input.len() {
				if let Ok(num) = input[offset..end].parse::<u64>() {
					if FIBONACCI_NUMBERS.binary_search(&num).is_ok() && fibonacci_found.binary_search(&num).is_err() {
						fibonacci_found.push(num);
					}
				} else {
					panic!("Input could not be converted to a number. Only numbers between 1 and 100000 are allowed");
				}

				end += 1;
			}

			offset += 1;
		}

		fibonacci_found.len()
	}
}
