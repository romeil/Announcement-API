use actix_web::{
    middleware::Logger, 
    web::Data, App, HttpServer, 
    cookie::{SameSite, Key}
};
use actix_session::{
    config::BrowserSession, config::CookieContentSecurity, 
    SessionMiddleware
};
use announcement_api::{app, build_ssl_accepter, connect_to_db, connect_to_redis, session, AppState};
use dotenv::dotenv;
use env_logger::Env;

use announcement_api::middleware::{
    check_login::CheckLogin, 
    check_token::CheckCSRFToken, 
    modify_token::ModifyCSRFToken,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = connect_to_db().await;
    let redis_store = connect_to_redis().await;
    let builder = build_ssl_accepter();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
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
            .wrap(Logger::default())
            .configure(app)
    })
    .bind_openssl("127.0.0.1:8080", builder)?
    .run()
    .await
}