use std::{io::{BufReader, Write, Result, BufWriter, Read}, net::TcpStream};

use serde::{Serialize, Deserialize};

pub const ADDRESS: &str = "127.0.0.1:39225";

const MAX_PACKET_SIZE: usize = 1024;

pub struct Buffers {
    pub reader: BufReader<TcpStream>,
    pub writer: BufWriter<TcpStream>,
}

impl Buffers {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            reader: BufReader::new(stream.try_clone().unwrap()),
            writer: BufWriter::new(stream),
        }
    }

    pub fn fetch_packet(&mut self) -> Result<Packet> {
        let mut buffer = vec![0; MAX_PACKET_SIZE]; // todo: use dynamic sized vec
        self.reader.read(&mut buffer)?;
        Ok(bincode::deserialize(&buffer).unwrap())
    }

    pub fn send_packet(&mut self, packet: Packet) {
        let encoded = bincode::serialize(&packet).unwrap();
        self.writer.write(&encoded).unwrap();
        self.writer.flush().unwrap();
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PacketKind {
    SetName,
    Message,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Packet {
    pub kind: PacketKind,
    pub data: String,
}

impl Packet {
    pub fn new(kind: PacketKind, mut data: String) -> Self {
        if data.ends_with("\n") { // get rid of \r\n without cloning
            data.pop();
            if data.ends_with("\r") {
                data.pop();
            }
        }

        Self {
            kind,
            data,
        }
    }
}
