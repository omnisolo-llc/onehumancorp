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
        "mesh_active": health.get("mesh_active").unwrap_or(&serde_json::json!(false))
    }))
}
