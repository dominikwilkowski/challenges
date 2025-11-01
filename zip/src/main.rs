mod network;
mod password;
mod unzip;

use zip::result::ZipError;

use crate::{
	network::{get_zip_url, load_zip, send_secret},
	password::Passwords,
	unzip::check_password_on_entry,
};

use std::time::Instant;

fn main() {
	let time = Instant::now();
	let token = "xxx"; // TODO: change me to your token!
	let zip_url = get_zip_url(token);
	let mut archive = load_zip(zip_url);

	for pwd in Passwords::new(4, 6) {
		match check_password_on_entry(&mut archive, &pwd) {
			Err(ZipError::InvalidPassword) => {},
			Err(_error) => {},
			Ok(secret) => {
				println!("✅ pass={pwd} secret={}", secret.trim());
				send_secret(secret.trim().to_string(), token);
				let duration = time.elapsed();
				println!("Time taken: {} seconds", duration.as_secs());
				break;
			},
		}
	}
}
