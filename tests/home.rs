#[cfg(test)]
mod home {
    use actix_web::{test, App};
    use announcement_api::app;

    #[actix_web::test]
    async fn test_home_get() {
        let app = test::init_service(App::new().configure(app)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}