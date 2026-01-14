use std::{
	io,
	process::{Child, Command, Stdio},
};

pub struct CoreDns {
	child: Child,
}

impl CoreDns {
	pub fn start(corefile_path: &str) -> io::Result<Self> {
		let child = Command::new("coredns")
			.args(["-conf", corefile_path])
			.stdin(Stdio::null())
			.stdout(Stdio::inherit())
			.stderr(Stdio::inherit())
			.current_dir("/Volumes/Macintosh HD/Users/dominik/Sites/challenges/dns/")
			.spawn()?;

		Ok(Self { child })
	}

	pub fn stop(&mut self) {
		let _ = self.child.kill(); // SIGKILL on Unix
		let _ = self.child.wait();
	}
}

impl Drop for CoreDns {
	fn drop(&mut self) {
		self.stop();
	}
}
