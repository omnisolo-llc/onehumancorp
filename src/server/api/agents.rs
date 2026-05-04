use axum::{
    response::IntoResponse,
    Json,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CommandRequest {
    pub task_id: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct CommandResponse {
    pub task_id: String,
    pub status: String,
}

pub async fn command_handler(
    Json(payload): Json<CommandRequest>,
) -> impl IntoResponse {
    let res = CommandResponse {
        task_id: payload.task_id,
        status: "processing".to_string(), // Echoing the status from the spec
    };

    Json(res)
}

pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/command", post(command_handler))
}
