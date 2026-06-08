use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    auth::middleware::TenantId,
    hub::Hub,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct AssistantWorkspace {
    pub id: String,
    pub name: String,
    pub default_model: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssistantTask {
    pub id: String,
    pub workspace_id: String,
    pub title: String,
    pub prompt: String,
    pub status: String,
    pub mode: Option<String>,
    pub model: Option<String>,
    pub permission_profile: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssistantMessage {
    pub id: String,
    pub task_id: String,
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssistantArtifact {
    pub id: String,
    pub task_id: String,
    pub type_: String,
    pub filename: String,
    pub path_ref: String,
    pub mime_type: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateWorkspaceRequest {
    pub name: String,
    pub default_model: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateTaskRequest {
    pub workspace_id: String,
    pub title: String,
    pub prompt: String,
    pub mode: Option<String>,
    pub model: Option<String>,
    pub permission_profile: Option<String>,
}

#[derive(Deserialize)]
pub struct ApprovalCallbackRequest {
    pub approved: bool,
    pub reason: Option<String>,
}

pub fn router() -> Router<Arc<Hub>> {
    Router::new()
        .route("/workspaces", post(create_workspace))
        .route("/tasks", post(create_task))
        .route("/messages/stream", get(stream_messages))
        .route("/approvals/:id/callback", post(approval_callback))
}

async fn create_workspace(
    State(hub): State<Arc<Hub>>,
    TenantId(tenant_id): TenantId,
    Json(payload): Json<CreateWorkspaceRequest>,
) -> Result<Json<AssistantWorkspace>, (StatusCode, String)> {
    let id = format!("ws_{}", Uuid::new_v4().simple());

    // Assume SQLx connection pool in hub, this might need adapting to real codebase
    sqlx::query!(
        r#"
        INSERT INTO assistant_workspaces (id, tenant_id, name, default_model)
        VALUES ($1, $2, $3, $4)
        "#,
        id,
        tenant_id,
        payload.name,
        payload.default_model
    )
    .execute(&hub.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(AssistantWorkspace {
        id,
        name: payload.name,
        default_model: payload.default_model,
    }))
}

async fn create_task(
    State(hub): State<Arc<Hub>>,
    TenantId(tenant_id): TenantId,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<Json<AssistantTask>, (StatusCode, String)> {
    let id = format!("task_{}", Uuid::new_v4().simple());

    sqlx::query!(
        r#"
        INSERT INTO assistant_tasks (id, tenant_id, workspace_id, title, prompt, status, mode, model, permission_profile)
        VALUES ($1, $2, $3, $4, $5, 'PENDING', $6, $7, $8)
        "#,
        id,
        tenant_id,
        payload.workspace_id,
        payload.title,
        payload.prompt,
        payload.mode,
        payload.model,
        payload.permission_profile
    )
    .execute(&hub.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(AssistantTask {
        id,
        workspace_id: payload.workspace_id,
        title: payload.title,
        prompt: payload.prompt,
        status: "PENDING".to_string(),
        mode: payload.mode,
        model: payload.model,
        permission_profile: payload.permission_profile,
    }))
}

async fn stream_messages(
    State(_hub): State<Arc<Hub>>,
    TenantId(_tenant_id): TenantId,
) -> Result<Json<Vec<AssistantMessage>>, (StatusCode, String)> {
    // In a real streaming endpoint, this would return SSE.
    // For this basic API layer implementation, we return an empty JSON array for now.
    Ok(Json(vec![]))
}

async fn approval_callback(
    State(_hub): State<Arc<Hub>>,
    TenantId(_tenant_id): TenantId,
    Path(id): Path<String>,
    Json(payload): Json<ApprovalCallbackRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    Ok(Json(serde_json::json!({
        "id": id,
        "approved": payload.approved,
        "status": if payload.approved { "approved" } else { "rejected" }
    })))
}
