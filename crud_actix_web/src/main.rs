// This is a simple example of a health checker API using Actix Web in Rust.
// cargo add actix-web
// cargo add actix-cors
// cargo add serde --features derive
// cargo add chrono --features serde
// cargo add env_logger
// cargo add uuid --features v4

mod handler;
mod model;
mod response;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, http::header, web};
use model::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    const ADDRESS: &str = "0.0.0.0";
    const PORT: u16 = 8081;

    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "actix_web=info");
        }
    }
    env_logger::init();

    let todo_db = AppState::init();
    let app_data = web::Data::new(todo_db);

    println!("🚀 Server started successfully at http://{}:{}", ADDRESS, PORT);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(app_data.clone())
            .configure(handler::config)
            .wrap(cors)
            .wrap(Logger::default())
    })
    .bind((ADDRESS, PORT))?
    .run()
    .await
}
