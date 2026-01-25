use actix_web::{App, HttpServer};
use log::info;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("Starting Hello World server");

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .map_err(|e| {
            log::error!("Invalid PORT value: {e}");
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid PORT value: {e}"),
            )
        })?;

    let bind_address = format!("{}:{}", host, port);
    info!("Starting server on {}", bind_address);

    HttpServer::new(|| App::new().configure(hello_world::configure_app))
        .bind(&bind_address)
        .map_err(|e| {
            log::error!("Failed to bind to {}: {}", bind_address, e);
            e
        })?
        .run()
        .await
}
