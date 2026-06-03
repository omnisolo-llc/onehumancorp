use axum::{extract::State, routing::post, Json, Router};
use std::sync::Arc;
use serde_json::{json, Value};

#[derive(Clone)]
pub struct AppState {}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/payroll/sync", post(sync_payroll))
}

async fn sync_payroll(State(_state): State<AppState>) -> Json<Value> {
    Json(json!({ "status": "ok", "message": "Payroll sync queued" }))
}
