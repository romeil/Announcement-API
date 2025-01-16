mod common;

#[cfg(test)]
mod signup {
    use actix_web::{test, App};
    use announcement_api::{app, ID};
    use super::*;

    use common::app_w_middleware;

    #[actix_web::test]
    async fn signup_get() {
        let app = test::init_service(App::new().configure(app)).await;
        let req = test::TestRequest::get().uri("/register").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn signup_invalid_id() {
        let app = app_w_middleware().await;
        let req = test::TestRequest::post().uri("/register").set_form(ID {value: "1233456789".to_string()} ).to_request();     
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_web::test]
    async fn signup_valid_id() {
        let app = app_w_middleware().await;
        let registration_id: String = std::env::var("SAMPLE_REGISTRATION_ID").expect("DATABASE_URL must be set");
        let req = test::TestRequest::post().uri("/register").set_form(ID {value: registration_id} ).to_request();     
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_redirection());
        assert_eq!(resp.headers().get("Location").unwrap(), "/create-pin");
    }
}