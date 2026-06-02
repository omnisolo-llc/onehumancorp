use axum::{
    extract::{Extension, State},
    response::IntoResponse,
    http::StatusCode,
    routing::{get},
    Router,
    Json,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::AIaaSWorkflow;
use ::server_common::Claims;
use tokio::process::Command;

#[derive(Serialize)]
pub struct WorkflowsResponse {
    pub workflows: Vec<AIaaSWorkflow>,
}

#[derive(Deserialize)]
pub struct CreateWorkflowRequest {
    pub name: String,
    pub task: String,
}

#[derive(Serialize)]
pub struct CreateWorkflowResponse {
    pub workflow: AIaaSWorkflow,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", get(list_workflows).post(create_workflow))
        .with_state(orchestrator)
}

async fn list_workflows(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(WorkflowsResponse { workflows: vec![] })).into_response(),
    };

    let pool = orchestrator.get_db().pool.clone();
    let is_sqlite = orchestrator.get_db().is_sqlite();
    let mut workflows = Vec::new();

    if is_sqlite {
        if let Ok(rows) = sqlx::query_as!(
            crate::orchestration::departments::types::AIaaSWorkflow,
            "SELECT id, tenant_id, name, task, workflow_type, status, command, output, error, created_at as \"created_at: chrono::DateTime<chrono::Utc>\" FROM aiaas_workflows WHERE tenant_id = ? ORDER BY created_at DESC",
            tenant_id
        )
        .fetch_all(&pool)
        .await {
            workflows = rows;
        }
    } else {
        if let Ok(rows) = sqlx::query_as!(
            crate::orchestration::departments::types::AIaaSWorkflow,
            "SELECT id, tenant_id, name, task, workflow_type, status, command, output, error, created_at as \"created_at: chrono::DateTime<chrono::Utc>\" FROM aiaas_workflows WHERE tenant_id = $1 ORDER BY created_at DESC",
            tenant_id
        )
        .fetch_all(&pool)
        .await {
            workflows = rows;
        }
    }

    (StatusCode::OK, Json(WorkflowsResponse { workflows })).into_response()
}

async fn create_workflow(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateWorkflowRequest>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) => org_id.to_string(),
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "Unauthorized" }))).into_response(),
    };

    if payload.name.trim().is_empty() || payload.task.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, Json(serde_json::json!({ "error": "Workflow name and task are required" }))).into_response();
    }

    let workflow_id = uuid::Uuid::new_v4().to_string();

    // Determine command to run, matching the node logic somewhat
    let agent_binary = std::env::var("OHC_BUILTIN_AGENT_BINARY").unwrap_or_else(|_| "ohc-builtin-agent".to_string());

    let agent_task = format!(
        "Use the built-in RunWorkflow tool. Arguments: {{\"workflow\": \"ohc_review_branch\", \"task\": \"{}\"}}. Return the final synthesized report.",
        payload.task.trim()
    );

    let command_str = format!("{} --task {}", agent_binary, agent_task);

    let workflow = AIaaSWorkflow {
        id: workflow_id.clone(),
        tenant_id: tenant_id.clone(),
        name: payload.name.trim().to_string(),
        task: payload.task.trim().to_string(),
        workflow_type: "ohc_review_branch".to_string(),
        status: "running".to_string(),
        command: Some(command_str.clone()),
        output: None,
        error: None,
        created_at: Some(chrono::Utc::now()),
    };

    let pool = orchestrator.get_db().pool.clone();
    let is_sqlite = orchestrator.get_db().is_sqlite();

    let res = if is_sqlite {
        sqlx::query!(
            "INSERT INTO aiaas_workflows (id, tenant_id, name, task, workflow_type, status, command, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            workflow.id, workflow.tenant_id, workflow.name, workflow.task, workflow.workflow_type, workflow.status, workflow.command, workflow.created_at
        )
        .execute(&pool)
        .await
    } else {
        sqlx::query!(
            "INSERT INTO aiaas_workflows (id, tenant_id, name, task, workflow_type, status, command, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            workflow.id, workflow.tenant_id, workflow.name, workflow.task, workflow.workflow_type, workflow.status, workflow.command, workflow.created_at
        )
        .execute(&pool)
        .await
    };

    if res.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Database error" }))).into_response();
    }

    // Spawn async task to run the actual command and update database
    tokio::spawn(async move {
        let output = Command::new(&agent_binary)
            .arg("--task")
            .arg(&agent_task)
            .output()
            .await;

        let (final_status, out_str, err_str) = match output {
            Ok(o) => {
                let status = if o.status.success() { "completed" } else { "failed" };
                (
                    status.to_string(),
                    String::from_utf8_lossy(&o.stdout).into_owned().trim().to_string(),
                    String::from_utf8_lossy(&o.stderr).into_owned().trim().to_string(),
                )
            }
            Err(e) => (
                "failed".to_string(),
                "".to_string(),
                format!("Failed to start process: {}", e),
            ),
        };

        let err_str_opt = if err_str.is_empty() { None } else { Some(err_str) };
        let out_str_opt = if out_str.is_empty() { None } else { Some(out_str) };

        if is_sqlite {
            let _ = sqlx::query!(
                "UPDATE aiaas_workflows SET status = ?, output = ?, error = ? WHERE id = ?",
                final_status, out_str_opt, err_str_opt, workflow_id
            )
            .execute(&pool)
            .await;
        } else {
            let _ = sqlx::query!(
                "UPDATE aiaas_workflows SET status = $1, output = $2, error = $3 WHERE id = $4",
                final_status, out_str_opt, err_str_opt, workflow_id
            )
            .execute(&pool)
            .await;
        }
    });

    (StatusCode::ACCEPTED, Json(CreateWorkflowResponse { workflow })).into_response()
}
