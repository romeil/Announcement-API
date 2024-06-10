use actix_web::{dev::ServiceRequest, web::Data, error::ErrorUnauthorized, Error};
use actix_web_httpauth::extractors::basic::BasicAuth;
use bcrypt;
use serde::Serialize;
use sqlx::{self, FromRow};
use uuid::Uuid;

use crate::{AppState, AuthClub};

#[derive(Serialize, FromRow)]
struct AuthPrefect {
    prefect_uid: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    password_hash: String,
}

pub async fn authenticator(req: ServiceRequest, creds: BasicAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let username = creds.user_id();
    let password = creds.password();

    let state = req.app_data::<Data<AppState>>().unwrap();
    let scope = req.path().split('/').collect::<Vec<&str>>()[1];

    match scope {
        "club" => {
            match password {
                None => Err((ErrorUnauthorized("Must provide a password"), req)),
                Some(pass) => {
                    match sqlx::query_as::<_, AuthClub>(
                        "SELECT CAST(club_uid AS TEXT), name, password_hash 
                            FROM club 
                            WHERE name = $1"
                    )
                    .bind(username.to_string())
                    .fetch_one(&state.db)
                    .await
                    {
                        Ok(club) => {
                            let is_valid = bcrypt::verify(pass.to_string(), &club.password_hash).unwrap();
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
        "admin" => {
            match password {
                None => Err((ErrorUnauthorized("Must provide a password"), req)),
                Some(pass) => {
                    let prefect_uid = Uuid::parse_str(username);
                    match prefect_uid {
                        Ok(uid) => {
                            match sqlx::query_as::<_, AuthPrefect>(
                                "SELECT prefect_uid, first_name, last_name, email, password_hash 
                                    FROM prefect 
                                    WHERE prefect_uid = $1"
                            )
                            .bind(uid)
                            .fetch_one(&state.db)
                            .await
                            {
                                Ok(prefect) => {
                                    let is_valid = bcrypt::verify(pass.to_string(), &prefect.password_hash).unwrap();
                                    if is_valid {
                                        Ok(req)
                                    } else {
                                        Err((ErrorUnauthorized("Invalid password"), req))
                                    }
                                }
                                Err(_) => Err((ErrorUnauthorized("Invalid admin prefect UUID"), req)),
                            }
                        }
                        Err(_) => Err((ErrorUnauthorized("Invalid admin prefect UUID"), req))
                    }
                }
            } 
        }
        _ => Err((ErrorUnauthorized("No such scope exists"), req))
    }
}
