use axum::extract::State;
use std::sync::Arc;
use crate::hub::Hub;
use crate::api::health::health_handler;

#[tokio::test]
async fn test_health_handler_basic() {
    let pool = crate::db::get_pool();
    let (tx, _) = tokio::sync::mpsc::channel(10);
    let hub = Arc::new(Hub::new(tx, pool));

    // Call the handler directly
    let response = health_handler(State(hub)).await;

    // The response is a Json<serde_json::Value>, get the internal value
    let json = response.0;

    // Verify fields from the handler exist
    assert!(json.get("mode").is_some(), "mode field missing");
    assert!(json.get("status").is_some(), "status field missing");
    assert!(json.get("db_ping").is_some(), "db_ping field missing");
    assert!(json.get("sync_backlog").is_some(), "sync_backlog field missing");
    assert!(json.get("sync_error_count").is_some(), "sync_error_count field missing");
    assert!(json.get("hybrid_mode_ready").is_some(), "hybrid_mode_ready field missing");
    assert!(json.get("mesh_active").is_some(), "mesh_active field missing");
    assert!(json.get("checklist").is_some(), "checklist field missing");

    // Verify the newly added fields exist
    assert!(json.get("pending_missions").is_some(), "pending_missions field missing");
    assert!(json.get("failed_missions").is_some(), "failed_missions field missing");
}
