use std::{
	env,
	fs::{self, File},
	io::Result,
	io::{self, Read},
	path::PathBuf,
	process,
	time::Instant,
};

pub mod args;
pub mod coord;
pub mod postfix;
pub mod sheet;

use crate::{args::Args, postfix::Postfix, sheet::Sheet};

fn read_to_string(file_path: PathBuf) -> Result<String> {
	let mut file = File::open(file_path)?;
	let mut contents = String::new();
	file.read_to_string(&mut contents)?;
	Ok(contents)
}

fn write_from_string(file_path: PathBuf, output: String) -> io::Result<()> {
	fs::write(file_path, output)?;
	Ok(())
}

fn main() -> Result<()> {
	let time = Instant::now();
	let cli_args = Args::parse(env::args().skip(1).collect::<Vec<String>>());

	if cli_args.help {
		println!("{}", Args::help());
	} else if cli_args.version {
		println!("{}", Args::version());
	} else {
		let csv = read_to_string(cli_args.csv_path)?;

		let mut sheet = Sheet::new(&csv, cli_args.separator);
		let postfix = Postfix::new(&mut sheet);
		let output = postfix.process_sheet(csv.capacity());

		if let Some(out_path) = cli_args.out_path {
			let mut out_path = out_path.to_owned();
			if out_path.is_dir() {
				out_path.push("postfix_out.csv");
			}

			if out_path.exists() && !cli_args.overwrite {
				eprintln!("Path for output file already exists {out_path:?}");
				process::exit(1);
			} else {
				match write_from_string(out_path.clone(), output) {
					Ok(()) => println!("File successfully written to {out_path:?}\nTime: {:#?}", time.elapsed()),
					Err(error) => {
						eprintln!("Failed to write file to {out_path:?}.\n{error}");
						process::exit(1);
					},
				}
			}
		} else {
			println!("{}", output);
		}
	}
	Ok(())
}
