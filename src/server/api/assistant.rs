use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    routing::get,
    Router,
    Json,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::db::DB;
use ::server_common::Claims;
use uuid::Uuid;
use chrono::Utc;

pub fn router<S>(db: Arc<DB>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/workspaces", get(list_workspaces).post(create_workspace))
        .route("/workspaces/:id", get(get_workspace))
        .route("/tasks", get(list_tasks).post(create_task))
        .route("/tasks/:id", get(get_task))
        .route("/tasks/:id/messages", get(list_messages).post(create_message))
        .route("/tasks/:id/artifacts", get(list_artifacts).post(create_artifact))
        .route("/tasks/:id/file_changes", get(list_file_changes).post(create_file_change))
        .route("/automations", get(list_automations).post(create_automation))
        .layer(Extension(db))
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub default_work_dir: Option<String>,
    pub default_model: Option<String>,
    pub created_at_unix: i64,
    pub updated_at_unix: i64,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Task {
    pub id: String,
    pub workspace_id: String,
    pub title: String,
    pub prompt: String,
    pub status: String,
    pub mode: Option<String>,
    pub permission_profile: String,
    pub model_config_json: Option<serde_json::Value>,
    pub current_step: Option<String>,
    pub archived: bool,
    pub created_at_unix: i64,
    pub updated_at_unix: i64,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Message {
    pub id: String,
    pub task_id: String,
    pub role: String,
    pub content: String,
    pub tool_metadata_json: Option<serde_json::Value>,
    pub created_at_unix: i64,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Artifact {
    pub id: String,
    pub task_id: String,
    #[sqlx(rename = "type_")]
    pub type_: String,
    pub filename: String,
    pub path: String,
    pub mime_type: String,
    pub size: Option<i64>,
    pub preview_ref: Option<String>,
    pub created_at_unix: i64,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct FileChange {
    pub id: String,
    pub task_id: String,
    pub path: String,
    pub change_type: String,
    pub summary: Option<String>,
    pub approval_status: String,
    pub created_at_unix: i64,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Automation {
    pub id: String,
    pub workspace_id: String,
    pub schedule: String,
    pub prompt: String,
    pub context: Option<String>,
    pub model: String,
    pub permission_profile: String,
    pub notification_channel: Option<String>,
    pub status: String,
    pub created_at_unix: i64,
    pub updated_at_unix: i64,
}

async fn list_workspaces(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<Workspace>>, (StatusCode, String)> {
    let workspaces = sqlx::query_as::<_, Workspace>(
        r#"SELECT id, name, default_work_dir, default_model, created_at_unix, updated_at_unix
        FROM workspaces
        WHERE tenant_id = $1"#
    )
    .bind(&claims.organization_id.unwrap_or_default())
    .fetch_all(&db.pool)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to list workspaces".to_string())
    })?;

    Ok(Json(workspaces))
}

async fn create_workspace(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<Workspace>,
) -> Result<Json<Workspace>, (StatusCode, String)> {
    let id = Uuid::new_v4().to_string();
    let created_at_unix = Utc::now().timestamp();
    let updated_at_unix = created_at_unix;

    sqlx::query(
        r#"INSERT INTO workspaces (id, tenant_id, name, default_work_dir, default_model, created_at_unix, updated_at_unix)
        VALUES ($1, $2, $3, $4, $5, $6, $7)"#
    )
    .bind(&id)
    .bind(&claims.organization_id.unwrap_or_default())
    .bind(&payload.name)
    .bind(&payload.default_work_dir)
    .bind(&payload.default_model)
    .bind(created_at_unix)
    .bind(updated_at_unix)
    .execute(&db.pool)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create workspace".to_string())
    })?;

    let mut ws = payload;
    ws.id = id;
    ws.created_at_unix = created_at_unix;
    ws.updated_at_unix = updated_at_unix;
    Ok(Json(ws))
}

async fn get_workspace(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<Workspace>, (StatusCode, String)> {
    let workspace = sqlx::query_as::<_, Workspace>(
        r#"SELECT id, name, default_work_dir, default_model, created_at_unix, updated_at_unix
        FROM workspaces
        WHERE id = $1 AND tenant_id = $2"#
    )
    .bind(&id)
    .bind(&claims.organization_id.unwrap_or_default())
    .fetch_optional(&db.pool)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get workspace".to_string())
    })?
    .ok_or((StatusCode::NOT_FOUND, "Workspace not found".to_string()))?;

    Ok(Json(workspace))
}

#[derive(Serialize)]
pub struct ListTasksResponse {
    tasks: Vec<Task>,
}

async fn list_tasks(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ListTasksResponse>, (StatusCode, String)> {
    let tasks = sqlx::query_as::<_, Task>(
        r#"SELECT id, workspace_id, title, prompt, status, mode, permission_profile, model_config_json, current_step, archived, created_at_unix, updated_at_unix
        FROM assistant_tasks
        WHERE tenant_id = $1
        ORDER BY created_at_unix DESC"#
    )
    .bind(&claims.organization_id.unwrap_or_default())
    .fetch_all(&db.pool)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to list tasks".to_string())
    })?;

    Ok(Json(ListTasksResponse { tasks }))
}

