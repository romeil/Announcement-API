use actix_session::Session;
use actix_web::{web::{self, Data}, HttpResponse, Responder};

use common::{AppState, PendingUsers, ID};
use validators::serde_json::json;

pub async fn signup_post(state: Data<AppState>, id: web::Form<ID>, session: Session) -> impl Responder {
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
                        let user_email = pending_user.email;
                        session.insert("email", &user_email).unwrap();

                        HttpResponse::Ok()
                            .json(json!({ "redirect": "/create-pin" }))
                    },
                    Err(_) => {
                        HttpResponse::Unauthorized().body("Invalid registration ID")
                    }
                }
            }
        }
    }
}