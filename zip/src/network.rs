use serde::{Deserialize, Serialize};
use zip::ZipArchive;

use std::io::Cursor;

#[derive(Deserialize, Debug)]
struct ApiResp {
	zip_url: String,
}

pub fn get_zip_url(token: &str) -> String {
	let url = format!("https://hackattic.com/challenges/brute_force_zip/problem?access_token={token}");
	let mut resp = ureq::get(url).call().unwrap();
	let data = resp.body_mut().read_json::<ApiResp>().unwrap();
	data.zip_url
}

pub fn load_zip(url: String) -> ZipArchive<Cursor<Vec<u8>>> {
	let mut resp = ureq::get(url).call().unwrap();
	let bytes = resp.body_mut().with_config().limit(5 * 1024 * 1024).read_to_vec().unwrap();
	let cursor = Cursor::new(bytes);
	ZipArchive::new(cursor).unwrap()
}

#[derive(Serialize, Debug)]
struct ApiPayload {
	secret: String,
}

pub fn send_secret(secret: String, token: &str) {
	let url = format!("https://hackattic.com/challenges/brute_force_zip/solve?access_token={token}");
	let mut res = ureq::post(url).send_json(&ApiPayload { secret }).unwrap();
	println!("{}", res.body_mut().read_to_string().unwrap());
}
