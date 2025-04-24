import socket
import ssl
import threading

ADDR = "127.0.0.1"
PORT = 8279

ROOT_CA_PATH = "../certs/rootCA.pem"
SERVER_CERT_PATH = "../certs/echo-server.pem"
SERVER_KEY_PATH = "../certs/echo-server-key.pem"

CLIENT_CERT_PATH = "../certs/echo-client.pem"
CLIENT_KEY_PATH = "../certs/echo-client-key.pem"

SERVER_NAME = "echo-server"


def handle_client(client_socket):
    data = client_socket.recv(1024)
    if data:
        print(f"recv & echo: {data.decode()}")
        client_socket.send(data)
    client_socket.close()


def server():
    context = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
    context.load_cert_chain(certfile=SERVER_CERT_PATH, keyfile=SERVER_KEY_PATH)
    context.load_verify_locations(cafile=ROOT_CA_PATH)
    context.verify_mode = ssl.CERT_REQUIRED  # 클라이언트 인증서 검증 요구

    server = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    server.bind((ADDR, PORT))
    server.listen(5)
    print(f"server started on {ADDR}:{PORT}")

    while True:
        client_socket, _ = server.accept()
        secure_socket = context.wrap_socket(client_socket, server_side=True)
        threading.Thread(target=handle_client, args=(secure_socket,)).start()


def client():
    context = ssl.SSLContext(ssl.PROTOCOL_TLS_CLIENT)
    context.load_verify_locations(cafile=ROOT_CA_PATH)
    context.load_cert_chain(certfile=CLIENT_CERT_PATH, keyfile=CLIENT_KEY_PATH)  # 클라이언트 인증서 로드

    client = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    secure_client = context.wrap_socket(client, server_hostname=SERVER_NAME)
    secure_client.connect((ADDR, PORT))

    print(f"send: hello")
    secure_client.send(b"hello")
    data = secure_client.recv(1024)
    print(f"recv: {data.decode()}")
    secure_client.close()


if __name__ == "__main__":
    import sys
    if len(sys.argv) > 1 and sys.argv[1] == "server":
        server()
    else:
        client()
