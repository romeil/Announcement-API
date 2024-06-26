use std::future::{ready, Ready};
use actix_web::{
    body::EitherBody,
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform},
    http, Error, HttpResponse,
};
use futures_util::future::LocalBoxFuture;

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

    fn call(&self, request: ServiceRequest) -> Self::Future {
        let settings = crate::settings::get_settings();

        let cookie = request.cookie(settings.auth_cookie_name.as_str());
        let username;
        match &cookie {
            Some(_found_cookie) => {
                let username_from_token = crate::secure_token::verify_token(cookie.unwrap().value());
                match username_from_token {
                    Ok(user) => {
                        username = Some(user);
                    },
                    Err(_) => {
                        username = None;
                    }
                }
            },
            None => {
                username = None;
            }
        }

        match username {
            Some(_username) => {
                let res = self.service.call(request);
                Box::pin(async move {
                    res.await.map(ServiceResponse::map_into_left_body)
                }) 
            },
            None => {
                if request.path() != "/" && request.path() != "/login/club" && request.path() != "/login/admin" {
                    let (request, _pl) = request.into_parts();

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