use actix_web::{cookie::{Cookie, SameSite}, 
    dev::{ResponseHead, ServiceRequest, ServiceResponse}, 
    http::header::{self, HeaderValue, SET_COOKIE}, Error, HttpRequest
};
use actix_session::Session;
use chrono::Utc;
use ring::{digest, error};
use dotenv::dotenv;
use uuid::Uuid;
use ring::hmac;


use crate::{settings, AuthClub, AuthPrefect};

pub fn generate_key() -> [u8; 64] {
    dotenv().ok();
    let csrf_token_key_str = std::env::var("SESSION_KEY").expect("CSRF_TOKEN_KEY must be set");

    let trimmed = csrf_token_key_str.trim_matches(|c: char| c == '[' || c == ']');
    let number_strings: Vec<&str> = trimmed.split(',').collect();

    let csrf_token_key_vec = number_strings
        .into_iter()
        .map(|s| s.trim().parse::<u8>())
        .collect::<Result<Vec<u8>, _>>()
        .unwrap();

    let csrf_token_key: [u8; 64] = csrf_token_key_vec.try_into().unwrap();
    csrf_token_key
}

pub fn generate_hmac_key_value() -> [u8; digest::SHA256_OUTPUT_LEN] {
    dotenv().ok();
    let csrf_token_key_str = std::env::var("HMAC_KEY_VALUE").expect("CSRF_TOKEN_KEY must be set");

    let trimmed = csrf_token_key_str.trim_matches(|c: char| c == '[' || c == ']');
    let number_strings: Vec<&str> = trimmed.split(',').collect();

    let csrf_token_key_vec = number_strings
        .into_iter()
        .map(|s| s.trim().parse::<u8>())
        .collect::<Result<Vec<u8>, _>>()
        .unwrap();

    let hmac_key_value: [u8; digest::SHA256_OUTPUT_LEN] = csrf_token_key_vec.try_into().unwrap();
    hmac_key_value
}

pub fn generate_csrf_token<B>(res: Result<&ServiceResponse<B>, &actix_web::Error>) -> String {
    let cookie_header_vals: Vec<&HeaderValue> = res.unwrap().response().headers().get_all(header::SET_COOKIE).collect();   
     let path = res.unwrap().request().path();
    let session_cookie_vals: &str;

    if path == "/login/club" ||  path == "/login/admin" {
        session_cookie_vals = cookie_header_vals[1].to_str().unwrap();
    } else {
        session_cookie_vals = cookie_header_vals[0].to_str().unwrap();
    } 

    let start = session_cookie_vals.find('=').unwrap();
    let end = session_cookie_vals.find(';').unwrap();

    let session_id = session_cookie_vals[start + 1..start + end].to_string().replace("; ", "");


    let hmac_key_value = generate_hmac_key_value();
    let s_key = hmac::Key::new(hmac::HMAC_SHA256, hmac_key_value.as_ref());
    let random_value = Uuid::new_v4();

    let message = session_id.clone() + "!" + random_value.to_string().as_str();
    let hmac = hmac::sign(&s_key, message.as_bytes());

    let hmac_string = hex::encode(hmac.as_ref());
    let csrf_token = hmac_string + "." + message.as_str();

    csrf_token
}

pub fn check_csrf_token(res: &ServiceRequest) -> Result<(), error::Unspecified> {
    let cookie_header_vals: Vec<&HeaderValue> = res.headers().get_all(header::COOKIE).collect(); 
    if cookie_header_vals.len() == 0 {
        return Err(error::Unspecified)
    }

    let csrf_cookie_vals = cookie_header_vals[0].to_str().unwrap().split("; ").nth(1);

    if let Some(csrf_cookie_vals) = csrf_cookie_vals {
        let csrf_token = csrf_cookie_vals.split("=").nth(1);
        let hmac_key_value = generate_hmac_key_value();
        let key = hmac::Key::new(hmac::HMAC_SHA256, hmac_key_value.as_ref());

        if let Some(csrf_token) = csrf_token {
            let tag = csrf_token.split(".").nth(0);
            let msg = csrf_token.split(".").nth(1);

            if let Some(tag) = tag {
                let tag_hex = hex::decode(tag);

                match tag_hex {
                    Ok(tag_hex) => {
                        hmac::verify(&key, msg.unwrap().as_bytes(), tag_hex.as_ref())
                    }
                    Err(_) => {
                        Err(error::Unspecified)
                    }
                }
            } else {
                Err(error::Unspecified)
            }
        } else {
            Err(error::Unspecified)
        }
    } else {
        Err(error::Unspecified)
    }
}

