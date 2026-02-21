//! Integration tests for the Hello World web service.
//!
//! These tests verify the HTTP endpoints work correctly when accessed
//! through the full Actix-web request/response pipeline.

use actix_web::{test, App};
use hello_world::{configure_app, HELLO_HTML};

#[actix_web::test]
async fn test_get_hello_returns_html() {
    let app = test::init_service(App::new().configure(configure_app)).await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    assert_eq!(
        resp.headers()
            .get("content-type")
            .expect("content-type header should be present"),
        "text/html; charset=utf-8"
    );
    assert_eq!(
        resp.headers()
            .get("x-content-type-options")
            .expect("x-content-type-options header should be present"),
        "nosniff"
    );
    assert_eq!(
        resp.headers()
            .get("referrer-policy")
            .expect("referrer-policy header should be present"),
        "strict-origin-when-cross-origin"
    );

    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).expect("Body should be valid UTF-8");
    assert_eq!(body_str, HELLO_HTML);
}

#[actix_web::test]
async fn test_health_endpoint_returns_healthy() {
    let app = test::init_service(App::new().configure(configure_app)).await;

    let req = test::TestRequest::get().uri("/health").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    assert_eq!(
        resp.headers()
            .get("content-type")
            .expect("content-type header should be present"),
        "application/json"
    );
    assert_eq!(
        resp.headers()
            .get("referrer-policy")
            .expect("referrer-policy header should be present"),
        "strict-origin-when-cross-origin"
    );

    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).expect("Body should be valid UTF-8");
    assert_eq!(body_str, r#"{"status":"healthy"}"#);
}

#[actix_web::test]
async fn test_unknown_route_returns_404() {
    let app = test::init_service(App::new().configure(configure_app)).await;

    let req = test::TestRequest::get().uri("/unknown").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}

#[actix_web::test]
async fn test_post_to_root_returns_404() {
    let app = test::init_service(App::new().configure(configure_app)).await;

    let req = test::TestRequest::post().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 404);
}
