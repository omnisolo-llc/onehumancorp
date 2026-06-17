use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Router,
    Json,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::db::{DB, DbStore};
use ::server_common::Claims;
use uuid::Uuid;
use chrono::Utc;
use sqlx::Row;

pub fn router<S>(db: Arc<DB>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/workspaces", get(list_workspaces).post(create_workspace))
        .route("/workspaces/{id}", get(get_workspace))
        .route("/tasks", get(list_tasks).post(create_task))
        .route("/tasks/{id}", get(get_task))
        .route("/tasks/{id}/messages", get(list_messages).post(create_message))
        .route("/tasks/{id}/artifacts", get(list_artifacts).post(create_artifact))
        .route("/tasks/{id}/file_changes", get(list_file_changes).post(create_file_change))
        .layer(Extension(db))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub default_work_dir: Option<String>,
    pub default_model: Option<String>,
    pub created_at_unix: i64,
    pub updated_at_unix: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: String,
    pub task_id: String,
    pub role: String,
    pub content: String,
    pub tool_metadata_json: Option<serde_json::Value>,
    pub created_at_unix: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());

    match &db.store {
        DbStore::Sqlite(pool) => {
            let rows = sqlx::query(
                "SELECT id, name, default_work_dir, default_model, 
                        strftime('%s', created_at) as c_unix, 
                        strftime('%s', updated_at) as u_unix 
                 FROM assistant_workspaces WHERE tenant_id = ?"
            )
            .bind(&tenant_id)
            .fetch_all(pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            let list = rows.into_iter().map(|row| Workspace {
                id: row.get("id"),
                name: row.get("name"),
                default_work_dir: row.get("default_work_dir"),
                default_model: row.get("default_model"),
                created_at_unix: row.get::<Option<String>, _>("c_unix").and_then(|s| s.parse().ok()).unwrap_or(0),
                updated_at_unix: row.get::<Option<String>, _>("u_unix").and_then(|s| s.parse().ok()).unwrap_or(0),
            }).collect();
            Ok(Json(list))
        }
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            
            let rows = sqlx::query(
                "SELECT id, name, default_work_dir, default_model, 
                        EXTRACT(EPOCH FROM created_at)::BIGINT as c_unix, 
                        EXTRACT(EPOCH FROM updated_at)::BIGINT as u_unix 
                 FROM assistant_workspaces"
            )
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            let list = rows.into_iter().map(|row| Workspace {
                id: row.get("id"),
                name: row.get("name"),
                default_work_dir: row.get("default_work_dir"),
                default_model: row.get("default_model"),
                created_at_unix: row.get("c_unix"),
                updated_at_unix: row.get("u_unix"),
            }).collect();
            Ok(Json(list))
        }
    }
}

async fn create_workspace(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<Workspace>,
) -> Result<Json<Workspace>, (StatusCode, String)> {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());
    let mut ws = payload;
    ws.id = if ws.id.is_empty() { Uuid::new_v4().to_string() } else { ws.id };
    ws.created_at_unix = Utc::now().timestamp();
    ws.updated_at_unix = Utc::now().timestamp();

    match &db.store {
        DbStore::Sqlite(pool) => {
            sqlx::query(
                "INSERT INTO assistant_workspaces (id, tenant_id, name, default_work_dir, default_model) VALUES (?, ?, ?, ?, ?)"
            )
            .bind(&ws.id)
            .bind(&tenant_id)
            .bind(&ws.name)
            .bind(&ws.default_work_dir)
            .bind(&ws.default_model)
            .execute(pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        }
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            
            sqlx::query(
                "INSERT INTO assistant_workspaces (id, tenant_id, name, default_work_dir, default_model) VALUES ($1, $2, $3, $4, $5)"
            )
            .bind(&ws.id)
            .bind(&tenant_id)
            .bind(&ws.name)
            .bind(&ws.default_work_dir)
            .bind(&ws.default_model)
            .execute(&mut *tx)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        }
    }

    Ok(Json(ws))
}