pub fn generate_club_session(club: &AuthClub, session: Session) -> Result<(), Error> {
    session.insert("club_auth", club)?;
    session.insert("identity", "club")?;
    session.insert("created_at", Utc::now().to_string())?;
    session.insert("last_modified", Utc::now().to_string())?;
    session.insert("club_id", &club.club_uid)?;
    Ok(())
}

pub fn update_club_session(session: Session) -> Result<(), Error> {
    session.insert("last_modified", Utc::now().to_string())?;
    Ok(())
}

pub fn generate_admin_session(prefect: &AuthPrefect, session: &Session) -> Result<(), Error> {
    session.insert("prefect_auth", prefect)?;
    session.insert("identity", "admin")?;
    session.insert("created_at", Utc::now().to_string())?;
    session.insert("last_modified", Utc::now().to_string())?;
    session.insert("prefect_id", &prefect.prefect_uid)?;
    Ok(())
}

pub fn set_csrf_cookie(response: &mut ResponseHead, csrf_token: String) -> () {
    let cookie = Cookie::build("csrf", csrf_token)
        .path("/")
        .secure(true)
        .http_only(false)
        .same_site(SameSite::Strict)
        .finish();
    let val = HeaderValue::from_str(cookie.to_string().as_str()).unwrap();

    response.headers_mut().append(SET_COOKIE, val);
}

pub fn change_csrf_cookie(response: &mut ResponseHead, csrf_token: String) -> () {
    let cookie = Cookie::build("csrf", csrf_token)
        .path("/")
        .secure(true)
        .http_only(false)
        .same_site(SameSite::Strict)
        .finish();
    let val = HeaderValue::from_str(cookie.to_string().as_str()).unwrap();

    response.headers_mut().insert(SET_COOKIE, val);
}

pub fn get_email_from_req(req: HttpRequest) -> String {
    let settings = settings::get_settings();
    let cookie = req.cookie(settings.auth_cookie_name.as_str()).unwrap();
    let email = crate::secure_token::verify_token(cookie.value(), req.path()).unwrap();
    email.replace("\"", "")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{cookie::Cookie, test};
    use crate::{settings, secure_token};

    #[actix_web::test]
    async fn get_email_from_req_ok() {
        let settings: &settings::Settings = settings::get_settings();
        let req = test::TestRequest::default().cookie(
                Cookie::build(settings.auth_cookie_name.clone(), 
                secure_token::generate_token("wbscodingclub@gmail.com", "login/club")
        )
            .path("/")
            .secure(true)
            .http_only(true)
            .finish())
        .to_http_request();

        let resp = get_email_from_req(req);
        assert_eq!(resp, "wbscodingclub@gmail.com")
    }

    #[actix_web::test]
    async fn check_csrf_token_not_ok1() {
        let srv_req = test::TestRequest::default().to_srv_request();
        let resp = check_csrf_token(&srv_req);
        assert_eq!(resp, Err(error::Unspecified))
    }

    #[actix_web::test]
    async fn check_csrf_token_not_ok2() {
        let srv_req = test::TestRequest::default().cookie(
            Cookie::build("csrf", "foo").finish()
        ).to_srv_request();
        let resp = check_csrf_token(&srv_req);
        assert_eq!(resp, Err(error::Unspecified))
    }

    #[actix_web::test]
    async fn check_csrf_token_not_ok3() {
        let srv_req = test::TestRequest::default().cookie(
            Cookie::build("csrf", "foo.bar").finish()
        ).to_srv_request();
        let resp = check_csrf_token(&srv_req);
        assert_eq!(resp, Err(error::Unspecified))
    }

    #[actix_web::test]
    async fn check_csrf_token_not_ok4() {
        let srv_req = test::TestRequest::default().cookie(
            Cookie::build("not_csrf", ".").finish()
        ).to_srv_request();
        let resp = check_csrf_token(&srv_req);
        assert_eq!(resp, Err(error::Unspecified))
    }
}