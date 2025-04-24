use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

const ADDR: &str = "0.0.0.0:8279";

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(n) if n > 0 => {
            println!("recv & echo: {}", String::from_utf8_lossy(&buffer[..n]));
            stream.write_all(&buffer[..n]).unwrap();
            stream.flush().unwrap();
        }
        _ => println!("error: failed to read from stream"),
    }
}

fn main() {
    let listener = TcpListener::bind(ADDR).unwrap();
    println!("server listening on {ADDR}");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => println!("error: {}", e),
        }
    }
}
