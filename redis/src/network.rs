use base64::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Requirements {
	pub check_type_of: String,
}

#[derive(Deserialize, Debug)]
pub struct ApiResp {
	pub rdb: String,
	pub requirements: Requirements,
}

pub fn get_data(token: &str) -> (Vec<u8>, Requirements) {
	let url = format!("https://hackattic.com/challenges/the_redis_one/problem?access_token={token}");
	let mut resp = ureq::get(url).call().unwrap();
	let data = resp.body_mut().read_json::<ApiResp>().unwrap();
	let mut rdb = BASE64_STANDARD.decode(data.rdb).unwrap();

	let head = rdb.get_mut(..5).expect("RDB blob too short");
	if head != b"REDIS" {
		head.copy_from_slice(b"REDIS");
	}

	(rdb, data.requirements)
}

pub fn send_secret(payload: String, token: &str) {
	let url = format!("https://hackattic.com/challenges/the_redis_one/solve?access_token={token}");
	let mut res = ureq::post(url).header("Content-Type", "application/json").send(payload).unwrap();
	println!("{}", res.body_mut().read_to_string().unwrap());
}
