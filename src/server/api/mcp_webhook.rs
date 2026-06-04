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
    let t_id = payload.task_id;

    match tracker.get_task(t_id).await {
        Ok(Some(task)) => {
            if let Err(e) = tracker.update_task_status(t_id, &payload.status, payload.result).await {
                ::server_telemetry::record_error_signal("Failed to update MCP task ");
                tracing::error!("Failed to update MCP task {}: {}", t_id, e);
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
            let tnt_id = task.tenant_id;
            tracing::info!("KAIROS Hook: Reactivating agent {} for org {} (Task {})", task.agent_id, tnt_id, task.id);

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
            ::server_telemetry::record_error_signal("Database error fetching MCP task ");
            tracing::error!("Database error fetching MCP task {}: {}", t_id, e);
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
