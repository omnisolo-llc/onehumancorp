use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct CreateWorkspaceRequest {
    pub name: String,
    pub default_work_directory: Option<String>,
    pub default_model: Option<String>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct AssistantWorkspace {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub default_work_directory: Option<String>,
    pub default_model: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

pub async fn create_workspace_handler(
    State(pool): State<PgPool>,
    axum::extract::Extension(user): axum::extract::Extension<::server_common::Claims>,
    Json(payload): Json<CreateWorkspaceRequest>,
) -> impl IntoResponse {
    let id = Uuid::new_v4().to_string();
    let tenant_id = user.organization_id.clone().unwrap_or_else(|| "default".to_string());

    let result: Result<AssistantWorkspace, sqlx::Error> = sqlx::query_as(
        r#"
        SELECT
            id,
            tenant_id,
            name,
            default_work_directory,
            default_model,
            COALESCE(created_at::text, '') as created_at,
            COALESCE(updated_at::text, '') as updated_at
        FROM (
            INSERT INTO assistant_workspaces (tenant_id, id, name, default_work_directory, default_model)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, name, default_work_directory, default_model, created_at, updated_at
        ) as t
        "#
    )
    .bind(tenant_id)
    .bind(id)
    .bind(payload.name)
    .bind(payload.default_work_directory)
    .bind(payload.default_model)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(row) => {
            let workspace = AssistantWorkspace {
                id: row.id,
                tenant_id: row.tenant_id,
                name: row.name,
                default_work_directory: row.default_work_directory,
                default_model: row.default_model,
                created_at: row.created_at,
                updated_at: row.updated_at,
            };
            (StatusCode::CREATED, Json(serde_json::json!({ "workspace": workspace }))).into_response()
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))).into_response()
        }
    }
}

pub async fn list_workspaces_handler(
    State(pool): State<PgPool>,
    axum::extract::Extension(user): axum::extract::Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = user.organization_id.clone().unwrap_or_else(|| "default".to_string());

    let result: Result<Vec<AssistantWorkspace>, sqlx::Error> = sqlx::query_as(
        r#"
        SELECT
            id,
            tenant_id,
            name,
            default_work_directory,
            default_model,
            COALESCE(created_at::text, '') as created_at,
            COALESCE(updated_at::text, '') as updated_at
        FROM assistant_workspaces
        WHERE tenant_id = $1
        ORDER BY created_at DESC
        "#
    )
    .bind(tenant_id)
    .fetch_all(&pool)
    .await;

    match result {
        Ok(rows) => {
            let workspaces: Vec<AssistantWorkspace> = rows.into_iter().map(|row| AssistantWorkspace {
                id: row.id,
                tenant_id: row.tenant_id,
                name: row.name,
                default_work_directory: row.default_work_directory,
                default_model: row.default_model,
                created_at: row.created_at,
                updated_at: row.updated_at,
            }).collect();
            (StatusCode::OK, Json(serde_json::json!({ "workspaces": workspaces }))).into_response()
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))).into_response()
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateTaskRequest {
    pub workspace: Option<String>,
    pub title: Option<String>,
    pub prompt: String,
    pub mode: Option<String>,
    pub model: Option<String>,
    pub provider: Option<String>,
    #[serde(rename = "permissionProfile")]
    pub permission_profile: Option<String>,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct AssistantTask {
    pub id: String,
    pub tenant_id: String,
    pub workspace_id: String,
    pub title: String,
    pub prompt: String,
    pub status: String,
    pub mode: String,
    pub model: String,
    pub provider: String,
    #[serde(rename = "permissionProfile")]
    pub permission_profile: String,
    #[serde(rename = "currentStep")]
    pub current_step: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub artifacts: serde_json::Value,
    pub changes: serde_json::Value,
    pub messages: serde_json::Value,
}

pub async fn create_task_handler(
    State(pool): State<PgPool>,
    axum::extract::Extension(user): axum::extract::Extension<::server_common::Claims>,
    Json(payload): Json<CreateTaskRequest>,
) -> impl IntoResponse {
    let id = Uuid::new_v4().to_string();
    let tenant_id = user.organization_id.clone().unwrap_or_else(|| "default".to_string());

    // In a real implementation we might create a default workspace if workspace_id is not provided
    let workspace_id = payload.workspace.unwrap_or_else(|| "default".to_string());

    let title = payload.title.unwrap_or_else(|| {
        if payload.prompt.len() > 30 {
            format!("{}...", &payload.prompt[..27])
        } else {
            payload.prompt.clone()
        }
    });

    let mode = payload.mode.unwrap_or_else(|| "Guarded".to_string());
    let model = payload.model.unwrap_or_else(|| "gpt-4o".to_string());
    let provider = payload.provider.unwrap_or_else(|| "openai".to_string());
    let permission_profile = payload.permission_profile.unwrap_or_else(|| "Guarded".to_string());

    let result: Result<AssistantTask, sqlx::Error> = sqlx::query_as(
        r#"
        SELECT
            id,
            tenant_id,
            workspace_id,
            title,
            prompt,
            status,
            mode,
            model,
            provider,
            permission_profile,
            current_step,
            COALESCE(created_at::text, '') as created_at,
            COALESCE(updated_at::text, '') as updated_at,
            '[]'::jsonb as artifacts,
            '[]'::jsonb as changes,
            '[]'::jsonb as messages
        FROM (
            INSERT INTO assistant_tasks (tenant_id, id, workspace_id, title, prompt, status, mode, model, provider, permission_profile, current_step)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            RETURNING id, tenant_id, workspace_id, title, prompt, status, mode, model, provider, permission_profile, current_step, created_at, updated_at
        ) as t
        "#
    )
    .bind(tenant_id)
    .bind(id)
    .bind(workspace_id)
    .bind(title)
    .bind(payload.prompt)
    .bind("running")
    .bind(mode)
    .bind(model)
    .bind(provider)
    .bind(permission_profile)
    .bind(Some("Initializing".to_string()))
    .fetch_one(&pool)
    .await;

    match result {
        Ok(row) => {
            let task = AssistantTask {
                id: row.id,
                tenant_id: row.tenant_id,
                workspace_id: row.workspace_id,
                title: row.title,
                prompt: row.prompt,
                status: row.status,
                mode: row.mode,
                model: row.model,
                provider: row.provider,
                permission_profile: row.permission_profile,
                current_step: row.current_step,
                created_at: row.created_at,
                updated_at: row.updated_at,
                artifacts: serde_json::json!([]),
                changes: serde_json::json!([]),
                messages: serde_json::json!([]),
            };
            (StatusCode::CREATED, Json(serde_json::json!({ "task": task }))).into_response()
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))).into_response()
        }
    }
}

