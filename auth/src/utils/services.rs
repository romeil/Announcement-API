use actix_session::Session;
use actix_web::{
    web::{Data, Form}, 
    HttpResponse, Responder
};
use lazy_static::lazy_static;
use tera::Tera;
use serde::{Deserialize, Serialize} ;
use sqlx::{self, postgres::PgRow, Error, FromRow, Row};
use uuid::{self, Uuid};

use common::{update_club_session, AppState, AuthClub};

#[derive(Serialize, Deserialize, Debug)]
struct Announcement {
    announcement_uid: String,
    info: String,
    date: String,
    club_uid: Uuid
}

#[derive(Serialize, Deserialize, Debug)]
struct AnnouncementWithClubName {
    announcement_uid: String,
    info: String,
    date: String,
    club_name: String,
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

impl<'r> FromRow<'r, PgRow> for AnnouncementWithClubName {
    fn from_row(row: &'r PgRow) -> Result<Self, Error> {
        let announcement_uid: String = row.try_get("announcement_uid")?;
        let info: String = row.try_get("info")?;
        let date: String = row.try_get("date")?;
        let club_name: String = row.try_get("name")?;

        Ok(AnnouncementWithClubName { announcement_uid, info, date, club_name })
    }
}

#[derive(Deserialize, Serialize)]
pub struct CreateAnnouncement {
    pub info: String,
    pub date: String,
}

#[derive(Deserialize, Serialize)]
pub struct PrefectCreateAnnouncement {
    pub name: String,
    pub info: String,
    pub date: String,
}

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let source = "app/src/static/**/*.html"; 
        let tera = Tera::new(source).unwrap();
        tera
    };
}

pub async fn create_club_announcement(state: Data<AppState>, body: Form<CreateAnnouncement>, session: Session) -> impl Responder {
    let email = session.get::<AuthClub>("club_auth").unwrap().unwrap().email;

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
                .fetch_all(&state.db)
                .await
            {
                Ok(announcements) => {
                    update_club_session(session).unwrap();

                    let mut context = tera::Context::new();
                    context.insert("announcements", &announcements);
                    let page_content = TEMPLATES.render("announcements.html", &context).unwrap();

                    HttpResponse::Ok()
                        .body(page_content)       
                },
                Err(_) => HttpResponse::InternalServerError().json("Failed to create club announcement"),
            }
        }
        Err(_) => HttpResponse::Unauthorized().json("Invalid email or password")
    }
}

pub async fn create_club_announcement_prefect(state: Data<AppState>, body: Form<PrefectCreateAnnouncement>) -> impl Responder {
    
    match sqlx::query_as::<_, AnnouncementWithClubName>(
        "INSERT INTO announcement (announcement_uid, info, date, club_uid)
        VALUES ($1, $2, $3, (SELECT club_uid FROM club WHERE name = $4))
        RETURNING CAST(announcement_uid AS TEXT), info, date, (SELECT name FROM club WHERE club_uid = announcement.club_uid)"
    )
        .bind(Uuid::new_v4())
        .bind(body.info.to_string())
        .bind(body.date.to_string())
        .bind(body.name.to_string())
        .fetch_all(&state.db)
        .await
    {
        Ok(announcements) => {
            let mut context = tera::Context::new();
            context.insert("announcements", &announcements);
            let page_content = TEMPLATES.render("prefect-announcements.html", &context).unwrap();

            HttpResponse::Ok()
                .body(page_content)       
        },
        Err(_) => HttpResponse::InternalServerError().json("An unexpected error occured"),
    }
}