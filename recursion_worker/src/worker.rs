use std::path::PathBuf;

pub fn get_files_via_worker(folder: PathBuf) -> Result<Vec<PathBuf>, crate::Error> {
	let mut files = Vec::new();

	if folder.is_dir() {
		let mut folders = vec![folder];

		while let Some(folder) = folders.pop() {
			for item in folder.read_dir().map_err(|_| crate::Error::FailedToReadDirectory)? {
				let item = item.map_err(|_| crate::Error::FailedToReadFile)?;
				let path = item.path();

				if path.is_file() {
					files.push(path);
				} else if path.is_dir() {
					folders.push(path);
				}
			}
		}
	}

	Ok(files)
}
