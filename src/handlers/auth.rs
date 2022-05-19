use actix_identity::Identity;
use actix_web::{
    web::{
        block,
        Data,
        Json,
    },
    HttpResponse,
};
use serde::Serialize;

use crate::{
    auth::{
        create_jwt,
        hash,
        PrivateClaim,
    },
    database::PoolType,
    errors::ApiError,
    handlers::user::UserResponse,
    helpers::{
        respond_json,
        respond_ok,
    },
    models::user::find_by_auth,
    validate::validate,
};

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "email must be a valid email"))]
    pub email: String,

    #[validate(length(
        min = 6,
        message = "password is required and must be at least 6 characters"
    ))]
    pub password: String,
}

/// Login a user
/// Create and remember their JWT
pub async fn login(
    id: Identity,
    pool: Data<PoolType>,
    params: Json<LoginRequest>,
) -> Result<Json<UserResponse>, ApiError> {
    validate(&params)?;

    // Validate that the email + hashed password matches
    let hashed = hash(&params.password);
    let user = block(move || find_by_auth(&pool, &params.email, &hashed)).await??;

    // Create a JWT
    let private_claim = PrivateClaim::new(user.id, user.email.clone());
    let jwt = create_jwt(private_claim)?;
    log::info!("JWT: {:?}", jwt);
    // Remember the token
    id.remember(jwt);
    respond_json(user)
}

/// Logout a user
/// Forget their user_id
pub async fn logout(id: Identity) -> Result<HttpResponse, ApiError> {
    id.forget();
    respond_ok()
}

#[cfg(test)]
pub mod tests {
    use actix_identity::Identity;
    use actix_web::{
        test,
        FromRequest,
    };

    use super::*;
    use crate::tests::helpers::tests::get_data_pool;

    async fn get_identity() -> Identity {
        let (request, mut payload) = test::TestRequest::default()
            .insert_header(actix_web::http::header::ContentType::json())
            .to_http_parts();
        let identity = Option::<Identity>::from_request(&request, &mut payload)
            .await
            .unwrap()
            .unwrap();
        identity
    }

    async fn login_user() -> Result<Json<UserResponse>, ApiError> {
        let params = LoginRequest {
            email: "satoshi@nakamotoinstitute.org".into(),
            password: "123456".into(),
        };
        let identity = get_identity().await;
        login(identity, get_data_pool(), Json(params)).await
    }

    async fn logout_user() -> Result<HttpResponse, ApiError> {
        let identity = get_identity().await;
        logout(identity).await
    }

    #[actix_rt::test]
    async fn it_logs_a_user_in() {
        let response = login_user().await;
        assert!(response.is_ok());
    }

    #[actix_rt::test]
    async fn it_logs_a_user_out() {
        login_user().await.unwrap();
        let response = logout_user().await;
        assert!(response.is_ok());
    }
}
