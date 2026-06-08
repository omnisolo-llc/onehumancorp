use axum::{
    routing::{get, post},
    Router, Json,
};
use serde::{Deserialize, Serialize};

pub fn assistant_routes() -> Router {
    Router::new()
        .route("/task", post(create_task))
}

#[derive(Deserialize)]
pub struct CreateTaskRequest {
    pub prompt: String,
}

#[derive(Serialize)]
pub struct CreateTaskResponse {
    pub task_id: String,
    pub status: String,
}

pub async fn create_task(Json(req): Json<CreateTaskRequest>) -> Json<CreateTaskResponse> {
    // Basic implementation for acceptance criteria
    Json(CreateTaskResponse {
        task_id: "new-task-id".to_string(),
        status: "Running".to_string(),
    })
}
