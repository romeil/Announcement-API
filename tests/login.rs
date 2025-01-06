#[cfg(test)]
mod login {
    use actix_web::{test, web::Data, App};
    use announcement_api::{app, connect_to_db, AppState, LoginForm};
    use dotenv::dotenv;

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
    async fn login_club_invalid_post() {
        dotenv().ok();
        let pool = connect_to_db().await;
        let app = test::init_service(App::new().app_data(Data::new(AppState {db: pool.clone()})).configure(app)).await;
        let req = test::TestRequest::post().uri("/login/club").set_form(LoginForm {email: "foo".to_string(), password_hash: "bar".to_string()} ).to_request();     
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_web::test]
    async fn login_club_valid_post() {
        dotenv().ok();
        let valid_pin: String = std::env::var("CODING_CLUB_PIN").expect("DATABASE_URL must be set");
        let pool = connect_to_db().await;
        let app = test::init_service(App::new().app_data(Data::new(AppState {db: pool.clone()})).configure(app)).await;
        let req = test::TestRequest::post().uri("/login/club").set_form(LoginForm {email: "wbscodingclub@gmail.com".to_string(), password_hash: valid_pin.to_string()} ).to_request();     
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}