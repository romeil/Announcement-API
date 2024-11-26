use actix_web::{web::{self, Data}, HttpResponse, Responder};
use dotenv::dotenv;
use lazy_static::lazy_static;
use tera::Tera;
use mail_send::{mail_builder::MessageBuilder, SmtpClientBuilder};
use serde::Deserialize;
use rand::Rng;

use crate::{AppState, PendingUsers};

#[derive(Deserialize)]
pub struct ID {
    value: String,
}


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
                    "SELECT CAST(user_uid AS TEXT), first_name, last_name, email, role, registration_id, temporary_pin, password_hash
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

                        let mut rng = rand::thread_rng();
                        let temporary_pin = rng.gen_range(100_000_000..1_000_000_000).to_string();

                        match sqlx::query_as::<_, PendingUsers>(
                            "UPDATE pending_users
                            SET temporary_pin = $1
                            WHERE registration_id = $2
                            RETURNING CAST(user_uid AS TEXT), first_name, last_name, email, role, registration_id, temporary_pin, password_hash
                            "
                        )
                        .bind(temporary_pin.as_str())
                        .bind(id.to_string())
                        .fetch_one(&state.db)
                        .await
                        {
                            Ok(_) => {
                                let message = MessageBuilder::new()
                                .from(("Romeil Grant", "romeil.grant.2023@wolmers.org"))
                                .to((valid_user.as_str(), email))
                                .subject("Wolmer's Boys' School Club President Online Registration") 
                                .html_body(format!(
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
                                                        Dear {first_name},
                                                        <br>
                                                        <br>
                                                        Thank you for your interest in Wolmer's Announcement System!&nbsp;
                                                        <br>
                                                        <br>
                                                        Use the attached link to begin creating your password.
                                                        <br>
                                                        <br>
                                                        When requested for a password, enter the following temporary PIN:
                                                        <br>
                                                        <br>
                                                        {temporary_pin}
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
                                <table>"));

                                SmtpClientBuilder::new("smtp.outlook.com", 587)
                                    .implicit_tls(false)
                                    .credentials(("romeil.grant.2023@wolmers.org", sender_pwd.as_str()))
                                    .connect()
                                    .await
                                    .unwrap()
                                    .send(message)
                                    .await
                                    .unwrap();

                                HttpResponse::SeeOther()
                                    .append_header(("Location", "activate"))
                                    .body("Check email to create your password")
                            },
                            Err(_) => {
                                HttpResponse::InternalServerError().body("Internal Server Error")
                            }
                        }
                    },
                    Err(_) => {
                        HttpResponse::Unauthorized().body("Invalid registration ID")
                    }
                }
            }
        }
    }
}

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let source = "src/static/**/*"; 
        let tera = Tera::new(source).unwrap();
        tera
    };
}


pub async fn home() -> impl Responder {
    let context = tera::Context::new();
    let page_content = TEMPLATES.render("registration.html", &context).unwrap();

    HttpResponse::Ok()
        .body(page_content)   
}