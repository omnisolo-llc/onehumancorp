use axum::{Json, routing::get, Router};
use std::sync::Arc;

pub fn router() -> Router<Arc<crate::hub::Hub>> {
    Router::new()
        .route("/ui/triage", get(|| async { Json(serde_json::json!([])) }))
        .route("/ui/triage/action", axum::routing::post(|| async { Json(serde_json::json!({"status": "success"})) }))
}
