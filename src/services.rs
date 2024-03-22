use actix_web::{
    get, post,
    web::{Data, Json, Path},
    Responder, HttpResponse
};
use serde::{Deserialize, Serialize} ;
use sqlx::{
    self, postgres::PgRow, Error, FromRow
};
use uuid::Uuid;
use crate::AppState;
use sqlx::Row;


#[derive(Serialize)]
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
    pub club_uid: String,
}

#[get("/announcement")]
pub async fn fetch_all_club_announcements(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, Announcement>(
        "SELECT CAST(announcement_uid AS TEXT), info, date, club_uid FROM announcement"
    )
        .fetch_all(&state.db)
        .await
    {
        Ok(announcements) => HttpResponse::Ok().json(announcements),
        Err(_) => HttpResponse::NotFound().json("No announcements found"),
    }
}

#[get("/announcement/club/{club_uid}")]
pub async fn fetch_club_announcements_by_uuid(state: Data<AppState>, path: Path<String>) -> impl Responder {
    let club_uid: String = path.into_inner();

    match sqlx::query_as::<_,  Announcement>(
        "SELECT CAST(announcement_uid AS TEXT), info, date, club_uid  FROM announcement WHERE club_uid = $1"
    )
        .bind(Uuid::parse_str(club_uid.as_str()).expect("Error in parsing UUID string literal"))
        .fetch_all(&state.db)
        .await
    {
        Ok(announcements) => HttpResponse::Ok().json(announcements),
        Err(_) => HttpResponse::NotFound().json("No announcements found"),
    }
}

#[get("/announcement/club/{club_uid}/{date}")]
pub async fn fetch_club_announcements_by_uuid_and_date(state: Data<AppState>, path: Path<(String, String)>) -> impl Responder {
    let (club_uid, date) = path.into_inner();

    match sqlx::query_as::<_, Announcement>(
        "SELECT CAST(announcement_uid AS TEXT), info, date, club_uid FROM announcement WHERE club_uid = $1 AND date = $2"
    )
        .bind(Uuid::parse_str(club_uid.as_str()).expect("foo"))
        .bind(&date)
        .fetch_all(&state.db)
        .await
    {
        Ok(announcements) => HttpResponse::Ok().json(announcements),
        Err(_) => HttpResponse::NotFound().json("No announcements found"),
    }
}

#[get("/announcement/date/{announcement_date}")]
pub async fn fetch_club_announcements_by_date(state: Data<AppState>, path: Path<String>) -> impl Responder {
    let announcement_date: String = path.into_inner();

    match sqlx::query_as::<_,  Announcement>(
        "SELECT CAST(announcement_uid AS TEXT), info, date, club_uid  FROM announcement WHERE date = $1"
    )
        .bind(&announcement_date)
        .fetch_all(&state.db)
        .await
    {
        Ok(announcements) => HttpResponse::Ok().json(announcements),
        Err(_) => HttpResponse::NotFound().json("No announcements found"),
    }
}

#[post("/announcement")]
pub async fn create_announcement(state: Data<AppState>, body: Json<CreateAnnouncement>) -> impl Responder {
    match sqlx::query_as::<_, Announcement>(
        "INSERT INTO announcement (announcement_uid, info, date, club_uid) VALUES ($1, $2, $3, $4) RETURNING CAST(announcement_uid AS TEXT), info, date, club_uid"
    )
        .bind(Uuid::new_v4())
        .bind(body.info.to_string())
        .bind(body.date.to_string())
        .bind(Uuid::parse_str(body.club_uid.as_str()).expect("Error in parsing UUID string literal"))
        .fetch_one(&state.db)
        .await
    {
        Ok(announcement) => HttpResponse::Ok().json(announcement),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create club announcement"),
    }
}