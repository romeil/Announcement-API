use std::{collections::HashSet, future::{ready, Ready}};
use actix_web::{
    body::EitherBody, cookie::{Key, CookieJar}, 
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform}, 
    http, Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use crate::{secure_token, session,settings};

pub struct CheckLogin;

impl<S, B> Transform<S, ServiceRequest> for CheckLogin
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = CheckLoginMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckLoginMiddleware { service }))
    }
}
pub struct CheckLoginMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CheckLoginMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    // Check on this middleware further
    fn call(&self, request: ServiceRequest) -> Self::Future {
        let session_cookie = request.cookie("id");

        match session_cookie {
            Some(cookie) => {
                let mut jar = CookieJar::new();
                jar.add_original(cookie);
                let cookie_verifier = jar.private(&Key::from(&session::generate_key())).get("id");

                match cookie_verifier {
                    Some(_valid_cookie) => {
                        if ["login/admin", "/login/club", "/login/prefect"].contains(&request.path()) {
                            let settings = settings::get_settings();
                            let paseto_token = request.cookie(settings.auth_cookie_name.as_str());

                            match paseto_token {
                                Some(paseto_token) => {                            
                                    match secure_token::verify_token(paseto_token.value(), request.path()) {
                                        Ok(_valid_paseto_token) => {
                                            let res = self.service.call(request);
                                            return Box::pin(async move {
                                                res.await.map(ServiceResponse::map_into_left_body)
                                            }) 
                                        },
                                        Err(_) => {
                                            let (request, _pl) = request.into_parts();

                                            let response = HttpResponse::Forbidden()
                                                .insert_header((http::header::LOCATION, "/"))
                                                .finish()
                                                .map_into_right_body();

                                            return Box::pin(async { 
                                                Ok(ServiceResponse::new(request, response)) 
                                            });
                                        }
                                    }
                                },
                                None => {
                                    let (request, _pl) = request.into_parts();

                                    let response = HttpResponse::Forbidden()
                                        .insert_header((http::header::LOCATION, "/"))
                                        .finish()
                                        .map_into_right_body();

                                    return Box::pin(async { 
                                        Ok(ServiceResponse::new(request, response)) 
                                    });
                                }
                            }
                        }
                        let res = self.service.call(request);
                        Box::pin(async move {
                            res.await.map(ServiceResponse::map_into_left_body)
                        }) 
                    },
                    None => {
                        let (request, _pl) = request.into_parts();

                        let response = HttpResponse::Forbidden()
                            .insert_header((http::header::LOCATION, "/"))
                            .finish()
                            .map_into_right_body();

                        return Box::pin(async { 
                            Ok(ServiceResponse::new(request, response)) 
                        });
                    }
                }
            }
            None => {
                if !allowed_paths().contains(&request.path()) {
                    let (request, _pl) = request.into_parts();

                    //TODO: Delete id and csrf cookie if they exists
                    
                    let response = HttpResponse::SeeOther()
                        .insert_header((http::header::LOCATION, "/"))
                        .finish()
                        .map_into_right_body();

                    return Box::pin(async { 
                        Ok(ServiceResponse::new(request, response)) 
                    });
                } else {
                    let res = self.service.call(request);
                    Box::pin(async move {
                        res.await.map(ServiceResponse::map_into_left_body)
                    }) 
                }
            }
        }
    }
}

fn allowed_paths() -> HashSet<&'static str> {
    [
        "/", "/app/src/static/styles.css", "/app/src/img/Wolmers-Logo.png", "/app/src/img/Wolmers-Campus.JPG", 
        "/favicon.ico", "/register", "/app/src/static/registration.html", "/app/src/static/authenticate.html", 
        "/create-pin", "/app/src/static/make-password.html", "/login/admin", "/login/club", "/app/src/static/club-login.html", 
        "/login/prefect", "/app/src/static/prefect-login.html", "/app/src/static/announcements.html" ,"/app/src/static/js/main.js"
    ].into()
}