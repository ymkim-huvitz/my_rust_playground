use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslVerifyMode};
use std::io::{Read, Write};
use std::net::TcpListener;

const ROOT_CA_PATH: &str = "../certs/rootCA.pem";
const SERVER_CERT_PATH: &str = "../certs/echo-server.pem";
const SERVER_KEY_PATH: &str = "../certs/echo-server-key.pem";

const BIND_ADDRESS: &str = "0.0.0.0:8443";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TLS acceptor 설정
    let mut acceptor = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;

    acceptor.set_private_key_file(SERVER_KEY_PATH, SslFiletype::PEM)?;
    acceptor.set_certificate_chain_file(SERVER_CERT_PATH)?;

    // 클라이언트 인증서 검증 설정
    acceptor.set_verify(SslVerifyMode::PEER | SslVerifyMode::FAIL_IF_NO_PEER_CERT);
    acceptor.set_ca_file(ROOT_CA_PATH)?;

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

                println!("TLS version: {:?}", ssl_stream.ssl().version_str());

                // 클라이언트 인증서 검증
                if let Some(cert) = ssl_stream.ssl().peer_certificate() {
                    println!("client certificate: {:?}", cert.subject_name());
                    println!("client certificate valid from: {:?}", cert.not_before());
                    println!("client certificate valid until: {:?}", cert.not_after());

                    if let Some(subject) = cert.subject_name().entries().next() {
                        println!("client subject: {}", subject.data().as_utf8()?);
                    }
                }

                // 클라이언트로부터 데이터 수신
                let mut buf = [0; 1024];
                loop {
                    match ssl_stream.read(&mut buf) {
                        Ok(0) => {
                            println!("disconnected");
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
