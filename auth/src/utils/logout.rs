use actix_web::{HttpRequest, HttpResponse, Responder};
use actix_session::Session;
use validators::serde_json::json;

use crate::settings;
 
pub async fn logout_post(req: HttpRequest, session: Session) -> impl Responder {
    let settings = settings::get_settings();
    let mut response = HttpResponse::Ok();

    if let Some(mut auth_cookie) = req.cookie(settings.auth_cookie_name.as_str()) {
        auth_cookie.make_removal();
        response.cookie(auth_cookie);
    }

    if let Some(mut csrf_cookie) = req.cookie("csrf") {
        csrf_cookie.make_removal();
        response.cookie(csrf_cookie);
    }

    session.purge();

    response.json(json!({ "redirect": "/" }))
}