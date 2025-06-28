use actix_web::web::{self, ServiceConfig};
use actix_files as fs;
pub mod utils;
pub mod middleware;
pub mod secure_token;
pub mod settings;
pub mod session;

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
            web::resource("register")
                .route(web::post().to(utils::signup::signup_post))
        )
        .service(
            web::resource("create-pin")
                .route(web::post().to(utils::password::create_password_post))
        )   
        .service(
            web::scope("login")
                .service(
                    web::resource("admin")
                        .route(web::post().to(utils::login::login_admin_post))
                )
                .service(
                    web::resource("club")
                        .route(web::post().to(utils::login::login_club_post))
                )
                .service(
                    web::resource("prefect")
                        .route(web::post().to(utils::login::login_prefect_post))
                )
        )
        .service(
            web::resource("logout")
                .route(web::post().to(utils::logout::logout_post))
        )
        .service(
            web::scope("club")
                .route("", web::post().to(utils::services::create_club_announcement))
        )
        .service(
            web::scope("prefect")
                .route("", web::post().to(utils::services::create_club_announcement_prefect))
        );
}