use std::future::{ready, Ready};
use actix_web::{
    body::EitherBody, 
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform}, 
    http::header::{self, HeaderValue}, Error
};
use futures_util::future::LocalBoxFuture;

use crate::session::{change_csrf_cookie, generate_csrf_token, set_csrf_cookie};

pub struct ModifyCSRFToken;

impl<S, B> Transform<S, ServiceRequest> for ModifyCSRFToken
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = ModifyCSRFTokenMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(ModifyCSRFTokenMiddleware { service }))
    }
}
pub struct ModifyCSRFTokenMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for ModifyCSRFTokenMiddleware<S>
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
        let alter_token = alter_csrf_token(&request); 
        let res = self.service.call(request);
        
        Box::pin(async move {
            let mut res = res.await;
            
            if alter_token {
                let path = res.as_ref().unwrap().request().path();
                let cookie_header_vals: Vec<&HeaderValue> = res.as_ref().unwrap().response().headers().get_all(header::SET_COOKIE).collect();
                
                if cookie_header_vals.len() > 0 {
                    match path {
                        "/login/club" | "/login/admin" | "/register" | "/create-pin" => {    
                            let csrf_token = generate_csrf_token(res.as_ref());
                            set_csrf_cookie(res.as_mut().unwrap().response_mut().head_mut(), csrf_token);
                        
                        },
                        "/club" => {
                            let csrf_token = generate_csrf_token(res.as_ref());
                            change_csrf_cookie(res.as_mut().unwrap().response_mut().head_mut(), csrf_token);
    
                        }
                        _ => {}
                    }
                } 
            } 
            res.map(ServiceResponse::map_into_left_body)
        })        
    }
}

pub fn alter_csrf_token(request: &ServiceRequest) -> bool {
    if request.path() == "/register" || request.path() == "/create-pin" || request.path() == "/login/club" || request.path() == "/login/admin" ||  request.path() == "/logout"|| (request.path() == "/club" && request.method().as_str() == "POST") {
        true
    } else {
        false
    }
}