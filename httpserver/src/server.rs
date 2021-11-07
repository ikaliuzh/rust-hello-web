use super::router::Router;
use http::httprequest::HttpRequest;
use std::net::{TcpListener};
use std::io::prelude::*;
use std::str;

pub struct Server<'a> {
	socket_addr: &'a str,
}

impl<'a> Server<'a> {
	pub fn new(socket_addr: &'a str) -> Self {
		Server { socket_addr }
	}

	pub fn run(&self) {
		let conn_listener = TcpListener::bind(self.socket_addr)
			.expect(&format!("Failed to connect to socket {}", self.socket_addr));
		println!("Running on {}", self.socket_addr);

		for stream in conn_listener.incoming() {
			let mut stream = stream.expect("Failed to connect");
			println!("Connection established");

			let mut buffer = [0; 256];
			stream.read(&mut buffer).expect("Failed to read");

			let req: HttpRequest = String::from_utf8(buffer.to_vec())
				.expect("Failed to convert to string").into();
			Router::route(req, &mut stream);
		}
	}
}