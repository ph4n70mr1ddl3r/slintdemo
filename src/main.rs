use actix_web::{middleware, web, App, HttpServer};
use log::{error, info};
use std::{env, time::Duration};

const MAX_PAYLOAD_SIZE: usize = 1024 * 1024;
const KEEP_ALIVE_SECS: u64 = 75;
const SHUTDOWN_TIMEOUT_SECS: u64 = 30;
const DEFAULT_HOST: &str = "127.0.0.1";
const DEFAULT_PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    info!("Starting Hello World server");

    let host = env::var("HOST").unwrap_or_else(|_| DEFAULT_HOST.to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| DEFAULT_PORT.to_string())
        .parse::<u16>()
        .map_err(|e| {
            error!("Invalid PORT value: {e}");
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid PORT value: {e}"),
            )
        })?;

    let bind_address = format!("{host}:{port}");
    info!("Starting server on {bind_address}");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .app_data(web::PayloadConfig::new(MAX_PAYLOAD_SIZE))
            .configure(hello_world::configure_app)
    })
    .keep_alive(Duration::from_secs(KEEP_ALIVE_SECS))
    .shutdown_timeout(SHUTDOWN_TIMEOUT_SECS)
    .bind(&bind_address)
    .map_err(|e| std::io::Error::new(e.kind(), format!("Failed to bind to {bind_address}: {e}")))?
    .run()
    .await
}
