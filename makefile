all: server client

server: src/server.rs
	rustc src/server.rs --out-dir target && ./target/server

client: src/client.rs
	rustc src/client.rs --out-dir target && ./target/client
