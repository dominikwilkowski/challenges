use chrono::DateTime;
use serde::{Deserialize, Serialize};

use std::{
	collections::HashMap,
	process::{Command, Stdio},
};

pub fn convert_to_json() -> String {
	let status = Command::new("rdb")
		.args(["-c", "json", "-o", "temp/dump.json", "temp/dump.rdb"])
		.stderr(Stdio::null())
		.status()
		.unwrap();

	if !status.success() {
		panic!("rdb exited with status {status}");
	}

	std::fs::read_to_string("temp/dump.json").unwrap()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RdbJson {
	db: u8,
	key: String,
	#[serde(default, rename = "type")]
	_type: String,
	#[serde(default)]
	expiration: String,
	#[serde(default)]
	value: String,
}

#[derive(Debug)]
pub struct ApiResp {
	pub db_count: usize,
	pub emoji_key_value: String,
	pub expiry_millis: i64,
	pub checked_key: String,
}

pub fn get_solution(json: String, check_type_of: String) -> ApiResp {
	let rdb_json: Vec<RdbJson> = serde_json::from_str(&json).unwrap();

	let mut db_count = HashMap::new();
	let mut emoji_key_value = String::new();
	let mut timestamp = String::new();
	let mut checked_key = String::new();

	for db in rdb_json {
		*db_count.entry(db.db).or_insert(0) += 1;
		if !db.key.chars().next().unwrap().is_ascii() {
			emoji_key_value = db.value.clone();
		}
		if !db.expiration.is_empty() {
			timestamp = db.expiration;
		}
		if db.key == check_type_of {
			checked_key = db._type;
		}
	}

	let mut expiry_millis = 0;
	if let Ok(dt) = DateTime::parse_from_rfc3339(&timestamp) {
		expiry_millis = dt.timestamp_millis();
	}

	ApiResp {
		db_count: db_count.len(),
		emoji_key_value,
		expiry_millis,
		checked_key,
	}
}
