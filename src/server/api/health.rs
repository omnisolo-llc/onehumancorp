use axum::{response::IntoResponse, http::StatusCode};
use axum::extract::State;
use std::sync::Arc;
use crate::hub::Hub;

pub async fn check_health_wrapper(State(hub): State<Arc<Hub>>) -> impl IntoResponse {
    match hub.check_health().await {
        Ok(health) => (StatusCode::OK, axum::Json(health)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, axum::Json(serde_json::json!({"error": e.to_string()}))).into_response(),
    }
}
