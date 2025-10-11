use std::time::{Duration, Instant};

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
