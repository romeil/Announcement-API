use actix_web::{HttpRequest, HttpResponse, Responder};

use crate::settings;

pub async fn logout() -> impl Responder {
    HttpResponse::Ok()
        .body("This will be the announcement system's logout page")       
}

pub async fn logout_post(req: HttpRequest) -> impl Responder {
    let settings = settings::get_settings();
    let mut cookie = req.cookie(settings.auth_cookie_name.as_str()).unwrap();
    cookie.make_removal();

    HttpResponse::SeeOther()
        .append_header(("Location", "/"))
        .cookie(cookie)
        .finish()
}