use std::net::{TcpStream};
use std::io::{Read, Write};
use std::io;
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            loop {
                let mut msg_input = String::new();
                io::stdin()
                    .read_line(&mut msg_input)
                    .expect("Failed to read line!");
                msg_input = msg_input.trim().to_owned();
                let msg = msg_input.as_bytes();
                
                stream.write(msg).unwrap();
                println!("Sent message \"{}\", awaiting reply...", msg_input.trim());

                let mut data = [0 as u8; 64];
                match stream.read(&mut data) {
                    Ok(_) => {
                        if &data[0..msg.len()] == msg{
                            println!("Reply is ok!");
                        } else {
                            let text = from_utf8(&data).unwrap();
                            println!("Unexpected reply: {}, {:?}, {:?}", text, data, msg);
                        }
                    },
                    Err(e) => {
                        println!("Failed to recieve data: {}", e);
                    }
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated!");
}
