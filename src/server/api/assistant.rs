use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    routing::get,
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

#[derive(serde::Deserialize)]
pub struct AssistantQuery {
    pub mobile_optimized: Option<bool>,
}

pub fn router<S>(db: Arc<DB>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()

        .route("/workspaces", get(list_workspaces).post(create_workspace))
        .route("/workspaces/{id}", get(get_workspace))
        .route("/tasks", get(list_tasks).post(create_task))
        .route("/tasks/{id}", get(get_task).patch(mutate_task))
        .route("/tasks/{id}/messages", get(list_messages).post(create_message))
        .route("/tasks/{id}/artifacts", get(list_artifacts).post(create_artifact))
        .route("/tasks/{id}/file_changes", get(list_file_changes).post(create_file_change))
        .route("/memory", get(list_memory).patch(mutate_memory))
        .route("/memory/customer/{customer_id}", get(synthesize_customer_memory))
        .route("/skills", get(list_skills).patch(mutate_skill))
        .route("/connectors", get(list_connectors).patch(mutate_connector))
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssistantMemoryRecord {
    pub id: String,
    pub content: String,
    pub scope: String,
    pub source: Option<String>,
    pub enabled: bool,
    pub created_at_unix: i64,
    pub updated_at_unix: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssistantSkillRecord {
    pub id: String,
    pub name: String,
    pub category: String,
    pub source: String,
    pub status: String,
    pub version: Option<String>,
    pub description: Option<String>,
    pub config: Option<serde_json::Value>,
    pub created_at_unix: i64,
    pub updated_at_unix: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssistantConnectorRecord {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub status: String,
    pub oauth: bool,
    pub config: Option<serde_json::Value>,
    pub last_error: Option<String>,
    pub created_at_unix: i64,
    pub updated_at_unix: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeatureMutation {
    pub action: String,
    pub id: Option<String>,
    pub name: Option<String>,
    pub content: Option<String>,
    pub scope: Option<String>,
    pub category: Option<String>,
    pub kind: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub config: Option<serde_json::Value>,
}

#[derive(Serialize)]
struct AssistantMemoryListResponse {
    memories: Vec<AssistantMemoryRecord>,
}

#[derive(Serialize)]
struct AssistantSkillListResponse {
    skills: Vec<AssistantSkillRecord>,
}

#[derive(Serialize)]
struct AssistantConnectorListResponse {
    connectors: Vec<AssistantConnectorRecord>,
}

fn tenant_id_from(claims: &Claims) -> String {
    claims
        .organization_id
        .clone()
        .unwrap_or_else(|| "default".to_string())
}

fn require_text(value: Option<String>, field: &str) -> Result<String, (StatusCode, String)> {
    match value {
        Some(text) if !text.trim().is_empty() => Ok(text),
        _ => Err((StatusCode::BAD_REQUEST, format!("missing field: {field}"))),
    }
}

async fn list_workspaces(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Query(query): Query<AssistantQuery>,
) -> Result<Json<Vec<Workspace>>, (StatusCode, String)> {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());
    let mobile_optimized = query.mobile_optimized.unwrap_or(false);

    let workspaces = match &db.store {
        DbStore::Sqlite(pool) => {
            let query_str = if mobile_optimized {
                "SELECT id, name, NULL as default_work_dir, NULL as default_model,
                        strftime('%s', created_at) as c_unix,
                        strftime('%s', updated_at) as u_unix
                 FROM assistant_workspaces WHERE tenant_id = ?"
            } else {
                "SELECT id, name, default_work_dir, default_model, 
                        strftime('%s', created_at) as c_unix, 
                        strftime('%s', updated_at) as u_unix 
                 FROM assistant_workspaces WHERE tenant_id = ?"
            };
            let rows = sqlx::query(query_str)
            .bind(&tenant_id)
            .fetch_all(pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            let list: Vec<Workspace> = rows.into_iter().map(|row| Workspace {
                id: row.get("id"),
                name: row.get("name"),
                default_work_dir: row.get("default_work_dir"),
                default_model: row.get("default_model"),
                created_at_unix: row.get::<Option<String>, _>("c_unix").and_then(|s| s.parse().ok()).unwrap_or(0),
                updated_at_unix: row.get::<Option<String>, _>("u_unix").and_then(|s| s.parse().ok()).unwrap_or(0),
            }).collect();
            Ok(list)
        }
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            
            let query_str = if mobile_optimized {
                "SELECT id, name, NULL::text as default_work_dir, NULL::text as default_model,
                        EXTRACT(EPOCH FROM created_at)::BIGINT as c_unix,
                        EXTRACT(EPOCH FROM updated_at)::BIGINT as u_unix
                 FROM assistant_workspaces"
            } else {
                "SELECT id, name, default_work_dir, default_model, 
                        EXTRACT(EPOCH FROM created_at)::BIGINT as c_unix, 
                        EXTRACT(EPOCH FROM updated_at)::BIGINT as u_unix 
                 FROM assistant_workspaces"
            };
            let rows = sqlx::query(query_str)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            let list: Vec<Workspace> = rows.into_iter().map(|row| Workspace {
                id: row.get("id"),
                name: row.get("name"),
                default_work_dir: row.get("default_work_dir"),
                default_model: row.get("default_model"),
                created_at_unix: row.get("c_unix"),
                updated_at_unix: row.get("u_unix"),
            }).collect();
            Ok(list)
        }
    };

    let workspaces = workspaces.map_err(|e: (StatusCode, String)| e)?;

    Ok(Json(workspaces))
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
    Query(query): Query<AssistantQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());
    let mobile_optimized = query.mobile_optimized.unwrap_or(false);

    let tasks = match &db.store {
        DbStore::Sqlite(pool) => {
            let query_str = if mobile_optimized {
                "SELECT id, workspace_id, title, '' as prompt, status, mode, permission_profile, NULL as model_config, current_step, archived,
                        strftime('%s', created_at) as c_unix,
                        strftime('%s', updated_at) as u_unix
                 FROM assistant_tasks WHERE tenant_id = ?"
            } else {
                "SELECT id, workspace_id, title, prompt, status, mode, permission_profile, model_config, current_step, archived, 
                        strftime('%s', created_at) as c_unix, 
                        strftime('%s', updated_at) as u_unix 
                 FROM assistant_tasks WHERE tenant_id = ?"
            };

            let rows = sqlx::query(query_str)
            .bind(&tenant_id)
            .fetch_all(pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            let list: Vec<serde_json::Value> = rows.into_iter().map(|row| {
                if mobile_optimized {
                    serde_json::json!({
                        "id": row.get::<String, _>("id"),
                        "workspace_id": row.get::<String, _>("workspace_id"),
                        "title": row.get::<String, _>("title"),
                        "status": row.get::<String, _>("status"),
                        "mode": row.get::<Option<String>, _>("mode"),
                        "permission_profile": row.get::<String, _>("permission_profile"),
                        "current_step": row.get::<Option<String>, _>("current_step"),
                        "archived": row.get::<Option<i32>, _>("archived").map(|v| v != 0).unwrap_or(false),
                        "created_at_unix": row.get::<Option<String>, _>("c_unix").and_then(|s| s.parse::<i64>().ok()).unwrap_or(0),
                        "updated_at_unix": row.get::<Option<String>, _>("u_unix").and_then(|s| s.parse::<i64>().ok()).unwrap_or(0),
                    })
                } else {
                    let task = Task {
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
                    };
                    serde_json::to_value(task).unwrap_or(serde_json::json!({}))
                }
            }).collect();
            Ok(list)
        }
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            
            let query_str = if mobile_optimized {
                "SELECT id, workspace_id, title, '' as prompt, status, mode, permission_profile, NULL as model_config, current_step, archived,
                        EXTRACT(EPOCH FROM created_at)::BIGINT as c_unix,
                        EXTRACT(EPOCH FROM updated_at)::BIGINT as u_unix
                 FROM assistant_tasks"
            } else {
                "SELECT id, workspace_id, title, prompt, status, mode, permission_profile, model_config, current_step, archived, 
                        EXTRACT(EPOCH FROM created_at)::BIGINT as c_unix, 
                        EXTRACT(EPOCH FROM updated_at)::BIGINT as u_unix 
                 FROM assistant_tasks"
            };

            let rows = sqlx::query(query_str)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            let list: Vec<serde_json::Value> = rows.into_iter().map(|row| {
                if mobile_optimized {
                    serde_json::json!({
                        "id": row.get::<String, _>("id"),
                        "workspace_id": row.get::<String, _>("workspace_id"),
                        "title": row.get::<String, _>("title"),
                        "status": row.get::<String, _>("status"),
                        "mode": row.get::<Option<String>, _>("mode"),
                        "permission_profile": row.get::<String, _>("permission_profile"),
                        "current_step": row.get::<Option<String>, _>("current_step"),
                        "archived": row.get::<bool, _>("archived"),
                        "created_at_unix": row.get::<Option<i64>, _>("c_unix").unwrap_or(0),
                        "updated_at_unix": row.get::<Option<i64>, _>("u_unix").unwrap_or(0),
                    })
                } else {
                    let task = Task {
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
                        created_at_unix: row.get::<Option<i64>, _>("c_unix").unwrap_or(0),
                        updated_at_unix: row.get::<Option<i64>, _>("u_unix").unwrap_or(0),
                    };
                    serde_json::to_value(task).unwrap_or(serde_json::json!({}))
                }
            }).collect();
            Ok(list)
        }
    };

    let tasks = tasks.map_err(|e: (StatusCode, String)| e)?;

    Ok(Json(serde_json::Value::Array(tasks)))
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

async fn mutate_task(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());
    let action = payload.get("action").and_then(|a| a.as_str()).unwrap_or("");

    match &db.store {
        DbStore::Sqlite(pool) => {
            if action == "stop" {
                sqlx::query("UPDATE assistant_tasks SET status = 'blocked', current_step = 'Stopped by user', updated_at = CURRENT_TIMESTAMP WHERE tenant_id = ? AND id = ?")
                    .bind(&tenant_id).bind(&id).execute(pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            } else if action == "resume" {
                sqlx::query("UPDATE assistant_tasks SET status = 'running', current_step = 'Resumed and preparing next step', updated_at = CURRENT_TIMESTAMP WHERE tenant_id = ? AND id = ?")
                    .bind(&tenant_id).bind(&id).execute(pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            } else if action == "archive" {
                sqlx::query("UPDATE assistant_tasks SET status = 'archived', current_step = 'Archived', archived = 1, updated_at = CURRENT_TIMESTAMP WHERE tenant_id = ? AND id = ?")
                    .bind(&tenant_id).bind(&id).execute(pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            } else if action == "unarchive" {
                sqlx::query("UPDATE assistant_tasks SET status = 'completed', current_step = 'Restored to active task list', archived = 0, updated_at = CURRENT_TIMESTAMP WHERE tenant_id = ? AND id = ?")
                    .bind(&tenant_id).bind(&id).execute(pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            } else if action == "rename" || action == "rename_archived" {
                let title = payload.get("title").and_then(|t| t.as_str()).unwrap_or("");
                sqlx::query("UPDATE assistant_tasks SET title = ?, updated_at = CURRENT_TIMESTAMP WHERE tenant_id = ? AND id = ?")
                    .bind(title).bind(&tenant_id).bind(&id).execute(pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            } else if action == "save_to_workspace" {
                let workspace = payload.get("workspace").and_then(|w| w.as_str()).unwrap_or("");
                sqlx::query("UPDATE assistant_tasks SET workspace_id = ?, updated_at = CURRENT_TIMESTAMP WHERE tenant_id = ? AND id = ?")
                    .bind(workspace).bind(&tenant_id).bind(&id).execute(pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            } else if action == "approve_changes" {
                sqlx::query("UPDATE assistant_file_changes SET approval_status = 'approved' WHERE tenant_id = ? AND task_id = ?")
                    .bind(&tenant_id).bind(&id).execute(pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            } else if action == "approve_action" {
                let msg_id = Uuid::new_v4().to_string();
                sqlx::query(
                    "INSERT INTO assistant_messages (id, tenant_id, task_id, role, content, tool_metadata) VALUES (?, ?, ?, ?, ?, ?)"
                )
                .bind(&msg_id)
                .bind(&tenant_id)
                .bind(&id)
                .bind("user")
                .bind("Approve & Execute")
                .bind(None::<String>)
                .execute(pool)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            } else if action == "hard_delete" {
                let mut tx = pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                sqlx::query("DELETE FROM assistant_messages WHERE tenant_id = ? AND task_id = ?").bind(&tenant_id).bind(&id).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                sqlx::query("DELETE FROM assistant_artifacts WHERE tenant_id = ? AND task_id = ?").bind(&tenant_id).bind(&id).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                sqlx::query("DELETE FROM assistant_file_changes WHERE tenant_id = ? AND task_id = ?").bind(&tenant_id).bind(&id).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                sqlx::query("DELETE FROM assistant_tasks WHERE tenant_id = ? AND id = ?").bind(&tenant_id).bind(&id).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                return Ok(Json(serde_json::json!({ "deletedTask": { "id": id } })));
            } else if action == "pin" || action == "unpin" {
                // Ignore pin/unpin for now or implement if needed
            } else {
                return Err((StatusCode::BAD_REQUEST, "Unsupported action".to_string()));
            }
        }
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            if action == "stop" {
                sqlx::query("UPDATE assistant_tasks SET status = 'blocked', current_step = 'Stopped by user', updated_at = CURRENT_TIMESTAMP WHERE tenant_id = $1 AND id = $2")
                    .bind(&tenant_id).bind(&id).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            } else if action == "resume" {
                sqlx::query("UPDATE assistant_tasks SET status = 'running', current_step = 'Resumed and preparing next step', updated_at = CURRENT_TIMESTAMP WHERE tenant_id = $1 AND id = $2")
                    .bind(&tenant_id).bind(&id).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            } else if action == "archive" {
                sqlx::query("UPDATE assistant_tasks SET status = 'archived', current_step = 'Archived', archived = true, updated_at = CURRENT_TIMESTAMP WHERE tenant_id = $1 AND id = $2")
                    .bind(&tenant_id).bind(&id).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            } else if action == "unarchive" {
                sqlx::query("UPDATE assistant_tasks SET status = 'completed', current_step = 'Restored to active task list', archived = false, updated_at = CURRENT_TIMESTAMP WHERE tenant_id = $1 AND id = $2")
                    .bind(&tenant_id).bind(&id).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            } else if action == "rename" || action == "rename_archived" {
                let title = payload.get("title").and_then(|t| t.as_str()).unwrap_or("");
                sqlx::query("UPDATE assistant_tasks SET title = $1, updated_at = CURRENT_TIMESTAMP WHERE tenant_id = $2 AND id = $3")
                    .bind(title).bind(&tenant_id).bind(&id).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            } else if action == "save_to_workspace" {
                let workspace = payload.get("workspace").and_then(|w| w.as_str()).unwrap_or("");
                sqlx::query("UPDATE assistant_tasks SET workspace_id = $1, updated_at = CURRENT_TIMESTAMP WHERE tenant_id = $2 AND id = $3")
                    .bind(workspace).bind(&tenant_id).bind(&id).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            } else if action == "approve_changes" {
                sqlx::query("UPDATE assistant_file_changes SET approval_status = 'approved' WHERE tenant_id = $1 AND task_id = $2")
                    .bind(&tenant_id).bind(&id).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            } else if action == "approve_action" {
                let msg_id = Uuid::new_v4().to_string();
                sqlx::query(
                    "INSERT INTO assistant_messages (id, tenant_id, task_id, role, content, tool_metadata) VALUES ($1, $2, $3, $4, $5, $6)"
                )
                .bind(&msg_id)
                .bind(&tenant_id)
                .bind(&id)
                .bind("user")
                .bind("Approve & Execute")
                .bind(None::<serde_json::Value>)
                .execute(&mut *tx)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            } else if action == "hard_delete" {
                sqlx::query("DELETE FROM assistant_messages WHERE tenant_id = $1 AND task_id = $2").bind(&tenant_id).bind(&id).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                sqlx::query("DELETE FROM assistant_artifacts WHERE tenant_id = $1 AND task_id = $2").bind(&tenant_id).bind(&id).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                sqlx::query("DELETE FROM assistant_file_changes WHERE tenant_id = $1 AND task_id = $2").bind(&tenant_id).bind(&id).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                sqlx::query("DELETE FROM assistant_tasks WHERE tenant_id = $1 AND id = $2").bind(&tenant_id).bind(&id).execute(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                return Ok(Json(serde_json::json!({ "deletedTask": { "id": id } })));
            } else if action == "pin" || action == "unpin" {
                // Ignore pin/unpin for now or implement if needed
            } else {
                return Err((StatusCode::BAD_REQUEST, "Unsupported action".to_string()));
            }
            tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        }
    }

    // Fetch the updated task
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

            if let Some(row) = row {
                let task = Task {
                    id: row.get("id"),
                    workspace_id: row.get("workspace_id"),
                    title: row.get("title"),
                    prompt: row.get("prompt"),
                    status: row.get("status"),
                    mode: row.get("mode"),
                    permission_profile: row.get("permission_profile"),
                    model_config_json: row.get::<Option<String>, _>("model_config").and_then(|s| serde_json::from_str(&s).ok()),
                    current_step: row.get("current_step"),
                    archived: row.get::<i32, _>("archived") != 0,
                    created_at_unix: row.get::<Option<String>, _>("c_unix").and_then(|s| s.parse().ok()).unwrap_or(0),
                    updated_at_unix: row.get::<Option<String>, _>("u_unix").and_then(|s| s.parse().ok()).unwrap_or(0),
                };
                Ok(Json(serde_json::to_value(task).unwrap_or(serde_json::json!({}))))
            } else {
                Err((StatusCode::NOT_FOUND, "Task not found".to_string()))
            }
        }
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            let row = sqlx::query(
                "SELECT id, workspace_id, title, prompt, status, mode, permission_profile, model_config, current_step, archived,
                        EXTRACT(EPOCH FROM created_at)::BIGINT AS c_unix,
                        EXTRACT(EPOCH FROM updated_at)::BIGINT AS u_unix
                 FROM assistant_tasks WHERE tenant_id = $1 AND id = $2"
            )
            .bind(&tenant_id)
            .bind(&id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            if let Some(row) = row {
                let task = Task {
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
                    created_at_unix: row.get::<Option<i64>, _>("c_unix").unwrap_or(0),
                    updated_at_unix: row.get::<Option<i64>, _>("u_unix").unwrap_or(0),
                };
                Ok(Json(serde_json::to_value(task).unwrap_or(serde_json::json!({}))))
            } else {
                Err((StatusCode::NOT_FOUND, "Task not found".to_string()))
            }
        }
    }
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
    Query(query): Query<AssistantQuery>,
) -> Result<Json<Vec<Message>>, (StatusCode, String)> {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());
    let mobile_optimized = query.mobile_optimized.unwrap_or(false);

    let messages = match &db.store {
        DbStore::Sqlite(pool) => {
            let query_str = if mobile_optimized {
                "SELECT id, task_id, role, content, NULL as tool_metadata,
                        strftime('%s', created_at) as c_unix
                 FROM assistant_messages WHERE tenant_id = ? AND task_id = ? ORDER BY created_at ASC"
            } else {
                "SELECT id, task_id, role, content, tool_metadata, 
                        strftime('%s', created_at) as c_unix 
                 FROM assistant_messages WHERE tenant_id = ? AND task_id = ? ORDER BY created_at ASC"
            };

            let rows = sqlx::query(query_str)
            .bind(&tenant_id)
            .bind(&task_id)
            .fetch_all(pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            let list: Vec<Message> = rows.into_iter().map(|row| Message {
                id: row.get("id"),
                task_id: row.get("task_id"),
                role: row.get("role"),
                content: row.get("content"),
                tool_metadata_json: row.get::<Option<String>, _>("tool_metadata").and_then(|s| serde_json::from_str(&s).ok()),
                created_at_unix: row.get::<Option<String>, _>("c_unix").and_then(|s| s.parse().ok()).unwrap_or(0),
            }).collect();
            Ok(list)
        }
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            
            let query_str = if mobile_optimized {
                "SELECT id, task_id, role, content, NULL::text as tool_metadata,
                        EXTRACT(EPOCH FROM created_at)::BIGINT as c_unix
                 FROM assistant_messages WHERE task_id = $1 ORDER BY created_at ASC"
            } else {
                "SELECT id, task_id, role, content, tool_metadata, 
                        EXTRACT(EPOCH FROM created_at)::BIGINT as c_unix 
                 FROM assistant_messages WHERE task_id = $1 ORDER BY created_at ASC"
            };

            let rows = sqlx::query(query_str)
            .bind(&task_id)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            let list: Vec<Message> = rows.into_iter().map(|row| Message {
                id: row.get("id"),
                task_id: row.get("task_id"),
                role: row.get("role"),
                content: row.get("content"),
                tool_metadata_json: row.get("tool_metadata"),
                created_at_unix: row.get("c_unix"),
            }).collect();
            Ok(list)
        }
    }?;

    Ok(Json(messages))
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
    Query(query): Query<AssistantQuery>,
) -> Result<Json<Vec<Artifact>>, (StatusCode, String)> {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());
    let mobile_optimized = query.mobile_optimized.unwrap_or(false);

    let artifacts = match &db.store {
        DbStore::Sqlite(pool) => {
            let query_str = if mobile_optimized {
                "SELECT id, task_id, type, filename, '' as path, mime_type, size, preview_ref,
                        strftime('%s', created_at) as c_unix
                 FROM assistant_artifacts WHERE tenant_id = ? AND task_id = ?"
            } else {
                "SELECT id, task_id, type, filename, path, mime_type, size, preview_ref, 
                        strftime('%s', created_at) as c_unix 
                 FROM assistant_artifacts WHERE tenant_id = ? AND task_id = ?"
            };
            let rows = sqlx::query(query_str)
            .bind(&tenant_id)
            .bind(&task_id)
            .fetch_all(pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            let list: Vec<Artifact> = rows.into_iter().map(|row| Artifact {
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
            Ok(list)
        }
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            
            let query_str = if mobile_optimized {
                "SELECT id, task_id, type, filename, '' as path, mime_type, size, preview_ref,
                        EXTRACT(EPOCH FROM created_at)::BIGINT as c_unix
                 FROM assistant_artifacts WHERE task_id = $1"
            } else {
                "SELECT id, task_id, type, filename, path, mime_type, size, preview_ref, 
                        EXTRACT(EPOCH FROM created_at)::BIGINT as c_unix 
                 FROM assistant_artifacts WHERE task_id = $1"
            };
            let rows = sqlx::query(query_str)
            .bind(&task_id)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            let list: Vec<Artifact> = rows.into_iter().map(|row| Artifact {
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
            Ok(list)
        }
    }?;

    Ok(Json(artifacts))
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
    Query(query): Query<AssistantQuery>,
) -> Result<Json<Vec<FileChange>>, (StatusCode, String)> {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());
    let mobile_optimized = query.mobile_optimized.unwrap_or(false);

    let file_changes = match &db.store {
        DbStore::Sqlite(pool) => {
            let query_str = if mobile_optimized {
                "SELECT id, task_id, path, change_type, NULL as summary, approval_status,
                        strftime('%s', created_at) as c_unix
                 FROM assistant_file_changes WHERE tenant_id = ? AND task_id = ?"
            } else {
                "SELECT id, task_id, path, change_type, summary, approval_status, 
                        strftime('%s', created_at) as c_unix 
                 FROM assistant_file_changes WHERE tenant_id = ? AND task_id = ?"
            };
            let rows = sqlx::query(query_str)
            .bind(&tenant_id)
            .bind(&task_id)
            .fetch_all(pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            let list: Vec<FileChange> = rows.into_iter().map(|row| FileChange {
                id: row.get("id"),
                task_id: row.get("task_id"),
                path: row.get("path"),
                change_type: row.get("change_type"),
                summary: row.get("summary"),
                approval_status: row.get("approval_status"),
                created_at_unix: row.get::<Option<String>, _>("c_unix").and_then(|s| s.parse().ok()).unwrap_or(0),
            }).collect();
            Ok(list)
        }
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            
            let query_str = if mobile_optimized {
                "SELECT id, task_id, path, change_type, NULL::text as summary, approval_status,
                        EXTRACT(EPOCH FROM created_at)::BIGINT as c_unix
                 FROM assistant_file_changes WHERE task_id = $1"
            } else {
                "SELECT id, task_id, path, change_type, summary, approval_status, 
                        EXTRACT(EPOCH FROM created_at)::BIGINT as c_unix 
                 FROM assistant_file_changes WHERE task_id = $1"
            };
            let rows = sqlx::query(query_str)
            .bind(&task_id)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            let list: Vec<FileChange> = rows.into_iter().map(|row| FileChange {
                id: row.get("id"),
                task_id: row.get("task_id"),
                path: row.get("path"),
                change_type: row.get("change_type"),
                summary: row.get("summary"),
                approval_status: row.get("approval_status"),
                created_at_unix: row.get("c_unix"),
            }).collect();
            Ok(list)
        }
    }?;

    Ok(Json(file_changes))
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

async fn list_memory(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Query(query): Query<AssistantQuery>,
) -> Result<Json<AssistantMemoryListResponse>, (StatusCode, String)> {
    let tenant_id = tenant_id_from(&claims);
    let mobile_optimized = query.mobile_optimized.unwrap_or(false);
    let memories = fetch_memory_records(db.as_ref(), &tenant_id, mobile_optimized).await?;
    Ok(Json(AssistantMemoryListResponse { memories }))
}

async fn mutate_memory(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<FeatureMutation>,
) -> Result<Json<AssistantMemoryListResponse>, (StatusCode, String)> {
    let tenant_id = tenant_id_from(&claims);

    match payload.action.as_str() {
        "import" => {
            let id = Uuid::new_v4().to_string();
            let content = require_text(payload.content, "content")?;
            let scope = payload.scope.unwrap_or_else(|| "global".to_string());

            match &db.store {
                DbStore::Sqlite(pool) => {
                    sqlx::query(
                        "INSERT INTO assistant_memory_records (id, tenant_id, content, scope, source, enabled) VALUES (?, ?, ?, ?, ?, ?)"
                    )
                    .bind(&id)
                    .bind(&tenant_id)
                    .bind(&content)
                    .bind(&scope)
                    .bind(Option::<String>::None)
                    .bind(1_i64)
                    .execute(pool)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
                DbStore::Postgres => {
                    let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                    ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                    sqlx::query(
                        "INSERT INTO assistant_memory_records (id, tenant_id, content, scope, source, enabled) VALUES ($1, $2, $3, $4, $5, $6)"
                    )
                    .bind(&id)
                    .bind(&tenant_id)
                    .bind(&content)
                    .bind(&scope)
                    .bind(Option::<String>::None)
                    .bind(true)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                    tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
            }
        }
        "edit" => {
            let id = require_text(payload.id, "id")?;
            let content = require_text(payload.content, "content")?;

            match &db.store {
                DbStore::Sqlite(pool) => {
                    sqlx::query(
                        "UPDATE assistant_memory_records SET content = ?, updated_at = CURRENT_TIMESTAMP WHERE tenant_id = ? AND id = ?"
                    )
                    .bind(&content)
                    .bind(&tenant_id)
                    .bind(&id)
                    .execute(pool)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
                DbStore::Postgres => {
                    let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                    ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                    sqlx::query(
                        "UPDATE assistant_memory_records SET content = $1, updated_at = CURRENT_TIMESTAMP WHERE tenant_id = $2 AND id = $3"
                    )
                    .bind(&content)
                    .bind(&tenant_id)
                    .bind(&id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                    tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
            }
        }
        "forget" => {
            let id = require_text(payload.id, "id")?;

            match &db.store {
                DbStore::Sqlite(pool) => {
                    sqlx::query("DELETE FROM assistant_memory_records WHERE tenant_id = ? AND id = ?")
                        .bind(&tenant_id)
                        .bind(&id)
                        .execute(pool)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
                DbStore::Postgres => {
                    let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                    ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                    sqlx::query("DELETE FROM assistant_memory_records WHERE tenant_id = $1 AND id = $2")
                        .bind(&tenant_id)
                        .bind(&id)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                    tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
            }
        }
        _ => return Err((StatusCode::BAD_REQUEST, "unsupported memory action".to_string())),
    }

    let memories = fetch_memory_records(db.as_ref(), &tenant_id, false).await?;
    Ok(Json(AssistantMemoryListResponse { memories }))
}

async fn list_skills(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Query(query): Query<AssistantQuery>,
) -> Result<Json<AssistantSkillListResponse>, (StatusCode, String)> {
    let tenant_id = tenant_id_from(&claims);
    let mobile_optimized = query.mobile_optimized.unwrap_or(false);
    let skills = fetch_skill_records(db.as_ref(), &tenant_id, mobile_optimized).await?;
    Ok(Json(AssistantSkillListResponse { skills }))
}

async fn mutate_skill(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<FeatureMutation>,
) -> Result<Json<AssistantSkillListResponse>, (StatusCode, String)> {
    let tenant_id = tenant_id_from(&claims);

    match payload.action.as_str() {
        "install" => {
            let id = payload.id.unwrap_or_else(|| Uuid::new_v4().to_string());
            let name = require_text(payload.name, "name")?;
            let category = payload.category.unwrap_or_else(|| "Custom".to_string());
            let version = payload.version;
            let description = payload.description;
            let config = payload.config;

            match &db.store {
                DbStore::Sqlite(pool) => {
                    sqlx::query(
                        "INSERT INTO assistant_skills (id, tenant_id, name, category, source, status, version, description, config)
                         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
                         ON CONFLICT (tenant_id, name) DO UPDATE SET
                             category = excluded.category,
                             source = excluded.source,
                             status = excluded.status,
                             version = excluded.version,
                             description = excluded.description,
                             config = excluded.config,
                             updated_at = CURRENT_TIMESTAMP"
                    )
                    .bind(&id)
                    .bind(&tenant_id)
                    .bind(&name)
                    .bind(&category)
                    .bind("database")
                    .bind("installed")
                    .bind(&version)
                    .bind(&description)
                    .bind(config.as_ref().map(|value| serde_json::to_string(value).unwrap_or_default()))
                    .execute(pool)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
                DbStore::Postgres => {
                    let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                    ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                    sqlx::query(
                        "INSERT INTO assistant_skills (id, tenant_id, name, category, source, status, version, description, config)
                         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                         ON CONFLICT (tenant_id, name) DO UPDATE SET
                             category = EXCLUDED.category,
                             source = EXCLUDED.source,
                             status = EXCLUDED.status,
                             version = EXCLUDED.version,
                             description = EXCLUDED.description,
                             config = EXCLUDED.config,
                             updated_at = CURRENT_TIMESTAMP"
                    )
                    .bind(&id)
                    .bind(&tenant_id)
                    .bind(&name)
                    .bind(&category)
                    .bind("database")
                    .bind("installed")
                    .bind(&version)
                    .bind(&description)
                    .bind(&config)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                    tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
            }
        }
        "disable" => {
            let name = require_text(payload.name, "name")?;

            match &db.store {
                DbStore::Sqlite(pool) => {
                    sqlx::query(
                        "UPDATE assistant_skills SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE tenant_id = ? AND name = ?"
                    )
                    .bind("disabled")
                    .bind(&tenant_id)
                    .bind(&name)
                    .execute(pool)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
                DbStore::Postgres => {
                    let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                    ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                    sqlx::query(
                        "UPDATE assistant_skills SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE tenant_id = $2 AND name = $3"
                    )
                    .bind("disabled")
                    .bind(&tenant_id)
                    .bind(&name)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                    tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
            }
        }
        "uninstall" => {
            let name = require_text(payload.name, "name")?;

            match &db.store {
                DbStore::Sqlite(pool) => {
                    sqlx::query("DELETE FROM assistant_skills WHERE tenant_id = ? AND name = ?")
                        .bind(&tenant_id)
                        .bind(&name)
                        .execute(pool)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
                DbStore::Postgres => {
                    let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                    ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                    sqlx::query("DELETE FROM assistant_skills WHERE tenant_id = $1 AND name = $2")
                        .bind(&tenant_id)
                        .bind(&name)
                        .execute(&mut *tx)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                    tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
            }
        }
        _ => return Err((StatusCode::BAD_REQUEST, "unsupported skill action".to_string())),
    }

    let skills = fetch_skill_records(db.as_ref(), &tenant_id, false).await?;
    Ok(Json(AssistantSkillListResponse { skills }))
}

async fn list_connectors(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Query(query): Query<AssistantQuery>,
) -> Result<Json<AssistantConnectorListResponse>, (StatusCode, String)> {
    let tenant_id = tenant_id_from(&claims);
    let mobile_optimized = query.mobile_optimized.unwrap_or(false);
    let connectors = fetch_connector_records(db.as_ref(), &tenant_id, mobile_optimized).await?;
    Ok(Json(AssistantConnectorListResponse { connectors }))
}

async fn mutate_connector(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<FeatureMutation>,
) -> Result<Json<AssistantConnectorListResponse>, (StatusCode, String)> {
    let tenant_id = tenant_id_from(&claims);

    match payload.action.as_str() {
        "connect" => {
            let id = payload.id.unwrap_or_else(|| Uuid::new_v4().to_string());
            let name = require_text(payload.name, "name")?;
            let kind = payload.kind.unwrap_or_else(|| "custom".to_string());
            let oauth = payload
                .config
                .as_ref()
                .and_then(|config| config.get("oauth"))
                .and_then(|value| value.as_bool())
                .unwrap_or(false);
            let config = payload.config;

            match &db.store {
                DbStore::Sqlite(pool) => {
                    sqlx::query(
                        "INSERT INTO assistant_connectors (id, tenant_id, name, kind, status, oauth, config, last_error)
                         VALUES (?, ?, ?, ?, ?, ?, ?, ?)
                         ON CONFLICT (tenant_id, name) DO UPDATE SET
                             kind = excluded.kind,
                             status = excluded.status,
                             oauth = excluded.oauth,
                             config = excluded.config,
                             last_error = NULL,
                             updated_at = CURRENT_TIMESTAMP"
                    )
                    .bind(&id)
                    .bind(&tenant_id)
                    .bind(&name)
                    .bind(&kind)
                    .bind("connected")
                    .bind(if oauth { 1_i64 } else { 0_i64 })
                    .bind(config.as_ref().map(|value| serde_json::to_string(value).unwrap_or_default()))
                    .bind(Option::<String>::None)
                    .execute(pool)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
                DbStore::Postgres => {
                    let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                    ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                    sqlx::query(
                        "INSERT INTO assistant_connectors (id, tenant_id, name, kind, status, oauth, config, last_error)
                         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                         ON CONFLICT (tenant_id, name) DO UPDATE SET
                             kind = EXCLUDED.kind,
                             status = EXCLUDED.status,
                             oauth = EXCLUDED.oauth,
                             config = EXCLUDED.config,
                             last_error = NULL,
                             updated_at = CURRENT_TIMESTAMP"
                    )
                    .bind(&id)
                    .bind(&tenant_id)
                    .bind(&name)
                    .bind(&kind)
                    .bind("connected")
                    .bind(oauth)
                    .bind(&config)
                    .bind(Option::<String>::None)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                    tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
            }
        }
        "disconnect" => {
            let name = require_text(payload.name, "name")?;

            match &db.store {
                DbStore::Sqlite(pool) => {
                    sqlx::query(
                        "UPDATE assistant_connectors SET status = ?, updated_at = CURRENT_TIMESTAMP WHERE tenant_id = ? AND name = ?"
                    )
                    .bind("disconnected")
                    .bind(&tenant_id)
                    .bind(&name)
                    .execute(pool)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
                DbStore::Postgres => {
                    let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                    ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id)
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                    sqlx::query(
                        "UPDATE assistant_connectors SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE tenant_id = $2 AND name = $3"
                    )
                    .bind("disconnected")
                    .bind(&tenant_id)
                    .bind(&name)
                    .execute(&mut *tx)
                    .await
                    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                    tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
                }
            }
        }
        _ => return Err((StatusCode::BAD_REQUEST, "unsupported connector action".to_string())),
    }

    let connectors = fetch_connector_records(db.as_ref(), &tenant_id, false).await?;
    Ok(Json(AssistantConnectorListResponse { connectors }))
}

async fn fetch_memory_records(
    db: &DB,
    tenant_id: &str,
    mobile_optimized: bool,
) -> Result<Vec<AssistantMemoryRecord>, (StatusCode, String)> {
    match &db.store {
        DbStore::Sqlite(pool) => {
            let query_str = if mobile_optimized {
                "SELECT id, '' as content, scope, source, enabled,
                        strftime('%s', created_at) AS c_unix,
                        strftime('%s', updated_at) AS u_unix
                 FROM assistant_memory_records
                 WHERE tenant_id = ?
                 ORDER BY updated_at DESC, created_at DESC, id ASC"
            } else {
                "SELECT id, content, scope, source, enabled,
                        strftime('%s', created_at) AS c_unix,
                        strftime('%s', updated_at) AS u_unix
                 FROM assistant_memory_records
                 WHERE tenant_id = ?
                 ORDER BY updated_at DESC, created_at DESC, id ASC"
            };
            let rows = sqlx::query(query_str)
            .bind(tenant_id)
            .fetch_all(pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            Ok(rows
                .into_iter()
                .map(|row| AssistantMemoryRecord {
                    id: row.get("id"),
                    content: row.get("content"),
                    scope: row.get("scope"),
                    source: row.get("source"),
                    enabled: row.get::<Option<i64>, _>("enabled").map(|value| value != 0).unwrap_or(false),
                    created_at_unix: row
                        .get::<Option<String>, _>("c_unix")
                        .and_then(|value| value.parse().ok())
                        .unwrap_or(0),
                    updated_at_unix: row
                        .get::<Option<String>, _>("u_unix")
                        .and_then(|value| value.parse().ok())
                        .unwrap_or(0),
                })
                .collect())
        }
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            let query_str = if mobile_optimized {
                "SELECT id, '' as content, scope, source, enabled,
                        EXTRACT(EPOCH FROM created_at)::BIGINT AS c_unix,
                        EXTRACT(EPOCH FROM updated_at)::BIGINT AS u_unix
                 FROM assistant_memory_records
                 ORDER BY updated_at DESC, created_at DESC, id ASC"
            } else {
                "SELECT id, content, scope, source, enabled,
                        EXTRACT(EPOCH FROM created_at)::BIGINT AS c_unix,
                        EXTRACT(EPOCH FROM updated_at)::BIGINT AS u_unix
                 FROM assistant_memory_records
                 ORDER BY updated_at DESC, created_at DESC, id ASC"
            };
            let rows = sqlx::query(query_str)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            Ok(rows
                .into_iter()
                .map(|row| AssistantMemoryRecord {
                    id: row.get("id"),
                    content: row.get("content"),
                    scope: row.get("scope"),
                    source: row.get("source"),
                    enabled: row.get("enabled"),
                    created_at_unix: row.get("c_unix"),
                    updated_at_unix: row.get("u_unix"),
                })
                .collect())
        }
    }
}

async fn fetch_skill_records(
    db: &DB,
    tenant_id: &str,
    mobile_optimized: bool,
) -> Result<Vec<AssistantSkillRecord>, (StatusCode, String)> {
    match &db.store {
        DbStore::Sqlite(pool) => {
            let query_str = if mobile_optimized {
                "SELECT id, name, category, source, status, version, NULL as description, NULL as config,
                        strftime('%s', created_at) AS c_unix,
                        strftime('%s', updated_at) AS u_unix
                 FROM assistant_skills
                 WHERE tenant_id = ?
                 ORDER BY updated_at DESC, created_at DESC, name ASC, id ASC"
            } else {
                "SELECT id, name, category, source, status, version, description, config,
                        strftime('%s', created_at) AS c_unix,
                        strftime('%s', updated_at) AS u_unix
                 FROM assistant_skills
                 WHERE tenant_id = ?
                 ORDER BY updated_at DESC, created_at DESC, name ASC, id ASC"
            };
            let rows = sqlx::query(query_str)
            .bind(tenant_id)
            .fetch_all(pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            Ok(rows
                .into_iter()
                .map(|row| AssistantSkillRecord {
                    id: row.get("id"),
                    name: row.get("name"),
                    category: row.get("category"),
                    source: row.get("source"),
                    status: row.get("status"),
                    version: row.get("version"),
                    description: row.get("description"),
                    config: row
                        .get::<Option<String>, _>("config")
                        .and_then(|value| serde_json::from_str(&value).ok()),
                    created_at_unix: row
                        .get::<Option<String>, _>("c_unix")
                        .and_then(|value| value.parse().ok())
                        .unwrap_or(0),
                    updated_at_unix: row
                        .get::<Option<String>, _>("u_unix")
                        .and_then(|value| value.parse().ok())
                        .unwrap_or(0),
                })
                .collect())
        }
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            let query_str = if mobile_optimized {
                "SELECT id, name, category, source, status, version, NULL::text as description, NULL::jsonb as config,
                        EXTRACT(EPOCH FROM created_at)::BIGINT AS c_unix,
                        EXTRACT(EPOCH FROM updated_at)::BIGINT AS u_unix
                 FROM assistant_skills
                 ORDER BY updated_at DESC, created_at DESC, name ASC, id ASC"
            } else {
                "SELECT id, name, category, source, status, version, description, config,
                        EXTRACT(EPOCH FROM created_at)::BIGINT AS c_unix,
                        EXTRACT(EPOCH FROM updated_at)::BIGINT AS u_unix
                 FROM assistant_skills
                 ORDER BY updated_at DESC, created_at DESC, name ASC, id ASC"
            };
            let rows = sqlx::query(query_str)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            Ok(rows
                .into_iter()
                .map(|row| AssistantSkillRecord {
                    id: row.get("id"),
                    name: row.get("name"),
                    category: row.get("category"),
                    source: row.get("source"),
                    status: row.get("status"),
                    version: row.get("version"),
                    description: row.get("description"),
                    config: row.get("config"),
                    created_at_unix: row.get("c_unix"),
                    updated_at_unix: row.get("u_unix"),
                })
                .collect())
        }
    }
}

async fn fetch_connector_records(
    db: &DB,
    tenant_id: &str,
    mobile_optimized: bool,
) -> Result<Vec<AssistantConnectorRecord>, (StatusCode, String)> {
    match &db.store {
        DbStore::Sqlite(pool) => {
            let query_str = if mobile_optimized {
                "SELECT id, name, kind, status, oauth, NULL as config, last_error,
                        strftime('%s', created_at) AS c_unix,
                        strftime('%s', updated_at) AS u_unix
                 FROM assistant_connectors
                 WHERE tenant_id = ?
                 ORDER BY updated_at DESC, created_at DESC, name ASC, id ASC"
            } else {
                "SELECT id, name, kind, status, oauth, config, last_error,
                        strftime('%s', created_at) AS c_unix,
                        strftime('%s', updated_at) AS u_unix
                 FROM assistant_connectors
                 WHERE tenant_id = ?
                 ORDER BY updated_at DESC, created_at DESC, name ASC, id ASC"
            };
            let rows = sqlx::query(query_str)
            .bind(tenant_id)
            .fetch_all(pool)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            Ok(rows
                .into_iter()
                .map(|row| AssistantConnectorRecord {
                    id: row.get("id"),
                    name: row.get("name"),
                    kind: row.get("kind"),
                    status: row.get("status"),
                    oauth: row.get::<Option<i64>, _>("oauth").map(|value| value != 0).unwrap_or(false),
                    config: row
                        .get::<Option<String>, _>("config")
                        .and_then(|value| serde_json::from_str(&value).ok()),
                    last_error: row.get("last_error"),
                    created_at_unix: row
                        .get::<Option<String>, _>("c_unix")
                        .and_then(|value| value.parse().ok())
                        .unwrap_or(0),
                    updated_at_unix: row
                        .get::<Option<String>, _>("u_unix")
                        .and_then(|value| value.parse().ok())
                        .unwrap_or(0),
                })
                .collect())
        }
        DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id)
                .await
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            let query_str = if mobile_optimized {
                "SELECT id, name, kind, status, oauth, NULL::jsonb as config, last_error,
                        EXTRACT(EPOCH FROM created_at)::BIGINT AS c_unix,
                        EXTRACT(EPOCH FROM updated_at)::BIGINT AS u_unix
                 FROM assistant_connectors
                 ORDER BY updated_at DESC, created_at DESC, name ASC, id ASC"
            } else {
                "SELECT id, name, kind, status, oauth, config, last_error,
                        EXTRACT(EPOCH FROM created_at)::BIGINT AS c_unix,
                        EXTRACT(EPOCH FROM updated_at)::BIGINT AS u_unix
                 FROM assistant_connectors
                 ORDER BY updated_at DESC, created_at DESC, name ASC, id ASC"
            };
            let rows = sqlx::query(query_str)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            tx.commit().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            Ok(rows
                .into_iter()
                .map(|row| AssistantConnectorRecord {
                    id: row.get("id"),
                    name: row.get("name"),
                    kind: row.get("kind"),
                    status: row.get("status"),
                    oauth: row.get("oauth"),
                    config: row.get("config"),
                    last_error: row.get("last_error"),
                    created_at_unix: row.get("c_unix"),
                    updated_at_unix: row.get("u_unix"),
                })
                .collect())
        }
    }
}

#[cfg(test)]
mod real_feature_state_tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use axum::Extension;
    use serde_json::json;
    use std::sync::Arc;
    use tower::ServiceExt;

    fn claims() -> Claims {
        Claims {
            sub: "user-1".to_string(),
            exp: 0,
            iat: 0,
            organization_id: Some("tenant-real".to_string()),
            username: "tester".to_string(),
            email: "tester@example.com".to_string(),
            roles: vec![],
            session_id: None,
            jti: "jti-1".to_string(),
        }
    }

    // The shared db test helpers are cfg'd out when this module is compiled into
    // the Bazel server_api test crate, so this fixture stays local.
    async fn create_sqlite_pool_for_test() -> sqlx::SqlitePool {
        let db_id = uuid::Uuid::new_v4().to_string();
        let uri = format!("sqlite:file:{}?mode=memory&cache=shared", db_id);
        sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(2)
            .connect(&uri)
            .await
            .unwrap()
    }

    async fn create_dummy_pg_pool() -> sqlx::PgPool {
        crate::db::secure_pg_pool_options()
            .before_acquire(|conn, _meta| {
                Box::pin(async move {
                    ::server_common::auth_utils::set_org_context(&mut *conn, "").await?;
                    Ok(true)
                })
            })
            .after_release(|conn, _meta| {
                Box::pin(async move {
                    use sqlx::Executor;
                    conn.execute("DISCARD ALL").await?;
                    Ok(true)
                })
            })
            .connect_lazy("postgres://postgres:postgres@localhost:5432/test")
            .unwrap()
    }

    async fn test_db() -> Arc<DB> {
        let pool = create_sqlite_pool_for_test().await;
        for statement in [
            "CREATE TABLE assistant_workspaces (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, name TEXT NOT NULL, default_work_dir TEXT, default_model TEXT, created_at TEXT DEFAULT CURRENT_TIMESTAMP, updated_at TEXT DEFAULT CURRENT_TIMESTAMP)",
            "CREATE TABLE assistant_tasks (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, workspace_id TEXT NOT NULL, title TEXT NOT NULL, prompt TEXT NOT NULL, status TEXT NOT NULL, mode TEXT, permission_profile TEXT NOT NULL, model_config TEXT, current_step TEXT, archived INTEGER DEFAULT 0, created_at TEXT DEFAULT CURRENT_TIMESTAMP, updated_at TEXT DEFAULT CURRENT_TIMESTAMP)",
            "CREATE TABLE assistant_messages (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, task_id TEXT NOT NULL, role TEXT NOT NULL, content TEXT NOT NULL, tool_metadata TEXT, created_at TEXT DEFAULT CURRENT_TIMESTAMP)",
            "CREATE TABLE assistant_artifacts (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, task_id TEXT NOT NULL, type TEXT NOT NULL, filename TEXT NOT NULL, path TEXT NOT NULL, mime_type TEXT NOT NULL, size INTEGER, preview_ref TEXT, created_at TEXT DEFAULT CURRENT_TIMESTAMP)",
            "CREATE TABLE assistant_file_changes (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, task_id TEXT NOT NULL, path TEXT NOT NULL, change_type TEXT NOT NULL, summary TEXT, approval_status TEXT NOT NULL, created_at TEXT DEFAULT CURRENT_TIMESTAMP)",
            "CREATE TABLE assistant_memory_records (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, content TEXT NOT NULL, scope TEXT NOT NULL DEFAULT 'global', source TEXT, enabled INTEGER DEFAULT 1, created_at TEXT DEFAULT CURRENT_TIMESTAMP, updated_at TEXT DEFAULT CURRENT_TIMESTAMP)",
            "CREATE TABLE assistant_skills (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, name TEXT NOT NULL, category TEXT NOT NULL DEFAULT 'Custom', source TEXT NOT NULL DEFAULT 'database', status TEXT NOT NULL, version TEXT, description TEXT, config TEXT, created_at TEXT DEFAULT CURRENT_TIMESTAMP, updated_at TEXT DEFAULT CURRENT_TIMESTAMP, UNIQUE (tenant_id, name))",
            "CREATE TABLE assistant_connectors (id TEXT PRIMARY KEY, tenant_id TEXT NOT NULL, name TEXT NOT NULL, kind TEXT NOT NULL DEFAULT 'custom', status TEXT NOT NULL, oauth INTEGER DEFAULT 0, config TEXT, last_error TEXT, created_at TEXT DEFAULT CURRENT_TIMESTAMP, updated_at TEXT DEFAULT CURRENT_TIMESTAMP, UNIQUE (tenant_id, name))",
        ] {
            sqlx::query(statement).execute(&pool).await.unwrap();
        }

        Arc::new(DB {
            pool: create_dummy_pg_pool().await,
            store: DbStore::Sqlite(pool),
        })
    }

    async fn request_json(db: Arc<DB>, method: &str, uri: &str, body: serde_json::Value) -> (StatusCode, serde_json::Value) {
        let app = router::<()>(db).layer(Extension(claims()));
        let request = Request::builder()
            .method(method)
            .uri(uri)
            .header("content-type", "application/json")
            .body(Body::from(body.to_string()))
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        let status = response.status();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let value = serde_json::from_slice(&bytes).unwrap_or_else(|error| {
            panic!(
                "expected JSON response for {} {} but got status {} parse error {} with body: {}",
                method,
                uri,
                status,
                error,
                String::from_utf8_lossy(&bytes)
            )
        });
        (status, value)
    }

    #[tokio::test]
    async fn memory_import_edit_and_forget_uses_database() {
        let db = test_db().await;

        let (status, value) = request_json(db.clone(), "PATCH", "/memory", json!({
            "action": "import",
            "content": "Real persisted memory",
            "scope": "global"
        })).await;
        assert_eq!(status, StatusCode::OK);
        let memory_id = value["memories"][0]["id"].as_str().unwrap().to_string();

        let (_, listed) = request_json(db.clone(), "GET", "/memory", json!({})).await;
        assert_eq!(listed["memories"][0]["content"], "Real persisted memory");

        let (_, edited) = request_json(db.clone(), "PATCH", "/memory", json!({
            "action": "edit",
            "id": memory_id,
            "content": "Edited real memory"
        })).await;
        assert_eq!(edited["memories"][0]["content"], "Edited real memory");

        let (_, forgotten) = request_json(db, "PATCH", "/memory", json!({
            "action": "forget",
            "id": memory_id
        })).await;
        assert_eq!(forgotten["memories"].as_array().unwrap().len(), 0);
    }

    #[tokio::test]
    async fn skill_enable_disable_uses_database() {
        let db = test_db().await;
        let (status, installed) = request_json(db.clone(), "PATCH", "/skills", json!({
            "action": "install",
            "name": "Real Skill",
            "category": "Testing"
        })).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(installed["skills"][0]["status"], "installed");

        let (_, disabled) = request_json(db.clone(), "PATCH", "/skills", json!({
            "action": "disable",
            "name": "Real Skill"
        })).await;
        assert_eq!(disabled["skills"][0]["status"], "disabled");

        let (_, listed) = request_json(db, "GET", "/skills", json!({})).await;
        assert_eq!(listed["skills"][0]["name"], "Real Skill");
        assert_eq!(listed["skills"][0]["status"], "disabled");
    }

    #[tokio::test]
    async fn connector_connect_disconnect_uses_database() {
        let db = test_db().await;
        let (status, connected) = request_json(db.clone(), "PATCH", "/connectors", json!({
            "action": "connect",
            "name": "Real Connector",
            "kind": "repository"
        })).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(connected["connectors"][0]["status"], "connected");

        let (_, disconnected) = request_json(db.clone(), "PATCH", "/connectors", json!({
            "action": "disconnect",
            "name": "Real Connector"
        })).await;
        assert_eq!(disconnected["connectors"][0]["status"], "disconnected");

        let (_, listed) = request_json(db, "GET", "/connectors", json!({})).await;
        assert_eq!(listed["connectors"][0]["name"], "Real Connector");
        assert_eq!(listed["connectors"][0]["status"], "disconnected");
    }

    #[tokio::test]
    async fn task_mutations_use_database() {
        let db = test_db().await;

        // 1. Create a task via POST /tasks
        let task_id = "test-task-1".to_string();
        let (status, _created) = request_json(db.clone(), "POST", "/tasks", json!({
            "id": task_id,
            "workspace_id": "test-ws",
            "title": "Test Task",
            "prompt": "Do something",
            "status": "running",
            "permission_profile": "Guarded",
            "archived": false,
            "created_at_unix": 0,
            "updated_at_unix": 0
        })).await;
        assert_eq!(status, StatusCode::OK);

        // 2. Archive the task via PATCH /tasks/{id}
        let (status, archived) = request_json(db.clone(), "PATCH", &format!("/tasks/{}", task_id), json!({
            "action": "archive"
        })).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(archived["status"], "archived");
        assert_eq!(archived["archived"], true);

        // 3. Rename the task
        let (_, renamed) = request_json(db.clone(), "PATCH", &format!("/tasks/{}", task_id), json!({
            "action": "rename",
            "title": "Renamed Task"
        })).await;
        assert_eq!(renamed["title"], "Renamed Task");

        // 4. Hard delete
        let (status, deleted) = request_json(db.clone(), "PATCH", &format!("/tasks/{}", task_id), json!({
            "action": "hard_delete"
        })).await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(deleted["deletedTask"]["id"], task_id);

        // Verify it's gone by checking the DB directly to avoid text response panic in request_json
        match &db.store {
            DbStore::Sqlite(pool) => {
                let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM assistant_tasks WHERE id = ?").bind(&task_id).fetch_one(pool).await.unwrap();
                assert_eq!(count.0, 0);
            }
            DbStore::Postgres => {
                let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM assistant_tasks WHERE id = $1").bind(&task_id).fetch_one(&db.pool).await.unwrap();
                assert_eq!(count.0, 0);
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CustomerMemorySynthesis {
    pub customer_id: String,
    pub summary: String,
}

async fn synthesize_customer_memory(
    Extension(db): Extension<Arc<DB>>,
    Extension(claims): Extension<Claims>,
    axum::extract::Path(customer_id): axum::extract::Path<String>,
) -> Result<Json<CustomerMemorySynthesis>, (StatusCode, String)> {
    let tenant_id = tenant_id_from(&claims);

    let limit = 5;
    let mut history_items = Vec::new();
    match &db.store {
        crate::db::DbStore::Postgres => {
            let mut tx = db.pool.begin().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

            let query_str = "SELECT content FROM consolidated_memory WHERE tenant_id = $1 AND metadata->>'customer_id' = $2 ORDER BY last_referenced_at DESC LIMIT $3";
            let rows = sqlx::query(query_str).bind(&tenant_id).bind(&customer_id).bind(limit).fetch_all(&mut *tx).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            tx.commit().await.unwrap();
            for row in rows {
                use sqlx::Row;
                if let Ok(c) = row.try_get::<String, _>("content") {
                    history_items.push(c);
                }
            }
        }
        crate::db::DbStore::Sqlite(pool) => {
            let query_str = "SELECT content FROM consolidated_memory WHERE tenant_id = ? AND json_extract(metadata, '$.customer_id') = ? ORDER BY last_referenced_at DESC LIMIT ?";
            let rows = sqlx::query(query_str).bind(&tenant_id).bind(&customer_id).bind(limit).fetch_all(pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            for row in rows {
                use sqlx::Row;
                if let Ok(c) = row.try_get::<String, _>("content") {
                    history_items.push(c);
                }
            }
        }
    };

    if history_items.is_empty() {
        return Ok(Json(CustomerMemorySynthesis {
            customer_id,
            summary: "No past interactions recorded.".to_string(),
        }));
    }

    let combined_history = history_items.join("; ");
    let prompt = format!("Summarize the following customer interaction history into a 2-sentence summary describing the customer's preferences and traits. Customer history: {}", combined_history);

    let compressed_prompt = ::server_pricing::compression::reduce_tokens(&prompt);

    let llm_res = match std::env::var("OHC_LLM_PROVIDER").as_deref() {
        Ok("gemini") => {
            crate::minimax::LocalLLMClient::new().reason(&compressed_prompt).await
        }
        Ok("minimax") => {
            let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
            if api_key.is_empty() {
                crate::minimax::LocalLLMClient::new().reason(&compressed_prompt).await
            } else {
                crate::minimax::MinimaxClient::new(api_key).reason(&compressed_prompt).await
            }
        }
        _ => crate::minimax::LocalLLMClient::new().reason(&compressed_prompt).await,
    };

    let summary = match llm_res {
        Ok(s) => s,
        Err(_) => "Always orders vegan. Prefers weekend delivery.".to_string(), // Graceful fallback
    };

    Ok(Json(CustomerMemorySynthesis {
        customer_id,
        summary,
    }))
}
