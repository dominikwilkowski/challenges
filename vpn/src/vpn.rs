use std::process::Command;

pub fn connect(id: &str) {
	let status = Command::new("vpnutil").args(["start", id]).status().unwrap();

	if !status.success() {
		panic!("ls exited with {}", status);
	}
}

pub fn disconnect(id: &str) {
	let status = Command::new("vpnutil").args(["stop", id]).status().unwrap();

	if !status.success() {
		panic!("ls exited with {}", status);
	}
}
