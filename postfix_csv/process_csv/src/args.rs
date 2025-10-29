use std::{env, path::PathBuf, process};

#[derive(Debug, PartialEq)]
pub struct Args {
	pub csv_path: PathBuf,
	pub separator: char,
	pub out_path: Option<PathBuf>,
	pub overwrite: bool,
	pub help: bool,
	pub version: bool,
}

impl Args {
	pub fn parse(args: Vec<String>) -> Self {
		let mut flags = Self {
			csv_path: PathBuf::new(),
			separator: ',',
			out_path: None,
			overwrite: false,
			help: false,
			version: false,
		};

		let mut iter = args.iter().peekable();
		while let Some(arg) = iter.next() {
			match arg.as_str() {
				"-v" | "-V" | "--version" => {
					flags.version = true;
				},
				"-h" | "--help" => {
					flags.help = true;
				},
				"-x" | "--overwrite" => {
					flags.overwrite = true;
				},
				"-s" | "--separator" => {
					if let Some(next_arg) = iter.next() {
						flags.separator = next_arg.chars().next().unwrap_or(',');
					} else {
						eprintln!("The flag `--separator` is missing its argument to specifiy what the separator is");
						process::exit(1);
					}
				},
				"-o" | "--out" => {
					if let Some(next_arg) = iter.next() {
						let mut out_path = PathBuf::new();
						out_path.push(next_arg);
						flags.out_path = Some(out_path);
					} else {
						eprintln!("The flag `--out` is missing its argument to specifiy where to save the output to");
						process::exit(1);
					}
				},
				_ => {
					if !arg.starts_with('-') {
						flags.csv_path.push(arg)
					}
				},
			}
		}

		if !(flags.help || flags.version) && flags.csv_path.as_path().components().count() == 0 {
			eprintln!("Please specifiy the path to the csv to be parsed");
			process::exit(1);
		}

		flags
	}

	pub fn help() -> String {
		format!("{name} v{version}\n\nUsage: $ {name} [path/to/csv] [OPTIONS]\n\nOptions:\n  -V, --version        Print version info and exit\n  -h, --help           Print help and exit\n  -s, --separator \",\"  Set the separator for the CSV\n  -x, --overwrite      Allow to overwrite the output file if it exists\n", name = env!("CARGO_PKG_NAME"), version = env!("CARGO_PKG_VERSION"))
	}

	pub fn version() -> String {
		format!("v{}", env!("CARGO_PKG_VERSION"))
	}
}

