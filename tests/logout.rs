mod common;

#[cfg(test)]
mod logout {
    use actix_web::{test, App};
    use announcement_api::{app, LoginForm};
    use super::*;

    use common::app_w_middleware;

    #[actix_web::test]
    async fn logout_get() {
        let app = test::init_service(App::new().configure(app)).await;
        let req = test::TestRequest::get().uri("/logout").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn logout_club_post() {
        let app = app_w_middleware().await;
        let club_email: String = std::env::var("SAMPLE_CLUB_EMAIL").expect("DATABASE_URL must be set");
        let valid_pin: String = std::env::var("SAMPLE_CLUB_PIN").expect("DATABASE_URL must be set");
        let req1 = test::TestRequest::post().uri("/login/club").set_form(LoginForm {email: club_email.to_string(), password_hash: valid_pin.to_string()} ).to_request();     
        let _resp1 = test::call_service(&app, req1).await;
        let req2 = test::TestRequest::post().uri("/logout").to_request();    
        let resp2 = test::call_service(&app, req2).await;
        assert!(resp2.status().is_redirection());
        assert_eq!(resp2.headers().get("Location").unwrap(), "/")
    }

    #[actix_web::test]
    async fn logout_admin_post() {
        let app = app_w_middleware().await;
        let prefect_email: String = std::env::var("SAMPLE_ADMIN_EMAIL").expect("DATABASE_URL must be set");
        let valid_pin: String = std::env::var("SAMPLE_ADMIN_PIN").expect("DATABASE_URL must be set");
        let req1 = test::TestRequest::post().uri("/login/admin").set_form(LoginForm {email: prefect_email.to_string(), password_hash: valid_pin.to_string()} ).to_request();     
        let _resp1 = test::call_service(&app, req1).await;
        let req2 = test::TestRequest::post().uri("/logout").to_request();    
        let resp2 = test::call_service(&app, req2).await;
        assert!(resp2.status().is_redirection());
        assert_eq!(resp2.headers().get("Location").unwrap(), "/")
    }
}