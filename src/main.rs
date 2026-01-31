use actix_web::{middleware, web, App, HttpServer};
use log::info;
use std::{env, time::Duration};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    log::info!("Starting Hello World server");

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
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

    let bind_address = format!("{host}:{port}");
    info!("Starting server on {bind_address}");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .app_data(web::PayloadConfig::new(1024 * 1024))
            .configure(hello_world::configure_app)
    })
    .keep_alive(Duration::from_secs(75))
    .bind(&bind_address)
    .map_err(|e| std::io::Error::new(e.kind(), format!("Failed to bind to {bind_address}: {e}")))?
    .run()
    .await
}
