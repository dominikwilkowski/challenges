use std::path::PathBuf;

mod recursion;
mod worker;

pub enum Error {
	FailedToReadDirectory,
	FailedToReadFile,
}

fn main() {
	if let Ok(files) = recursion::get_files_via_recursion(PathBuf::from("./")) {
		println!("{files:#?}");
	}

	if let Ok(files) = worker::get_files_via_worker(PathBuf::from("./")) {
		println!("{files:#?}");
	}
}
