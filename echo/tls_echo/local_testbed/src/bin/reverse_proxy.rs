use anyhow::Result;
use openssl::ssl::{Ssl, SslAcceptor, SslFiletype, SslMethod, SslStream};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

const PROXY_ADDR: &str = "0.0.0.0:8443";
const ECHO_SERVER_ADDR: &str = "127.0.0.1:8279";

const SERVER_CERT_PATH: &str = "../certs/echo-server.pem";
const SERVER_KEY_PATH: &str = "../certs/echo-server-key.pem";

fn handle_client(mut tls_stream: SslStream<TcpStream>) -> Result<()> {
    // 에코 서버에 연결
    let mut echo_stream = TcpStream::connect(ECHO_SERVER_ADDR)?;

    // 클라이언트와 에코 서버 간의 양방향 데이터 전송
    let mut buf = [0u8; 1024];
    loop {
        // 클라이언트로부터 데이터 읽기
        match tls_stream.read(&mut buf) {
            Ok(0) => break, // 연결 종료
            Ok(n) => {
                // 에코 서버로 데이터 전송
                echo_stream.write_all(&buf[..n])?;
                println!("Client -> Server: {} bytes", n);

                // 에코 서버로부터 응답 받기
                let mut echo_buf = [0u8; 1024];
                match echo_stream.read(&mut echo_buf) {
                    Ok(0) => break,
                    Ok(m) => {
                        // 클라이언트에게 응답 전송
                        tls_stream.write_all(&echo_buf[..m])?;
                        tls_stream.flush()?;
                        println!("Server -> Client: {} bytes", m);
                    }
                    Err(e) => {
                        eprintln!("Error reading from echo server: {}", e);
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading from client: {}", e);
                break;
            }
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    // TLS 설정
    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    acceptor.set_private_key_file(SERVER_KEY_PATH, SslFiletype::PEM)?;
    acceptor.set_certificate_chain_file(SERVER_CERT_PATH)?;
    acceptor.check_private_key()?;
    let acceptor = acceptor.build();

    // 리스너 시작
    let listener = TcpListener::bind(PROXY_ADDR)?;
    println!("TLS reverse proxy listening on {}", PROXY_ADDR);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let acceptor = acceptor.clone();
                thread::spawn(move || match Ssl::new(acceptor.context()) {
                    Ok(ssl) => match ssl.accept(stream) {
                        Ok(tls_stream) => {
                            if let Err(e) = handle_client(tls_stream) {
                                eprintln!("Error handling client: {}", e);
                            }
                        }
                        Err(e) => eprintln!("TLS handshake failed: {}", e),
                    },
                    Err(e) => eprintln!("Failed to create SSL context: {}", e),
                });
            }
            Err(e) => eprintln!("Failed to accept connection: {}", e),
        }
    }
    Ok(())
}
