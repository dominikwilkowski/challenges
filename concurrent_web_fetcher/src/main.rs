use std::{env::args, time::Instant};

#[tokio::main]
async fn main() {
	let start_time = Instant::now();
	let urls = args().skip(1).collect::<Vec<String>>();
	let mut handles = Vec::new();

	for url in urls {
		handles.push(tokio::spawn(async move {
			let UrlResponds { url, time, code } = do_url(url);
			println!("{url}: {code} in {time}ms");
		}));
	}

	for handle in handles {
		let _ = handle.await;
	}

	println!("\nLookup has taken {}ms", start_time.elapsed().as_millis());
}

struct UrlResponds {
	url: String,
	time: u128,
	code: u16,
}

fn do_url(url: String) -> UrlResponds {
	let start_time = Instant::now();
	let code = match ureq::get(&url).call() {
		Ok(response) => response.status().as_u16(),
		Err(ureq::Error::StatusCode(code)) => code,
		Err(_) => 0,
	};

	UrlResponds {
		url,
		time: start_time.elapsed().as_millis(),
		code,
	}
}
