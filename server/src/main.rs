use std::{
	io::prelude::*,
	net::{TcpListener, TcpStream},
};

fn handle_connection(mut stream: TcpStream) {
	let response = "HTTP/1.1 200 OK\n\
		Content-Length: 13\n\
		Content-Type: text/plain\n\
		\n\
		Hello, world!";

	let _ = stream.write_all(response.as_bytes());
}

fn main() {
	let port = 7878;
	let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();

	println!("Listening on http://127.0.0.1:{port}");

	for stream in listener.incoming() {
		let stream = stream.unwrap();
		handle_connection(stream);
	}
}
