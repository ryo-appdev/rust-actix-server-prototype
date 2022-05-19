use actix::prelude::*;
use actix_redis::{
    Command,
    RedisActor,
};
use actix_web::web;
use redis_async::resp::{
    FromResp,
    RespValue,
};

use crate::{
    config::CONFIG,
    errors::ApiError,
};

pub type Cache = web::Data<Addr<RedisActor>>;

/// Retrieve an entry in redis
#[allow(dead_code)]
// FFIXED: pub async fn get<'a>(redis: Cache, key: &'a str) -> Result<String, ApiError> {
pub async fn get(redis: Cache, key: &str) -> Result<String, ApiError> {
    let command = resp_array!["GET", key];
    send(redis, command).await
}

/// Insert or update an entry in redis
#[allow(dead_code)]
pub async fn set<'a>(redis: Cache, key: &'a str, value: &'a str) -> Result<String, ApiError> {
    let command = resp_array!["SET", key, value];
    send(redis, command).await
}

/// Delete an entry in redis
#[allow(dead_code)]
// FIXED: pub async fn delete<'a>(redis: Cache, key: &'a str) -> Result<String, ApiError> {
pub async fn delete(redis: Cache, key: &str) -> Result<String, ApiError> {
    let command = resp_array!["DEL", key];
    send(redis, command).await
}

/// Send a command to the redis actor
// FIXED: async fn send<'a>(redis: Cache, command: RespValue) -> Result<String, ApiError> {
async fn send(redis: Cache, command: RespValue) -> Result<String, ApiError> {
    let error_message = format!("Could not send {:?} command to Redis", command);
    let error = ApiError::CacheError(error_message);
    let response = redis.send(Command(command)).await.map_err(|_| error)?;
    match response {
        Ok(message) => Ok::<String, _>(Result::unwrap_or(FromResp::from_resp(message), "".into())),
        Err(message) => Err(ApiError::CacheError(format!("{:?}", message))),
    }
}

/// Add the redis actor to actix data if the URL is set
pub fn add_cache(cfg: &mut web::ServiceConfig) {
    if !&CONFIG.redis_url.is_empty() {
        // Start a new supervisor with redis actor
        let cache = RedisActor::start(&CONFIG.redis_url);
        // FIXED: cfg.data(cache);
        cfg.app_data(web::Data::new(cache));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_cache() -> Cache {
        let cache = RedisActor::start(&CONFIG.redis_url);
        web::Data::new(cache)
    }
    #[actix_rt::test]
    async fn it_creates_new_application_cache_and_sets_and_reads_it() {
        let cache = get_cache();

        set(cache.clone(), "testing", "123").await.unwrap();
        let value = get(cache, "testing").await.unwrap();
        assert_eq!(value, "123");
    }
    #[actix_rt::test]
    async fn it_removes_an_entry_in_application_cache() {
        let cache = get_cache();

        set(cache.clone(), "testing", "123").await.unwrap();
        let value = get(cache.clone(), "testing").await.unwrap();
        assert_eq!(value, "123");

        delete(cache.clone(), "testing").await.unwrap();
        let value = get(cache, "testing").await.unwrap();
        assert_eq!(value, "");
    }
}
