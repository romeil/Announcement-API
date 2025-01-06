use actix_web::{web::{self, Data}, HttpResponse, Responder};
use dotenv::dotenv;
use lazy_static::lazy_static;
use tera::Tera;
use serde::Deserialize;

use crate::{AppState, PendingUsers};

#[derive(Deserialize)]
pub struct ID {
    value: String,
}


pub async fn signup_post(state: Data<AppState>, id: web::Form<ID>) -> impl Responder {
    dotenv().ok();
    let new_user_id = Option::from(id.value.as_str());

    match new_user_id {
        None => HttpResponse::Unauthorized().body("Must provide registration ID"),
        Some(id) => {
            if id.len() != 9 {
                HttpResponse::Unauthorized().body("Invalid registration ID")
            }
            else {
                match sqlx::query_as::<_, PendingUsers>(
                    "SELECT CAST(user_uid AS TEXT), first_name, last_name, email, role, registration_id, temporary_pin, password_hash
                        FROM pending_users
                        WHERE registration_id = $1"
                )
                .bind(id.to_string())
                .fetch_one(&state.db)
                .await
                {
                    Ok(_pending_user) => {
                        HttpResponse::SeeOther()
                            .append_header(("Location", "create-pin"))
                            .body("Create your new password")
                    },
                    Err(_) => {
                        HttpResponse::Unauthorized().body("Invalid registration ID")
                    }
                }
            }
        }
    }
}

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let source = "src/static/**/*.html"; 
        let tera = Tera::new(source).unwrap();
        tera
    };
}


pub async fn home() -> impl Responder {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("authenticate.html", &context).unwrap();

    HttpResponse::Ok()
        .body(page_content)   
}