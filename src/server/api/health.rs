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

    let stuck_missions: i64 = sqlx::query_scalar("SELECT count(*) FROM agent_missions WHERE status = 'STUCK'")
        .fetch_one(&hub.pool)
        .await
        .unwrap_or(0);

    Json(serde_json::json!({
        "mode": health.get("mode").unwrap_or(&serde_json::json!("standalone")),
        "status": health.get("status").unwrap_or(&serde_json::json!("degraded")),
        "db_ping": health.get("db_ping_ms").unwrap_or(&serde_json::json!(0)),
        "sync_backlog": health.get("local_to_cloud_sync_queue").unwrap_or(&serde_json::json!(0)),
        "stuck_missions": stuck_missions,
        "mesh_active": health.get("mesh_active").unwrap_or(&serde_json::json!(false)),
        "hybrid_mode_ready": health.get("hybrid_mode_ready").unwrap_or(&serde_json::json!(false)),
        "cloud_connected": health.get("cloud_connected").unwrap_or(&serde_json::json!(false)),
        "sync_error_count": health.get("sync_error_count").unwrap_or(&serde_json::json!(0))
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::State;
    use tokio::sync::mpsc;
    use std::sync::Arc;
    use crate::hub::Hub;

    #[tokio::test]
    async fn test_health_handler_fields() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let db_url = std::env::var("DATABASE_URL").unwrap();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy(&db_url)
            .unwrap();
        let (tx, _) = mpsc::channel(100);
        let hub = Arc::new(Hub::new(tx, pool));

        let res = health_handler(State(hub)).await;

        assert!(res.get("mode").is_some());
        assert!(res.get("status").is_some());
        assert!(res.get("db_ping").is_some());
        assert!(res.get("sync_backlog").is_some());
        assert!(res.get("stuck_missions").is_some());
        assert!(res.get("mesh_active").is_some());
        assert!(res.get("hybrid_mode_ready").is_some());
        assert!(res.get("cloud_connected").is_some());
        assert!(res.get("sync_error_count").is_some());
    }
}
