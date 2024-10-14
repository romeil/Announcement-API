use actix_web::{HttpResponse, Responder};

pub async fn home() -> impl Responder {
    HttpResponse::Ok()
        .body("This will be the announcement system's homepage")       
}