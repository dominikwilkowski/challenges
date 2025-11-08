use serde::{Deserialize, Serialize};

use std::{
	fs::{File, create_dir_all},
	io::{self, Write},
};

#[derive(Deserialize, Debug)]
pub struct ApiResp {
	wav_url: String,
}

pub fn download_wav(token: &str) {
	let url = format!("https://hackattic.com/challenges/touch_tone_dialing/problem?access_token={token}");
	let mut resp = ureq::get(url).call().unwrap();
	let data = resp.body_mut().read_json::<ApiResp>().unwrap();

	create_dir_all("temp").unwrap();

	let mut wav_resp = ureq::get(&data.wav_url).call().unwrap();
	let mut body = wav_resp.body_mut().with_config().limit(20 * 1024 * 1024).reader();
	let mut out = File::create("temp/tone.wav").unwrap();
	io::copy(&mut body, &mut out).unwrap();
	out.flush().unwrap();
}

#[derive(Serialize, Debug)]
pub struct ApiSolve {
	sequence: String,
}

pub fn send_secret(sequence: String, token: &str) {
	let url = format!("https://hackattic.com/challenges/touch_tone_dialing/solve?access_token={token}");
	let mut res = ureq::post(url).send_json(&ApiSolve { sequence }).unwrap();
	println!("{}", res.body_mut().read_to_string().unwrap());
}
