mod common;

#[cfg(test)]
mod login {
    use actix_web::{test, App};
    use announcement_api::{app, LoginForm};
    use super::*;

    use common::app_w_middleware;

    #[actix_web::test]
    async fn login_club_get() {
        let app = test::init_service(App::new().configure(app)).await;
        let req = test::TestRequest::get().uri("/login/club").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn login_admin_get() {
        let app = test::init_service(App::new().configure(app)).await;
        let req = test::TestRequest::get().uri("/login/admin").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn login_club_invalid_pwd() {
        let app = app_w_middleware().await;
        let club_email: String = std::env::var("SAMPLE_CLUB_EMAIL").expect("DATABASE_URL must be set");
        let req = test::TestRequest::post().uri("/login/club").set_form(LoginForm {email: club_email.to_string(), password_hash: "bar".to_string()} ).to_request();     
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_web::test]
    async fn login_admin_invalid_pwd() {
        let app = app_w_middleware().await;
        let prefect_email: String = std::env::var("SAMPLE_ADMIN_EMAIL").expect("DATABASE_URL must be set");
        let req = test::TestRequest::post().uri("/login/admin").set_form(LoginForm {email: prefect_email.to_string(), password_hash: "foo".to_string()} ).to_request();     
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_web::test]
    async fn login_club_valid_pwd() {
        let app = app_w_middleware().await;
        let club_email: String = std::env::var("SAMPLE_CLUB_EMAIL").expect("DATABASE_URL must be set");
        let valid_pin: String = std::env::var("SAMPLE_CLUB_PIN").expect("DATABASE_URL must be set");
        let req = test::TestRequest::post().uri("/login/club").set_form(LoginForm {email: club_email.to_string(), password_hash: valid_pin.to_string()} ).to_request();     
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_redirection());
        assert_eq!(resp.headers().get("Location").unwrap(), "/club")
    }

    #[actix_web::test]
    async fn login_admin_valid_pwd() {
        let app = app_w_middleware().await;
        let prefect_email: String = std::env::var("SAMPLE_ADMIN_EMAIL").expect("DATABASE_URL must be set");
        let valid_pin: String = std::env::var("SAMPLE_ADMIN_PIN").expect("DATABASE_URL must be set");
        let req = test::TestRequest::post().uri("/login/admin").set_form(LoginForm {email: prefect_email.to_string(), password_hash: valid_pin.to_string()} ).to_request();     
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_redirection());
        assert_eq!(resp.headers().get("Location").unwrap(), "/admin")
    }
    // Check if user tries making an announcement with invalid CSRF token
}