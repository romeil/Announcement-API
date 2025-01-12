use actix_http::Request;
use actix_session::{
    config::{BrowserSession, CookieContentSecurity}, 
    SessionMiddleware
};
use actix_web::{
    body::{BoxBody, EitherBody}, 
    cookie::{Key, SameSite}, 
    dev::{Service, ServiceResponse}, 
    test, web::Data, App, Error
};
use announcement_api::{
    app, connect_to_db, connect_to_redis, 
    middleware::{check_login::CheckLogin, check_token::CheckCSRFToken, modify_token::ModifyCSRFToken}, 
    session, AppState
};
use dotenv::dotenv;

pub async fn app_w_middleware() -> impl Service<Request, Response = ServiceResponse<EitherBody<EitherBody<EitherBody<BoxBody>>>>, Error = Error>
{
    dotenv().ok();
    let pool = connect_to_db().await;
    let redis_store = connect_to_redis().await;

    let app = test::init_service(
    App::new()
            .app_data(Data::new(AppState {db: pool.clone()}))
            .wrap(
                SessionMiddleware::builder(redis_store.clone(), Key::from(&session::generate_key()))
                    .cookie_secure(true)
                    .session_lifecycle(BrowserSession::default())
                    .cookie_same_site(SameSite::Strict)
                    .cookie_content_security(CookieContentSecurity::Private)
                    .cookie_http_only(true)
                    .build()
            )
            .wrap(ModifyCSRFToken)
            .wrap(CheckCSRFToken)
            .wrap(CheckLogin)
            .configure(app)
    ).await;
    app
}