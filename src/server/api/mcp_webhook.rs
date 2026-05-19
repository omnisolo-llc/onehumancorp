use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::{StatusCode, HeaderMap},
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use crate::hub::Hub;
use crate::integrations::mcp::mcp_async::AsyncTaskTracker;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct McpWebhookPayload {
    pub task_id: Uuid,
    pub status: String,
    pub result: Option<Value>,
}

#[derive(Debug, Serialize)]
pub struct McpWebhookResponse {
    pub success: bool,
    pub message: String,
}

pub async fn handle_mcp_webhook(
    State(hub): State<Arc<Hub>>,
    headers: HeaderMap,
    Json(payload): Json<McpWebhookPayload>,
) -> impl IntoResponse {
    // Basic Bearer token verification.
    // In a real implementation, you would check against a specific integration token
    // or verify an HMAC signature of the payload.
    let expected_token = std::env::var("MCP_WEBHOOK_SECRET").unwrap_or_else(|_| "secret-token".to_string());

    let auth_header = headers.get("Authorization").and_then(|v| v.to_str().ok());

    if auth_header != Some(&format!("Bearer {}", expected_token)) {
        tracing::warn!("Unauthorized MCP webhook access attempt");
        return (
            StatusCode::UNAUTHORIZED,
            Json(McpWebhookResponse {
                success: false,
                message: "Unauthorized".to_string(),
            }),
        );
    }

    let tracker = AsyncTaskTracker::new(Arc::new(hub.db.clone()));

    match tracker.get_task(payload.task_id).await {
        Ok(Some(task)) => {
            let task_uuid = payload.task_id;
            if let Err(e) = tracker.update_task_status(task_uuid, &payload.status, payload.result).await {
                tracing::error!(task_uuid = %task_uuid, "Failed to update MCP task: {}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(McpWebhookResponse {
                        success: false,
                        message: "Failed to update task status".to_string(),
                    }),
                );
            }

            // In a real KAIROS implementation, this would trigger agent resumption via the orchestrator.
            // For now, we simulate reactivating the agent.
            let agent_uuid = task.agent_id;
            let tenant_uuid = task.tenant_id;
            let task_record_uuid = task.id;
            tracing::info!(agent_uuid = %agent_uuid, tenant_uuid = %tenant_uuid, task_uuid = %task_record_uuid, "KAIROS Hook: Reactivating agent");

            (
                StatusCode::OK,
                Json(McpWebhookResponse {
                    success: true,
                    message: "Task updated and agent reactivated".to_string(),
                }),
            )
        }
        Ok(None) => {
            (
                StatusCode::NOT_FOUND,
                Json(McpWebhookResponse {
                    success: false,
                    message: "Task not found".to_string(),
                }),
            )
        }
        Err(e) => {
            let task_uuid = payload.task_id;
            tracing::error!(task_uuid = %task_uuid, "Database error fetching MCP task: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(McpWebhookResponse {
                    success: false,
                    message: "Database error".to_string(),
                }),
            )
        }
    }
}
