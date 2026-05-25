use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::hub::Hub;

#[derive(Serialize, Deserialize)]
pub struct HybridHealthProbe {
    pub mode: String,
    pub status: String,
    pub db_ping: u64,
    pub unsynced_missions: i64,
    pub sync_error_count: i64,
    pub hybrid_mode_ready: bool,
    pub stuck_missions: i64,
    pub mesh_active: bool,
    pub checklist: Vec<String>,
}

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

    Json(serde_json::to_value(HybridHealthProbe {
        mode: health.get("mode").and_then(|v| v.as_str()).unwrap_or("standalone").to_string(),
        status: health.get("status").and_then(|v| v.as_str()).unwrap_or("degraded").to_string(),
        db_ping: health.get("db_ping_ms").and_then(|v| v.as_u64()).unwrap_or(0),
        unsynced_missions: health.get("local_to_cloud_sync_queue").and_then(|v| v.as_i64()).unwrap_or(0),
        sync_error_count: health.get("sync_error_count").and_then(|v| v.as_i64()).unwrap_or(0),
        hybrid_mode_ready: health.get("hybrid_mode_ready").and_then(|v| v.as_bool()).unwrap_or(false),
        stuck_missions,
        mesh_active: health.get("mesh_active").and_then(|v| v.as_bool()).unwrap_or(false),
        checklist: health.get("checklist").and_then(|v| v.as_array()).map(|a| a.iter().filter_map(|i| i.as_str().map(|s| s.to_string())).collect()).unwrap_or_default(),
    }).unwrap())
}
