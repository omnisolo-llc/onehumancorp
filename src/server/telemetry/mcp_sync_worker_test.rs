use sqlx::{SqlitePool, PgPool, Row};
use super::mcp_sync_worker::McpSyncWorker;

#[tokio::test]
async fn test_mcp_sync_worker() {
    let sqlite_pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    // use connect_lazy for a dummy compile so it doesn't try to connect immediately
    let pg_pool = sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://ohc:ohc@localhost:5432/ohc").unwrap();

    let _worker = McpSyncWorker::new(sqlite_pool, pg_pool);
    // Since we don't have a real postgres instance in unit tests, we'll just test that it compiles and we can create it
}

#[tokio::test]
async fn test_mcp_sync_worker_pii_redaction() {
    let email_label = serde_json::json!({"email": "test@example.com", "other": "value"});
    let labels_json_str = email_label.to_string();

    let redacted_str = McpSyncWorker::process_telemetry_labels(&labels_json_str);

    assert!(redacted_str.contains("[REDACTED]"));
    assert!(!redacted_str.contains("test@example.com"));
    assert!(redacted_str.contains("other"));
    assert!(redacted_str.contains("value"));
}
