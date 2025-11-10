use serde::{Deserialize, Serialize};

use std::{
	fs::{File, create_dir_all},
	io::{self, Write},
};

#[derive(Deserialize, Debug)]
pub struct ApiResp {
	image_url: String,
}

pub fn download_img(token: &str) -> String {
	let url = format!("https://hackattic.com/challenges/visual_basic_math/problem?access_token={token}");
	let mut resp = ureq::get(url).call().unwrap();
	let data = resp.body_mut().read_json::<ApiResp>().unwrap();

	create_dir_all("temp").unwrap();
	let img_path = String::from("temp/img.png");

	let mut wav_resp = ureq::get(&data.image_url).call().unwrap();
	let mut body = wav_resp.body_mut().with_config().limit(20 * 1024 * 1024).reader();
	let mut out = File::create(&img_path).unwrap();
	io::copy(&mut body, &mut out).unwrap();
	out.flush().unwrap();

	img_path
}

#[derive(Serialize, Debug)]
pub struct ApiSolve {
	result: i64,
}

pub fn send_secret(result: i64, token: &str) {
	let url = format!("https://hackattic.com/challenges/visual_basic_math/solve?access_token={token}");
	let mut res = ureq::post(url).send_json(&ApiSolve { result }).unwrap();
	println!("{}", res.body_mut().read_to_string().unwrap());
}
