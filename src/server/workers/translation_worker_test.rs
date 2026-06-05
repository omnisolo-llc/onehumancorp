use std::sync::Arc;
use crate::workers::translation_worker::TranslationWorker;
use crate::queue::Job;
use crate::queue::TaskJobHandler as JobHandler;
use crate::db::{DB, DbStore};
use sqlx::sqlite::SqlitePoolOptions;
use chrono::Utc;
use sha2::Digest;

#[tokio::test]
async fn test_translation_worker_logic() {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("Failed to connect");

    sqlx::query("CREATE TABLE IF NOT EXISTS translation_cache (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, text_hash TEXT NOT NULL, source_lang TEXT NOT NULL, target_lang TEXT NOT NULL, translated_text TEXT NOT NULL, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP)")
        .execute(&pool)
        .await
        .unwrap();

    let db = Arc::new(DB {
        pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://postgres:postgres@localhost:5432/test").unwrap(),
        store: DbStore::Sqlite(pool.clone()),
    });

    let worker = TranslationWorker::new(db.clone());

    let payload = serde_json::json!({
        "source_text": "Hello world",
        "source_lang": "en",
        "target_lang": "ar"
    });

    let job = Job {
        id: "test_job_1".to_string(),
        tenant_id: "test_tenant".to_string(),
        parent_task_id: "".to_string(),
        job_type: "translation_task".to_string(),
        payload: payload.to_string(),
        status: "RUNNING".to_string(),
        retry_count: 0,
        max_retries: 3,
        next_retry_at: Utc::now(),
        locked_until: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let handle = worker.handle(job.clone());
    handle.await.unwrap();

    // Check if inserted
    let text_hash = format!("{:x}", sha2::Sha256::digest("Hello world".as_bytes()));
    let row: (String,) = sqlx::query_as("SELECT translated_text FROM translation_cache WHERE text_hash = ? AND target_lang = ?")
        .bind(&text_hash)
        .bind("ar")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(row.0, "[ar] Hello world");

    // Second run should hit cache (no error, just return Ok(()))
    let handle2 = worker.handle(job);
    handle2.await.unwrap();
}