#[test]
fn parse_test() {
	assert_eq!(
		Args::parse(vec![String::from("path/to/somehwere")]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere"),
			separator: ',',
			out_path: None,
			overwrite: false,
			help: false,
			version: false,
		}
	);
	assert_eq!(
		Args::parse(vec![String::from("path/to/somehwere"), String::from("-v")]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere"),
			separator: ',',
			out_path: None,
			overwrite: false,
			help: false,
			version: true,
		}
	);
	assert_eq!(
		Args::parse(vec![String::from("path/to/somehwere"), String::from("-V")]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere"),
			separator: ',',
			out_path: None,
			overwrite: false,
			help: false,
			version: true,
		}
	);
	assert_eq!(
		Args::parse(vec![String::from("path/to/somehwere"), String::from("--version")]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere"),
			separator: ',',
			out_path: None,
			overwrite: false,
			help: false,
			version: true,
		}
	);
	assert_eq!(
		Args::parse(vec![String::from("path/to/somehwere"), String::from("-h")]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere"),
			separator: ',',
			out_path: None,
			overwrite: false,
			help: true,
			version: false,
		}
	);
	assert_eq!(
		Args::parse(vec![String::from("path/to/somehwere"), String::from("--help")]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere"),
			separator: ',',
			out_path: None,
			overwrite: false,
			help: true,
			version: false,
		}
	);
	assert_eq!(
		Args::parse(vec![String::from("path/to/somehwere"), String::from("-x")]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere"),
			separator: ',',
			out_path: None,
			overwrite: true,
			help: false,
			version: false,
		}
	);
	assert_eq!(
		Args::parse(vec![String::from("path/to/somehwere"), String::from("--overwrite")]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere"),
			separator: ',',
			out_path: None,
			overwrite: true,
			help: false,
			version: false,
		}
	);
	assert_eq!(
		Args::parse(vec![String::from("path/to/somehwere"), String::from("--unknown")]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere"),
			separator: ',',
			out_path: None,
			overwrite: false,
			help: false,
			version: false,
		}
	);
	assert_eq!(
		Args::parse(vec![String::from("path/to/somehwere"), String::from("-z")]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere"),
			separator: ',',
			out_path: None,
			overwrite: false,
			help: false,
			version: false,
		}
	);
	assert_eq!(
		Args::parse(vec![String::from("path/to/somehwere"), String::from("-s"), String::from(".")]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere"),
			separator: '.',
			out_path: None,
			overwrite: false,
			help: false,
			version: false,
		}
	);
	assert_eq!(
		Args::parse(vec![
			String::from("path/to/somehwere"),
			String::from("--separator"),
			String::from(".")
		]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere"),
			separator: '.',
			out_path: None,
			overwrite: false,
			help: false,
			version: false,
		}
	);
	assert_eq!(
		Args::parse(vec![
			String::from("path/to/somehwere"),
			String::from("-o"),
			String::from("path/to")
		]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere"),
			separator: ',',
			out_path: Some(PathBuf::from("path/to")),
			overwrite: false,
			help: false,
			version: false,
		}
	);
	assert_eq!(
		Args::parse(vec![
			String::from("path/to/somehwere"),
			String::from("--out"),
			String::from("path/to")
		]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere"),
			separator: ',',
			out_path: Some(PathBuf::from("path/to")),
			overwrite: false,
			help: false,
			version: false,
		}
	);
	assert_eq!(
		Args::parse(vec![
			String::from("-v"),
			String::from("path/to/somehwere"),
			String::from("-s"),
			String::from("…"),
			String::from("-o"),
			String::from("path/to"),
			String::from("-x"),
			String::from("-w"),
			String::from("-h"),
		]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere"),
			separator: '…',
			out_path: Some(PathBuf::from("path/to")),
			overwrite: true,
			help: true,
			version: true,
		}
	);
	assert_eq!(
		Args::parse(vec![
			String::from("-o"),
			String::from("path/to"),
			String::from("-v"),
			String::from("-x"),
			String::from("-s"),
			String::from("…"),
			String::from("-h"),
			String::from("-w"),
			String::from("path/to/somehwere")
		]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere"),
			separator: '…',
			out_path: Some(PathBuf::from("path/to")),
			overwrite: true,
			help: true,
			version: true,
		}
	);
	assert_eq!(
		Args::parse(vec![
			String::from("path/to/somehwere"),
			String::from("--version"),
			String::from("--out"),
			String::from("path/to"),
			String::from("--help"),
			String::from("--overwrite"),
			String::from("--separator"),
			String::from("…"),
			String::from("--windows"),
			String::from("path/to/elsewhere")
		]),
		Args {
			csv_path: PathBuf::from("path/to/somehwere/path/to/elsewhere"),
			separator: '…',
			out_path: Some(PathBuf::from("path/to")),
			overwrite: true,
			help: true,
			version: true,
		}
	);
}

#[test]
fn parse_test_failiure() {
	let output = std::process::Command::new("cargo").args(&["run", "--"]).output().unwrap();
	assert!(!output.status.success());
	assert!(String::from_utf8_lossy(&output.stderr).contains("Please specifiy the path to the csv to be parsed"));
	assert!(String::from_utf8_lossy(&output.stdout).is_empty());

	let output = std::process::Command::new("cargo").args(&["run", "--", "path/to/somehwere", "-s"]).output().unwrap();
	assert!(!output.status.success());
	assert!(String::from_utf8_lossy(&output.stderr)
		.contains("The flag `--separator` is missing its argument to specifiy what the separator is"));
	assert!(String::from_utf8_lossy(&output.stdout).is_empty());

	let output = std::process::Command::new("cargo").args(&["run", "--", "path/to/somehwere", "-o"]).output().unwrap();
	assert!(!output.status.success());
	assert!(String::from_utf8_lossy(&output.stderr)
		.contains("The flag `--out` is missing its argument to specifiy where to save the output to"));
	assert!(String::from_utf8_lossy(&output.stdout).is_empty());
}