async fn get_workspace(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<Workspace>, (StatusCode, String)> {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());

    match &db.store {
        DbStore::Sqlite(pool) => {
            let row = sqlx::query(
                "SELECT id, name, default_work_dir, default_model, 
                        strftime('%s', created_at) as c_unix, 
                        strftime('%s', updated_at) as u_unix 
                 FROM assistant_workspaces WHERE tenant_id = ? AND id = ?"
            )
            .bind(&tenant_id)
            .bind(&id)
            .fetch_optional(pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            if let Some(r) = row {
                Ok(Json(Workspace {
                    id: r.get("id"),
                    name: r.get("name"),
                    default_work_dir: r.get("default_work_dir"),
                    default_model: r.get("default_model"),
                    created_at_unix: r.get::<Option<String>, _>("c_unix").and_then(|s| s.parse().ok()).unwrap_or(0),
                    updated_at_unix: r.get::<Option<String>, _>("u_unix").and_then(|s| s.parse().ok()).unwrap_or(0),
                }))
            } else {
                Err((StatusCode::NOT_FOUND, "Workspace not found".to_string()))
            }
        }
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            
            let row = sqlx::query(
                "SELECT id, name, default_work_dir, default_model, 
                        EXTRACT(EPOCH FROM created_at)::BIGINT as c_unix, 
                        EXTRACT(EPOCH FROM updated_at)::BIGINT as u_unix 
                 FROM assistant_workspaces WHERE id = $1"
            )
            .bind(&id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            if let Some(r) = row {
                Ok(Json(Workspace {
                    id: r.get("id"),
                    name: r.get("name"),
                    default_work_dir: r.get("default_work_dir"),
                    default_model: r.get("default_model"),
                    created_at_unix: r.get("c_unix"),
                    updated_at_unix: r.get("u_unix"),
                }))
            } else {
                Err((StatusCode::NOT_FOUND, "Workspace not found".to_string()))
            }
        }
    }
}

async fn list_tasks(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<Task>>, (StatusCode, String)> {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());

    match &db.store {
        DbStore::Sqlite(pool) => {
            let rows = sqlx::query(
                "SELECT id, workspace_id, title, prompt, status, mode, permission_profile, model_config, current_step, archived, 
                        strftime('%s', created_at) as c_unix, 
                        strftime('%s', updated_at) as u_unix 
                 FROM assistant_tasks WHERE tenant_id = ?"
            )
            .bind(&tenant_id)
            .fetch_all(pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            let list = rows.into_iter().map(|row| Task {
                id: row.get("id"),
                workspace_id: row.get("workspace_id"),
                title: row.get("title"),
                prompt: row.get("prompt"),
                status: row.get("status"),
                mode: row.get("mode"),
                permission_profile: row.get("permission_profile"),
                model_config_json: row.get::<Option<String>, _>("model_config").and_then(|s| serde_json::from_str(&s).ok()),
                current_step: row.get("current_step"),
                archived: row.get::<Option<i32>, _>("archived").map(|v| v != 0).unwrap_or(false),
                created_at_unix: row.get::<Option<String>, _>("c_unix").and_then(|s| s.parse().ok()).unwrap_or(0),
                updated_at_unix: row.get::<Option<String>, _>("u_unix").and_then(|s| s.parse().ok()).unwrap_or(0),
            }).collect();
            Ok(Json(list))
        }
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            
            let rows = sqlx::query(
                "SELECT id, workspace_id, title, prompt, status, mode, permission_profile, model_config, current_step, archived, 
                        EXTRACT(EPOCH FROM created_at)::BIGINT as c_unix, 
                        EXTRACT(EPOCH FROM updated_at)::BIGINT as u_unix 
                 FROM assistant_tasks"
            )
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            let list = rows.into_iter().map(|row| Task {
                id: row.get("id"),
                workspace_id: row.get("workspace_id"),
                title: row.get("title"),
                prompt: row.get("prompt"),
                status: row.get("status"),
                mode: row.get("mode"),
                permission_profile: row.get("permission_profile"),
                model_config_json: row.get("model_config"),
                current_step: row.get("current_step"),
                archived: row.get("archived"),
                created_at_unix: row.get("c_unix"),
                updated_at_unix: row.get("u_unix"),
            }).collect();
            Ok(Json(list))
        }
    }
}

async fn create_task(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<Task>,
) -> Result<Json<Task>, (StatusCode, String)> {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());
    let mut task = payload;
    task.id = if task.id.is_empty() { Uuid::new_v4().to_string() } else { task.id };
    task.created_at_unix = Utc::now().timestamp();
    task.updated_at_unix = Utc::now().timestamp();

    // Verify workspace exists or create a default one
    match &db.store {
        DbStore::Sqlite(pool) => {
            let ws_exists: (i64,) = sqlx::query_as("SELECT count(*) FROM assistant_workspaces WHERE id = ?")
                .bind(&task.workspace_id)
                .fetch_one(pool)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            
            if ws_exists.0 == 0 {
                sqlx::query("INSERT INTO assistant_workspaces (id, tenant_id, name) VALUES (?, ?, ?)")
                    .bind(&task.workspace_id)
                    .bind(&tenant_id)
                    .bind("Default Workspace")
                    .execute(pool)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            }

            sqlx::query(
                "INSERT INTO assistant_tasks (id, tenant_id, workspace_id, title, prompt, status, mode, permission_profile, model_config, current_step, archived) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(&task.id)
            .bind(&tenant_id)
            .bind(&task.workspace_id)
            .bind(&task.title)
            .bind(&task.prompt)
            .bind(&task.status)
            .bind(&task.mode)
            .bind(&task.permission_profile)
            .bind(task.model_config_json.as_ref().map(|v| serde_json::to_string(v).unwrap_or_default()))
            .bind(&task.current_step)
            .bind(task.archived as i32)
            .execute(pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        }
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            
            let ws_exists: (i64,) = sqlx::query_as("SELECT count(*) FROM assistant_workspaces WHERE id = $1")
                .bind(&task.workspace_id)
                .fetch_one(&mut *tx)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            
            if ws_exists.0 == 0 {
                sqlx::query("INSERT INTO assistant_workspaces (id, tenant_id, name) VALUES ($1, $2, $3)")
                    .bind(&task.workspace_id)
                    .bind(&tenant_id)
                    .bind("Default Workspace")
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            }

            sqlx::query(
                "INSERT INTO assistant_tasks (id, tenant_id, workspace_id, title, prompt, status, mode, permission_profile, model_config, current_step, archived) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)"
            )
            .bind(&task.id)
            .bind(&tenant_id)
            .bind(&task.workspace_id)
            .bind(&task.title)
            .bind(&task.prompt)
            .bind(&task.status)
            .bind(&task.mode)
            .bind(&task.permission_profile)
            .bind(&task.model_config_json)
            .bind(&task.current_step)
            .bind(task.archived)
            .execute(&mut *tx)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            
            tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        }
    }

    Ok(Json(task))
}

async fn get_task(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
) -> Result<Json<Task>, (StatusCode, String)> {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());

    match &db.store {
        DbStore::Sqlite(pool) => {
            let row = sqlx::query(
                "SELECT id, workspace_id, title, prompt, status, mode, permission_profile, model_config, current_step, archived, 
                        strftime('%s', created_at) as c_unix, 
                        strftime('%s', updated_at) as u_unix 
                 FROM assistant_tasks WHERE tenant_id = ? AND id = ?"
            )
            .bind(&tenant_id)
            .bind(&id)
            .fetch_optional(pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            if let Some(r) = row {
                Ok(Json(Task {
                    id: r.get("id"),
                    workspace_id: r.get("workspace_id"),
                    title: r.get("title"),
                    prompt: r.get("prompt"),
                    status: r.get("status"),
                    mode: r.get("mode"),
                    permission_profile: r.get("permission_profile"),
                    model_config_json: r.get::<Option<String>, _>("model_config").and_then(|s| serde_json::from_str(&s).ok()),
                    current_step: r.get("current_step"),
                    archived: r.get::<Option<i32>, _>("archived").map(|v| v != 0).unwrap_or(false),
                    created_at_unix: r.get::<Option<String>, _>("c_unix").and_then(|s| s.parse().ok()).unwrap_or(0),
                    updated_at_unix: r.get::<Option<String>, _>("u_unix").and_then(|s| s.parse().ok()).unwrap_or(0),
                }))
            } else {
                Err((StatusCode::NOT_FOUND, "Task not found".to_string()))
            }
        }
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            
            let row = sqlx::query(
                "SELECT id, workspace_id, title, prompt, status, mode, permission_profile, model_config, current_step, archived, 
                        EXTRACT(EPOCH FROM created_at)::BIGINT as c_unix, 
                        EXTRACT(EPOCH FROM updated_at)::BIGINT as u_unix 
                 FROM assistant_tasks WHERE id = $1"
            )
            .bind(&id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            if let Some(r) = row {
                Ok(Json(Task {
                    id: r.get("id"),
                    workspace_id: r.get("workspace_id"),
                    title: r.get("title"),
                    prompt: r.get("prompt"),
                    status: r.get("status"),
                    mode: r.get("mode"),
                    permission_profile: r.get("permission_profile"),
                    model_config_json: r.get("model_config"),
                    current_step: r.get("current_step"),
                    archived: r.get("archived"),
                    created_at_unix: r.get("c_unix"),
                    updated_at_unix: r.get("u_unix"),
                }))
            } else {
                Err((StatusCode::NOT_FOUND, "Task not found".to_string()))
            }
        }
    }
}

async fn list_messages(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(task_id): Path<String>,
) -> Result<Json<Vec<Message>>, (StatusCode, String)> {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());

    match &db.store {
        DbStore::Sqlite(pool) => {
            let rows = sqlx::query(
                "SELECT id, task_id, role, content, tool_metadata, 
                        strftime('%s', created_at) as c_unix 
                 FROM assistant_messages WHERE tenant_id = ? AND task_id = ? ORDER BY created_at ASC"
            )
            .bind(&tenant_id)
            .bind(&task_id)
            .fetch_all(pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            let list = rows.into_iter().map(|row| Message {
                id: row.get("id"),
                task_id: row.get("task_id"),
                role: row.get("role"),
                content: row.get("content"),
                tool_metadata_json: row.get::<Option<String>, _>("tool_metadata").and_then(|s| serde_json::from_str(&s).ok()),
                created_at_unix: row.get::<Option<String>, _>("c_unix").and_then(|s| s.parse().ok()).unwrap_or(0),
            }).collect();
            Ok(Json(list))
        }
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            
            let rows = sqlx::query(
                "SELECT id, task_id, role, content, tool_metadata, 
                        EXTRACT(EPOCH FROM created_at)::BIGINT as c_unix 
                 FROM assistant_messages WHERE task_id = $1 ORDER BY created_at ASC"
            )
            .bind(&task_id)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            let list = rows.into_iter().map(|row| Message {
                id: row.get("id"),
                task_id: row.get("task_id"),
                role: row.get("role"),
                content: row.get("content"),
                tool_metadata_json: row.get("tool_metadata"),
                created_at_unix: row.get("c_unix"),
            }).collect();
            Ok(Json(list))
        }
    }
}

