//! Hello World web service module
//!
//! Provides a simple HTTP endpoint that returns HTML content.

use actix_web::{web, HttpResponse};

pub const HELLO_HTML: &str = "<!DOCTYPE html>
<html>
<body>
    <h1>Hello World</h1>
</body>
</html>";

pub async fn hello_handler() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(HELLO_HTML)
}

pub fn configure_app(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(hello_handler));
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::body::to_bytes;

    #[actix_web::test]
    async fn test_hello_handler_returns_correct_content() {
        let response = hello_handler().await;
        assert_eq!(response.status(), 200);
        assert_eq!(response.headers().get("content-type").unwrap(), "text/html; charset=utf-8");
        let body = to_bytes(response.into_body()).await.unwrap();
        let body_str = std::str::from_utf8(&body).unwrap();
        assert_eq!(body_str, HELLO_HTML);
    }
}
