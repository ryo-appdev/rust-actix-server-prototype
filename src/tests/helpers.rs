#[cfg(test)]
pub mod tests {
    use actix_web::{
        dev::ServiceResponse,
        test,
        web::Data,
        App,
    };
    use diesel::PgConnection;
    use serde::Serialize;

    use crate::{
        cache::add_cache,
        config::CONFIG,
        database::{
            add_pool,
            init_pool,
            Pool,
        },
        handlers::auth::LoginRequest,
        routes::routes,
        state::{
            new_state,
            AppState,
        },
    };

    /// Helper for HTTP GET integration tests
    pub async fn test_get(route: &str) -> ServiceResponse {
        let login_request = LoginRequest {
            email: "satoshi@nakamotoinstitute.org".into(),
            password: "123456".into(),
        };

        // let mut app = test::init_service(
        let app = test::init_service(
            App::new()
                // FIXME: error: mismatched types label: expected struct `BoxBody`, found enum `EitherBody`
                // .wrap(get_identity_service())
                .configure(add_cache)
                .configure(add_pool)
                .configure(routes)
                .app_data(app_state()),
        )
        .await;

        let response = test::call_service(
            &app,
            test::TestRequest::post()
                .set_json(&login_request)
                .uri("/api/v1/auth/login")
                .to_request(),
        )
        .await;

        let cookie = response.response().cookies().next().unwrap().to_owned();
        test::call_service(
            &app,
            test::TestRequest::get()
                .cookie(cookie.clone())
                .uri(route)
                .to_request(),
        )
        .await
    }

    /// Helper for HTTP POST integration tests
    pub async fn test_post<T: Serialize>(route: &str, params: T) -> ServiceResponse {
        // let mut app = test::init_service(
        let app = test::init_service(
            App::new()
                // FIXME: error: mismatched types label: expected struct `BoxBody`, found enum `EitherBody`
                // .wrap(get_identity_service())
                .configure(add_cache)
                .configure(add_pool)
                .configure(routes)
                .app_data(app_state()),
        )
        .await;
        let login = login().await;
        let cookie = login.response().cookies().next().unwrap().to_owned();
        test::call_service(
            &app,
            test::TestRequest::post()
                .set_json(&params)
                .cookie(cookie.clone())
                .uri(route)
                .to_request(),
        )
        .await
    }

    /// Helper to login for tests
    // pub fn login_request() -> Request {
    //     let login_request = LoginRequest {
    //         email: "satoshi@nakamotoinstitute.org".into(),
    //         password: "123456".into(),
    //     };
    //     test::TestRequest::post()
    //         .set_json(&login_request)
    //         .uri("/api/v1/auth/login")
    //         .to_request()
    // }

    /// Assert that a route is successful for HTTP GET requests
    pub async fn assert_get(route: &str) -> ServiceResponse {
        let response = test_get(route).await;
        assert!(response.status().is_success());
        response
    }

    /// Assert that a route is successful for HTTP POST requests
    pub async fn assert_post<T: Serialize>(route: &str, params: T) -> ServiceResponse {
        let response = test_post(route, params).await;
        assert!(response.status().is_success());
        response
    }

    /// Returns a r2d2 Pooled Connection to be used in tests
    pub fn get_pool() -> Pool<PgConnection> {
        init_pool::<PgConnection>(CONFIG.clone()).unwrap()
    }

    /// Returns a r2d2 Pooled Connection wrappedn in Actix Application Data
    pub fn get_data_pool() -> Data<Pool<PgConnection>> {
        Data::new(get_pool())
    }

    /// Login to routes  
    pub async fn login() -> ServiceResponse {
        let login_request = LoginRequest {
            email: "satoshi@nakamotoinstitute.org".into(),
            password: "123456".into(),
        };
        // let mut app = test::init_service(
        let app = test::init_service(
            App::new()
                // FIXME: error: mismatched types label: expected struct `BoxBody`, found enum `EitherBody`
                // .wrap(get_identity_service())
                .configure(add_pool)
                .configure(routes),
        )
        .await;

        test::call_service(
            &app,
            test::TestRequest::post()
                .set_json(&login_request)
                .uri("/api/v1/auth/login")
                .to_request(),
        )
        .await
    }

    // Mock applicate state
    pub fn app_state() -> AppState<'static, String> {
        new_state::<String>()
    }
}
