use actix_web::{
    dev::ServiceRequest, 
    middleware::Logger, 
    web::{self, Data}, error::ErrorUnauthorized,
    HttpServer, App, Error, 
};
use actix_web_httpauth::{
    extractors::basic::{self, BasicAuth}, 
    middleware::HttpAuthentication
};
use serde::Serialize;
use sqlx::{self, FromRow};
use bcrypt;
use dotenv::dotenv;
use env_logger::Env;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

mod services;
use services::{
    fetch_all_club_announcements, 
    fetch_club_announcements_by_uuid, fetch_club_announcements_by_uuid_and_date, 
    fetch_club_announcements_by_date, 
    create_announcement
};

pub struct AppState {
    db: Pool<Postgres>,
}

#[derive(Serialize, FromRow)]
struct AuthUser {
    club_uid: String,
    name: String,
    password_hash: String,
}

async fn authenticator(req: ServiceRequest, creds: BasicAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let username = creds.user_id();
    let password = creds.password();

    let state = req.app_data::<Data<AppState>>().unwrap();

    match password {
        None => Err((ErrorUnauthorized("Must provide a password"), req)),
        Some(pass) => {
            match sqlx::query_as::<_, AuthUser>(
                "SELECT CAST(club_uid AS TEXT), name, password_hash 
                    FROM club WHERE name = $1"
            )
            .bind(username.to_string())
            .fetch_one(&state.db)
            .await
            {
                Ok(user) => {
                    let is_valid = bcrypt::verify(pass.to_string(), &user.password_hash).unwrap();
                    if is_valid {
                        Ok(req)
                    } else {
                        Err((ErrorUnauthorized("Invalid password"), req))
                    }
                }
                Err(_) => Err((ErrorUnauthorized("No such club exists"), req)),
            }
        }
    }
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

    // ------------------------------------TODO---------------------------------------
    // GET /admin -> fetch_all_club_announcemnts
    // GET /admin/date/{announcement_date} -> fetch_club_announcements_by_date
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {db: pool.clone()}))
            .app_data(basic::Config::default().realm("Wolmer's Boys' School Announcement System"))
            .wrap(HttpAuthentication::basic(authenticator))
            .wrap(Logger::default())
            .service(
                web::scope("club")
                    .route("", web::get().to(fetch_club_announcements_by_uuid))
                    .route("", web::post().to(create_announcement))
                    .route("date/{announcement_date}", web::get().to(fetch_club_announcements_by_uuid_and_date))
            )
            .service(
                web::scope("admin")
                    .route("", web::get().to(fetch_all_club_announcements))
                    .route("date/{date}", web::get().to(fetch_club_announcements_by_date))
            )
    })
    .bind_openssl("127.0.0.1:8080", builder)?
    .run()
    .await
}