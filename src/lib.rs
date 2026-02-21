//! Hello World web service module
//!
//! Provides a simple HTTP endpoint that returns HTML content.

#![warn(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]

use actix_web::{web, HttpResponse};

/// Adds security-related HTTP headers to the response.
///
/// These headers help protect against common web vulnerabilities:
/// - `X-Content-Type-Options`: Prevents MIME type sniffing
/// - `X-Frame-Options`: Prevents clickjacking attacks
/// - `X-XSS-Protection`: Enables browser XSS filtering
/// - `Referrer-Policy`: Controls referrer information
/// - `Content-Security-Policy`: Restricts resource loading
/// - `Strict-Transport-Security`: Enforces HTTPS connections
/// - `Permissions-Policy`: Restricts browser features
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
            response
                .headers()
                .get("content-type")
                .expect("content-type header should be present"),
            "text/html; charset=utf-8"
        );
        assert_eq!(
            response
                .headers()
                .get("x-content-type-options")
                .expect("x-content-type-options header should be present"),
            "nosniff"
        );
        assert_eq!(
            response
                .headers()
                .get("x-frame-options")
                .expect("x-frame-options header should be present"),
            "DENY"
        );
        assert_eq!(
            response
                .headers()
                .get("x-xss-protection")
                .expect("x-xss-protection header should be present"),
            "1; mode=block"
        );
        assert_eq!(
            response
                .headers()
                .get("content-security-policy")
                .expect("content-security-policy header should be present"),
            "default-src 'self'"
        );
        assert_eq!(
            response
                .headers()
                .get("strict-transport-security")
                .expect("strict-transport-security header should be present"),
            "max-age=31536000; includeSubDomains"
        );
        assert_eq!(
            response
                .headers()
                .get("referrer-policy")
                .expect("referrer-policy header should be present"),
            "strict-origin-when-cross-origin"
        );
        assert_eq!(
            response
                .headers()
                .get("permissions-policy")
                .expect("permissions-policy header should be present"),
            "geolocation=(), microphone=(), camera=()"
        );
        let body = to_bytes(response.into_body())
            .await
            .expect("Body should be readable");
        let body_str = std::str::from_utf8(&body).expect("Body should be valid UTF-8");
        assert_eq!(body_str, HELLO_HTML);
    }

    #[actix_web::test]
    async fn test_health_handler_returns_healthy() {
        let response = health_handler().await;
        assert_eq!(response.status(), 200);
        assert_eq!(
            response
                .headers()
                .get("content-type")
                .expect("content-type header should be present"),
            "application/json"
        );
        assert_eq!(
            response
                .headers()
                .get("referrer-policy")
                .expect("referrer-policy header should be present"),
            "no-referrer"
        );
        let body = to_bytes(response.into_body())
            .await
            .expect("Body should be readable");
        let body_str = std::str::from_utf8(&body).expect("Body should be valid UTF-8");
        assert_eq!(body_str, r#"{"status":"healthy"}"#);
    }
}
