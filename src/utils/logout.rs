use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_session::Session;

use crate::settings;

pub async fn logout() -> impl Responder {
    HttpResponse::Ok()
        .body("This will be the announcement system's logout page")       
}

pub async fn logout_post(req: HttpRequest, session: Session) -> impl Responder {
    let settings = settings::get_settings();
    let mut cookie = req.cookie(settings.auth_cookie_name.as_str()).unwrap();
    cookie.make_removal();

    session.purge();

    HttpResponse::SeeOther()
        .append_header(("Location", "/"))
        .cookie(cookie)
        .finish()
}