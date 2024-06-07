use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod services;
use services::{fetch_all_club_announcements, fetch_club_announcements_by_uuid, 
    fetch_club_announcements_by_uuid_and_date, fetch_club_announcements_by_date, create_announcement
};

pub struct AppState {
    db: Pool<Postgres>,
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

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {db: pool.clone()}))
            .service(fetch_all_club_announcements)
            .service(fetch_club_announcements_by_uuid)
            .service(fetch_club_announcements_by_uuid_and_date)
            .service(fetch_club_announcements_by_date)
            .service(create_announcement)
    })
    .bind_openssl("127.0.0.1:8080", builder)?
    .run()
    .await
}