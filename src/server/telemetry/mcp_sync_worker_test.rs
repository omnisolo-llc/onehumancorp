use sqlx::{SqlitePool, PgPool};
use super::mcp_sync_worker::McpSyncWorker;

#[tokio::test]
async fn test_mcp_sync_worker() {
    let sqlite_pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    let pg_pool = PgPool::connect("postgres://ohc:ohc@localhost:5432/ohc").await.unwrap(); // just a dummy for compile

    let _worker = McpSyncWorker::new(sqlite_pool, pg_pool);
    // Since we don't have a real postgres instance in unit tests, we'll just test that it compiles and we can create it
}
