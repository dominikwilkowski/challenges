use base64::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Requirements {
	check_type_of: String,
}

#[derive(Deserialize, Debug)]
pub struct ApiResp {
	rdb: String,
	requirements: Requirements,
}

pub fn get_data(token: &str) -> (Vec<u8>, Requirements) {
	let url = format!("https://hackattic.com/challenges/the_redis_one/problem?access_token={token}");
	let mut resp = ureq::get(url).call().unwrap();
	let mut data = resp.body_mut().read_json::<ApiResp>().unwrap();
	let rdb = BASE64_STANDARD.decode(data.rdb).unwrap();
	// data.rdb = String::from_utf8_lossy(&rdb).to_string().replace("mysql0009", "REDIS0009");

	(rdb, data.requirements)
}
