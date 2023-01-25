use std::{net::TcpStream, io::{Write, Read, BufReader, BufRead}};

use shared::Buffers;

fn main() {
    let stream = TcpStream::connect(shared::ADDRESS).unwrap();

    let mut buffers = Buffers::new(stream.try_clone().unwrap());
    buffers.send_message("hello world from client");
    let response = buffers.read_message();
    println!("Received response: {}", response);
}
