use std::{io::{BufReader, Write, BufRead, LineWriter, Result}, net::TcpStream};

pub const ADDRESS: &str = "127.0.0.1:39225";

pub struct Buffers {
    pub reader: BufReader<TcpStream>,
    pub writer: LineWriter<TcpStream>,
}

impl Buffers {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            reader: BufReader::new(stream.try_clone().unwrap()),
            writer: LineWriter::new(stream),
        }
    }

    pub fn read_message(&mut self) -> Result<String> {
        let mut line = String::new();
        self.reader.read_line(&mut line)?;
        line.pop();
        Ok(line)
    }

    pub fn send_message(&mut self, message: &str) {
        self.writer.write(message.as_bytes()).unwrap();
        self.writer.flush().unwrap();
    }
}
