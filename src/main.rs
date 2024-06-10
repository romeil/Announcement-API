use actix_web::{middleware::Logger, web::{self, Data}, HttpServer, App};
use actix_web_httpauth::{extractors::basic, middleware::HttpAuthentication};
use dotenv::dotenv;
use env_logger::Env;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use sqlx::{self, FromRow, postgres::PgPoolOptions, Pool, Postgres};
use serde::Serialize;

mod utils;

pub struct AppState {
    db: Pool<Postgres>,
}

#[derive(Serialize, FromRow)]
pub struct AuthClub {
    club_uid: String,
    name: String,
    password_hash: String,
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
    
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {db: pool.clone()}))
            .app_data(basic::Config::default().realm("Wolmer's Boys' School Announcement System"))
            .wrap(HttpAuthentication::basic(utils::middleware::authenticator))
            .wrap(Logger::default())
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