use std::{
	env,
	fs::File,
	io::{self, Write},
	time::Instant,
};

mod infix;
mod postfix;

use crate::{infix::generate_random_expression, postfix::Postfix};

const OUTPUT_FILE_NAME: &str = "./postfix.csv";

fn main() -> io::Result<()> {
	let time = Instant::now();
	let cli_args = env::args().skip(1).collect::<Vec<String>>();

	#[allow(clippy::get_first)]
	let rows = cli_args.get(0).unwrap_or(&String::from("100")).parse::<usize>().unwrap_or(100);
	let cols = cli_args.get(1).unwrap_or(&String::from("100")).parse::<usize>().unwrap_or(100);

	let num_expressions = rows * cols;
	let max_depth = 3;
	let mut file =
		File::create(OUTPUT_FILE_NAME).expect("Creating write lock for output file {OUTPUT_FILE_NAME:?} failed.");

	for i in 0..num_expressions {
		if i % cols == 0 && i != 0 {
			writeln!(file)?;
		} else if i % cols != 0 {
			write!(file, ",")?;
		}

		let expression = generate_random_expression(max_depth);
		let postfix = Postfix::infix_to_postfix(&expression, rows, cols, &mut rand::thread_rng()).unwrap();
		write!(file, "{}", postfix)?;
	}

	println!("File successfully written to {OUTPUT_FILE_NAME:?}\nTime: {:#?}", time.elapsed());

	Ok(())
}
