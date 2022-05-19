use actix_web::{
    error::{
        BlockingError,
        ResponseError,
    },
    http::StatusCode,
    HttpResponse,
};
use derive_more::Display;
use diesel::{
    r2d2::PoolError,
    result::{
        DatabaseErrorKind,
        Error as DBError,
    },
};

#[derive(Debug, Display, PartialEq)]
#[allow(dead_code)]
pub enum ApiError {
    BadRequest(String),
    BlockingError(String),
    CacheError(String),
    CannotDecodeJwtToken(String),
    CannotEncodeJwtToken(String),
    InternalServerError(String),
    NotFound(String),
    PoolError(String),
    #[display(fmt = "")]
    ValidationError(Vec<String>),
    Unauthorized(String),
    UuidError(String),
}

/// User-friendly error messages
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    errors: Vec<String>,
}

/// Automatically convert ApiErrors to external Response Errors
impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::BadRequest(error) => {
                // HttpResponse::BadRequest().json::<ErrorResponse>(error.into())
                HttpResponse::BadRequest().json(error)
            }
            ApiError::NotFound(message) => HttpResponse::NotFound().json(message),
            ApiError::ValidationError(errors) => {
                HttpResponse::UnprocessableEntity().json(errors.to_vec())
            }
            ApiError::Unauthorized(error) => HttpResponse::Unauthorized().json(error),
            _ => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

/// Utility to make transforming a string reference into an ErrorResponse
impl From<&String> for ErrorResponse {
    fn from(error: &String) -> Self {
        ErrorResponse {
            errors: vec![error.into()],
        }
    }
}

/// Utility to make transforming a vector of strings into an ErrorResponse
impl From<Vec<String>> for ErrorResponse {
    fn from(errors: Vec<String>) -> Self {
        ErrorResponse {
            errors,
        }
    }
}

/// Convert DBErrors to ApiErrors
impl From<DBError> for ApiError {
    fn from(error: DBError) -> ApiError {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return ApiError::BadRequest(message);
                }
                ApiError::InternalServerError("Unknown database error".into())
            }
            _ => ApiError::InternalServerError("Unknown database error".into()),
        }
    }
}

/// Convert PoolErrors to ApiErrors
impl From<PoolError> for ApiError {
    fn from(error: PoolError) -> ApiError {
        ApiError::PoolError(error.to_string())
    }
}

/// Convert ParseErrors to ApiErrors
impl From<uuid::Error> for ApiError {
    fn from(error: uuid::Error) -> ApiError {
        ApiError::UuidError(error.to_string())
    }
}

/// Convert Thread BlockingErrors to ApiErrors
impl From<BlockingError> for ApiError {
    // remove ver.4
    // impl From<BlockingError> for ApiError {
    fn from(error: BlockingError) -> ApiError {
        ApiError::BlockingError(error.to_string())
        // remove ver.4
        // fn from(error: BlockingError) -> ApiError {
        // match error {
        //     BlockingError::Error(api_error) => api_error,
        //     BlockingError::Canceled => {
        //         ApiError::BlockingError("Thread blocking error".into())
        //     }
        // }
    }
}
