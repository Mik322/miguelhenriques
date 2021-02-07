use actix_web::{dev::ServiceRequest, dev::ServiceResponse, web::Data, Error};
use actix_web::{
    dev::{Service, Transform},
    HttpResponse,
};
use futures::future::{ok, Ready};
use futures::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::{models::user_token::UserToken, Pool};

const NO_AUTH_ENDPOINTS: [&str; 3] = ["/get/projects", "/send_email", "/login"];

pub struct Authentication;

impl<S, B> Transform<S> for Authentication
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthenticationMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let mut authenticated = false;

        println!("{:?}", req);

        for path in NO_AUTH_ENDPOINTS.iter() {
            if req.path() == *path {
                authenticated = true;
                break;
            }
        }

        if !authenticated {
            if let Some(pool) = req.app_data::<Data<Pool>>() {
                if let Some(header) = req.headers().get("authorization") {
                    if let Ok(header_str) = header.to_str() {
                        if header_str.starts_with("bearer") || header_str.starts_with("Bearer") {
                            if let Ok(token) = UserToken::decode_token(
                                header_str[6..header_str.len()].trim().to_string(),
                            ) {
                                let conn = pool.get().unwrap();
                                if let Ok(_) = UserToken::verify_token(&token, &conn) {
                                    authenticated = true;
                                }
                            }
                        }
                    }
                }
            }
        }

        if authenticated {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            Box::pin(async move {
                Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .json("Authentication not valid.")
                        .into_body(),
                ))
            })
        }
    }
}
