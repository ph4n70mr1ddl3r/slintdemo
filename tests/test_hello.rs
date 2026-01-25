use actix_web::{test, App};
use hello_world::{configure_app, HELLO_HTML};

#[actix_web::test]
async fn test_get_hello_returns_html() {
    let app = test::init_service(App::new().configure(|app| {
        configure_app(app);
    }))
    .await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);
    assert_eq!(
        resp.headers().get("content-type").unwrap(),
        "text/html; charset=utf-8"
    );

    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    assert_eq!(body_str, HELLO_HTML);
}
