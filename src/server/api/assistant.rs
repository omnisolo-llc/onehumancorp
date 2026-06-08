use axum::{
    extract::{Extension, Path, State},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post, put, delete},
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
        .layer(Extension(db))
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

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub task_id: String,
    pub role: String,
    pub content: String,
    pub tool_metadata_json: Option<serde_json::Value>,
    pub created_at_unix: i64,
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
    // Placeholder implementation
    Ok(Json(vec![]))
}

async fn create_workspace(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<Workspace>,
) -> Result<Json<Workspace>, (StatusCode, String)> {
    // Placeholder implementation
    let mut ws = payload;
    ws.id = Uuid::new_v4().to_string();
    ws.created_at_unix = Utc::now().timestamp();
    ws.updated_at_unix = Utc::now().timestamp();
    Ok(Json(ws))
}

async fn get_workspace(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<Workspace>, (StatusCode, String)> {
    // Placeholder implementation
    Ok(Json(Workspace {
        id,
        name: "Test Workspace".to_string(),
        default_work_dir: None,
        default_model: None,
        created_at_unix: 0,
        updated_at_unix: 0,
    }))
}


async fn list_tasks(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<Task>>, (StatusCode, String)> {
    // Placeholder implementation
    Ok(Json(vec![]))
}

async fn create_task(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<Task>,
) -> Result<Json<Task>, (StatusCode, String)> {
    // Placeholder implementation
    let mut task = payload;
    task.id = Uuid::new_v4().to_string();
    task.created_at_unix = Utc::now().timestamp();
    task.updated_at_unix = Utc::now().timestamp();
    Ok(Json(task))
}

async fn get_task(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<Task>, (StatusCode, String)> {
    // Placeholder implementation
    Ok(Json(Task {
        id,
        workspace_id: "workspace-123".to_string(),
        title: "Test Task".to_string(),
        prompt: "Do something".to_string(),
        status: "pending".to_string(),
        mode: None,
        permission_profile: "default".to_string(),
        model_config_json: None,
        current_step: None,
        archived: false,
        created_at_unix: 0,
        updated_at_unix: 0,
    }))
}

async fn list_messages(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<Vec<Message>>, (StatusCode, String)> {
    // Placeholder implementation
    Ok(Json(vec![]))
}

async fn create_message(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(payload): Json<Message>,
) -> Result<Json<Message>, (StatusCode, String)> {
    // Placeholder implementation
    let mut msg = payload;
    msg.task_id = id;
    msg.id = Uuid::new_v4().to_string();
    msg.created_at_unix = Utc::now().timestamp();
    Ok(Json(msg))
}


async fn list_artifacts(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<Vec<Artifact>>, (StatusCode, String)> {
    // Placeholder implementation
    Ok(Json(vec![]))
}

async fn create_artifact(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(payload): Json<Artifact>,
) -> Result<Json<Artifact>, (StatusCode, String)> {
    // Placeholder implementation
    let mut artifact = payload;
    artifact.task_id = id;
    artifact.id = Uuid::new_v4().to_string();
    artifact.created_at_unix = Utc::now().timestamp();
    Ok(Json(artifact))
}


async fn list_file_changes(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<Vec<FileChange>>, (StatusCode, String)> {
    // Placeholder implementation
    Ok(Json(vec![]))
}

async fn create_file_change(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(payload): Json<FileChange>,
) -> Result<Json<FileChange>, (StatusCode, String)> {
    // Placeholder implementation
    let mut change = payload;
    change.task_id = id;
    change.id = Uuid::new_v4().to_string();
    change.created_at_unix = Utc::now().timestamp();
    Ok(Json(change))
}
