use crate::{
    config::{db, routes},

    toolbox::response::ResponseBody,
    jwt::user_token::UserToken,
    // ::{decode_token, verify_token},
};
use actix_service::{Service, Transform};

use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    http::{
        header::{HeaderName, HeaderValue},
        Method,
    },
    Error, HttpResponse,
};

use futures::{
    future::{ok, Ready},
    Future,
};

use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub struct Authentication;

impl<S, B> Transform<S> for Authentication
where
    S: Service<
        Request = ServiceRequest,
        Response = ServiceResponse<B>,
        Error = Error,
    >,
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
    S: Service<
        Request = ServiceRequest,
        Response = ServiceResponse<B>,
        Error = Error,
    >,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &mut self,
        ctx: &mut Context<'_>,
    ) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, mut request: ServiceRequest) -> Self::Future {
        debug!("Hi! authen_middleware speaking");

        // bypass account routes
        let headers = request.headers_mut();
        headers.append(
            HeaderName::from_static("content-length"),
            HeaderValue::from_static("true"),
        );
        if Method::OPTIONS == *request.method() {
            debug!("The request verb is OPTIONS! It's a pass.");
            let future = self.service.call(request);
            return Box::pin(async move {
                let response = future.await?;
                Ok(response)
            });
        }

        // allow /auth/login and /auth/signup
        for ignore_route in routes::IGNORE_ROUTES.iter() {
            if request.path().starts_with(ignore_route) {
                debug!("The request path is in the ignored routes! It's a pass.");
                let future = self.service.call(request);
                return Box::pin(async move {
                    let response = future.await?;
                    Ok(response)
                });
            }
        }

        debug!("Finding the authorization header...");
        let authen_header = match request.headers_mut().get("Authorization") {
            Some(authen_header) => authen_header,
            None => {
                return Box::pin(async move {
                    Ok(request.into_response(
                        HttpResponse::Unauthorized()
                            .json(ResponseBody::new(
                                "We did not find an authentication header…",
                                "",
                            ))
                            .into_body(),
                    ))
                });
            }
        };

        debug!("Parsing authorization header...");
        let str_authen_header = match authen_header.to_str() {
            Ok(str) => str,
            Err(_) => {
                return Box::pin(async move {
                    Ok(request.into_response(
                        HttpResponse::Unauthorized()
                            .json(ResponseBody::new(
                                "The authorization header doesn't seem to be stringifyable",
                                "",
                            ))
                            .into_body(),
                    ))
                });
            }
        };

        debug!("Checking the start of the authorization header...");
        if !str_authen_header.starts_with("Bearer")
            && !str_authen_header.starts_with("bearer")
        {
            return Box::pin(async move {
                Ok(request.into_response(
                    HttpResponse::Unauthorized()
                        .json(ResponseBody::new(
                            "The authorization header doesn't start with bearer",
                            "",
                        ))
                        .into_body(),
                ))
            });
        }

        debug!("Parsing token");
        let token = str_authen_header[6..str_authen_header.len()].trim();

        debug!("Decoding the token");
        let token_data = match UserToken::decode_token(token.to_string()) {
            Ok(decoded_data) => decoded_data,
            Err(_) => {
                return Box::pin(async move {
                    Ok(request.into_response(
                        HttpResponse::Unauthorized()
                            .json(ResponseBody::new(
                                "Could not decode the token",
                                "",
                            ))
                            .into_body(),
                    ))
                });
            }
        };

        debug!("Connecting to the database");
        let conn = match db::connection() {
            Ok(conn) => conn,
            Err(_) => {
                return Box::pin(async move {
                    Ok(request.into_response(
                        HttpResponse::Unauthorized()
                            .json(ResponseBody::new(
                                "Could not connect to the database",
                                "",
                            ))
                            .into_body(),
                    ))
                });
            }
        };

        match UserToken::verify_token(&token_data, &conn) {
            Ok(user_string) => {
                debug!("We accept the validity of user {}'s request", user_string);
                let future = self.service.call(request);
                return Box::pin(async move {
                    let response = future.await?;
                    Ok(response)
                });
            }
            Err(error_string) => {
                error!("invalid jwt");
                Box::pin(async move {
                    Ok(request.into_response(
                        HttpResponse::Unauthorized()
                            .json(ResponseBody::new(
                                "Token verification failed with this error: ",
                                &error_string,
                            ))
                            .into_body(),
                    ))
                })
            }
        }
    }
}
