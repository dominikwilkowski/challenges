mod coredns;
mod network;
mod zone;

fn main() {
	let token = "xxx"; // TODO: change me to your token!
	let dns_ip = String::from("xxx");
	let dns_port = 55353;

	let records = crate::network::download_records(token);
	crate::zone::generate_zone_file(records);

	let _dns = crate::coredns::CoreDns::start("Corefile");
	std::thread::sleep(std::time::Duration::from_millis(200));
	crate::network::send_secret(dns_ip, dns_port, token);
}
