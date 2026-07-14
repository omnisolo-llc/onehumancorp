use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

pub fn router(db: Arc<()>) -> axum::Router {
    axum::Router::new()
        .route("/{id}/edit", axum::routing::put(edit_draft))
        .with_state(db)
}

pub async fn edit_draft(
    State(_db): State<Arc<()>>,
    Path(_id): Path<String>,
    Json(_payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    (StatusCode::OK, "Draft updated")
}
