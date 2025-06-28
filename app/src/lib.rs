use actix_web::web::{self, ServiceConfig};
use actix_files as fs;

pub mod utils;


pub fn app(app: &mut ServiceConfig) -> () {
    app
        .service(
            fs::Files::new("/app/src/static", "./app/src/static").show_files_listing()
        )
        .service(
            fs::Files::new("/app/src/static/js", "./app/src/static/js").show_files_listing()
        )  
        .service(
            fs::Files::new("/app/src/img", "./app/src/img").show_files_listing()
        )
        .service(
            web::resource("/")
                .route(web::get().to(utils::home::home))
        )
        .service(
            web::resource("register")
                .route(web::get().to(utils::signup::home))
        )
        .service(
            web::resource("create-pin")
                .route(web::get().to(utils::password::create_password_home))
        )   
        .service(
            web::scope("login")
                .service(
                    web::resource("club")
                        .route(web::get().to(utils::login::login_club))
                )
                .service(
                    web::resource("prefect")
                        .route(web::get().to(utils::login::login_prefect))
                )
        )
        .service(
            web::resource("logout")
                .route(web::get().to(utils::logout::logout))
        )
        .service(
            web::scope("club")
                .route("", web::get().to(utils::services::fetch_club_announcements_by_uuid))
                .route("date/{announcement_date}", web::get().to(utils::services::fetch_club_announcements_by_uuid_and_date))
        )
        .service(
            web::scope("prefect")
                .route("", web::get().to(utils::services::fetch_all_club_announcements))
                .route("date/{date}", web::get().to(utils::services::fetch_club_announcements_by_date))
        );
}