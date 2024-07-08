use actix_web::{
    middleware::Logger, 
    web::{self, Data}, App, HttpServer, 
    cookie::{SameSite, Key}
};
use actix_session::{
    config::BrowserSession, config::CookieContentSecurity, 
    storage::RedisSessionStore, SessionMiddleware
};
use dotenv::dotenv;
use env_logger::Env;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use sqlx::{self, FromRow, postgres::PgPoolOptions, Pool, Postgres};
use serde::Serialize;
use uuid::Uuid;

use middleware::{
    check_login::CheckLogin, 
    check_token::CheckCSRFToken, 
    modify_token::ModifyCSRFToken,
    content_type::CheckContentType,
};

mod utils;
mod middleware;
pub mod settings;
pub mod secure_token;
pub mod session;

pub struct AppState {
    db: Pool<Postgres>,
}

#[derive(Serialize, FromRow)]
pub struct AuthClub {
    club_uid: String,
    name: String,
    password_hash: String,
}

#[derive(Serialize, FromRow)]
pub struct AuthPrefect {
    prefect_uid: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    pub password_hash: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {    
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    let redis_store = RedisSessionStore::new("redis://127.0.0.1:6379")
        .await
        .unwrap();
    
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("keys/key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("keys/cert.pem").unwrap();

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
            .wrap(CheckContentType)
            .wrap(Logger::default())
            .service(
                web::resource("/")
                    .route(web::get().to(utils::home::home))
                    .route(web::post().to(utils::home::home_post))
            )
            .service(
                web::scope("login")
                    .service(
                        web::resource("admin")
                            .route(web::get().to(utils::login::login_admin))
                            .route(web::post().to(utils::login::login_admin_post))
                    )
                    .service(
                        web::resource("club")
                            .route(web::get().to(utils::login::login_club))
                            .route(web::post().to(utils::login::login_club_post))
                    )
            )
            .service(
                web::resource("logout")
                    .route(web::get().to(utils::logout::logout))
                    .route(web::post().to(utils::logout::logout_post))
            )
            .service(
                web::scope("club")
                    .route("", web::get().to(utils::services::fetch_club_announcements_by_uuid))
                    .route("", web::post().to(utils::services::create_club_announcement))
                    .route("date/{announcement_date}", web::get().to(utils::services::fetch_club_announcements_by_uuid_and_date))
            )
            .service(
                web::scope("admin")
                    .route("", web::get().to(utils::services::fetch_all_club_announcements))
                    .route("date/{date}", web::get().to(utils::services::fetch_club_announcements_by_date))
            )
    })
    .bind_openssl("127.0.0.1:8080", builder)?
    .run()
    .await
}