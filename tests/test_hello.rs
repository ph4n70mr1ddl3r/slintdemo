use actix_web::{test, App};
use hello_world::configure_app;

#[actix_rt::test]
async fn test_get_hello_returns_html() {
    let mut app = test::init_service(App::new().configure(|app| {
        configure_app(app);
    }))
    .await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert!(body_str.contains("Hello World"));
}
