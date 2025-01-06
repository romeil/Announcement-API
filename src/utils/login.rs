use actix_web::{
    cookie::Cookie, web::{self, Data}, HttpRequest, HttpResponse, Responder};
use actix_session::Session;
use lazy_static::lazy_static;
use tera::Tera;
use sqlx;
use validators::prelude::*;
use validators::models::Host;

use crate::{settings, secure_token, session};
use crate::{AuthClub, AppState, AuthPrefect, LoginForm};

#[derive(Validator)]
#[validator(email(comment(Disallow), ip(Allow), local(Allow), at_least_two_labels(Allow), non_ascii(Allow)))]
pub struct EmailWithoutComment {
    pub local_part: String,
    pub need_quoted: bool,
    pub domain_part: Host,
}

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let source = "src/static/**/*.html"; 
        let tera = Tera::new(source).unwrap();
        tera
    };
}

pub async fn login_club() -> impl Responder {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("club-login.html", &context).unwrap();

    HttpResponse::Ok()
        .body(page_content)    
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
                                println!("{:?}", req.path());
                                HttpResponse::SeeOther()
                                    .append_header(("Location", "/club"))
                                    .cookie(
                                        Cookie::build(settings.auth_cookie_name.clone(), secure_token::generate_token(email, req.path()))
                                            .path("/")
                                            .secure(true)
                                            .http_only(true)
                                            .finish()
                                    )
                                    .finish()
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

pub async fn login_admin() -> impl Responder {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("prefect-login.html", &context).unwrap();

    HttpResponse::Ok()
        .body(page_content) 
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
                                session::generate_admin_session(&prefect, &session).unwrap();

                                let settings = settings::get_settings();
                                HttpResponse::SeeOther()
                                    .append_header(("Location", "/admin"))
                                    .cookie(
                                        Cookie::build(settings.auth_cookie_name.clone(), secure_token::generate_token(email, req.path()))
                                            .path("/")
                                            .secure(true)
                                            .http_only(false)
                                            .finish()
                                    )
                                    .finish()
                            } else {
                                HttpResponse::Unauthorized()
                                    .body("Invalid admin prefect ID or password")
                            }
                        }
                        Err(_) => HttpResponse::Unauthorized().body("Invalid admin prefect ID or password")
                    }
                }
                Err(_) => HttpResponse::Unauthorized().body("Invalid email or password")
            }
        }
    }
}