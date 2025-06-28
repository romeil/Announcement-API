use actix_session::{storage::RedisSessionStore, Session};
use actix_web::Error;
use chrono::Utc;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use serde::{Serialize, Deserialize};
use sqlx::{postgres::PgPoolOptions, FromRow, Pool, Postgres};
use uuid::Uuid;

pub struct AppState {
    pub db: Pool<Postgres>,
}

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct AuthAdmin {
    pub admin_uid: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, FromRow)]
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

pub fn update_club_session(session: Session) -> Result<(), Error> {
    session.insert("last_modified", Utc::now().to_string())?;
    Ok(())
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
        .set_private_key_file("C:/certs/key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("C:/certs/cert.pem").unwrap();
    builder
}
