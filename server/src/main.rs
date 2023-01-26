use std::{net::{TcpListener, TcpStream}, thread, io::ErrorKind};

use shared::Buffers;

fn handle_client(stream: TcpStream) { // todo: keep client connected
    let mut buffers = Buffers::new(stream);
    loop {
        match buffers.read_message() {
            Ok(message) => {
                println!("Received message: {}", message);
            }
            Err(e) => {
                if e.kind() != ErrorKind::ConnectionReset {
                    println!("Failed to read message: {}", e);
                }
                break;
            }
        }
    }
}

fn main() {
    match TcpListener::bind(shared::ADDRESS) {
        Ok(listener) => {
            for stream in listener.incoming() {
                thread::spawn(|| { // spawning new thread per connection is very inefficient but for now it's ok
                    handle_client(stream.unwrap())
                });
            }
        },
        Err(e) => println!("Failed to bind: {}", e)
    };
}
