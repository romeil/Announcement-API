use actix_web::{HttpResponse, Responder};
use lazy_static::lazy_static;
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let source = "src/static/**/*"; 
        let tera = Tera::new(source).unwrap();
        tera
    };
}

pub async fn home() -> impl Responder {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("index.html", &context).unwrap();

    HttpResponse::Ok()
        .body(page_content)       
}