async fn create_task(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<Task>,
) -> Result<Json<Task>, (StatusCode, String)> {
    let id = Uuid::new_v4().to_string();
    let created_at_unix = Utc::now().timestamp();
    let updated_at_unix = created_at_unix;

    sqlx::query(
        r#"INSERT INTO assistant_tasks (id, tenant_id, workspace_id, title, prompt, status, mode, permission_profile, model_config_json, current_step, archived, created_at_unix, updated_at_unix)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)"#
    )
    .bind(&id)
    .bind(&claims.organization_id.unwrap_or_default())
    .bind(&payload.workspace_id)
    .bind(&payload.title)
    .bind(&payload.prompt)
    .bind(&payload.status)
    .bind(&payload.mode)
    .bind(&payload.permission_profile)
    .bind(&payload.model_config_json)
    .bind(&payload.current_step)
    .bind(payload.archived)
    .bind(created_at_unix)
    .bind(updated_at_unix)
    .execute(&db.pool)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create task".to_string())
    })?;

    let mut task = payload;
    task.id = id;
    task.created_at_unix = created_at_unix;
    task.updated_at_unix = updated_at_unix;
    Ok(Json(task))
}

async fn get_task(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<Task>, (StatusCode, String)> {
    let task = sqlx::query_as::<_, Task>(
        r#"SELECT id, workspace_id, title, prompt, status, mode, permission_profile, model_config_json, current_step, archived, created_at_unix, updated_at_unix
        FROM assistant_tasks
        WHERE id = $1 AND tenant_id = $2"#
    )
    .bind(&id)
    .bind(&claims.organization_id.unwrap_or_default())
    .fetch_optional(&db.pool)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to get task".to_string())
    })?
    .ok_or((StatusCode::NOT_FOUND, "Task not found".to_string()))?;

    Ok(Json(task))
}

async fn list_messages(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<Vec<Message>>, (StatusCode, String)> {
    let messages = sqlx::query_as::<_, Message>(
        r#"SELECT id, task_id, role, content, tool_metadata_json, created_at_unix
        FROM assistant_task_messages
        WHERE task_id = $1 AND tenant_id = $2
        ORDER BY created_at_unix ASC"#
    )
    .bind(&id)
    .bind(&claims.organization_id.unwrap_or_default())
    .fetch_all(&db.pool)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to list messages".to_string())
    })?;

    Ok(Json(messages))
}

async fn create_message(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(payload): Json<Message>,
) -> Result<Json<Message>, (StatusCode, String)> {
    let msg_id = Uuid::new_v4().to_string();
    let created_at_unix = Utc::now().timestamp();

    sqlx::query(
        r#"INSERT INTO assistant_task_messages (id, tenant_id, task_id, role, content, tool_metadata_json, created_at_unix)
        VALUES ($1, $2, $3, $4, $5, $6, $7)"#
    )
    .bind(&msg_id)
    .bind(&claims.organization_id.unwrap_or_default())
    .bind(&id)
    .bind(&payload.role)
    .bind(&payload.content)
    .bind(&payload.tool_metadata_json)
    .bind(created_at_unix)
    .execute(&db.pool)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create message".to_string())
    })?;

    let mut msg = payload;
    msg.task_id = id;
    msg.id = msg_id;
    msg.created_at_unix = created_at_unix;
    Ok(Json(msg))
}

async fn list_artifacts(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<Vec<Artifact>>, (StatusCode, String)> {
    let artifacts = sqlx::query_as::<_, Artifact>(
        r#"SELECT id, task_id, type_, filename, path, mime_type, size, preview_ref, created_at_unix
        FROM assistant_task_artifacts
        WHERE task_id = $1 AND tenant_id = $2
        ORDER BY created_at_unix ASC"#
    )
    .bind(&id)
    .bind(&claims.organization_id.unwrap_or_default())
    .fetch_all(&db.pool)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to list artifacts".to_string())
    })?;

    Ok(Json(artifacts))
}

