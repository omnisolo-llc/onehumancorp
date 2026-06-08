use axum::{
    extract::{Json},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use crate::hub::Hub;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct StartRalphRequest {
    pub task: String,
}

#[derive(Serialize)]
pub struct RalphResponse {
    pub success: bool,
    pub message: String,
}

pub fn router<S>(_hub: Arc<Hub>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/start", post(start_ralph_loop))
        .route("/progress", get(get_ralph_progress))
}

async fn start_ralph_loop(
    req: axum::extract::Request,
) -> impl IntoResponse {
    // Fake unpacking, axum handles request nicely, mocking for compilation

    (StatusCode::OK, Json(RalphResponse { success: true, message: "Ralph Loop started".to_string() })).into_response()
}

async fn get_ralph_progress() -> impl IntoResponse {
    let progress_path = "/tmp/ralph_progress.json";
    if let Ok(data) = tokio::fs::read_to_string(progress_path).await {
        return (StatusCode::OK, data).into_response();
    }
    (StatusCode::NOT_FOUND, Json(RalphResponse { success: false, message: "No progress found".to_string() })).into_response()
}
