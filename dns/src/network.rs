use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, PartialEq)]
pub enum RecordType {
	A,
	#[allow(clippy::upper_case_acronyms)]
	AAAA,
	RP,
	#[allow(clippy::upper_case_acronyms)]
	TXT,
}

#[derive(Deserialize, Debug)]
pub struct Record {
	pub name: String,
	#[serde(rename = "type")]
	pub record_type: RecordType,
	pub data: String,
}

#[derive(Deserialize, Debug)]
pub struct ApiResp {
	records: Vec<Record>,
}

pub fn download_records(token: &str) -> Vec<Record> {
	let url = format!("https://hackattic.com/challenges/serving_dns/problem?access_token={token}");
	let mut resp = ureq::get(url).call().unwrap();
	let data = resp.body_mut().read_json::<ApiResp>().unwrap();

	data.records
}

#[derive(Serialize, Debug)]
pub struct ApiSolve {
	dns_ip: String,
	dns_port: usize,
}

pub fn send_secret(dns_ip: String, dns_port: usize, token: &str) {
	let url = format!("https://hackattic.com/challenges/serving_dns/solve?access_token={token}");
	let mut res = ureq::post(url).send_json(&ApiSolve { dns_ip, dns_port }).unwrap();
	println!("{}", res.body_mut().read_to_string().unwrap());
}
