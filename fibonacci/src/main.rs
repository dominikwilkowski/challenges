use std::time::Instant;

mod fibonacci_calc;
mod fibonacci_embed;

use fibonacci_calc::FibonacciCalc;
use fibonacci_embed::FibonacciEmbed;

fn main() {
	let input = std::env::args().collect::<Vec<String>>();
	if input.len() < 2 {
		eprintln!("Error: No input given");
		std::process::exit(1);
	} else if input.len() > 2 {
		eprintln!("Error: Too many arguments");
		std::process::exit(1);
	} else {
		// Calc path
		let start = Instant::now();
		let found = FibonacciCalc::calc(&input[1]);
		let duration = start.elapsed();
		println!(
			"For the input \"{}\" we found \"{found}\" fibonacci numbers by calculating each number within {} nanoseconds",
			input[1],
			duration.as_nanos()
		);

		// Embed path
		let start = Instant::now();
		let found = FibonacciEmbed::calc(&input[1]);
		let duration = start.elapsed();
		println!(
			"For the input \"{}\" we found \"{found}\" fibonacci numbers by looking each number up within  {} nanoseconds",
			input[1],
			duration.as_nanos()
		);
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn calc_fibonacci() {
		assert_eq!(FibonacciCalc::calc("10"), 1);
		assert_eq!(FibonacciCalc::calc("5021"), 4);
		assert_eq!(FibonacciCalc::calc("12345"), 5);
		assert_eq!(FibonacciCalc::calc("1000000"), 1);
	}

	#[should_panic]
	#[test]
	fn calc_fibonacci_panic_non_num() {
		FibonacciCalc::calc("abc");
	}

	#[should_panic]
	#[test]
	fn calc_fibonacci_panic_invalid_input() {
		FibonacciCalc::calc("1_2");
	}

	#[should_panic]
	#[test]
	fn calc_fibonacci_panic_zero() {
		FibonacciCalc::calc("0");
	}

	#[should_panic]
	#[test]
	fn calc_fibonacci_panic_too_large() {
		FibonacciCalc::calc("10000001");
	}

	#[test]
	fn embed_fibonacci() {
		assert_eq!(FibonacciEmbed::calc("10"), 1);
		assert_eq!(FibonacciEmbed::calc("5021"), 4);
		assert_eq!(FibonacciEmbed::calc("12345"), 5);
		assert_eq!(FibonacciEmbed::calc("1000000"), 1);
	}

	#[should_panic]
	#[test]
	fn embed_fibonacci_panic_non_num() {
		FibonacciEmbed::calc("abc");
	}

	#[should_panic]
	#[test]
	fn embed_fibonacci_panic_invalid_input() {
		FibonacciEmbed::calc("1_2");
	}

	#[should_panic]
	#[test]
	fn embed_fibonacci_panic_zero() {
		FibonacciEmbed::calc("0");
	}

	#[should_panic]
	#[test]
	fn embed_fibonacci_panic_too_large() {
		FibonacciEmbed::calc("10000001");
	}
}
