use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserIdentity {
    identity: String,
}

pub async fn home_post(data: web::Form<UserIdentity>) -> impl Responder {
    let identity = data.identity.as_str();

    match identity {    
        "admin" => {
            HttpResponse::SeeOther()
                .append_header(("Location", "/login/admin"))
                .finish()
        }
        "club" => {
            HttpResponse::SeeOther()
                .append_header(("Location", "/login/club"))
                .finish()
        }
        _ => {
            HttpResponse::NotFound()
            .append_header(("Location", "/"))
            .finish()
        }
    }
}

pub async fn home() -> impl Responder {
    HttpResponse::Ok()
        .body("This will be the announcement system's homepage")       
}