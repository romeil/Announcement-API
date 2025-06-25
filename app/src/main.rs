use actix_cors::Cors;
use actix_web::{
    middleware::Logger, 
    web::Data, App, HttpServer, 
    cookie::{SameSite, Key}, http
};
use actix_session::{
    config::BrowserSession, config::CookieContentSecurity, 
    SessionMiddleware
};
use app::app;
use dotenv::dotenv;
use env_logger::Env;

use common::{AppState, connect_to_db, build_ssl_accepter, connect_to_redis};
use auth::{middleware::check_login::CheckLogin, session};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = connect_to_db().await;
    let redis_store = connect_to_redis().await;
    let _builder = build_ssl_accepter();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {db: pool.clone()}))
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:8000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::CONTENT_TYPE])
                    .supports_credentials()
            )
            .wrap(
                SessionMiddleware::builder(redis_store.clone(), Key::from(&session::generate_key()))
                    .cookie_secure(false)
                    .session_lifecycle(BrowserSession::default())
                    .cookie_same_site(SameSite::Strict)
                    .cookie_content_security(CookieContentSecurity::Private)
                    .cookie_http_only(true)
                    .build()
            )
            .wrap(CheckLogin)
            .wrap(Logger::default())
            .configure(app)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}