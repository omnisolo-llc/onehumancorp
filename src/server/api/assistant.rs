use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    routing::{get, post, patch},
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
        .route("/workspaces", get(list_workspaces).post(create_workspace).patch(update_workspace))
        .route("/workspaces/:id", get(get_workspace))
        .route("/tasks", get(list_tasks).post(create_task))
        .route("/tasks/:id", get(get_task).patch(update_task))
        .route("/tasks/:id/messages", get(list_messages).post(create_message))
        .route("/tasks/:id/artifacts", get(list_artifacts).post(create_artifact))
        .route("/tasks/:id/file_changes", get(list_file_changes).post(create_file_change))
        .route("/memory", get(get_memory).post(post_memory))
        .route("/settings", get(get_settings).patch(patch_settings))
        .route("/billing", get(get_billing))
        .route("/share", post(post_share).patch(patch_share))
        .route("/previews", patch(patch_previews))
        .route("/artifacts", post(post_artifacts))
        .route("/uploads", post(post_uploads))
        .route("/experts", patch(patch_experts))
        .route("/commands", post(post_commands))
        .route("/mcp", patch(patch_mcp))
        .route("/models", patch(patch_models))
        .route("/explore", post(post_explore).patch(patch_explore))
        .route("/cloud", post(post_cloud).patch(patch_cloud))
        .route("/plugins", patch(patch_plugins))
        .route("/claw", patch(patch_claw))
        .route("/approvals", post(post_approvals))
        .route("/support", post(post_support))
        .layer(Extension(db))
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct WorkspaceRow {
    pub id: String,
    pub name: String,
    pub default_work_dir: Option<String>,
    pub default_model: Option<String>,
    pub created_at_unix: Option<i64>,
    pub updated_at_unix: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub default_work_dir: Option<String>,
    pub default_model: Option<String>,
    pub created_at_unix: i64,
    pub updated_at_unix: i64,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct TaskRow {
    pub id: String,
    pub workspace_id: String,
    pub title: String,
    pub prompt: String,
    pub status: String,
    pub mode: Option<String>,
    pub permission_profile: String,
    pub model_config: Option<serde_json::Value>,
    pub current_step: Option<String>,
    pub archived: Option<bool>,
    pub created_at_unix: Option<i64>,
    pub updated_at_unix: Option<i64>,
}

#[derive(Serialize, Deserialize)]
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
pub struct MessageRow {
    pub id: String,
    pub task_id: String,
    pub role: String,
    pub content: String,
    pub tool_metadata: Option<serde_json::Value>,
    pub created_at_unix: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub task_id: String,
    pub role: String,
    pub content: String,
    pub tool_metadata_json: Option<serde_json::Value>,
    pub created_at_unix: i64,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct ArtifactRow {
    pub id: String,
    pub task_id: String,
    pub type_: String,
    pub filename: String,
    pub path: String,
    pub mime_type: String,
    pub size: Option<i64>,
    pub preview_ref: Option<String>,
    pub created_at_unix: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Artifact {
    pub id: String,
    pub task_id: String,
    pub type_: String,
    pub filename: String,
    pub path: String,
    pub mime_type: String,
    pub size: Option<i64>,
    pub preview_ref: Option<String>,
    pub created_at_unix: i64,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct FileChangeRow {
    pub id: String,
    pub task_id: String,
    pub path: String,
    pub change_type: String,
    pub summary: Option<String>,
    pub approval_status: String,
    pub created_at_unix: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct FileChange {
    pub id: String,
    pub task_id: String,
    pub path: String,
    pub change_type: String,
    pub summary: Option<String>,
    pub approval_status: String,
    pub created_at_unix: i64,
}

async fn list_workspaces(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<Workspace>>, (StatusCode, String)> {
    let rows: Vec<WorkspaceRow> = sqlx::query_as(
        r#"
        SELECT id, name, default_work_dir, default_model, EXTRACT(EPOCH FROM created_at)::bigint as created_at_unix, EXTRACT(EPOCH FROM updated_at)::bigint as updated_at_unix
        FROM assistant_workspaces
        WHERE tenant_id = $1
        "#
    )
    .bind(claims.organization_id)
    .fetch_all(&db.pool)
    .await
    .map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let workspaces = rows.into_iter().map(|row| Workspace {
        id: row.id,
        name: row.name,
        default_work_dir: row.default_work_dir,
        default_model: row.default_model,
        created_at_unix: row.created_at_unix.unwrap_or(0),
        updated_at_unix: row.updated_at_unix.unwrap_or(0),
    }).collect();

    Ok(Json(workspaces))
}

async fn create_workspace(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<Workspace>,
) -> Result<Json<Workspace>, (StatusCode, String)> {
    let mut ws = payload;
    ws.id = Uuid::new_v4().to_string();
    ws.created_at_unix = Utc::now().timestamp();
    ws.updated_at_unix = Utc::now().timestamp();

    sqlx::query(
        r#"
        INSERT INTO assistant_workspaces (id, tenant_id, name, default_work_dir, default_model)
        VALUES ($1, $2, $3, $4, $5)
        "#
    )
    .bind(&ws.id)
    .bind(&claims.organization_id)
    .bind(&ws.name)
    .bind(&ws.default_work_dir)
    .bind(&ws.default_model)
    .execute(&db.pool)
    .await
    .map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(ws))
}

async fn get_workspace(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<Workspace>, (StatusCode, String)> {
    let row: Option<WorkspaceRow> = sqlx::query_as(
        r#"
        SELECT id, name, default_work_dir, default_model, EXTRACT(EPOCH FROM created_at)::bigint as created_at_unix, EXTRACT(EPOCH FROM updated_at)::bigint as updated_at_unix
        FROM assistant_workspaces
        WHERE id = $1 AND tenant_id = $2
        "#
    )
    .bind(id)
    .bind(claims.organization_id)
    .fetch_optional(&db.pool)
    .await
    .map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match row {
        Some(row) => Ok(Json(Workspace {
            id: row.id,
            name: row.name,
            default_work_dir: row.default_work_dir,
            default_model: row.default_model,
            created_at_unix: row.created_at_unix.unwrap_or(0),
            updated_at_unix: row.updated_at_unix.unwrap_or(0),
        })),
        None => Err((StatusCode::NOT_FOUND, "Workspace not found".to_string())),
    }
}

async fn update_workspace(
    Extension(_db): Extension<Arc<DB>>,
    Extension(_claims): Extension<Claims>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    Ok(Json(payload))
}

async fn list_tasks(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<Task>>, (StatusCode, String)> {
    let rows: Vec<TaskRow> = sqlx::query_as(
        r#"
        SELECT id, workspace_id, title, prompt, status, mode, permission_profile, model_config, current_step, archived, EXTRACT(EPOCH FROM created_at)::bigint as created_at_unix, EXTRACT(EPOCH FROM updated_at)::bigint as updated_at_unix
        FROM assistant_tasks
        WHERE tenant_id = $1
        ORDER BY created_at DESC
        "#
    )
    .bind(claims.organization_id)
    .fetch_all(&db.pool)
    .await
    .map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let tasks = rows.into_iter().map(|row| Task {
        id: row.id,
        workspace_id: row.workspace_id,
        title: row.title,
        prompt: row.prompt,
        status: row.status,
        mode: row.mode,
        permission_profile: row.permission_profile,
        model_config_json: row.model_config,
        current_step: row.current_step,
        archived: row.archived.unwrap_or(false),
        created_at_unix: row.created_at_unix.unwrap_or(0),
        updated_at_unix: row.updated_at_unix.unwrap_or(0),
    }).collect();

    Ok(Json(tasks))
}

async fn create_task(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<Task>,
) -> Result<Json<Task>, (StatusCode, String)> {
    let mut task = payload;
    task.id = Uuid::new_v4().to_string();
    task.created_at_unix = Utc::now().timestamp();
    task.updated_at_unix = Utc::now().timestamp();

    sqlx::query(
        r#"
        INSERT INTO assistant_tasks (id, tenant_id, workspace_id, title, prompt, status, mode, permission_profile, model_config, current_step, archived)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        "#
    )
    .bind(&task.id)
    .bind(&claims.organization_id)
    .bind(&task.workspace_id)
    .bind(&task.title)
    .bind(&task.prompt)
    .bind(&task.status)
    .bind(&task.mode)
    .bind(&task.permission_profile)
    .bind(&task.model_config_json)
    .bind(&task.current_step)
    .bind(&task.archived)
    .execute(&db.pool)
    .await
    .map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(task))
}

async fn get_task(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<Task>, (StatusCode, String)> {
    let row: Option<TaskRow> = sqlx::query_as(
        r#"
        SELECT id, workspace_id, title, prompt, status, mode, permission_profile, model_config, current_step, archived, EXTRACT(EPOCH FROM created_at)::bigint as created_at_unix, EXTRACT(EPOCH FROM updated_at)::bigint as updated_at_unix
        FROM assistant_tasks
        WHERE id = $1 AND tenant_id = $2
        "#
    )
    .bind(id)
    .bind(claims.organization_id)
    .fetch_optional(&db.pool)
    .await
    .map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match row {
        Some(row) => Ok(Json(Task {
            id: row.id,
            workspace_id: row.workspace_id,
            title: row.title,
            prompt: row.prompt,
            status: row.status,
            mode: row.mode,
            permission_profile: row.permission_profile,
            model_config_json: row.model_config,
            current_step: row.current_step,
            archived: row.archived.unwrap_or(false),
            created_at_unix: row.created_at_unix.unwrap_or(0),
            updated_at_unix: row.updated_at_unix.unwrap_or(0),
        })),
        None => Err((StatusCode::NOT_FOUND, "Task not found".to_string())),
    }
}

async fn update_task(
    Extension(_db): Extension<Arc<DB>>,
    Extension(_claims): Extension<Claims>,
    Path(_id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    Ok(Json(payload))
}

async fn list_messages(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<Vec<Message>>, (StatusCode, String)> {
    let rows: Vec<MessageRow> = sqlx::query_as(
        r#"
        SELECT id, task_id, role, content, tool_metadata, EXTRACT(EPOCH FROM created_at)::bigint as created_at_unix
        FROM assistant_messages
        WHERE task_id = $1 AND tenant_id = $2
        ORDER BY created_at ASC
        "#
    )
    .bind(id)
    .bind(claims.organization_id)
    .fetch_all(&db.pool)
    .await
    .map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let messages = rows.into_iter().map(|row| Message {
        id: row.id,
        task_id: row.task_id,
        role: row.role,
        content: row.content,
        tool_metadata_json: row.tool_metadata,
        created_at_unix: row.created_at_unix.unwrap_or(0),
    }).collect();

    Ok(Json(messages))
}

async fn create_message(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(payload): Json<Message>,
) -> Result<Json<Message>, (StatusCode, String)> {
    let mut msg = payload;
    msg.task_id = id;
    msg.id = Uuid::new_v4().to_string();
    msg.created_at_unix = Utc::now().timestamp();

    sqlx::query(
        r#"
        INSERT INTO assistant_messages (id, tenant_id, task_id, role, content, tool_metadata)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#
    )
    .bind(&msg.id)
    .bind(&claims.organization_id)
    .bind(&msg.task_id)
    .bind(&msg.role)
    .bind(&msg.content)
    .bind(&msg.tool_metadata_json)
    .execute(&db.pool)
    .await
    .map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(msg))
}

async fn list_artifacts(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<Vec<Artifact>>, (StatusCode, String)> {
    let rows: Vec<ArtifactRow> = sqlx::query_as(
        r#"
        SELECT id, task_id, type as type_, filename, path, mime_type, size, preview_ref, EXTRACT(EPOCH FROM created_at)::bigint as created_at_unix
        FROM assistant_artifacts
        WHERE task_id = $1 AND tenant_id = $2
        ORDER BY created_at ASC
        "#
    )
    .bind(id)
    .bind(claims.organization_id)
    .fetch_all(&db.pool)
    .await
    .map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let artifacts = rows.into_iter().map(|row| Artifact {
        id: row.id,
        task_id: row.task_id,
        type_: row.type_,
        filename: row.filename,
        path: row.path,
        mime_type: row.mime_type,
        size: row.size,
        preview_ref: row.preview_ref,
        created_at_unix: row.created_at_unix.unwrap_or(0),
    }).collect();

    Ok(Json(artifacts))
}

async fn create_artifact(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(payload): Json<Artifact>,
) -> Result<Json<Artifact>, (StatusCode, String)> {
    let mut artifact = payload;
    artifact.task_id = id;
    artifact.id = Uuid::new_v4().to_string();
    artifact.created_at_unix = Utc::now().timestamp();

    sqlx::query(
        r#"
        INSERT INTO assistant_artifacts (id, tenant_id, task_id, type, filename, path, mime_type, size, preview_ref)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#
    )
    .bind(&artifact.id)
    .bind(&claims.organization_id)
    .bind(&artifact.task_id)
    .bind(&artifact.type_)
    .bind(&artifact.filename)
    .bind(&artifact.path)
    .bind(&artifact.mime_type)
    .bind(&artifact.size)
    .bind(&artifact.preview_ref)
    .execute(&db.pool)
    .await
    .map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(artifact))
}

async fn list_file_changes(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<Vec<FileChange>>, (StatusCode, String)> {
    let rows: Vec<FileChangeRow> = sqlx::query_as(
        r#"
        SELECT id, task_id, path, change_type, summary, approval_status, EXTRACT(EPOCH FROM created_at)::bigint as created_at_unix
        FROM assistant_file_changes
        WHERE task_id = $1 AND tenant_id = $2
        ORDER BY created_at ASC
        "#
    )
    .bind(id)
    .bind(claims.organization_id)
    .fetch_all(&db.pool)
    .await
    .map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let changes = rows.into_iter().map(|row| FileChange {
        id: row.id,
        task_id: row.task_id,
        path: row.path,
        change_type: row.change_type,
        summary: row.summary,
        approval_status: row.approval_status,
        created_at_unix: row.created_at_unix.unwrap_or(0),
    }).collect();

    Ok(Json(changes))
}

async fn create_file_change(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(payload): Json<FileChange>,
) -> Result<Json<FileChange>, (StatusCode, String)> {
    let mut change = payload;
    change.task_id = id;
    change.id = Uuid::new_v4().to_string();
    change.created_at_unix = Utc::now().timestamp();

    sqlx::query(
        r#"
        INSERT INTO assistant_file_changes (id, tenant_id, task_id, path, change_type, summary, approval_status)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#
    )
    .bind(&change.id)
    .bind(&claims.organization_id)
    .bind(&change.task_id)
    .bind(&change.path)
    .bind(&change.change_type)
    .bind(&change.summary)
    .bind(&change.approval_status)
    .execute(&db.pool)
    .await
    .map_err(|e: sqlx::Error| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(change))
}

async fn get_memory(Extension(_claims): Extension<Claims>) -> Result<Json<serde_json::Value>, (StatusCode, String)> { Ok(Json(serde_json::json!({}))) }
async fn post_memory(Extension(_claims): Extension<Claims>, Json(p): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, (StatusCode, String)> { Ok(Json(p)) }
async fn get_settings(Extension(_claims): Extension<Claims>) -> Result<Json<serde_json::Value>, (StatusCode, String)> { Ok(Json(serde_json::json!({}))) }
async fn patch_settings(Extension(_claims): Extension<Claims>, Json(p): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, (StatusCode, String)> { Ok(Json(p)) }
async fn get_billing(Extension(_claims): Extension<Claims>) -> Result<Json<serde_json::Value>, (StatusCode, String)> { Ok(Json(serde_json::json!({}))) }
async fn post_share(Extension(_claims): Extension<Claims>, Json(p): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, (StatusCode, String)> { Ok(Json(p)) }
async fn patch_share(Extension(_claims): Extension<Claims>, Json(p): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, (StatusCode, String)> { Ok(Json(p)) }
async fn patch_previews(Extension(_claims): Extension<Claims>, Json(p): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, (StatusCode, String)> { Ok(Json(p)) }
async fn post_artifacts(Extension(_claims): Extension<Claims>, Json(p): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, (StatusCode, String)> { Ok(Json(p)) }
async fn post_uploads(Extension(_claims): Extension<Claims>, Json(p): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, (StatusCode, String)> { Ok(Json(p)) }
async fn patch_experts(Extension(_claims): Extension<Claims>, Json(p): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, (StatusCode, String)> { Ok(Json(p)) }
async fn post_commands(Extension(_claims): Extension<Claims>, Json(p): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, (StatusCode, String)> { Ok(Json(p)) }
async fn patch_mcp(Extension(_claims): Extension<Claims>, Json(p): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, (StatusCode, String)> { Ok(Json(p)) }
async fn patch_models(Extension(_claims): Extension<Claims>, Json(p): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, (StatusCode, String)> { Ok(Json(p)) }
async fn post_explore(Extension(_claims): Extension<Claims>, Json(p): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, (StatusCode, String)> { Ok(Json(p)) }
async fn patch_explore(Extension(_claims): Extension<Claims>, Json(p): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, (StatusCode, String)> { Ok(Json(p)) }
async fn post_cloud(Extension(_claims): Extension<Claims>, Json(p): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, (StatusCode, String)> { Ok(Json(p)) }
async fn patch_cloud(Extension(_claims): Extension<Claims>, Json(p): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, (StatusCode, String)> { Ok(Json(p)) }
async fn patch_plugins(Extension(_claims): Extension<Claims>, Json(p): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, (StatusCode, String)> { Ok(Json(p)) }
async fn patch_claw(Extension(_claims): Extension<Claims>, Json(p): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, (StatusCode, String)> { Ok(Json(p)) }
async fn post_approvals(Extension(_claims): Extension<Claims>, Json(p): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, (StatusCode, String)> { Ok(Json(p)) }
async fn post_support(Extension(_claims): Extension<Claims>, Json(p): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, (StatusCode, String)> { Ok(Json(p)) }
