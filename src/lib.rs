use actix_web::{web, HttpResponse};

const HELLO_HTML: &str = "<html><body><h1>Hello World</h1></body></html>";

pub async fn hello_handler() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
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
        assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
        let body = to_bytes(response.into_body()).await.unwrap();
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        assert_eq!(body_str, HELLO_HTML);
    }
}
