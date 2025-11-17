mod network;
mod proxy;

use std::{
	sync::{
		Arc,
		atomic::{AtomicBool, Ordering},
	},
	thread,
	time::{Duration, Instant},
};

pub const COUNTRIES: [&str; 19] = [
	"jp", "nl", "vn", "gb", "sg", "ca", "hf", "all", "all", "all", "hg", "hh", "hi", "hj", "all", "in", "de", "all", "oj",
];

fn main() {
	let time = Instant::now();
	let token = "xxx"; // TODO: change me to your token!
	let mut handles = Vec::new();
	let stop = Arc::new(AtomicBool::new(false));
	let proxies = crate::proxy::get("us");
	let presence_token = crate::network::get_token(token);

	// solve before timeout expires
	{
		let stop_clone = Arc::clone(&stop);

		let handle = thread::spawn(move || {
			thread::sleep(Duration::from_secs(29));

			stop_clone.swap(true, Ordering::SeqCst);
			crate::network::solve(token);
			println!("Time taken: {} seconds", time.elapsed().as_secs());
		});
		handles.push(handle);
	}

	// AU ping
	crate::network::ping_server(&presence_token);

	// prefetched ping
	{
		let presence_token_clone = presence_token.clone();
		let proxies_clone = proxies.clone();
		let stop_clone = Arc::clone(&stop);

		let handle = thread::spawn(move || {
			crate::network::ping_all(&presence_token_clone, proxies_clone, &stop_clone);
		});
		handles.push(handle);
	}

	// all other countries
	for &country in &COUNTRIES {
		let presence_token_clone = presence_token.clone();
		let stop_clone = Arc::clone(&stop);

		let handle = thread::spawn(move || {
			let proxies = crate::proxy::get(country);
			crate::network::ping_all(&presence_token_clone, proxies, &stop_clone);
		});
		handles.push(handle);
	}

	for handle in handles {
		let _ = handle.join();
	}
}
