use axum::{routing::post, Router, Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use tracing::{instrument, error};
use std::sync::Arc;
use ohc_builtin_agent::mesh::transport::MeshTransport;
use chrono::Utc;
use ohc_builtin_agent::proto::hub::TeammateMeshEvent;

#[derive(Debug, Deserialize)]
pub struct ManychatWebhookPayload {
    pub subscriber_id: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct WebhookResponse {
    pub success: bool,
}

#[instrument(skip(payload, transport))]
pub async fn handle_manychat_webhook(
    State(transport): State<Arc<dyn MeshTransport>>,
    Json(payload): Json<ManychatWebhookPayload>,
) -> Result<Json<WebhookResponse>, (StatusCode, String)> {
    let subscriber = payload.subscriber_id;
    let content = payload.message;

    let payload_json = serde_json::json!({
        "subscriber_id": subscriber,
        "content": content,
    });
    let payload_bytes = serde_json::to_vec(&payload_json).unwrap_or_default();

    let event = TeammateMeshEvent {
        agent_id: "system".to_string(),
        action: "manychat_message".to_string(),
        status: "".to_string(),
        payload: payload_bytes,
        msg_id: format!("manychat-{}-{}", subscriber, Utc::now().timestamp_millis()),
    };

    match transport.publish("system", event).await {
        Ok(_) => Ok(Json(WebhookResponse { success: true })),
        Err(e) => {
            error!("Failed to publish manychat message: {}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()))
        }
    }
}

pub fn manychat_router() -> Router<Arc<dyn MeshTransport>> {
    Router::new().route("/webhook", post(handle_manychat_webhook))
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use ohc_builtin_agent::proto::hub::TeammateMeshEvent as Message;

    struct MockTransport {
        fail: bool,
    }

    #[async_trait]
    impl MeshTransport for MockTransport {
        async fn publish(&self, _topic: &str, _message: Message) -> Result<(), String> {
            if self.fail { Err("failed".to_string()) } else { Ok(()) }
        }
        async fn subscribe(&self, _topic: &str, _handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
            unimplemented!()
        }
        async fn acquire_lock(&self, _resource: &str, _owner: &str, _ttl_seconds: u64) -> Result<bool, String> {
            unimplemented!()
        }
        async fn release_lock(&self, _resource: &str, _owner: &str) -> Result<(), String> {
            unimplemented!()
        }
        async fn register_presence(&self, _agent_id: &str, _status: &str, _ttl_seconds: u64) -> Result<(), String> {
            unimplemented!()
        }
        async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
            unimplemented!()
        }
    }

    #[tokio::test]
    async fn test_manychat_webhook_success() {
        let transport: Arc<dyn MeshTransport> = Arc::new(MockTransport { fail: false });
        let payload = Json(ManychatWebhookPayload {
            subscriber_id: "123".to_string(),
            message: "Hello".to_string(),
        });

        let res = handle_manychat_webhook(State(transport), payload).await;
        assert!(res.is_ok());
        assert!(res.unwrap().success);
    }

    #[tokio::test]
    async fn test_manychat_webhook_failure() {
        let transport: Arc<dyn MeshTransport> = Arc::new(MockTransport { fail: true });
        let payload = Json(ManychatWebhookPayload {
            subscriber_id: "123".to_string(),
            message: "Hello".to_string(),
        });

        let res = handle_manychat_webhook(State(transport), payload).await;
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().0, StatusCode::INTERNAL_SERVER_ERROR);
    }
}