async fn create_message(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(task_id): Path<String>,
    Json(payload): Json<Message>,
) -> Result<Json<Message>, (StatusCode, String)> {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());
    let mut msg = payload;
    msg.task_id = task_id;
    msg.id = if msg.id.is_empty() { Uuid::new_v4().to_string() } else { msg.id };
    msg.created_at_unix = Utc::now().timestamp();

    match &db.store {
        DbStore::Sqlite(pool) => {
            sqlx::query(
                "INSERT INTO assistant_messages (id, tenant_id, task_id, role, content, tool_metadata) VALUES (?, ?, ?, ?, ?, ?)"
            )
            .bind(&msg.id)
            .bind(&tenant_id)
            .bind(&msg.task_id)
            .bind(&msg.role)
            .bind(&msg.content)
            .bind(msg.tool_metadata_json.as_ref().map(|v| serde_json::to_string(v).unwrap_or_default()))
            .execute(pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        }
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            
            sqlx::query(
                "INSERT INTO assistant_messages (id, tenant_id, task_id, role, content, tool_metadata) VALUES ($1, $2, $3, $4, $5, $6)"
            )
            .bind(&msg.id)
            .bind(&tenant_id)
            .bind(&msg.task_id)
            .bind(&msg.role)
            .bind(&msg.content)
            .bind(&msg.tool_metadata_json)
            .execute(&mut *tx)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        }
    }

    Ok(Json(msg))
}

