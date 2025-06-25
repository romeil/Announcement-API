use actix_session::Session;
use actix_web::{
    web::{Data, Path}, 
    HttpResponse, Responder
};
use lazy_static::lazy_static;
use tera::Tera;
use serde::{Deserialize, Serialize} ;
use sqlx::{self, postgres::PgRow, Error, FromRow, Row};
use uuid::{self, Uuid};

use common::{AppState, AuthClub};

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

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let source = "app/src/static/**/*.html"; 
        let tera = Tera::new(source).unwrap();
        tera
    };
}

pub async fn fetch_club_announcements_by_uuid(state: Data<AppState>, session: Session) -> impl Responder {
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
            match sqlx::query_as::<_,  Announcement>(
                "SELECT CAST(announcement_uid AS TEXT), info, date, club_uid  
                    FROM announcement WHERE club_uid = $1"
            )
                .bind(Uuid::parse_str(club.club_uid.as_str()).expect("Error in parsing UUID string literal"))
                .fetch_all(&state.db)
                .await
            {
                Ok(announcements) => {
                    let mut context = tera::Context::new();
                    context.insert("announcements", &announcements);
                    let page_content = TEMPLATES.render("announcements.html", &context).unwrap();

                    HttpResponse::Ok()
                        .body(page_content)       
                },
                Err(_) => HttpResponse::NotFound().json("No announcements found"),
            }
        }
        Err(_) => HttpResponse::Unauthorized().json("Invalid email or password")
    }
}

pub async fn fetch_club_announcements_by_uuid_and_date(state: Data<AppState>, path: Path<String>, session: Session) -> impl Responder {
    let date = path.into_inner();
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
                "SELECT CAST(announcement_uid AS TEXT), info, date, club_uid 
                    FROM announcement 
                    WHERE club_uid = $1 AND date = $2"
            )
                .bind(Uuid::parse_str(club.club_uid.as_str()).expect("Error in parsing UUID string literal"))
                .bind(&date)
                .fetch_all(&state.db)
                .await
            {
                Ok(announcements) => {
                    let mut context = tera::Context::new();
                    context.insert("announcements", &announcements);
                    let page_content = TEMPLATES.render("announcements.html", &context).unwrap();

                    HttpResponse::Ok()
                        .body(page_content)       
                },
                Err(_) => HttpResponse::NotFound().json("No announcements found"),
            }
        }
        Err(_) => HttpResponse::NotFound().json("No such club exists")
    }
}

pub async fn fetch_all_club_announcements(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, AnnouncementWithClubName>(
        "SELECT CAST(announcement.announcement_uid AS TEXT), announcement.info, announcement.date, club.name
            FROM announcement
            JOIN club
            ON announcement.club_uid = club.club_uid"
    )
        .fetch_all(&state.db)
        .await
    {
        Ok(announcements) => {
            match sqlx::query_as::<_, AuthClub>(
                "SELECT CAST(club_uid AS TEXT), name, password_hash, email
                FROM club"
            )
            .fetch_all(&state.db)
            .await
            {
                Ok(clubs) => {
                    let mut context = tera::Context::new();
                    context.insert("announcements", &announcements);
                    context.insert("clubs", &clubs);
                    let page_content = TEMPLATES.render("prefect-announcements.html", &context).unwrap();

                    HttpResponse::Ok()
                        .body(page_content)   
                },
                Err(_) => HttpResponse::InternalServerError().body("An unexpected error occured."),
            }
        } ,
        Err(_) => HttpResponse::InternalServerError().body("An unexpected error occured."),
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
        Ok(announcements) => {
            let mut context = tera::Context::new();
            context.insert("announcements", &announcements);
            let page_content = TEMPLATES.render("announcements.html", &context).unwrap();

            HttpResponse::Ok()
                .body(page_content)     
        },
        Err(_) => HttpResponse::NotFound().json("No announcements found"),
    }
}