mod fibonacci_calc;

use fibonacci_calc::Fibonacci;

fn main() {
	let input = std::env::args().collect::<Vec<String>>();
	if input.len() < 2 {
		eprintln!("Error: No input given");
		std::process::exit(1);
	} else if input.len() > 2 {
		eprintln!("Error: Too many arguments");
		std::process::exit(1);
	} else {
		let found = Fibonacci::calc(&input[1]);
		println!("For the input \"{}\" we found \"{found}\" fibonacci numbers", input[1]);
	}
}
