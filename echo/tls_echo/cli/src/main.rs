use openssl::ssl::{SslConnector, SslMethod};
use std::io::{self, Read, Write};
use std::net::TcpStream;

const ROOT_CA_PATH: &str = "../certs/rootCA.pem";

const SERVER_ADDRESS: &str = "127.0.0.1:8279";
const SERVER_HOSTNAME: &str = "echo-server";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TLS connector 설정
    let mut connector = SslConnector::builder(SslMethod::tls())?;
    connector.set_ca_file(ROOT_CA_PATH)?;
    let connector = connector.build();

    // 서버에 연결
    let stream = TcpStream::connect(SERVER_ADDRESS)?;
    let mut ssl_stream = connector.connect(SERVER_HOSTNAME, stream)?;

    println!("TLS version: {:?}", ssl_stream.ssl().version_str());

    println!("Connected. Enter message(type 'quit' to exit):");

    loop {
        // 사용자 입력 받기
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        if input.trim() == "quit" {
            break;
        }

        // 서버에 데이터 전송
        ssl_stream.write_all(input.as_bytes())?;

        // 서버로부터 응답 수신
        let mut buf = [0; 1024];
        match ssl_stream.read(&mut buf) {
            Ok(size) => {
                let response = String::from_utf8_lossy(&buf[..size]);
                println!("recv: {}", response);
            }
            Err(e) => println!("read error: {}", e),
        }

        // 100ms 대기
        std::thread::sleep(std::time::Duration::from_millis(100));

        // 서버 연결 상태 확인
        if ssl_stream.get_ref().peer_addr().is_err() {
            println!("Server connection lost. Exiting loop.");
            break;
        }
    }

    Ok(())
}
