use actix_session::Session;
use actix_web::{
    web::{Data, Json, Path}, 
    HttpRequest, HttpResponse, Responder
};
use serde::{Deserialize, Serialize} ;
use sqlx::{self, postgres::PgRow, Error, FromRow, Row};
use uuid::{self, Uuid};

use crate::{settings, session, AppState, AuthClub};

#[derive(Serialize, Debug)]
struct Announcement {
    announcement_uid: String,
    info: String,
    date: String,
    club_uid: Uuid
}

impl<'r> FromRow<'r, PgRow> for Announcement {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let announcement_uid: String = row.try_get("announcement_uid")?;
        let info: String = row.try_get("info")?;
        let date: String = row.try_get("date")?;
        let club_uid: Uuid = row.try_get("club_uid")?;

        Ok(Announcement { announcement_uid, info, date, club_uid })
    }
}

#[derive(Deserialize)]
pub struct CreateAnnouncement {
    pub info: String,
    pub date: String,
}

fn get_club_email(req: HttpRequest) -> String {
    let settings = settings::get_settings();
    let cookie = req.cookie(settings.auth_cookie_name.as_str()).unwrap();
    let club_name = crate::secure_token::verify_token(cookie.value()).unwrap();
    club_name.replace("\"", "")
}

pub async fn fetch_club_announcements_by_uuid(state: Data<AppState>, req: HttpRequest) -> impl Responder {
    let email = get_club_email(req);

    match sqlx::query_as::<_, AuthClub>(
        "SELECT CAST(club_uid AS TEXT), name, password_hash, email
            FROM club WHERE email = $1"
    )
    .bind(email)
    .fetch_one(&state.db)
    .await
    {
        Ok(club) => {
            match sqlx::query_as::<_,  Announcement>(
                "SELECT CAST(announcement_uid AS TEXT), info, date, club_uid  
                    FROM announcement WHERE club_uid = $1"
            )
                .bind(Uuid::parse_str(club.club_uid.as_str()).expect("Error in parsing UUID string literal"))
                .fetch_all(&state.db)
                .await
            {
                Ok(announcements) => {
                    HttpResponse::Ok().json(announcements)
                },
                Err(_) => HttpResponse::NotFound().json("No announcements found"),
            }
        }
        Err(_) => HttpResponse::Unauthorized().json("Invalid email or password")
    }
}

pub async fn create_club_announcement(state: Data<AppState>, body: Json<CreateAnnouncement>, req: HttpRequest, session: Session) -> impl Responder {
    let email = get_club_email(req.clone());

    match sqlx::query_as::<_, AuthClub>(
        "SELECT CAST(club_uid AS TEXT), name, password_hash, email
            FROM club WHERE email = $1"
    )
    .bind(email)
    .fetch_one(&state.db)
    .await
    {
        Ok(club) => {
            match sqlx::query_as::<_, Announcement>(
                "INSERT INTO announcement (announcement_uid, info, date, club_uid) 
                    VALUES ($1, $2, $3, $4) 
                    RETURNING CAST(announcement_uid AS TEXT), info, date, club_uid"
            )
                .bind(Uuid::new_v4())
                .bind(body.info.to_string())
                .bind(body.date.to_string())
                .bind(Uuid::parse_str(club.club_uid.as_str()).expect("Error in parsing UUID string literal"))
                .fetch_one(&state.db)
                .await
            {
                Ok(announcement) => {
                    session::update_club_session(session).unwrap();
                    HttpResponse::Ok()
                        .json(announcement)
                },
                Err(_) => HttpResponse::InternalServerError().json("Failed to create club announcement"),
            }
        }
        Err(_) => HttpResponse::Unauthorized().json("Invalid email or password")
    }
}

pub async fn fetch_club_announcements_by_uuid_and_date(state: Data<AppState>, path: Path<String>, req: HttpRequest) -> impl Responder {
    let date = path.into_inner();
    let email = get_club_email(req);

    match sqlx::query_as::<_, AuthClub>(
        "SELECT CAST(club_uid AS TEXT), name, password_hash, email
            FROM club WHERE email = $1"
    )
    .bind(email)
    .fetch_one(&state.db)
    .await
    {
        Ok(club) => {
            match sqlx::query_as::<_, Announcement>(
                "SELECT CAST(announcement_uid AS TEXT), info, date, club_uid 
                    FROM announcement 
                    WHERE club_uid = $1 AND date = $2"
            )
                .bind(Uuid::parse_str(club.club_uid.as_str()).expect("Error in parsing UUID string literal"))
                .bind(&date)
                .fetch_all(&state.db)
                .await
            {
                Ok(announcements) => HttpResponse::Ok().json(announcements),
                Err(_) => HttpResponse::NotFound().json("No announcements found"),
            }
        }
        Err(_) => HttpResponse::NotFound().json("No such club exists")
    }
}

pub async fn fetch_all_club_announcements(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, Announcement>(
        "SELECT CAST(announcement_uid AS TEXT), info, date, club_uid 
            FROM announcement"
    )
        .fetch_all(&state.db)
        .await
    {
        Ok(announcements) => HttpResponse::Ok().json(announcements),
        Err(_) => HttpResponse::NotFound().json("No announcements found"),
    }
}

pub async fn fetch_club_announcements_by_date(state: Data<AppState>, path: Path<String>) -> impl Responder {
    let announcement_date: String = path.into_inner();

    match sqlx::query_as::<_,  Announcement>(
        "SELECT CAST(announcement_uid AS TEXT), info, date, club_uid  
            FROM announcement WHERE date = $1"
    )
        .bind(&announcement_date)
        .fetch_all(&state.db)
        .await
    {
        Ok(announcements) => HttpResponse::Ok().json(announcements),
        Err(_) => HttpResponse::NotFound().json("No announcements found"),
    }
}