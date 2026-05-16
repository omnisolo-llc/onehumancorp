use super::daemon::HybridSyncDaemon;
use sqlx::{Pool, Postgres, Sqlite, sqlite::SqlitePoolOptions};
use std::sync::Arc;

#[tokio::test]
async fn test_payload_redaction() {
    let raw = serde_json::json!({"email": "test@example.com", "name": "John Doe", "secret": "123"});
    let redacted = crate::telemetry::redact_interface_pii(raw);
    let s = redacted.to_string();
    assert!(!s.contains("test@example.com") || s.contains("[REDACTED]"));
}

#[tokio::test]
async fn test_sync_pending_escalations() {
    let sqlite_pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .unwrap();

    sqlx::query("CREATE TABLE agent_missions (id TEXT, status TEXT, payload BLOB, escalation_required BOOLEAN);")
        .execute(&sqlite_pool)
        .await
        .unwrap();

    let json_payload = serde_json::json!({"email": "secret@example.com"}).to_string();
    sqlx::query("INSERT INTO agent_missions (id, status, payload, escalation_required) VALUES ('1', 'PENDING', $1, true)")
        .bind(json_payload.as_bytes())
        .execute(&sqlite_pool)
        .await
        .unwrap();
}
