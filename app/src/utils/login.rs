use actix_web::{
    HttpResponse, Responder
};
use lazy_static::lazy_static;
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let source = "app/src/static/**/*.html"; 
        let tera = Tera::new(source).unwrap();
        tera
    };
}

// Create admin-login.html
pub async fn login_admin() -> impl Responder {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("admin-login.html", &context).unwrap();

    HttpResponse::Ok()
        .body(page_content)
}

pub async fn login_club() -> impl Responder {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("club-login.html", &context).unwrap();

    HttpResponse::Ok()
        .body(page_content)    
}

pub async fn login_prefect() -> impl Responder {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("prefect-login.html", &context).unwrap();

    HttpResponse::Ok()
        .body(page_content) 
}