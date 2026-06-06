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

    // Fetch lightweight in-memory cache status
    let last_prune = crate::sip::LAST_SUCCESSFUL_PRUNE.load(std::sync::atomic::Ordering::SeqCst);

    Json(serde_json::json!({
        "mode": health.get("mode").unwrap_or(&serde_json::json!("standalone")),
        "status": health.get("status").unwrap_or(&serde_json::json!("degraded")),
        "db_ping": health.get("db_ping_ms").unwrap_or(&serde_json::json!(0)),
        "sync_backlog": health.get("local_to_cloud_sync_queue").unwrap_or(&serde_json::json!(0)),
        "sync_error_count": health.get("sync_error_count").unwrap_or(&serde_json::json!(0)),
        "hybrid_mode_ready": health.get("hybrid_mode_ready").unwrap_or(&serde_json::json!(false)),
        "last_successful_prune_ts": last_prune,
        "mesh_active": health.get("mesh_active").unwrap_or(&serde_json::json!(false)),
        "checklist": health.get("checklist").unwrap_or(&serde_json::json!(Vec::<String>::new()))
    }))
}
