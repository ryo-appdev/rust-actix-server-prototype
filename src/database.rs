//! Database-related functions
use actix_web::web;
use diesel::{
    pg::PgConnection,
    r2d2::{
        ConnectionManager,
        PoolError,
    },
    sqlite::SqliteConnection,
    Connection,
    MysqlConnection,
};

use crate::config::{
    Config,
    CONFIG,
};

#[derive(Clone, Deserialize, Debug, PartialEq)]
#[serde(field_identifier, rename_all = "lowercase", untagged)]
pub enum DatabaseConnection {
    Mysql,
    Postgres,
    Sqlite,
}

pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;

pub type MysqlPool = Pool<MysqlConnection>;
pub type PostgresPool = Pool<PgConnection>;
pub type SqlitePool = Pool<SqliteConnection>;

#[cfg(feature = "mysql")]
pub type PoolType = MysqlPool;

#[cfg(feature = "postgres")]
pub type PoolType = PostgresPool;

#[cfg(feature = "sqlite")]
pub type PoolType = SqlitePool;

#[derive(Clone)]
pub enum InferPool {
    Mysql(MysqlPool),
    Postgres(PostgresPool),
    Sqlite(SqlitePool),
}

impl InferPool {
    pub fn init_pool(config: Config) -> Result<Self, r2d2::Error> {
        match config.database {
            DatabaseConnection::Mysql => init_pool::<MysqlConnection>(config).map(InferPool::Mysql),
            DatabaseConnection::Postgres => {
                init_pool::<PgConnection>(config).map(InferPool::Postgres)
            }
            DatabaseConnection::Sqlite => {
                init_pool::<SqliteConnection>(config).map(InferPool::Sqlite)
            }
        }
        .map_err(Into::into)
    }
}

pub fn init_pool<T>(config: Config) -> Result<Pool<T>, PoolError>
where
    T: Connection + 'static,
{
    let manager = ConnectionManager::<T>::new(config.database_url);
    // postgres の場合デフォルト接続上限が 100 のため調整。API 内での同時必要コネクション数に依存する
    Pool::builder().max_size(2).build(manager)
}

pub fn add_pool(cfg: &mut web::ServiceConfig) {
    // log::info!("add_pool");
    let pool = InferPool::init_pool(CONFIG.clone()).expect("Failed to create connection pool");
    match pool {
        // FIXED: cfg.data(***)
        InferPool::Mysql(mysql_pool) => cfg.app_data(web::Data::new(mysql_pool)),
        InferPool::Postgres(postgres_pool) => cfg.app_data(web::Data::new(postgres_pool)),
        InferPool::Sqlite(sqlite_pool) => cfg.app_data(web::Data::new(sqlite_pool)),
    };
}
