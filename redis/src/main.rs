use std::{fs::File, io::prelude::*};

mod network;
mod rdb;

fn main() {
	let token = "xxx"; // TODO: change me to your token!
	let (data, requirements) = crate::network::get_data(token);

	let mut buffer = File::create("temp/dump.rdb").unwrap();
	let _ = buffer.write(&data);
	let json = crate::rdb::convert_to_json();
	let solution = crate::rdb::get_solution(json, requirements.check_type_of.clone());
	let payload = format!(
		"{{\"db_count\":{},\"emoji_key_value\":\"{}\", \"expiry_millis\":{},\"{}\":\"{}\"}}",
		solution.db_count,
		solution.emoji_key_value,
		solution.expiry_millis,
		requirements.check_type_of,
		solution.checked_key
	);
	println!("{payload}");
	crate::network::send_secret(payload, token);
}
