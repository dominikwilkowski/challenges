use serde::{Deserialize, Serialize};

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

pub fn ping_server(token: &str, region: &str) {
	let url = format!("https://hackattic.com/_/presence/{token}");
	if let Ok(mut resp) = ureq::get(url).call() {
		let body = resp.body_mut().read_to_string().unwrap();
		if body.contains(region) {
			println!("{body}");
		} else {
			ping_server(token, region);
		}
	} else {
		ping_server(token, region);
	}
}

#[derive(Serialize, Debug)]
pub struct ApiSolve {}

pub fn solve(token: &str) {
	let url = format!("https://hackattic.com/challenges/a_global_presence/solve?access_token={token}");
	let mut res = ureq::post(url).send_json(&ApiSolve {}).unwrap();
	println!("{}", res.body_mut().read_to_string().unwrap());
}
