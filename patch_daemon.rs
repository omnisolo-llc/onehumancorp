<<<<<<< SEARCH
pub struct HybridSyncDaemon {
    sqlite_pool: SqlitePool,
    pg_pool: PgPool,
}

impl HybridSyncDaemon {
    pub fn new(sqlite_pool: SqlitePool, pg_pool: PgPool) -> Self {
        Self { sqlite_pool, pg_pool }
    }
=======
pub struct HybridSyncDaemon {
    sqlite_pool: SqlitePool,
    pg_pool: PgPool,
    cloud_url: String,
    client: reqwest::Client,
}

impl HybridSyncDaemon {
    pub fn new(sqlite_pool: SqlitePool, pg_pool: PgPool, cloud_url: String) -> Self {
        Self {
            sqlite_pool,
            pg_pool,
            cloud_url,
            client: reqwest::Client::new(),
        }
    }
>>>>>>> REPLACE
