use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use crate::api::assistant::AssistantState;
use ::server_common::Claims;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Artifact {
    pub id: String,
    pub task_id: String,
    pub type_name: String,
    pub filename: String,
    pub path: Option<String>,
    pub mime_type: Option<String>,
    pub size: Option<i64>,
    pub preview: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

pub fn router<S>(state: AssistantState) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(list_artifacts))
        .route("/task/:task_id", get(list_task_artifacts))
        .with_state(state)
}

async fn list_artifacts(
    State(state): State<AssistantState>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, Json::<Vec<Artifact>>(vec![])).into_response(),
    };

    let artifacts = match sqlx::query_as::<_, Artifact>(
        "SELECT id, task_id, type as type_name, filename, path, mime_type, size, preview, created_at FROM assistant_artifacts WHERE tenant_id = $1 ORDER BY created_at DESC"
    )
    .bind(tenant_id)
    .fetch_all(&state.db.pool)
    .await {
        Ok(a) => a,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json::<Vec<Artifact>>(vec![])).into_response(),
    };

    (StatusCode::OK, Json(artifacts)).into_response()
}

async fn list_task_artifacts(
    State(state): State<AssistantState>,
    Extension(claims): Extension<Claims>,
    Path(task_id): Path<String>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, Json::<Vec<Artifact>>(vec![])).into_response(),
    };

    let artifacts = match sqlx::query_as::<_, Artifact>(
        "SELECT id, task_id, type as type_name, filename, path, mime_type, size, preview, created_at FROM assistant_artifacts WHERE task_id = $1 AND tenant_id = $2 ORDER BY created_at DESC"
    )
    .bind(task_id)
    .bind(tenant_id)
    .fetch_all(&state.db.pool)
    .await {
        Ok(a) => a,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json::<Vec<Artifact>>(vec![])).into_response(),
    };

    (StatusCode::OK, Json(artifacts)).into_response()
}
