use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_session::Session;
use lazy_static::lazy_static;
use tera::Tera;

use crate::settings;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let source = "src/static/**/*"; 
        let tera = Tera::new(source).unwrap();
        tera
    };
}

pub async fn logout() -> impl Responder {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("logout.html", &context).unwrap();

    HttpResponse::Ok()
        .body(page_content)             
}

pub async fn logout_post(req: HttpRequest, session: Session) -> impl Responder {
    let settings = settings::get_settings();

    let mut auth_cookie = req.cookie(settings.auth_cookie_name.as_str()).unwrap();
    auth_cookie.make_removal();
    let mut csrf_cookie = req.cookie("csrf").unwrap();
    csrf_cookie.make_removal();

    session.purge();

    HttpResponse::SeeOther()
        .append_header(("Location", "/"))
        .cookie(auth_cookie)
        .cookie(csrf_cookie)
        .finish()
}