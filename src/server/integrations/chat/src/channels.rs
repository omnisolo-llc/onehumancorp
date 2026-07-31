use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::service::ChatService;
use super::ai_agent::AmbassadorAgent;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IncomingMessage {
    pub sender_id: Option<Uuid>,
    pub content: String,
}

#[async_trait]
pub trait ChannelAdapter: Send + Sync {
    async fn receive_message(&self, tenant_id: Uuid, conversation_id: Uuid, message: IncomingMessage) -> Result<(), String>;
}

pub struct DummyWebWidgetAdapter {
    chat_service: Arc<ChatService>,
    ai_agent: Arc<AmbassadorAgent>,
}

impl DummyWebWidgetAdapter {
    pub fn new(chat_service: Arc<ChatService>, ai_agent: Arc<AmbassadorAgent>) -> Self {
        Self { chat_service, ai_agent }
    }
}

#[async_trait]
impl ChannelAdapter for DummyWebWidgetAdapter {
    async fn receive_message(&self, tenant_id: Uuid, conversation_id: Uuid, message: IncomingMessage) -> Result<(), String> {
        self.chat_service
            .save_message(
                tenant_id,
                conversation_id,
                "contact",
                message.sender_id,
                &message.content,
            )
            .await
            .map_err(|e| e.to_string())?;

        // Broadcast to AI agent to generate draft
        let _ = self.ai_agent.draft_reply(tenant_id, conversation_id, &message.content).await;

        Ok(())
    }
}
