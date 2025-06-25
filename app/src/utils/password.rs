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

pub async fn create_password_home() -> impl Responder {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("make-password.html", &context).unwrap();

    HttpResponse::Ok()
        .body(page_content)  
}