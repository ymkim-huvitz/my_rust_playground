use std::io::{self, Read, Write};
use std::net::TcpStream;

const ADDR: &str = "127.0.0.1:18279";

fn main() {
    let mut stream = TcpStream::connect(ADDR).unwrap();
    println!("Connected to the server. Type your message and press Enter. Type 'quit' to exit.");

    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        input.clear();
        stdin.read_line(&mut input).unwrap();
        let message = input.trim();

        if message == "quit" {
            println!("Exiting...");
            break;
        }

        stream.write_all(message.as_bytes()).unwrap();
        stream.flush().unwrap();
        println!("send: {}", message);

        let mut buffer = [0; 1024];
        match stream.read(&mut buffer) {
            Ok(n) => println!("recv: {}", String::from_utf8_lossy(&buffer[..n])),
            Err(e) => println!("error: {}", e),
        }
    }
}
