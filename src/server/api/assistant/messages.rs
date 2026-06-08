use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::api::assistant::AssistantState;
use ::server_common::Claims;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Message {
    pub id: String,
    pub task_id: String,
    pub role: String,
    pub content: String,
    pub attachments: serde_json::Value,
    pub tool_calls: serde_json::Value,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct CreateMessageRequest {
    pub task_id: String,
    pub role: String,
    pub content: String,
    pub attachments: Option<serde_json::Value>,
}

pub fn router<S>(state: AssistantState) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(list_messages).post(create_message))
        .route("/task/:task_id", get(list_task_messages))
        .with_state(state)
}

async fn list_messages(
    State(state): State<AssistantState>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, Json::<Vec<Message>>(vec![])).into_response(),
    };

    let messages = match sqlx::query_as::<_, Message>(
        "SELECT id, task_id, role, content, attachments, tool_calls, created_at FROM assistant_messages WHERE tenant_id = $1 ORDER BY created_at ASC"
    )
    .bind(tenant_id)
    .fetch_all(&state.db.pool)
    .await {
        Ok(m) => m,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json::<Vec<Message>>(vec![])).into_response(),
    };

    (StatusCode::OK, Json(messages)).into_response()
}

async fn list_task_messages(
    State(state): State<AssistantState>,
    Extension(claims): Extension<Claims>,
    Path(task_id): Path<String>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, Json::<Vec<Message>>(vec![])).into_response(),
    };

    let messages = match sqlx::query_as::<_, Message>(
        "SELECT id, task_id, role, content, attachments, tool_calls, created_at FROM assistant_messages WHERE task_id = $1 AND tenant_id = $2 ORDER BY created_at ASC"
    )
    .bind(task_id)
    .bind(tenant_id)
    .fetch_all(&state.db.pool)
    .await {
        Ok(m) => m,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json::<Vec<Message>>(vec![])).into_response(),
    };

    (StatusCode::OK, Json(messages)).into_response()
}

async fn create_message(
    State(state): State<AssistantState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateMessageRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(id) => id,
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({"error": "Unauthorized"}))).into_response(),
    };

    let id = Uuid::new_v4().to_string();
    let attachments = payload.attachments.unwrap_or(serde_json::json!([]));

    match sqlx::query(
        "INSERT INTO assistant_messages (id, tenant_id, task_id, role, content, attachments) VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(&id)
    .bind(tenant_id)
    .bind(payload.task_id)
    .bind(payload.role)
    .bind(payload.content)
    .bind(attachments)
    .execute(&state.db.pool)
    .await {
        Ok(_) => (StatusCode::CREATED, Json(serde_json::json!({"id": id}))).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to create message"}))).into_response(),
    }
}
