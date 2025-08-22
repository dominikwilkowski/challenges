mod range;

use range::Range;

fn main() {
	let input = std::env::args().collect::<Vec<String>>();
	if input.len() < 2 {
		eprintln!("No input given");
		std::process::exit(1);
	} else if input.len() > 3 {
		eprintln!("Too many arguments");
		std::process::exit(1);
	} else {
		let range = Range::new(&input[1]);
		match range {
			Ok(value) => println!("{:?}", value.get_range()),
			Err(error) => eprintln!("Error: {}", error),
		}
	}
}