async fn list_artifacts(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(task_id): Path<String>,
) -> Result<Json<Vec<Artifact>>, (StatusCode, String)> {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());

    match &db.store {
        DbStore::Sqlite(pool) => {
            let rows = sqlx::query(
                "SELECT id, task_id, type, filename, path, mime_type, size, preview_ref, 
                        strftime('%s', created_at) as c_unix 
                 FROM assistant_artifacts WHERE tenant_id = ? AND task_id = ?"
            )
            .bind(&tenant_id)
            .bind(&task_id)
            .fetch_all(pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            let list = rows.into_iter().map(|row| Artifact {
                id: row.get("id"),
                task_id: row.get("task_id"),
                type_: row.get("type"),
                filename: row.get("filename"),
                path: row.get("path"),
                mime_type: row.get("mime_type"),
                size: row.get("size"),
                preview_ref: row.get("preview_ref"),
                created_at_unix: row.get::<Option<String>, _>("c_unix").and_then(|s| s.parse().ok()).unwrap_or(0),
            }).collect();
            Ok(Json(list))
        }
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            
            let rows = sqlx::query(
                "SELECT id, task_id, type, filename, path, mime_type, size, preview_ref, 
                        EXTRACT(EPOCH FROM created_at)::BIGINT as c_unix 
                 FROM assistant_artifacts WHERE task_id = $1"
            )
            .bind(&task_id)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            let list = rows.into_iter().map(|row| Artifact {
                id: row.get("id"),
                task_id: row.get("task_id"),
                type_: row.get("type"),
                filename: row.get("filename"),
                path: row.get("path"),
                mime_type: row.get("mime_type"),
                size: row.get("size"),
                preview_ref: row.get("preview_ref"),
                created_at_unix: row.get("c_unix"),
            }).collect();
            Ok(Json(list))
        }
    }
}

