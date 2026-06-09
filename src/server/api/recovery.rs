use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

#[derive(Clone)]
pub struct AppState {}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/campaigns", get(list_campaigns))
        .route("/attempts", get(list_attempts))
        .route("/attempts/:id/approve", post(approve_attempt))
}

async fn list_campaigns() -> impl IntoResponse {
    (StatusCode::OK, Json(vec![] as Vec<String>))
}

async fn list_attempts() -> impl IntoResponse {
    (StatusCode::OK, Json(vec![] as Vec<String>))
}

async fn approve_attempt(Path(id): Path<String>) -> impl IntoResponse {
    (StatusCode::OK, Json(format!("Attempt {} approved", id)))
}
