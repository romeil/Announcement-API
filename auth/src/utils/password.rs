use actix_session::Session;
use actix_web::{
    web::{self, Data}, HttpResponse, Responder
};
use uuid::Uuid;
use validators::{prelude::*, serde_json::json};

use common::{AppState, AuthClub, AuthPrefect, PendingUsers, NewPassword};

#[derive(Validator)]
#[validator(text(char_length(trimmed_min = 1, min = 1, max = 1000)))] 
pub struct TextNotAllowEmpty(pub String);

pub async fn create_password_post(state: Data<AppState>, password_form: web::Form<NewPassword>, session: Session) -> impl Responder {
    let new_password = Option::from(password_form.new_password.as_str());
    let confirm_password = Option::from(password_form.confirm_password.as_str());

    match new_password {
        None => HttpResponse::Unauthorized().body("Please provide a password"),
        Some(pwd) => {
            let password_validation = TextNotAllowEmpty::parse_str(pwd);
            match password_validation {
                Ok(_) => {
                    match confirm_password {
                        None => HttpResponse::Unauthorized().body("Please provide a password"),
                        Some(confirm_pwd) => {
                            let confirm_password_validation = TextNotAllowEmpty::parse_str(confirm_pwd);
                            match confirm_password_validation {
                                Ok(_) => {
                                    if pwd == confirm_pwd {
                                        let uuid_value = Uuid::new_v4();
                                        let pwd_hash = bcrypt::hash(pwd, 6).unwrap();                                        
                                        let email = session.get::<String>("email").unwrap().unwrap();
                                        let email_str = email.as_str();

                                        match sqlx::query_as::<_, PendingUsers>(
                                            "SELECT CAST(user_uid AS TEXT), first_name, last_name, email, role, registration_id, temporary_pin, password_hash
                                                FROM pending_users
                                                WHERE email = $1"
                                        )
                                        .bind(email_str)
                                        .fetch_one(&state.db)
                                        .await
                                        {
                                            Ok(pending_user) => {
                                                let pending_user_role = pending_user.role;
                                                let username = pending_user.first_name + " " + pending_user.last_name.as_str();
                                                
                                                if pending_user_role.as_str() == "club" {
                                                    match sqlx::query_as::<_, AuthClub>(
                                                        "INSERT INTO club(club_uid, name, password_hash, email)
                                                            VALUES($1, $2, $3, $4)
                                                            RETURNING CAST(club_uid AS TEXT), name, password_hash, email"
                                                    )
                                                    .bind(uuid_value)
                                                    .bind(username)
                                                    .bind(pwd_hash)
                                                    .bind(email_str)
                                                    .fetch_one(&state.db)
                                                    .await
                                                    {
                                                        Ok(_) => {
                                                            HttpResponse::Ok()
                                                                .json(json!({ "redirect": "/login/club" }))
                                                        },
                                                        Err(e) => {
                                                            println!("{:?}", e);
                                                            HttpResponse::Unauthorized()
                                                                .body("Invalid PIN")
                                                        }
                                                    }     
                                                } else {
                                                    match sqlx::query_as::<_, AuthPrefect>(
                                                        "INSERT INTO prefect(prefect_uid, first_name, last_name, email, password_hash)
                                                            VALUES($1, $2, $3, $4, $5)
                                                            RETURNING CAST(prefect_uid AS TEXT), first_name, last_name, email, password_hash"
                                                    )
                                                    .bind(uuid_value)
                                                    .bind(username)
                                                    .bind(pwd_hash)
                                                    .bind(email_str)
                                                    .fetch_one(&state.db)
                                                    .await
                                                    {
                                                        Ok(_) => {
                                                            HttpResponse::Ok()
                                                                .json(json!({ "redirect": "/login/prefect" }))
                                                        },
                                                        Err(e) => {
                                                            println!("{:?}", e);
                                                            HttpResponse::Unauthorized()
                                                                .body("Invalid PIN")
                                                        }
                                                    }
                                                }
                                            },
                                            Err(e) => {
                                                println!("{:?}", e);
                                                HttpResponse::Unauthorized()
                                                    .body("Invalid PIN")
                                            }   
                                        }
                                    }
                                    else {
                                        HttpResponse::Unauthorized()
                                            .body("The passwords do not match")
                                    }
                                },
                                Err(e) => {
                                    println!("{:?}", e);
                                    HttpResponse::Unauthorized()
                                        .body("Please provide a password")
                                }
                            }
                        }
                    }
                },
                Err(_) => {
                    HttpResponse::BadRequest()
                        .body("Please provide a password")
                }
            }
        }
    }
}