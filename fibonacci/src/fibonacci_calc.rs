pub struct Fibonacci {}

impl Fibonacci {
	pub fn calc(input: &str) -> usize {
		let mut fibonacci_found = 0;
		let mut offset = 0;

		while offset < input.len() {
			let mut end = offset + 1;
			while end <= input.len() {
				let num = input[offset..end].parse::<u64>().unwrap();
				if Self::is_fibonacci(num) {
					fibonacci_found += 1;
				}

				end += 1;
			}

			offset += 1;
		}

		fibonacci_found
	}

	#[inline]
	fn is_perfect_square(x: u128) -> bool {
		let s = (x as f64).sqrt() as u128;
		s * s == x || (s + 1) * (s + 1) == x || s.saturating_sub(1).pow(2) == x
	}

	#[inline]
	fn is_fibonacci(n: u64) -> bool {
		let nn = 5 * n as u128 * n as u128;
		Self::is_perfect_square(nn + 4) || Self::is_perfect_square(nn.saturating_sub(4))
	}
}
