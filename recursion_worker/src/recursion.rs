use std::path::PathBuf;

pub fn get_files_via_recursion(folder: PathBuf) -> Result<Vec<PathBuf>, crate::Error> {
	let mut files = Vec::new();

	if folder.is_dir() {
		for item in folder.read_dir().map_err(|_| crate::Error::FailedToReadDirectory)? {
			let item = item.map_err(|_| crate::Error::FailedToReadFile)?;
			let path = item.path();

			if path.is_file() {
				files.push(path);
			} else if path.is_dir() {
				files.extend(get_files_via_recursion(path)?);
			}
		}
	}

	Ok(files)
}