async fn create_artifact(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(payload): Json<Artifact>,
) -> Result<Json<Artifact>, (StatusCode, String)> {
    let artifact_id = Uuid::new_v4().to_string();
    let created_at_unix = Utc::now().timestamp();

    sqlx::query(
        r#"INSERT INTO assistant_task_artifacts (id, tenant_id, task_id, type_, filename, path, mime_type, size, preview_ref, created_at_unix)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"#
    )
    .bind(&artifact_id)
    .bind(&claims.organization_id.unwrap_or_default())
    .bind(&id)
    .bind(&payload.type_)
    .bind(&payload.filename)
    .bind(&payload.path)
    .bind(&payload.mime_type)
    .bind(payload.size)
    .bind(&payload.preview_ref)
    .bind(created_at_unix)
    .execute(&db.pool)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create artifact".to_string())
    })?;

    let mut artifact = payload;
    artifact.task_id = id;
    artifact.id = artifact_id;
    artifact.created_at_unix = created_at_unix;
    Ok(Json(artifact))
}

async fn list_file_changes(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<Vec<FileChange>>, (StatusCode, String)> {
    let changes = sqlx::query_as::<_, FileChange>(
        r#"SELECT id, task_id, path, change_type, summary, approval_status, created_at_unix
        FROM assistant_task_file_changes
        WHERE task_id = $1 AND tenant_id = $2
        ORDER BY created_at_unix ASC"#
    )
    .bind(&id)
    .bind(&claims.organization_id.unwrap_or_default())
    .fetch_all(&db.pool)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to list file changes".to_string())
    })?;

    Ok(Json(changes))
}

async fn create_file_change(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(payload): Json<FileChange>,
) -> Result<Json<FileChange>, (StatusCode, String)> {
    let change_id = Uuid::new_v4().to_string();
    let created_at_unix = Utc::now().timestamp();

    sqlx::query(
        r#"INSERT INTO assistant_task_file_changes (id, tenant_id, task_id, path, change_type, summary, approval_status, created_at_unix)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#
    )
    .bind(&change_id)
    .bind(&claims.organization_id.unwrap_or_default())
    .bind(&id)
    .bind(&payload.path)
    .bind(&payload.change_type)
    .bind(&payload.summary)
    .bind(&payload.approval_status)
    .bind(created_at_unix)
    .execute(&db.pool)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create file change".to_string())
    })?;

    let mut change = payload;
    change.task_id = id;
    change.id = change_id;
    change.created_at_unix = created_at_unix;
    Ok(Json(change))
}

async fn list_automations(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<Automation>>, (StatusCode, String)> {
    let automations = sqlx::query_as::<_, Automation>(
        r#"SELECT id, workspace_id, schedule, prompt, context, model, permission_profile, notification_channel, status, created_at_unix, updated_at_unix
        FROM assistant_automations
        WHERE tenant_id = $1
        ORDER BY created_at_unix DESC"#
    )
    .bind(&claims.organization_id.unwrap_or_default())
    .fetch_all(&db.pool)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to list automations".to_string())
    })?;

    Ok(Json(automations))
}

async fn create_automation(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<Automation>,
) -> Result<Json<Automation>, (StatusCode, String)> {
    let automation_id = Uuid::new_v4().to_string();
    let created_at_unix = Utc::now().timestamp();
    let updated_at_unix = created_at_unix;

    sqlx::query(
        r#"INSERT INTO assistant_automations (id, tenant_id, workspace_id, schedule, prompt, context, model, permission_profile, notification_channel, status, created_at_unix, updated_at_unix)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)"#
    )
    .bind(&automation_id)
    .bind(&claims.organization_id.unwrap_or_default())
    .bind(&payload.workspace_id)
    .bind(&payload.schedule)
    .bind(&payload.prompt)
    .bind(&payload.context)
    .bind(&payload.model)
    .bind(&payload.permission_profile)
    .bind(&payload.notification_channel)
    .bind(&payload.status)
    .bind(created_at_unix)
    .bind(updated_at_unix)
    .execute(&db.pool)
    .await
    .map_err(|e| {
        tracing::error!("DB error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create automation".to_string())
    })?;

    let mut automation = payload;
    automation.id = automation_id;
    automation.created_at_unix = created_at_unix;
    automation.updated_at_unix = updated_at_unix;
    Ok(Json(automation))
}
