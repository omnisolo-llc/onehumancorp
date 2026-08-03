use std::sync::Arc;
use crate::provider::{ChatProvider, ChatMessage};
use uuid::Uuid;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct WebhookPayload {
    pub message: String,
    pub sender_id: String,
    pub channel: String,
}

pub struct WebhookState<P: ChatProvider> {
    pub provider: Arc<P>,
    pub tenant_id: Uuid,
}

pub async fn handle_meta_webhook<P: ChatProvider>(
    state: Arc<WebhookState<P>>,
    channel_id: String,
    payload: WebhookPayload,
) -> String {
    let msg = ChatMessage {
        id: Uuid::new_v4().to_string(),
        tenant_id: state.tenant_id.clone(),
        channel_id,
        contact_id: payload.sender_id,
        content: payload.message,
        role: "user".to_string(),
        created_at: chrono::Utc::now().timestamp(),
    };

    if let Err(e) = state.provider.receive_message(msg).await {
        tracing::error!("Failed to process webhook: {}", e);
        return "Error".to_string();
    }

    "OK".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::InMemoryChatProvider;

    #[tokio::test]
    async fn test_webhook() {
        let provider = Arc::new(InMemoryChatProvider::new());
        let tenant_id = Uuid::new_v4();
        let state = Arc::new(WebhookState {
            provider: provider.clone(),
            tenant_id,
        });

        let payload = WebhookPayload {
            message: "Test message".to_string(),
            sender_id: "user123".to_string(),
            channel: "whatsapp".to_string(),
        };

        let result = handle_meta_webhook(state, "whatsapp-channel".to_string(), payload).await;

        assert_eq!(result, "OK");

        let msgs = provider.messages.lock().await;
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0].content, "Test message");
        assert_eq!(msgs[0].contact_id, "user123");
    }
}
