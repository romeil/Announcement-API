use actix_http::header::{HeaderValue, SET_COOKIE};
use actix_web::{
    cookie::Cookie, web::{self, Data}, HttpRequest, HttpResponse, Responder};
use actix_session::Session;
use sqlx;
use validators::{prelude::*, serde_json::json};
use validators::models::Host;

use common::{AppState, AuthAdmin, AuthClub, AuthPrefect, LoginForm};
use crate::{secure_token, session, settings};

#[derive(Validator)]
#[validator(email(comment(Disallow), ip(Allow), local(Allow), at_least_two_labels(Allow), non_ascii(Allow)))]
pub struct EmailWithoutComment {
    pub local_part: String,
    pub need_quoted: bool,
    pub domain_part: Host,
}

pub async fn login_admin_post(state: Data<AppState>, data: web::Form<LoginForm>, req: HttpRequest, session: Session) -> impl Responder {
    let email = data.email.as_str();
    let password = Option::from(data.password_hash.as_str());

    match password {
        None => HttpResponse::Unauthorized().body("Must provide a password"),
        Some(pass) => {
            let email_validation = EmailWithoutComment::parse_string(email);
            match email_validation {
                Ok(_) => {
                    match sqlx::query_as::<_, AuthAdmin>(
                        "SELECT admin_uid, first_name, last_name, email, password_hash 
                            FROM admin
                            WHERE email = $1"
                    )
                    .bind(email)
                    .fetch_one(&state.db)
                    .await
                    {
                        Ok(admin) => {
                            let is_valid = bcrypt::verify(pass.to_string(), &admin.password_hash).unwrap();
                            if is_valid {
                                session::generate_admin_session(&admin, &session).unwrap();

                                let settings = settings::get_settings();

                                let cookie = Cookie::build(settings.auth_cookie_name.clone(), secure_token::generate_token(email, req.path()))
                                    .path("/")
                                    .secure(true)
                                    .http_only(false)
                                    .finish();
                                let val = HeaderValue::from_str(cookie.to_string().as_str()).unwrap();

                                HttpResponse::Ok()
                                    .insert_header((SET_COOKIE, val))
                                    .json(json!({ "redirect": "/admin"}))
                            } else {
                                HttpResponse::Unauthorized()
                                    .body("Invalid admin ID or password")
                            }
                        }
                        Err(_) => HttpResponse::Unauthorized().body("Invalid admin ID or password")
                    }
                }
                Err(_) => HttpResponse::Unauthorized().body("Invalid email or password")
            }
        }
    }
}

pub async fn login_club_post(state: Data<AppState>, data: web::Form<LoginForm>, req: HttpRequest, session: Session) -> impl Responder {
    let email = data.email.as_str();
    let password = Option::from(data.password_hash.as_str());

    match password {
        None => HttpResponse::Unauthorized().body("Must provide a password"),
        Some(pass) => {
            let email_validation = EmailWithoutComment::parse_string(email);
            match email_validation {
                Ok(_) => {
                    match sqlx::query_as::<_, AuthClub>(
                        "SELECT CAST(club_uid AS TEXT), name, password_hash, email
                            FROM club
                            WHERE email = $1"
                    )
                    .bind(email.to_string())
                    .fetch_one(&state.db)
                    .await
                    {
                        Ok(club) => {
                            let is_valid = bcrypt::verify(pass.to_string(), &club.password_hash).unwrap();
                            if is_valid {
                                session::generate_club_session(&club, session).unwrap();

                                let settings = settings::get_settings();

                                let cookie = Cookie::build(settings.auth_cookie_name.clone(), secure_token::generate_token(email, req.path()))
                                    .path("/")
                                    .secure(true)
                                    .http_only(false)
                                    .finish();
                                let val = HeaderValue::from_str(cookie.to_string().as_str()).unwrap();

                                HttpResponse::Ok()
                                    .insert_header((SET_COOKIE, val))
                                    .json(json!({ "redirect": "/club" }))
                            } else {
                                HttpResponse::Unauthorized()
                                    .body("Invalid club or password")
                            }
                        }
                        Err(_) => {
                            HttpResponse::Unauthorized()
                                .body("Invalid club or password")
                        }
                    }
                },
                Err(_) => {
                    HttpResponse::Unauthorized().body("Invalid club or password")
                }
            }
        }
    }
}

pub async fn login_prefect_post(state: Data<AppState>, data: web::Form<LoginForm>, req: HttpRequest, session: Session) -> impl Responder {
    let email = data.email.as_str();
    let password = Option::from(data.password_hash.as_str());

    match password {
        None => HttpResponse::Unauthorized().body("Must provide a password"),
        Some(pass) => {
            let email_validation = EmailWithoutComment::parse_string(email);
            match email_validation {
                Ok(_) => {
                    match sqlx::query_as::<_, AuthPrefect>(
                        "SELECT prefect_uid, first_name, last_name, email, password_hash 
                            FROM prefect 
                            WHERE email = $1"
                    )
                    .bind(email)
                    .fetch_one(&state.db)
                    .await
                    {
                        Ok(prefect) => {
                            let is_valid = bcrypt::verify(pass.to_string(), &prefect.password_hash).unwrap();
                            if is_valid {
                                session::generate_prefect_session(&prefect, &session).unwrap();

                                let settings = settings::get_settings();

                                let cookie = Cookie::build(settings.auth_cookie_name.clone(), secure_token::generate_token(email, req.path()))
                                    .path("/")
                                    .secure(true)
                                    .http_only(false)
                                    .finish();
                                let val = HeaderValue::from_str(cookie.to_string().as_str()).expect("bar");

                                HttpResponse::Ok()
                                    .insert_header((SET_COOKIE, val))
                                    .json(json!({ "redirect": "/prefect"}))
                            } else {
                                HttpResponse::Unauthorized()
                                    .body("Invalid prefect ID or password")
                            }
                        }
                        Err(_) => HttpResponse::Unauthorized().body("Invalid prefect ID or password")
                    }
                }
                Err(_) => HttpResponse::Unauthorized().body("Invalid email or password")
            }
        }
    }
}