pub struct FibonacciCalc {}

impl FibonacciCalc {
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
					if Self::is_fibonacci(num) && fibonacci_found.binary_search(&num).is_err() {
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

	#[inline]
	fn is_perfect_square(x: u128) -> bool {
		let s = (x as f64).sqrt() as u128;
		s * s == x || (s + 1) * (s + 1) == x || s.saturating_sub(1).pow(2) == x
	}

	#[inline]
	fn is_fibonacci(n: u64) -> bool {
		if n == 0 {
			return false;
		}

		let nn = 5 * n as u128 * n as u128;
		Self::is_perfect_square(nn + 4) || Self::is_perfect_square(nn.saturating_sub(4))
	}
}
