use std::future::{ready, Ready};
use actix_web::{
    body::EitherBody, 
    dev::{self, Service, ServiceRequest, ServiceResponse, Transform}, 
    http::header::CONTENT_TYPE, Error, HttpResponse
};
use futures_util::future::LocalBoxFuture;

pub struct CheckContentType;

impl<S, B> Transform<S, ServiceRequest> for CheckContentType
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = CheckContentTypeMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CheckContentTypeMiddleware { service }))
    }
}
pub struct CheckContentTypeMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CheckContentTypeMiddleware<S>
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
        let session_cookie = request.cookie("id");
        let content_type = request.headers().get(CONTENT_TYPE);
        let path = request.path();

        if session_cookie == None {
            if path == "/login/admin" || path == "/login/club" {
                let error_message = if path == "/login/admin" {
                    "Invalid admin prefect ID or password"
                } else {
                    "Invalid club or password"
                };
    
                if let Some(content_type) = content_type {
                    let content_type_str = content_type.to_str().unwrap();
                    match content_type_str {
                        "application/x-www-form-urlencoded" => {
                            let res = self.service.call(request);
                            Box::pin(async move {
                                res.await.map(ServiceResponse::map_into_left_body)
                            }) 
                        },
                        _ => {
                            let (request, _pl) = request.into_parts();
                            let response = HttpResponse::Unauthorized()
                                .body(error_message)
                                .map_into_right_body();
                            
                            return Box::pin(async { 
                                Ok(ServiceResponse::new(request, response)) 
                            });
                        }
                    }
                } else {
                    let (request, _pl) = request.into_parts();
                    let response = HttpResponse::Unauthorized()
                        .body(error_message)
                        .map_into_right_body();
                    
                    return Box::pin(async { 
                        Ok(ServiceResponse::new(request, response)) 
                    });
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