use actix_web::web::{self, ServiceConfig};
use actix_files as fs;
use actix_session::{self, storage::RedisSessionStore};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use sqlx::{self, FromRow, Pool, Postgres, postgres::PgPoolOptions};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

pub mod utils;
pub mod middleware;

pub mod settings;
pub mod secure_token;
pub mod session;

pub struct AppState {
    pub db: Pool<Postgres>,
}

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password_hash: String,
}

#[derive(Serialize, FromRow)]
pub struct AuthClub {
    pub club_uid: String,
    pub name: String,
    pub password_hash: String,
    pub email: String,
}

#[derive(Serialize, FromRow)]
pub struct AuthPrefect {
    pub prefect_uid: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Serialize, FromRow)]
pub struct PendingUsers {
    pub user_uid: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub role: String,
    pub registration_id: String,
    pub temporary_pin: Option<String>,
    pub password_hash: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ID {
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct NewPassword {
    pub new_password: String,
    pub confirm_password: String,
}

pub async fn connect_to_db() -> Pool<Postgres> {
    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await.expect("foo");
    pool
}

pub async fn connect_to_redis() -> RedisSessionStore {
    let redis_store = RedisSessionStore::new("redis://127.0.0.1:6379")
        .await
        .unwrap();
    redis_store
}

pub fn build_ssl_accepter() -> SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("keys/key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("keys/cert.pem").unwrap();
    builder
}

pub fn app(app: &mut ServiceConfig) -> () {
    app
        .service(
            fs::Files::new("/src/static", "./src/static").show_files_listing()
        )
        .service(
            fs::Files::new("/src/static/js", "./src/static/js").show_files_listing()
        )  
        .service(
            fs::Files::new("/src/img", "./src/img").show_files_listing()
        )
        .service(
            web::resource("/")
                .route(web::get().to(utils::home::home))
        )
        .service(
            web::resource("register")
                .route(web::get().to(utils::signup::home))
                .route(web::post().to(utils::signup::signup_post))
        )
        .service(
            web::resource("create-pin")
                .route(web::get().to(utils::password::create_password_home))
                .route(web::post().to(utils::password::create_password_post))
        )   
        .service(
            web::scope("login")
                .service(
                    web::resource("club")
                        .route(web::get().to(utils::login::login_club))
                        .route(web::post().to(utils::login::login_club_post))
                )
                .service(
                    web::resource("admin")
                        .route(web::get().to(utils::login::login_admin))
                        .route(web::post().to(utils::login::login_admin_post))
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
        );
}