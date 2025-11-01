use zip::{ZipArchive, result::ZipError};

use std::io::{Cursor, copy};

pub fn check_password_on_entry(archive: &mut ZipArchive<Cursor<Vec<u8>>>, password: &str) -> Result<String, ZipError> {
	let inner_path = "secret.txt";
	let mut entry = archive.by_name_decrypt(inner_path, password.as_bytes())?;

	let mut sink = Vec::new();
	copy(&mut entry, &mut sink).map_err(ZipError::Io)?;
	Ok(String::from_utf8(sink).unwrap())
}
