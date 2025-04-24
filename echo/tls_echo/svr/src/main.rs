use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::io::{Read, Write};
use std::net::TcpListener;

const SERVER_CERT_PATH: &str = "../certs/echo-server.pem";
const SERVER_KEY_PATH: &str = "../certs/echo-server-key.pem";

const BIND_ADDRESS: &str = "0.0.0.0:8279";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TLS acceptor 설정
    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;

    acceptor.set_private_key_file(SERVER_KEY_PATH, SslFiletype::PEM)?;
    acceptor.set_certificate_chain_file(SERVER_CERT_PATH)?;
    acceptor.check_private_key()?;
    let acceptor = acceptor.build();

    // TCP 리스너 생성
    let listener = TcpListener::bind(BIND_ADDRESS)?;
    println!("server is running on {}...", BIND_ADDRESS);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // TLS 스트림 생성
                let mut ssl_stream = match acceptor.accept(stream) {
                    Ok(stream) => stream,
                    Err(e) => {
                        println!("tls handshake failed: {}", e);
                        continue;
                    }
                };

                // 클라이언트로부터 데이터 수신
                let mut buf = [0; 1024];
                loop {
                    match ssl_stream.read(&mut buf) {
                        Ok(0) => {
                            println!("client disconnected");
                            break;
                        }
                        Ok(size) => {
                            ssl_stream.write_all(&buf[..size])?;
                            println!("recv & echo: {}", String::from_utf8_lossy(&buf[..size]));
                        }
                        Err(e) => {
                            println!("read error: {}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => println!("accept error: {}", e),
        }
    }

    Ok(())
}
