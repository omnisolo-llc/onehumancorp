use axum::{extract::State, Json};
use std::sync::Arc;
use crate::hub::Hub;

pub async fn health_handler(
    State(hub): State<Arc<Hub>>,
) -> Json<serde_json::Value> {
    let health = hub.check_health().await.unwrap_or(serde_json::json!({
        "mode": "standalone",
        "status": "degraded",
        "db_ping_ms": 0,
        "mesh_active": false,
        "cloud_connected": false,
        "hybrid_mode_ready": false,
        "local_to_cloud_sync_queue": 0,
        "sync_error_count": 0,
    }));

    let failed_missions: i64 = sqlx::query_scalar("SELECT count(*) FROM agent_missions WHERE status = 'FAILED'")
        .fetch_one(&hub.pool)
        .await
        .unwrap_or(0);

    Json(serde_json::json!({
        "mode": health.get("mode").unwrap_or(&serde_json::json!("standalone")),
        "status": health.get("status").unwrap_or(&serde_json::json!("degraded")),
        "db_ping": health.get("db_ping_ms").unwrap_or(&serde_json::json!(0)),
        "sync_backlog": health.get("local_to_cloud_sync_queue").unwrap_or(&serde_json::json!(0)),
        "sync_error_count": health.get("sync_error_count").unwrap_or(&serde_json::json!(0)),
        "hybrid_mode_ready": health.get("hybrid_mode_ready").unwrap_or(&serde_json::json!(false)),
        "failed_missions": failed_missions,
        "mesh_active": health.get("mesh_active").unwrap_or(&serde_json::json!(false)),
        "checklist": health.get("checklist").unwrap_or(&serde_json::json!(Vec::<String>::new()))
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_handler_output() {
        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        let fallback_pg = sqlx::PgPool::connect_lazy("postgres://localhost/dummy").unwrap();
        let db_standalone = crate::db::DB { pool: fallback_pg, store: crate::db::DbStore::Sqlite(sqlx::sqlite::SqlitePoolOptions::new().connect("sqlite::memory:").await.unwrap()) };
        let hub = Arc::new(Hub::new(tx, db_standalone.pool.clone()));

        let response = health_handler(State(hub)).await;

        let json = response.0;
        assert!(json.get("mode").is_some());
        assert!(json.get("status").is_some());
        assert!(json.get("db_ping").is_some());
        assert!(json.get("sync_backlog").is_some());
        assert!(json.get("sync_error_count").is_some());
        assert!(json.get("hybrid_mode_ready").is_some());
        assert!(json.get("failed_missions").is_some());
        assert!(json.get("mesh_active").is_some());
        assert!(json.get("checklist").is_some());
    }
}
