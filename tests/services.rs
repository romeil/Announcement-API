mod common;

#[cfg(test)]
mod services {
    use actix_web::test;
    use announcement_api::LoginForm;
    use super::*;

    use common::app_w_middleware;

    #[actix_web::test]
    async fn fetch_club_announcements_by_uuid_and_date() {
        let app = app_w_middleware().await;
        let club_email: String = std::env::var("SAMPLE_CLUB_EMAIL").expect("DATABASE_URL must be set");
        let valid_pin: String = std::env::var("SAMPLE_CLUB_PIN").expect("DATABASE_URL must be set");
        let req = test::TestRequest::post().uri("/login/club").set_form(LoginForm {email: club_email.to_string(), password_hash: valid_pin.to_string()} ).to_request();     
        let _resp = test::call_service(&app, req).await;

        let req  = test::TestRequest::get().uri("/club/2024-03-18").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_redirection());
    }

    #[actix_web::test]
    async fn fetch_club_announcements_date() {
        let app = app_w_middleware().await;
        let prefect_email: String = std::env::var("SAMPLE_ADMIN_EMAIL").expect("DATABASE_URL must be set");
        let valid_pin: String = std::env::var("SAMPLE_ADMIN_PIN").expect("DATABASE_URL must be set");
        let req = test::TestRequest::post().uri("/login/club").set_form(LoginForm {email: prefect_email.to_string(), password_hash: valid_pin.to_string()} ).to_request();     
        let _resp = test::call_service(&app, req).await;

        let req  = test::TestRequest::get().uri("/admin/date/2024-03-18").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_redirection());
    }
    // Create a test for creating announcement
}