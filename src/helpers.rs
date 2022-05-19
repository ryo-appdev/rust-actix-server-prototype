use actix_web::{
    web::Json,
    HttpResponse,
};
use serde::Serialize;

use crate::errors::ApiError;

/// Helper function to reduce boilerplate of an OK/Json response
pub fn respond_json<T>(data: T) -> Result<Json<T>, ApiError>
where
    T: Serialize,
{
    Ok(Json(data))
}

/// Helper function to reduce boilerplate of an empty OK response
pub fn respond_ok() -> Result<HttpResponse, ApiError> {
    // FIXED: Ok(HttpResponse::Ok().body(Body::Empty))
    Ok(HttpResponse::Ok().body(()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
    pub struct TestResponse {
        pub first_name: String,
    }
    #[test]
    fn it_responds_json() {
        let response = TestResponse {
            first_name: "Satoshi".into(),
        };
        let result = respond_json(response.clone());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().into_inner(), response);
    }
    #[test]
    fn it_responds_ok() {
        let result = respond_ok();
        assert!(result.is_ok());
    }
}
