use std::future::{ready, Ready};
use actix_web::{
    body::EitherBody, 
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform}, 
    http::header::{self, HeaderValue}, Error, HttpResponse
};
use futures_util::future::LocalBoxFuture;

use crate::session::check_csrf_token;

pub struct CheckCSRFToken;

impl<S, B> Transform<S, ServiceRequest> for CheckCSRFToken
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = CheckCSRFTokenMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckCSRFTokenMiddleware { service }))
    }
}
pub struct CheckCSRFTokenMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CheckCSRFTokenMiddleware<S>
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
        if request.path() == "/club"  && request.method().as_str() == "POST" {
            let cookie_header_vals: Vec<&HeaderValue> = request.headers().get_all(header::COOKIE).collect();

            if cookie_header_vals.len() > 0 {
                let token_validation = check_csrf_token(&request);

                match token_validation {
                    Ok(_) => {
                        let res = self.service.call(request);
                        Box::pin(async move {
                            res.await.map(ServiceResponse::map_into_left_body)
                        }) 
                    },
                    Err(_) => {
                        let (request, _pl) = request.into_parts();
                        let response = HttpResponse::Forbidden()
                            .finish()
                            .map_into_right_body();
                        
                        return Box::pin(async { 
                            Ok(ServiceResponse::new(request, response)) 
                        });
                    }
                }
            } else {
                let res = self.service.call(request);
                Box::pin(async move {
                    res.await.map(ServiceResponse::map_into_left_body)
                }) 
            }            
        } else {
            let res = self.service.call(request);
            Box::pin(async move {
                res.await.map(ServiceResponse::map_into_left_body)
            }) 
        }     
    }
}

#[cfg(test)]
mod tests {

}