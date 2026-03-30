use std::{env::args, time::Instant};

fn main() {
	let start_time = Instant::now();
	let urls = args().skip(1).collect::<Vec<String>>();

	println!("Hello, world!");
}
