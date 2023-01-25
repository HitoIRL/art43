use std::{net::{TcpListener, TcpStream}, thread};

use shared::Buffers;

fn handle_client(stream: TcpStream) { // todo: keep client connected
    println!("New client: {}", stream.peer_addr().unwrap());
    let mut buffers = Buffers::new(stream);
    let message = buffers.read_message();
    println!("Received message: {}", message);
    //buffers.send_message(&message);
}

fn main() {
    match TcpListener::bind(shared::ADDRESS) {
        Ok(listener) => {
            for stream in listener.incoming() {
                handle_client(stream.unwrap())
            }
        },
        Err(e) => println!("Failed to bind: {}", e)
    };
}
