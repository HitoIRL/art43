use std::{net::{TcpListener, TcpStream}, thread, io::ErrorKind};

use shared::{Buffers, PacketKind};

#[derive(Debug)]
struct Client {
    name: Option<String>,
}

impl Client {
    fn new() -> Self {
        Self {
            name: None,
        }
    }

    fn handle_packets(&mut self, stream: TcpStream) {
        let mut buffers = Buffers::new(stream);
        loop {
            match buffers.fetch_packet() {
                Ok(packet) => {
                    match packet.kind {
                        PacketKind::SetName => self.name = Some(packet.data),
                        PacketKind::Message => println!("{}: {}", self.name.as_ref().unwrap(), packet.data),
                    }
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
}

struct Server {

}

impl Server {
    fn new() -> Self {
        Self {

        }
    }

    fn start(&mut self) {
        match TcpListener::bind(shared::ADDRESS) {
            Ok(listener) => {
                for stream in listener.incoming() {
                    let mut client = Client::new();
                    thread::spawn(move || {
                        client.handle_packets(stream.unwrap());
                    });
                }
            },
            Err(e) => println!("Failed to bind: {}", e)
        };
    }
}

fn main() {
    let mut server = Server::new();
    server.start();
}
