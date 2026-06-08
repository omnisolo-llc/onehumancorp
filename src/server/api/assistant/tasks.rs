use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, patch},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::api::assistant::AssistantState;
use ::server_common::Claims;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Task {
    pub id: String,
    pub workspace_id: String,
    pub title: String,
    pub prompt: String,
    pub status: String,
    pub mode: Option<String>,
    pub model: Option<String>,
    pub provider: Option<String>,
    pub permission_profile: Option<String>,
    pub current_step: Option<String>,
    pub archived: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct CreateTaskRequest {
    pub workspace_id: String,
    pub title: String,
    pub prompt: String,
    pub mode: Option<String>,
    pub model: Option<String>,
    pub provider: Option<String>,
    pub permission_profile: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateTaskRequest {
    pub action: Option<String>,
    pub status: Option<String>,
    pub current_step: Option<String>,
}

pub fn router<S>(state: AssistantState) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(list_tasks).post(create_task))
        .route("/:id", get(get_task).patch(update_task))
        .with_state(state)
}

async fn list_tasks(
    State(state): State<AssistantState>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, Json::<Vec<Task>>(vec![])).into_response(),
    };

    let tasks = match sqlx::query_as::<_, Task>(
        "SELECT id, workspace_id, title, prompt, status, mode, model, provider, permission_profile, current_step, archived, created_at, updated_at FROM assistant_tasks WHERE tenant_id = $1 ORDER BY created_at DESC"
    )
    .bind(tenant_id)
    .fetch_all(&state.db.pool)
    .await {
        Ok(t) => t,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json::<Vec<Task>>(vec![])).into_response(),
    };

    (StatusCode::OK, Json(tasks)).into_response()
}

async fn get_task(
    State(state): State<AssistantState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Unauthorized"}))).into_response(),
    };

    let task = match sqlx::query_as::<_, Task>(
        "SELECT id, workspace_id, title, prompt, status, mode, model, provider, permission_profile, current_step, archived, created_at, updated_at FROM assistant_tasks WHERE id = $1 AND tenant_id = $2"
    )
    .bind(&id)
    .bind(tenant_id)
    .fetch_optional(&state.db.pool)
    .await {
        Ok(Some(t)) => t,
        Ok(None) => return (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Task not found"}))).into_response(),
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to fetch task"}))).into_response(),
    };

    (StatusCode::OK, Json(task)).into_response()
}

async fn create_task(
    State(state): State<AssistantState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateTaskRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(id) => id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Unauthorized"}))).into_response(),
    };

    let id = Uuid::new_v4().to_string();

    match sqlx::query(
        "INSERT INTO assistant_tasks (id, tenant_id, workspace_id, title, prompt, mode, model, provider, permission_profile) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
    )
    .bind(&id)
    .bind(&tenant_id)
    .bind(payload.workspace_id)
    .bind(payload.title)
    .bind(payload.prompt)
    .bind(payload.mode)
    .bind(payload.model)
    .bind(payload.provider)
    .bind(payload.permission_profile)
    .execute(&state.db.pool)
    .await {
        Ok(_) => {
            // Trigger the assistant orchestrator
            let orchestrator = crate::orchestration::assistant::AssistantOrchestrator::new(state.db.clone());
            let _ = orchestrator.start_task(id.clone(), tenant_id).await;

            (StatusCode::CREATED, Json(serde_json::json!({"id": id}))).into_response()
        },
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to create task"}))).into_response(),
    }
}

async fn update_task(
    State(state): State<AssistantState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateTaskRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Unauthorized"}))).into_response(),
    };

    if let Some(action) = payload.action {
        if action == "archive" {
            let _ = sqlx::query("UPDATE assistant_tasks SET archived = TRUE, updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2")
                .bind(&id)
                .bind(tenant_id)
                .execute(&state.db.pool).await;
        } else if action == "unarchive" {
            let _ = sqlx::query("UPDATE assistant_tasks SET archived = FALSE, updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2")
                .bind(&id)
                .bind(tenant_id)
                .execute(&state.db.pool).await;
        }
    }

    if let Some(status) = payload.status {
        let _ = sqlx::query("UPDATE assistant_tasks SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND tenant_id = $3")
            .bind(status)
            .bind(&id)
            .bind(tenant_id)
            .execute(&state.db.pool).await;
    }

    if let Some(step) = payload.current_step {
        let _ = sqlx::query("UPDATE assistant_tasks SET current_step = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2 AND tenant_id = $3")
            .bind(step)
            .bind(&id)
            .bind(tenant_id)
            .execute(&state.db.pool).await;
    }

    (StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
}
