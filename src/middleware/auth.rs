use std::{
    pin::Pin,
    task::{
        Context,
        Poll,
    },
};

use actix_identity::RequestIdentity;
use actix_service::{
    Service,
    Transform,
};
use actix_web::{
    body::BoxBody,
    dev::{
        ServiceRequest,
        ServiceResponse,
    },
    Error,
    HttpResponse,
};
use futures::{
    future::{
        ok,
        Ready,
    },
    Future,
};

use crate::{
    auth::{
        decode_jwt,
        PrivateClaim,
    },
    errors::ApiError,
};

pub struct Auth;

impl<S> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware {
            service,
        })
    }
}

pub struct AuthMiddleware<S> {
    service: S,
}

// FIXED: impl<S, B> Service for AuthMiddleware<S>. B cahnged to BoxBody
impl<S> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error>,
    S::Future: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let identity = Option::unwrap_or(RequestIdentity::get_identity(&req), "".into());
        log::info!("identity: {}", identity);
        let private_claim: Result<PrivateClaim, ApiError> = decode_jwt(&identity);
        let is_logged_in = private_claim.is_ok();
        let unauthorized = !is_logged_in && req.path() != "/api/v1/auth/login";

        if unauthorized {
            return Box::pin(async move {
                // FIXED: parameter type B influenced this method call
                Ok(req.into_response(HttpResponse::Unauthorized().finish()))
            });
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
