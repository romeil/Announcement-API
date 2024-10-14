use std::fmt::format;

use actix_web::{web::{self, Data}, HttpResponse, Responder};
use dotenv::dotenv;
use mail_send::{mail_builder::MessageBuilder, smtp::message::Message, SmtpClientBuilder};
use serde::{Deserialize, Serialize};
use regex::Regex;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::AppState;


#[derive(Deserialize)]
pub struct ID {
    value: String,
}

#[derive(Serialize, FromRow)]
pub struct PendingUsers {
    user_uid: String,
    first_name: String,
    last_name: String,
    email: String,
    role: String,
    registration_id: String,
    password_hash: Option<String>,
}

// Web Pages:
// Home Page
// President Login Page
// Prefect Login Page
// President Main Page
// Prefect Main Page
// Logout Page
// New User Registration Page
// Password Creation Page

pub async fn signup_post(state: Data<AppState>, id: web::Form<ID>) -> impl Responder {
    dotenv().ok();
    let sender_pwd = std::env::var("SENDER_PWD").expect("SENDER_PWD must be set");
    let new_user_id = Option::from(id.value.as_str());

    match new_user_id {
        None => HttpResponse::Unauthorized().body("Must provide registration ID"),
        Some(id) => {
            if id.len() != 9 {
                HttpResponse::Unauthorized().body("Invalid registration ID")
            }
            else {
                match sqlx::query_as::<_, PendingUsers>(
                    "SELECT CAST(user_uid AS TEXT), first_name, last_name, email, role, registration_id, password_hash
                        FROM pending_users
                        WHERE registration_id = $1"
                )
                .bind(id.to_string())
                .fetch_one(&state.db)
                .await
                {
                    Ok(pending_user) => {
                        let email = pending_user.email.as_str();
                        let first_name = pending_user.first_name;
                        let last_name = pending_user.last_name;
                        let valid_user = format!("{first_name} {last_name}");


                        let message = MessageBuilder::new()
                            .from(("Romeil Grant", "romeil.grant.2023@wolmers.org"))
                            .to((valid_user.as_str(), email))
                            .subject("Wolmer's Boys' School Club President Online Registration") 
                            .html_body(
                            "
                            <table align=\"center\" style=\"width:100%;max-width:750px;padding-top:20px;padding-right:20px;padding-left:20px;border:1px solid #dddddd;border-radius:4px\">
                                <tbody>
                                    <tr>
                                        <td>
                                            <div align=\"left\" >
                                                <img alt=\"Wolmers Logo\" height=\"20%\" width=\"20%\" src=\"https://wolmersouthfla.org/2020/images/logos/wbs_logo.png\" >
                                            <div>
                                        <td>
                                    <tr>
                                    <tr style=\"font-family:Times New Roman, Timrs, serif\">
                                        <td>
                                            &nbsp;
                                            <p>
                                                <span style=\"font-family:Verdana,Geneva,sans-serif\">
                                                      Dear Romeil,
                                                    <br>
                                                    <br>
                                                      Thank you for your interest in Wolmer's Announcement System!&nbsp;
                                                    <br>
                                                    <br>
                                                      Use the attached link to begin creating your password
                                                <span>
                                            <p>
                                            <p>
                                                <span style=\"font-family:Verdana,Geneva,sans-serif\">
                                                    <br>
                                                      Sincerely,&nbsp;
                                                    <br>
                                                    <br>
                                                      Wolmer's Boys' School
                                                    <br>
                                                      WBS Coding Club
                                                <span>
                                            <p>
                                        <td>
                                    <tr>
                                <tbody>
                             <table>");

                        SmtpClientBuilder::new("smtp.outlook.com", 587)
                            .implicit_tls(false)
                            .credentials(("romeil.grant.2023@wolmers.org", sender_pwd.as_str()))
                            .connect()
                            .await
                            .unwrap()
                            .send(message)
                            .await
                            .unwrap();

                        HttpResponse::Ok().body("Check your email to create your password")
                    },
                    Err(e) => {
                        HttpResponse::Unauthorized().body(format!("{e}"))
                    }
                }
    
            }
        }
    }
}

pub async fn home() -> impl Responder {
    HttpResponse::Ok()
        .body("This will be the web app's sign-up page")
}