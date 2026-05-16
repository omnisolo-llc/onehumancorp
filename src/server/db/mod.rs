pub mod models;
pub mod impls;
pub use models::*;
pub use impls::*;
#[cfg(test)]
pub mod tests;
use sqlx::PgPool;
use sqlx::sqlite::{SqlitePoolOptions, SqliteConnectOptions};
use sqlx::SqlitePool;
use std::str::FromStr;
use std::env;
use sqlx::Row;
use chrono::{DateTime, Utc};
use std::path::Path;
use std::sync::OnceLock;

static GLOBAL_POOL: std::sync::OnceLock<PgPool> = std::sync::OnceLock::new();

pub fn get_pool() -> PgPool {
    GLOBAL_POOL.get().cloned().unwrap_or_else(|| {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());
        sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .acquire_timeout(std::time::Duration::from_millis(500))
            .connect_lazy(&database_url)
            .expect("Failed to connect to DB pool lazily")
    })
}
