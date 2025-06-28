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
use auth::app;
use common::{connect_to_db, connect_to_redis, build_ssl_accepter, AppState};
use auth::session::generate_key;
use dotenv::dotenv;
use env_logger::Env;

use auth::middleware::{
    check_login::CheckLogin, 
    check_token::CheckCSRFToken, 
    modify_token::ModifyCSRFToken,
};
use pasetors::{keys::{Generate, SymmetricKey}, paserk::FormatAsPaserk};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let sk = SymmetricKey::generate().unwrap();
    let mut paserk = String::new();
    sk.fmt(&mut paserk).unwrap();
    println!("Paserk: {:?}", paserk);

    let sk2 = SymmetricKey::generate().unwrap();
    let mut paserk2 = String::new();
    sk2.fmt(&mut paserk2).unwrap();
    println!("Paserk: {:?}", paserk2);

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
                    .allowed_origin("http://localhost:8080")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::CONTENT_TYPE])
                    .supports_credentials()
            )
            .wrap(
                SessionMiddleware::builder(redis_store.clone(), Key::from(&generate_key()))
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
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}