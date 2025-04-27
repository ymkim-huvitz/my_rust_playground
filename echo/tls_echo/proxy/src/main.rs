use openssl::ssl::{HandshakeError, SslConnector, SslMethod};
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

const PROXY_ADDR: &str = "0.0.0.0:8279";
const TLS_SERVER_ADDR: &str = "127.0.0.1:8443";
const TLS_SERVER_NAME: &str = "echo-server";
const ROOT_CA_PATH: &str = "../certs/rootCA.pem";

fn handle_client(mut client_stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    // TLS connection setup
    let mut connector = SslConnector::builder(SslMethod::tls())?;
    connector.set_ca_file(ROOT_CA_PATH)?;
    let connector = connector.build();

    // Connect to TLS server
    let tls_stream = TcpStream::connect(TLS_SERVER_ADDR)?;
    let mut tls_stream = match connector.connect(TLS_SERVER_NAME, tls_stream) {
        Ok(stream) => stream,
        Err(HandshakeError::Failure(e)) => {
            println!("TLS handshake failed: {:?}", e);
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "TLS handshake failed",
            )));
        }
        Err(HandshakeError::WouldBlock(_)) => {
            println!("TLS handshake would block");
            return Err(Box::new(io::Error::new(
                io::ErrorKind::WouldBlock,
                "TLS handshake would block",
            )));
        }
        Err(HandshakeError::SetupFailure(e)) => {
            println!("TLS setup failed: {:?}", e);
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "TLS setup failed",
            )));
        }
    };

    // Message relay between client and TLS server
    let mut client_buf = [0; 1024];
    let mut server_buf = [0; 1024];

    loop {
        // Receive message from client
        match client_stream.read(&mut client_buf) {
            Ok(0) => break, // Connection closed
            Ok(n) => {
                // Send to TLS server
                if let Err(e) = tls_stream.write_all(&client_buf[..n]) {
                    println!("Failed to send to TLS server: {}", e);
                    break;
                }
                tls_stream.flush()?;
                println!("Client -> Server: {} bytes", n);

                // Receive response from TLS server
                match tls_stream.read(&mut server_buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        // Send response to client
                        if let Err(e) = client_stream.write_all(&server_buf[..n]) {
                            println!("Failed to send to client: {}", e);
                            break;
                        }
                        client_stream.flush()?;
                        println!("Server -> Client: {} bytes", n);
                    }
                    Err(e) => {
                        println!("Failed to read from TLS server: {}", e);
                        break;
                    }
                }
            }
            Err(e) => {
                println!("Failed to read from client: {}", e);
                break;
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(PROXY_ADDR)?;
    println!("Proxy server is running on {}...", PROXY_ADDR);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connection: {}", stream.peer_addr()?);
                thread::spawn(move || {
                    if let Err(e) = handle_client(stream) {
                        println!("Error handling client: {}", e);
                    }
                });
            }
            Err(e) => println!("Connection error: {}", e),
        }
    }

    Ok(())
}
