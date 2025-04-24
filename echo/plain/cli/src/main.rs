use std::io::{Read, Write};
use std::net::TcpStream;

const ADDR: &str = "127.0.0.1:8279";
// 클라이언트 코드
fn main() {
    let mut stream = TcpStream::connect(ADDR).unwrap();
    let message = b"hello";
    stream.write_all(message).unwrap();
    stream.flush().unwrap();
    println!("send: {}", String::from_utf8_lossy(message));

    let mut buffer = [0; 1024];
    match stream.read(&mut buffer) {
        Ok(n) => println!("recv: {}", String::from_utf8_lossy(&buffer[..n])),
        Err(e) => println!("error: {}", e),
    }
}
