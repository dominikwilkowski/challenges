mod network;
mod vpn;

use std::time::Instant;

pub const VPNS: [&str; 5] = [
	// "nord_US",
	"nord_NZ", "nord_SE", "nord_SG", "nord_IE", "nord_DE",
	// "nord_SG",
];

fn main() {
	let time = Instant::now();
	let token = "xxx"; // TODO: change me to your token!

	let presence_token = crate::network::get_token(token);

	crate::network::ping_server(&presence_token, "US");
	crate::vpn::disconnect("nord_US");

	for vpn in VPNS {
		crate::vpn::connect(vpn);
		crate::network::ping_server(&presence_token, vpn[5..].as_ref());
		crate::vpn::disconnect(vpn);
	}

	// crate::network::ping_server(&presence_token, "AU");

	crate::network::solve(token);
	println!("Time taken: {} seconds", time.elapsed().as_secs());
}
