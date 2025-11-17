pub fn get(country: &str) -> Vec<String> {
	let url = format!(
		"https://api.proxyscrape.com/v4/free-proxy-list/get?request=displayproxies&protocol=http&timeout=10000&country={country}&ssl=all&anonymity=all&skip=0&limit=200"
	);
	let mut resp = ureq::get(&url).call().unwrap();
	let text = resp.body_mut().read_to_string().unwrap();
	text.lines().map(|line| line.to_string()).collect()
}
