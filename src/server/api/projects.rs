use axum::{Router, routing::{get, post}, extract::{State, Path}, Json, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct ProjectResponse {
    pub status: String,
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(list_projects).post(create_project))
}

async fn list_projects() -> impl IntoResponse {
    Json(ProjectResponse { status: "ok".to_string() })
}

async fn create_project() -> impl IntoResponse {
    Json(ProjectResponse { status: "created".to_string() })
}
