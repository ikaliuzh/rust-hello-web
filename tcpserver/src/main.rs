use std::net::TcpListener;

fn main() {
    let conn_listener = TcpListener::bind("localhost:3000").unwrap();
    println!("Running on port 3000");
    for stream in conn_listener.incoming() {
        let _stream = stream.unwrap();
        println!("Connection established!");
    }
}
