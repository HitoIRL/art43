use std::{net::TcpStream, io::{Write, stdout, stdin}, thread, sync::mpsc};

use shared::{Buffers, Packet, PacketKind};

fn main() {
    let (tx, rx) = mpsc::channel::<Packet>();
    let stream = TcpStream::connect(shared::ADDRESS).unwrap();
    let mut buffers = Buffers::new(stream);

    thread::spawn(move || {
        for received in rx {
            buffers.send_packet(received);
        }
    });

    print!("Enter your name: ");
    stdout().flush().unwrap();
    let mut name = String::new();
    stdin().read_line(&mut name).unwrap();
    let name_packet = Packet::new(PacketKind::SetName, name);
    tx.send(name_packet).unwrap();

    loop {
        print!("Enter a message: ");
        stdout().flush().unwrap();
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        let msg_packet = Packet::new(PacketKind::Message, line);
        tx.send(msg_packet).unwrap();
    }
}
