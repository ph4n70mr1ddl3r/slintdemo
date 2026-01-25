use actix_web::{test, App};
use hello_world::configure_app;

#[actix_web::test]
async fn test_get_hello_returns_html() {
    let mut app = test::init_service(App::new().configure(|app| {
        configure_app(app);
    }))
    .await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.headers().get("content-type").unwrap(), "text/html");

    let body = test::read_body(resp).await;
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    assert_eq!(body_str, "<html><body><h1>Hello World</h1></body></html>");
}
