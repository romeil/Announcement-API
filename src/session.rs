use actix_web::Error;
use actix_session::Session;
use chrono::Utc;
use dotenv::dotenv;

use crate::{AuthClub, AuthPrefect};

pub fn generate_key() -> [u8; 64] {
    dotenv().ok();
    let csrf_token_key_str = std::env::var("SESSION_KEY").expect("CSRF_TOKEN_KEY must be set");

    let trimmed = csrf_token_key_str.trim_matches(|c: char| c == '[' || c == ']');
    let number_strings: Vec<&str> = trimmed.split(',').collect();

    let csrf_token_key_vec = number_strings
        .into_iter()
        .map(|s| s.trim().parse::<u8>())
        .collect::<Result<Vec<u8>, _>>()
        .unwrap();

    let csrf_token_key: [u8; 64] = csrf_token_key_vec.try_into().unwrap();

    csrf_token_key
}

pub fn generate_club_session(club: &AuthClub, session: Session) -> Result<(), Error> {
    session.insert("club_auth", club)?;
    session.insert("created_at", Utc::now().to_string())?;
    session.insert("last_modified", Utc::now().to_string())?;
    session.insert("club_id", &club.club_uid)?;
    Ok(())
}

pub fn generate_admin_session(prefect: &AuthPrefect, session: Session) -> Result<(), Error> {
    session.insert("prefect_auth", prefect)?;
    session.insert("created_at", Utc::now().to_string())?;
    session.insert("last_modified", Utc::now().to_string())?;
    session.insert("prefect_id", &prefect.prefect_uid)?;
    Ok(())
}