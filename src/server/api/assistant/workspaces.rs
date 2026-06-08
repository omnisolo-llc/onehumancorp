use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::api::assistant::AssistantState;
use ::server_common::Claims;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub default_work_directory: Option<String>,
    pub default_model: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateWorkspaceRequest {
    pub name: String,
    pub default_work_directory: Option<String>,
    pub default_model: Option<String>,
}

pub fn router<S>(state: AssistantState) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(list_workspaces).post(create_workspace))
        .with_state(state)
}

async fn list_workspaces(
    State(state): State<AssistantState>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, Json::<Vec<Workspace>>(vec![])).into_response(),
    };

    let workspaces = match sqlx::query_as::<_, Workspace>(
        "SELECT id, name, default_work_directory, default_model FROM assistant_workspaces WHERE tenant_id = $1"
    )
    .bind(tenant_id)
    .fetch_all(&state.db.pool)
    .await {
        Ok(w) => w,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json::<Vec<Workspace>>(vec![])).into_response(),
    };

    (StatusCode::OK, Json(workspaces)).into_response()
}

async fn create_workspace(
    State(state): State<AssistantState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateWorkspaceRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Unauthorized"}))).into_response(),
    };

    let id = Uuid::new_v4().to_string();

    match sqlx::query(
        "INSERT INTO assistant_workspaces (id, tenant_id, name, default_work_directory, default_model) VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(&id)
    .bind(tenant_id)
    .bind(payload.name)
    .bind(payload.default_work_directory)
    .bind(payload.default_model)
    .execute(&state.db.pool)
    .await {
        Ok(_) => (StatusCode::CREATED, Json(serde_json::json!({"id": id}))).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to create workspace"}))).into_response(),
    }
}
