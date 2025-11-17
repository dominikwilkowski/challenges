use serde::{Deserialize, Serialize};
use ureq::{Agent, Proxy};

use std::{
	sync::atomic::{AtomicBool, Ordering},
	time::Duration,
};

#[derive(Deserialize, Debug)]
pub struct ApiResp {
	presence_token: String,
}

pub fn get_token(token: &str) -> String {
	let url = format!("https://hackattic.com/challenges/a_global_presence/problem?access_token={token}");
	let mut resp = ureq::get(url).call().unwrap();
	let data = resp.body_mut().read_json::<ApiResp>().unwrap();

	data.presence_token
}

pub fn ping_server(token: &str) {
	let url = format!("https://hackattic.com/_/presence/{token}");
	let mut resp = ureq::get(url).call().unwrap();
	let body = resp.body_mut().read_to_string().unwrap();
	println!("{body}");
}

pub fn ping_all(token: &str, proxies: Vec<String>, stop: &AtomicBool) {
	for proxy in proxies {
		if stop.load(Ordering::Relaxed) {
			println!("Overshot timeout; stopping");
			break;
		}

		let proxy = Proxy::new(&format!("http://{proxy}")).unwrap();

		let agent: Agent =
			Agent::config_builder().proxy(Some(proxy)).timeout_connect(Some(Duration::from_secs(5))).build().into();

		let url = format!("https://hackattic.com/_/presence/{token}");
		match agent.get(url).call() {
			Ok(mut resp) => {
				let body = resp.body_mut().read_to_string().unwrap();
				println!("{body}");
			},
			Err(_error) => {
				// println!("Failed request: {error}");
			},
		}
	}
}

#[derive(Serialize, Debug)]
pub struct ApiSolve {}

pub fn solve(token: &str) {
	let url = format!("https://hackattic.com/challenges/a_global_presence/solve?access_token={token}");
	let mut res = ureq::post(url).send_json(&ApiSolve {}).unwrap();
	println!("{}", res.body_mut().read_to_string().unwrap());
}
