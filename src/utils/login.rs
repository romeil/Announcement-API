use actix_web::{
    web::{self, Data}, 
    HttpResponse, Responder, cookie::Cookie};
use actix_session::Session;
use sqlx;
use serde::Deserialize;
use uuid::Uuid;

use crate::{settings, secure_token, session};
use crate::{AuthClub, AppState, AuthPrefect};

#[derive(Deserialize)]
pub struct LoginForm {
    username: String,
    password_hash: String,
}

pub async fn login_club_post(state: Data<AppState>, data: web::Form<LoginForm>, session: Session) -> impl Responder {
    let club = data.username.as_str();
    let password = Option::from(data.password_hash.as_str());

    match password {
        None => HttpResponse::Unauthorized().body("Must provide a password"),
        Some(pass) => {
            match sqlx::query_as::<_, AuthClub>(
                "SELECT CAST(club_uid AS TEXT), name, password_hash
                    FROM club
                    WHERE name = $1"
            )
            .bind(club.to_string())
            .fetch_one(&state.db)
            .await
            {
                Ok(club) => {
                    let is_valid = bcrypt::verify(pass.to_string(), &club.password_hash).unwrap();
                    if is_valid {
                        session::generate_club_session(&club, session).unwrap();

                        let settings = settings::get_settings();
                        HttpResponse::SeeOther()
                            .append_header(("Location", "/club"))
                            .cookie(
                                Cookie::build(settings.auth_cookie_name.clone(), secure_token::generate_token(&data.username))
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
        }
    }
}

pub async fn login_admin_post(state: Data<AppState>, data: web::Form<LoginForm>, session: Session) -> impl Responder {
    let username = data.username.as_str();
    let password = Option::from(data.password_hash.as_str());

    match password {
        None => HttpResponse::Unauthorized().body("Must provide a password"),
        Some(pass) => {
            let prefect_uid = Uuid::parse_str(username);
            match prefect_uid {
                Ok(uid) => {
                    match sqlx::query_as::<_, AuthPrefect>(
                        "SELECT prefect_uid, first_name, last_name, email, password_hash 
                            FROM prefect 
                            WHERE prefect_uid = $1"
                    )
                    .bind(uid)
                    .fetch_one(&state.db)
                    .await
                    {
                        Ok(prefect) => {
                            let is_valid = bcrypt::verify(pass.to_string(), &prefect.password_hash).unwrap();
                            if is_valid {
                                session::generate_admin_session(&prefect, session).unwrap();

                                let settings = settings::get_settings();
                                HttpResponse::SeeOther()
                                    .append_header(("Location", "/admin"))
                                    .cookie(
                                        Cookie::build(settings.auth_cookie_name.clone(), secure_token::generate_token(&data.username))
                                            .path("/")
                                            .secure(true)
                                            .http_only(false)
                                            .finish()
                                    )
                                    .finish()
                            } else {
                                HttpResponse::Unauthorized()
                                    .body("Invalid admin prefect UUID or password")
                            }
                        }
                        Err(_) => HttpResponse::Unauthorized().body("Invalid admin prefect UUID or password")
                    }
                }
                Err(_) => HttpResponse::Unauthorized().body("Invalid admin prefect UUID or password")
            }
        }
    }
}

pub async fn login_club() -> impl Responder {
    HttpResponse::Ok()
        .body("This will be the club president's login page")
}

pub async fn login_admin() -> impl Responder {
    HttpResponse::Ok()
        .body("This will be the admin prefect's login page")
}