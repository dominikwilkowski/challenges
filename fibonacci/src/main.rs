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
		const ITERATIONS: u32 = 1_000_000;

		{
			// CALC PATH
			// --- Warm-up phase ---
			for _ in 0..1000 {
				let _ = FibonacciCalc::calc(&input[1]);
			}

			// --- Measurement phase ---
			let start = Instant::now();
			for _ in 0..ITERATIONS - 1 {
				let _ = FibonacciCalc::calc(&input[1]);
			}
			let found = FibonacciCalc::calc(&input[1]);
			let duration = start.elapsed();

			// --- Reporting phase ---
			let avg = duration.as_nanos() as f64 / ITERATIONS as f64;
			println!(
				"For the input \"{}\" we found \"{found}\" fibonacci numbers by calculating each number within an average of {avg:.2} nanoseconds",
				input[1]
			);
		}

		{
			// EMBED PATH
			// --- Warm-up phase ---
			for _ in 0..1000 {
				let _ = FibonacciEmbed::calc(&input[1]);
			}

			// --- Measurement phase ---
			let start = Instant::now();
			for _ in 0..ITERATIONS - 1 {
				let _ = FibonacciEmbed::calc(&input[1]);
			}
			let found = FibonacciEmbed::calc(&input[1]);
			let duration = start.elapsed();

			// --- Reporting phase ---
			let avg = duration.as_nanos() as f64 / ITERATIONS as f64;
			println!(
				"For the input \"{}\" we found \"{found}\" fibonacci numbers by looking each number up within an average of  {avg:.2} nanoseconds",
				input[1]
			);
		}
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