pub async fn list_tasks_handler(
    State(pool): State<PgPool>,
    axum::extract::Extension(user): axum::extract::Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = user.organization_id.clone().unwrap_or_else(|| "default".to_string());

    let result: Result<Vec<AssistantTask>, sqlx::Error> = sqlx::query_as(
        r#"
        SELECT
            id,
            tenant_id,
            workspace_id,
            title,
            prompt,
            status,
            mode,
            model,
            provider,
            permission_profile,
            current_step,
            COALESCE(created_at::text, '') as created_at,
            COALESCE(updated_at::text, '') as updated_at,
            '[]'::jsonb as artifacts,
            '[]'::jsonb as changes,
            '[]'::jsonb as messages
        FROM assistant_tasks
        WHERE tenant_id = $1
        ORDER BY created_at DESC
        "#
    )
    .bind(tenant_id)
    .fetch_all(&pool)
    .await;

    match result {
        Ok(rows) => {
            let tasks: Vec<AssistantTask> = rows.into_iter().map(|row| AssistantTask {
                id: row.id,
                tenant_id: row.tenant_id,
                workspace_id: row.workspace_id,
                title: row.title,
                prompt: row.prompt,
                status: row.status,
                mode: row.mode,
                model: row.model,
                provider: row.provider,
                permission_profile: row.permission_profile,
                current_step: row.current_step,
                created_at: row.created_at,
                updated_at: row.updated_at,
                artifacts: serde_json::json!([]),
                changes: serde_json::json!([]),
                messages: serde_json::json!([]),
            }).collect();

            // For now, hardcode capabilities to match the expected format from store.ts
            let capabilities = serde_json::json!({
                "modes": ["Guarded", "Full Access"],
                "models": ["gpt-4o", "gemini-pro"],
                "fileOps": true,
                "remoteControl": false,
                "experts": true
            });

            (StatusCode::OK, Json(serde_json::json!({ "tasks": tasks, "capabilities": capabilities }))).into_response()
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.to_string() }))).into_response()
        }
    }
}
