#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>
#include <openssl/ssl.h>
#include <openssl/err.h>

#define BIND_ADDR "0.0.0.0"
#define PORT 8279
#define ECHO_SERVER_ADDR "127.0.0.1"

#define ROOT_CA_PATH "../certs/rootCA.pem"
#define SERVER_CERT_PATH "../certs/echo-server.pem"
#define SERVER_KEY_PATH "../certs/echo-server-key.pem"

void init_openssl()
{
    SSL_load_error_strings();
    OpenSSL_add_ssl_algorithms();
}

SSL_CTX *create_context(int server)
{
    const SSL_METHOD *method = server ? TLS_server_method() : TLS_client_method();
    SSL_CTX *ctx = SSL_CTX_new(method);
    if (!ctx)
    {
        ERR_print_errors_fp(stderr);
        exit(1);
    }
    return ctx;
}

void configure_server_context(SSL_CTX *ctx)
{
    SSL_CTX_use_certificate_file(ctx, SERVER_CERT_PATH, SSL_FILETYPE_PEM);
    SSL_CTX_use_PrivateKey_file(ctx, SERVER_KEY_PATH, SSL_FILETYPE_PEM);
}

void handle_client(SSL *ssl)
{
    char buffer[1024] = {0};
    int bytes = SSL_read(ssl, buffer, sizeof(buffer));
    if (bytes > 0)
    {
        printf("recv & echo: %s\n", buffer);
        SSL_write(ssl, buffer, bytes);
    }
    SSL_free(ssl);
}

void server()
{
    init_openssl();
    SSL_CTX *ctx = create_context(1);
    configure_server_context(ctx);

    int sock = socket(AF_INET, SOCK_STREAM, 0);
    struct sockaddr_in addr = {.sin_family = AF_INET, .sin_addr.s_addr = INADDR_ANY, .sin_port = htons(PORT)};
    bind(sock, (struct sockaddr *)&addr, sizeof(addr));
    listen(sock, 5);
    printf("Listening on %s:%d\n", BIND_ADDR, PORT);

    while (1)
    {
        int client = accept(sock, NULL, NULL);
        SSL *ssl = SSL_new(ctx);
        SSL_set_fd(ssl, client);
        if (SSL_accept(ssl) <= 0)
        {
            ERR_print_errors_fp(stderr);
        }
        else
        {
            printf("TLS version: %s\n", SSL_get_version(ssl));
            handle_client(ssl);
        }
        close(client);
    }
    SSL_CTX_free(ctx);
}

void client()
{
    init_openssl();
    SSL_CTX *ctx = create_context(0);
    SSL_CTX_load_verify_locations(ctx, ROOT_CA_PATH, NULL);

    int sock = socket(AF_INET, SOCK_STREAM, 0);
    struct sockaddr_in addr = {.sin_family = AF_INET, .sin_port = htons(PORT)};
    inet_pton(AF_INET, ECHO_SERVER_ADDR, &addr.sin_addr);
    connect(sock, (struct sockaddr *)&addr, sizeof(addr));

    SSL *ssl = SSL_new(ctx);
    SSL_set_fd(ssl, sock);
    if (SSL_connect(ssl) <= 0)
    {
        ERR_print_errors_fp(stderr);
        exit(1);
    }
    printf("TLS version: %s\n", SSL_get_version(ssl));

    const char *msg = "hello";
    SSL_write(ssl, msg, strlen(msg));
    char buffer[1024] = {0};
    int bytes = SSL_read(ssl, buffer, sizeof(buffer));
    printf("send & recv: %s\n", buffer);

    SSL_free(ssl);
    close(sock);
    SSL_CTX_free(ctx);
}

int main(int argc, char *argv[])
{
    if (argc > 1 && strcmp(argv[1], "server") == 0)
    {
        server();
    }
    else
    {
        client();
    }
    return 0;
}