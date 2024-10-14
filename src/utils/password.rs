use actix_web::{
    web::{self, Data}, HttpResponse, Responder};
use serde::Deserialize;
use validators::prelude::*;

use crate::{AppState};

#[derive(Validator)]
#[validator(text(char_length(trimmed_min = 1, min = 1, max = 1000)))] 
pub struct TextNotAllowEmpty(pub String);

#[derive(Deserialize)]
pub struct NewPassword {
    new_password: String,
    confirm_password: String,
}

pub async fn create_password(state: Data<AppState>, password_form: web::Form<NewPassword>) -> impl Responder {
    let new_password = Option::from(password_form.new_password.as_str());
    let confirm_password = Option::from(password_form.confirm_password.as_str());

    match new_password {
        None => HttpResponse::BadRequest().body("Please provide a password"),
        Some(pwd) => {
            let password_validation = TextNotAllowEmpty::parse_str(pwd);
            match password_validation {
                Ok(_) => {
                    match confirm_password {
                        None => HttpResponse::BadRequest().body("Please provide a password"),
                        Some(confirm_pwd) => {
                            let confirm_password_validation = TextNotAllowEmpty::parse_str(confirm_pwd);
                            match confirm_password_validation {
                                Ok(_) => {
                                    if pwd == confirm_pwd {
                                        HttpResponse::SeeOther()
                                            .append_header(("Location", "/login"))
                                            .finish()
                                    }
                                    else {
                                        HttpResponse::BadRequest()
                                            .body("The passwords do not match")
                                    }
                                },
                                Err(_) => {
                                    HttpResponse::BadRequest()
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