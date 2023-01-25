use std::{net::TcpStream, io::Write, thread, sync::mpsc};

use shared::Buffers;

fn main() {
    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        for received in rx {
            let stream = TcpStream::connect(shared::ADDRESS).unwrap();
            let mut buffers = Buffers::new(stream.try_clone().unwrap());
            buffers.send_message(&received);
        }
    });

    loop {
        print!("Enter a message: ");
        std::io::stdout().flush().unwrap();
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        tx.send(line).unwrap();
    }
}
