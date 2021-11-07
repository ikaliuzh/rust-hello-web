use super::router::Router;
use http::httprequest::HttpRequest;
use std::net::{TcpListener, TcpStream, Shutdown};
use::std::thread;
use std::io::prelude::*;
use std::str;

pub struct Server<'a> {
    socket_addr: &'a str,
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 256];
    
    match stream.read(&mut buffer) {
        Ok(_) => {
            let req: HttpRequest = String::from_utf8(buffer.to_vec())
                .expect("Failed to convert to string").into();
            Router::route(req, &mut stream);
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}",
                stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
        }
    }
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
            match stream {
                Ok(stream) => {
                    println!("New connection: {}", stream.peer_addr().unwrap());
                    thread::spawn(move||{
                        handle_client(stream)
                    });
                },
                Err(e) => {
                    println!("Error: {}", e);
                },
            }
        }

        // drop(conn_listener);
    }
}