async fn create_artifact(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(task_id): Path<String>,
    Json(payload): Json<Artifact>,
) -> Result<Json<Artifact>, (StatusCode, String)> {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());
    let mut artifact = payload;
    artifact.task_id = task_id;
    artifact.id = if artifact.id.is_empty() { Uuid::new_v4().to_string() } else { artifact.id };
    artifact.created_at_unix = Utc::now().timestamp();

    match &db.store {
        DbStore::Sqlite(pool) => {
            sqlx::query(
                "INSERT INTO assistant_artifacts (id, tenant_id, task_id, type, filename, path, mime_type, size, preview_ref) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(&artifact.id)
            .bind(&tenant_id)
            .bind(&artifact.task_id)
            .bind(&artifact.type_)
            .bind(&artifact.filename)
            .bind(&artifact.path)
            .bind(&artifact.mime_type)
            .bind(artifact.size)
            .bind(&artifact.preview_ref)
            .execute(pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        }
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            
            sqlx::query(
                "INSERT INTO assistant_artifacts (id, tenant_id, task_id, type, filename, path, mime_type, size, preview_ref) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
            )
            .bind(&artifact.id)
            .bind(&tenant_id)
            .bind(&artifact.task_id)
            .bind(&artifact.type_)
            .bind(&artifact.filename)
            .bind(&artifact.path)
            .bind(&artifact.mime_type)
            .bind(artifact.size)
            .bind(&artifact.preview_ref)
            .execute(&mut *tx)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        }
    }

    Ok(Json(artifact))
}

async fn list_file_changes(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(task_id): Path<String>,
) -> Result<Json<Vec<FileChange>>, (StatusCode, String)> {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());

    match &db.store {
        DbStore::Sqlite(pool) => {
            let rows = sqlx::query(
                "SELECT id, task_id, path, change_type, summary, approval_status, 
                        strftime('%s', created_at) as c_unix 
                 FROM assistant_file_changes WHERE tenant_id = ? AND task_id = ?"
            )
            .bind(&tenant_id)
            .bind(&task_id)
            .fetch_all(pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            let list = rows.into_iter().map(|row| FileChange {
                id: row.get("id"),
                task_id: row.get("task_id"),
                path: row.get("path"),
                change_type: row.get("change_type"),
                summary: row.get("summary"),
                approval_status: row.get("approval_status"),
                created_at_unix: row.get::<Option<String>, _>("c_unix").and_then(|s| s.parse().ok()).unwrap_or(0),
            }).collect();
            Ok(Json(list))
        }
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            
            let rows = sqlx::query(
                "SELECT id, task_id, path, change_type, summary, approval_status, 
                        EXTRACT(EPOCH FROM created_at)::BIGINT as c_unix 
                 FROM assistant_file_changes WHERE task_id = $1"
            )
            .bind(&task_id)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            let list = rows.into_iter().map(|row| FileChange {
                id: row.get("id"),
                task_id: row.get("task_id"),
                path: row.get("path"),
                change_type: row.get("change_type"),
                summary: row.get("summary"),
                approval_status: row.get("approval_status"),
                created_at_unix: row.get("c_unix"),
            }).collect();
            Ok(Json(list))
        }
    }
}

async fn create_file_change(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(task_id): Path<String>,
    Json(payload): Json<FileChange>,
) -> Result<Json<FileChange>, (StatusCode, String)> {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());
    let mut change = payload;
    change.task_id = task_id;
    change.id = if change.id.is_empty() { Uuid::new_v4().to_string() } else { change.id };
    change.created_at_unix = Utc::now().timestamp();

    match &db.store {
        DbStore::Sqlite(pool) => {
            sqlx::query(
                "INSERT INTO assistant_file_changes (id, tenant_id, task_id, path, change_type, summary, approval_status) VALUES (?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(&change.id)
            .bind(&tenant_id)
            .bind(&change.task_id)
            .bind(&change.path)
            .bind(&change.change_type)
            .bind(&change.summary)
            .bind(&change.approval_status)
            .execute(pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        }
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            
            sqlx::query(
                "INSERT INTO assistant_file_changes (id, tenant_id, task_id, path, change_type, summary, approval_status) VALUES ($1, $2, $3, $4, $5, $6, $7)"
            )
            .bind(&change.id)
            .bind(&tenant_id)
            .bind(&change.task_id)
            .bind(&change.path)
            .bind(&change.change_type)
            .bind(&change.summary)
            .bind(&change.approval_status)
            .execute(&mut *tx)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        }
    }

    Ok(Json(change))
}
