use sqlx::{PgPool, SqlitePool};
use sqlx::PgPool;
use sqlx::sqlite::{SqlitePoolOptions, SqliteConnectOptions};
use sqlx::SqlitePool;
use std::str::FromStr;
use std::env;
use sqlx::Row;
use chrono::{DateTime, Utc};
use std::path::Path;
use std::sync::OnceLock;

#[derive(Clone)]
pub enum DbStore {
    Postgres,
    Sqlite(SqlitePool),
}


#[derive(Clone)]
pub struct DB {
    pub pool: PgPool,
    pub store: DbStore,
}
