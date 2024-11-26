use actix_web::{
    cookie::Cookie, web::{self, Data}, HttpResponse, Responder};
use serde::Deserialize;
use lazy_static::lazy_static;
use tera::Tera;

use crate::{settings, secure_token};
use crate::{AppState, PendingUsers};

#[derive(Deserialize)]
pub struct TemporaryPin {
    value: String,
}

pub async fn temp_pin_post(state: Data<AppState>, temppin: web::Form<TemporaryPin>) -> impl Responder {
    let pin = Option::from(temppin.value.as_str());

    match pin {
        None => HttpResponse::Unauthorized().body("Must provide PIN"),
        Some(pin) => {
            if pin.len() != 9 {
                HttpResponse::Unauthorized().body("Invalid PIN")
            }
            else {
                match sqlx::query_as::<_, PendingUsers>(
                    "SELECT CAST(user_uid AS TEXT), first_name, last_name, email, role, registration_id, temporary_pin, password_hash
                        FROM pending_users
                        WHERE temporary_pin = $1"
                )
                .bind(pin)
                .fetch_one(&state.db)
                .await
                {
                    Ok(user) => {
                        let user_email = user.email;
                        let settings = settings::get_settings();

                        HttpResponse::SeeOther()
                        .append_header(("Location", "create-pin"))
                        .cookie(
                            Cookie::build(settings.auth_cookie_name.clone(), secure_token::generate_token(&user_email))
                                .path("/")
                                .secure(true)
                                .http_only(true)
                                .finish()
                        )
                        .finish()
                    },
                    Err(_) => {
                        HttpResponse::Unauthorized()
                            .body("Invalid PIN")
                    }
                }
            }
        }
    }
}

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let source = "src/static/**/*"; 
        let tera = Tera::new(source).unwrap();
        tera
    };
}

pub async fn temp_pin_home() -> impl Responder {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("make-password.html", &context).unwrap();

    HttpResponse::Ok()
        .body(page_content)   
}