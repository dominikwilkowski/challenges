use std::fs;

use crate::network::{Record, RecordType};

pub fn generate_zone_file(records: Vec<Record>) {
	let mut zone_file = String::new();

	let domain = records.iter().find(|r| r.record_type == RecordType::AAAA).map(|r| r.name.clone()).unwrap();
	zone_file.push_str(&format!(
		r#"$ORIGIN {domain}.
$TTL 60

@   IN  SOA ns1.{domain}. hostmaster.{domain}. (
        1   ; serial
        60  ; refresh
        60  ; retry
        60  ; expire
        60  ; minimum
)
    IN  NS  ns1.{domain}.

"#
	));

	for record in records {
		match record.record_type {
			RecordType::A => {
				zone_file.push_str(&format!("@   IN  A     {}\n", record.data));
			},
			RecordType::AAAA => {
				zone_file.push_str(&format!("@   IN  AAAA  {}\n", record.data));
			},
			RecordType::RP => {
				zone_file.push_str(&format!("@   IN  RP    {}.  .\n", record.data));
			},
			RecordType::TXT => {
				zone_file.push_str(&format!("*   IN  TXT   \"{}\"\n", record.data));
			},
		}
	}

	let _ = fs::write(format!("temp/{domain}"), zone_file);
}
