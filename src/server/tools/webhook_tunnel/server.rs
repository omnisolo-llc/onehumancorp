use ::server_ohc::orchestration::{McpInvokeRequest, McpInvokeResponse, McpToolProto};
use std::sync::Arc;
use sqlx::PgPool;
use tracing::Instrument;

pub struct WebhookTunnelMcpServer {
    pool: Arc<PgPool>,
}

impl WebhookTunnelMcpServer {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub fn get_tools(&self) -> Vec<McpToolProto> {
        vec![
            McpToolProto {
                id: "webhook_forward".to_string(),
                name: "Webhook Forward".to_string(),
                description: "Forward a webhook payload. Input schema: {\"type\":\"object\",\"properties\":{\"payload\":{\"type\":\"string\"}}}".to_string(),
                category: "webhook".to_string(),
                status: "active".to_string(),
            },
        ]
    }

    pub async fn invoke_tool(&self, req: &McpInvokeRequest) -> Result<McpInvokeResponse, tonic::Status> {
        let params: serde_json::Value = serde_json::from_str(&req.params)
            .map_err(|e| tonic::Status::invalid_argument(format!("invalid JSON params: {}", e)))?;

        match req.tool_id.as_str() {
            "webhook_forward" => {
                let payload_str = params.as_str().unwrap_or(&req.params);

                let id = uuid::Uuid::new_v4().to_string();
                let spiffe_id_str = req.spiffe_id.clone();
                let parsed = ::server_auth::parse_spiffe_id(&spiffe_id_str).map_err(|_| tonic::Status::unauthenticated("invalid spiffe id"))?;
                let tenant_id = parsed.0;

                let event_type = "webhook_received";

                async {
                    let query = "
                        INSERT INTO agent_event_bus (id, tenant_id, event_type, payload, status, created_at, updated_at)
                        VALUES ($1, $2, $3, $4, 'PENDING', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
                    ";

                    sqlx::query(query)
                        .bind(&id)
                        .bind(&tenant_id)
                        .bind(event_type)
                        .bind(payload_str)
                        .execute(&*self.pool)
                        .await
                        .map_err(|e| tonic::Status::internal(format!("failed to enqueue webhook event: {}", e)))?;

                    let resp = serde_json::json!({"status": "success", "event_id": id});
                    Ok(McpInvokeResponse {
                        payload: serde_json::to_string(&resp).unwrap(),
                    })
                }.instrument(tracing::info_span!("webhook_forward")).await
            }
            _ => Err(tonic::Status::not_found(format!("tool {} not implemented", req.tool_id))),
        }
    }
}
