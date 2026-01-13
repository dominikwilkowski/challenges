mod network;
mod zone;

fn main() {
	let token = "xxx"; // TODO: change me to your token!

	let records = crate::network::download_records(token);
	crate::zone::generate_zone_file(records);

	// TODO:
	// get IP + port
	// convert records to zone file
	// configure coreDNS

	// crate::network::send_secret(dns_ip, dns_port, token);
}
