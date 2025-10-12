use std::{env, fs, path::PathBuf};

const MAX_FIB_NUMBER: u64 = 1_000_000;

fn main() {
	let mut fibs = vec![1u64, 1];

	loop {
		let next = fibs[fibs.len() - 1] + fibs[fibs.len() - 2];
		if next > MAX_FIB_NUMBER {
			break;
		}
		fibs.push(next);
	}

	fibs.dedup();

	let content = format!("pub const FIBONACCI_NUMBERS: [u64; {}] = {:?};\n", fibs.len(), fibs);

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("fibonacci_data.rs");
	fs::write(out_path, content).unwrap();
	println!("cargo:rerun-if-env-changed=FORCE_REBUILD");
}
