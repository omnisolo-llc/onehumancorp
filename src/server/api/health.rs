use axum::{extract::State, Json};
use std::sync::Arc;
use crate::hub::Hub;

pub async fn health_handler(
    State(hub): State<Arc<Hub>>,
) -> Json<serde_json::Value> {
    let hub_clone = hub.clone();

    let (health_res, stuck_missions_res) = tokio::join!(
        async {
            hub_clone.check_health().await.unwrap_or(serde_json::json!({
                "mode": "standalone",
                "status": "degraded",
                "db_ping_ms": 0,
                "mesh_active": false,
                "cloud_connected": false,
                "hybrid_mode_ready": false,
                "local_to_cloud_sync_queue": 0,
                "sync_error_count": 0,
            }))
        },
        async {
            sqlx::query_scalar::<_, i64>("SELECT count(*) FROM agent_missions WHERE status = 'STUCK'")
                .fetch_one(&hub.pool)
                .await
                .unwrap_or(0)
        }
    );

    let health = health_res;
    let stuck_missions = stuck_missions_res;

    Json(serde_json::json!({
        "mode": health.get("mode").unwrap_or(&serde_json::json!("standalone")),
        "status": health.get("status").unwrap_or(&serde_json::json!("degraded")),
        "db_ping": health.get("db_ping_ms").unwrap_or(&serde_json::json!(0)),
        "sync_backlog": health.get("local_to_cloud_sync_queue").unwrap_or(&serde_json::json!(0)),
        "sync_error_count": health.get("sync_error_count").unwrap_or(&serde_json::json!(0)),
        "hybrid_mode_ready": health.get("hybrid_mode_ready").unwrap_or(&serde_json::json!(false)),
        "stuck_missions": stuck_missions,
        "mesh_active": health.get("mesh_active").unwrap_or(&serde_json::json!(false)),
        "checklist": health.get("checklist").unwrap_or(&serde_json::json!(Vec::<String>::new()))
    }))
}
