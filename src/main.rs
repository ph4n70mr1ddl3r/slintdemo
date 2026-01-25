use actix_web::{App, HttpServer};
use log::info;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or_else(|e| panic!("Invalid PORT value: {e}"));

    let bind_address = format!("{}:{}", host, port);
    info!("Starting server on {}", bind_address);

    HttpServer::new(|| App::new().configure(hello_world::configure_app))
        .bind(&bind_address)?
        .run()
        .await
}
