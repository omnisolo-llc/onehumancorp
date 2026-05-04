use axum::{
    response::IntoResponse,
    Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct SyncStateResponse {
    pub meetings: Vec<String>,
    pub agents: Vec<String>,
}

pub async fn sync_state_handler() -> impl IntoResponse {
    let res = SyncStateResponse {
        meetings: vec![],
        agents: vec![],
    };
    Json(res)
}

#[derive(Deserialize)]
pub struct SyncMutationsRequest {
    pub mutations: Vec<serde_json::Value>,
}

#[derive(Serialize)]
pub struct SyncMutationsResponse {
    pub status: String,
}

pub async fn sync_mutations_handler(
    Json(_payload): Json<SyncMutationsRequest>,
) -> impl IntoResponse {
    let res = SyncMutationsResponse {
        status: "synced".to_string(),
    };
    Json(res)
}

pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/state", get(sync_state_handler))
        .route("/mutations", post(sync_mutations_handler))
}
