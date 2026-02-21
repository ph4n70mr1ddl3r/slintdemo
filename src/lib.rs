//! Hello World web service module
//!
//! Provides a simple HTTP endpoint that returns HTML content.

#![warn(missing_docs)]

use actix_web::{web, HttpResponse};

fn add_security_headers(builder: &mut actix_web::HttpResponseBuilder) {
    builder
        .insert_header(("X-Content-Type-Options", "nosniff"))
        .insert_header(("X-Frame-Options", "DENY"))
        .insert_header(("X-XSS-Protection", "1; mode=block"))
        .insert_header(("Referrer-Policy", "strict-origin-when-cross-origin"))
        .insert_header(("Content-Security-Policy", "default-src 'self'"))
        .insert_header((
            "Strict-Transport-Security",
            "max-age=31536000; includeSubDomains",
        ))
        .insert_header((
            "Permissions-Policy",
            "geolocation=(), microphone=(), camera=()",
        ));
}

/// HTML content for the Hello World page.
pub const HELLO_HTML: &str = "<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\">
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
    <title>Hello World</title>
</head>
<body>
    <h1>Hello World</h1>
    <p>A simple web service built with Actix.</p>
</body>
</html>";

/// Handles GET requests to the root path.
///
/// Returns a simple "Hello World" HTML page with security headers.
pub async fn hello_handler() -> HttpResponse {
    let mut response = HttpResponse::Ok();
    response.content_type("text/html; charset=utf-8");
    add_security_headers(&mut response);
    response.insert_header(("Cache-Control", "no-cache"));
    response.body(HELLO_HTML)
}

/// Handles GET requests to the health check endpoint.
///
/// Returns a simple JSON response indicating the service is healthy.
pub async fn health_handler() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .insert_header(("Cache-Control", "no-store"))
        .insert_header(("Referrer-Policy", "no-referrer"))
        .body(r#"{"status":"healthy"}"#)
}

/// Configures the Actix-web application routes.
///
/// # Arguments
/// * `cfg` - The Actix-web service configuration to add routes to
pub fn configure_app(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(hello_handler))
        .route("/health", web::get().to(health_handler));
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::body::to_bytes;

    #[actix_web::test]
    async fn test_hello_handler_returns_correct_content() {
        let response = hello_handler().await;
        assert_eq!(response.status(), 200);
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "text/html; charset=utf-8"
        );
        assert_eq!(
            response.headers().get("x-content-type-options").unwrap(),
            "nosniff"
        );
        assert_eq!(response.headers().get("x-frame-options").unwrap(), "DENY");
        assert_eq!(
            response.headers().get("x-xss-protection").unwrap(),
            "1; mode=block"
        );
        assert_eq!(
            response.headers().get("content-security-policy").unwrap(),
            "default-src 'self'"
        );
        assert_eq!(
            response.headers().get("strict-transport-security").unwrap(),
            "max-age=31536000; includeSubDomains"
        );
        assert_eq!(
            response.headers().get("referrer-policy").unwrap(),
            "strict-origin-when-cross-origin"
        );
        assert_eq!(
            response.headers().get("permissions-policy").unwrap(),
            "geolocation=(), microphone=(), camera=()"
        );
        let body = to_bytes(response.into_body()).await.unwrap();
        let body_str = std::str::from_utf8(&body).unwrap();
        assert_eq!(body_str, HELLO_HTML);
    }

    #[actix_web::test]
    async fn test_health_handler_returns_healthy() {
        let response = health_handler().await;
        assert_eq!(response.status(), 200);
        assert_eq!(
            response.headers().get("content-type").unwrap(),
            "application/json"
        );
        assert_eq!(
            response.headers().get("referrer-policy").unwrap(),
            "no-referrer"
        );
        let body = to_bytes(response.into_body()).await.unwrap();
        let body_str = std::str::from_utf8(&body).unwrap();
        assert_eq!(body_str, r#"{"status":"healthy"}"#);
    }
}
