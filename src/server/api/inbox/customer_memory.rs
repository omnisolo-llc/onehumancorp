use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

pub struct CustomerMemoryState {
    pub db: Arc<()>,
}

pub fn router(db: Arc<()>) -> axum::Router {
    let state = CustomerMemoryState { db };
    axum::Router::new()
        .route("/ingest", axum::routing::post(ingest_event))
        .with_state(state)
}

pub async fn ingest_event(
    State(_state): State<CustomerMemoryState>,
    Json(_payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    (StatusCode::ACCEPTED, "Event ingested")
}
