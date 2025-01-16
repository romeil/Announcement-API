mod common;

#[cfg(test)]
mod password {
    use actix_http::header::SET_COOKIE;
    use actix_web::{cookie::Cookie, test, App};
    use announcement_api::{app, NewPassword, ID};
    use super::*;

    use common::app_w_middleware;

    #[actix_web::test]
    async fn create_password_get() {
        let app = test::init_service(App::new().configure(app)).await;
        let req = test::TestRequest::get().uri("/create-pin").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn no_password() {
        let app = app_w_middleware().await;
        let req = test::TestRequest::post().uri("/create-pin").set_form(NewPassword {new_password: "".to_string(), confirm_password: "".to_string()} ).to_request();     
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_web::test]
    async fn empty_password_field1() {
        let app = app_w_middleware().await;
        let req = test::TestRequest::post().uri("/create-pin").set_form(NewPassword {new_password: "".to_string(), confirm_password: "bar".to_string()} ).to_request();     
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_web::test]
    async fn empty_password_field2() {
        let app = app_w_middleware().await;
        let req = test::TestRequest::post().uri("/create-pin").set_form(NewPassword {new_password: "foo".to_string(), confirm_password: "".to_string()} ).to_request();     
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_web::test]
    async fn not_matching_passwords() {
        let app = app_w_middleware().await;
        let req = test::TestRequest::post().uri("/create-pin").set_form(NewPassword {new_password: "foo".to_string(), confirm_password: "bar".to_string()} ).to_request();     
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_client_error());
    }

    // Create a middleware that allows the user to navigate to create-pin only after valid registration
    #[actix_web::test]
    #[ignore = "will trigger an error with UNIQUE constraint in club table"]
    async fn matching_passwords() {
        let app = app_w_middleware().await;
        let registration_id: String = std::env::var("SAMPLE_REGISTRATION_ID").expect("DATABASE_URL must be set");

        let req1 = test::TestRequest::post().uri("/register").set_form(ID {value: registration_id} ).to_request();     
        let resp1 = test::call_service(&app, req1).await;

        let cookie_header_val = resp1.headers().get(SET_COOKIE).unwrap();
        let cookie = cookie_header_val.to_str().ok().and_then(|cookie_str|
            Cookie::parse(cookie_str).ok()
        ).unwrap();
        assert!(resp1.headers().contains_key(SET_COOKIE));
        assert!(resp1.status().is_redirection());
        
        let req2 = test::TestRequest::post().uri("/create-pin").cookie(cookie).set_form(NewPassword {new_password: "foo".to_string(), confirm_password: "foo".to_string()} ).to_request();    
        let resp2 = test::call_service(&app, req2).await; 
        assert!(resp2.status().is_redirection());
    }
}