use crate::autodream_sync::AutoDreamSyncWorker;
use sqlx::SqlitePool;

#[tokio::test]
async fn test_process_forecast_tick_not_sqlite() {
    std::env::set_var("DATABASE_URL", "postgres://localhost/testdb");

    let worker = AutoDreamSyncWorker::new().await.unwrap();
    let res = worker.ProcessForecastTick().await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_process_forecast_tick_sqlite_sync() {
    std::env::set_var("DATABASE_URL", "sqlite::memory:");

    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

    // Create schemas to avoid erroring out
    sqlx::query("CREATE TABLE agent_missions (id TEXT PRIMARY KEY, payload TEXT, updated_at TIMESTAMP, synced_to_cloud BOOLEAN)")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("CREATE TABLE embedding_cache (cache_key TEXT PRIMARY KEY, response_json TEXT, created_at TIMESTAMP, synced_to_cloud BOOLEAN)")
        .execute(&pool)
        .await
        .unwrap();

    let worker = AutoDreamSyncWorker::new().await.unwrap();

    // We recreate the schema in the worker's pool just in case since memory DBs might be separate unless named or shared cache is used
    if let Some(worker_pool) = worker.get_pool() {
        sqlx::query("CREATE TABLE IF NOT EXISTS agent_missions (id TEXT PRIMARY KEY, payload TEXT, updated_at TIMESTAMP, synced_to_cloud BOOLEAN)")
            .execute(worker_pool)
            .await
            .unwrap();

        sqlx::query("CREATE TABLE IF NOT EXISTS embedding_cache (cache_key TEXT PRIMARY KEY, response_json TEXT, created_at TIMESTAMP, synced_to_cloud BOOLEAN)")
            .execute(worker_pool)
            .await
            .unwrap();
    }

    let res = worker.ProcessForecastTick().await;
    assert!(res.is_ok());
}
