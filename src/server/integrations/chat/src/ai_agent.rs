use uuid::Uuid;
use super::service::ChatService;
use std::sync::Arc;

pub struct AmbassadorAgent {
    chat_service: Arc<ChatService>,
}

impl AmbassadorAgent {
    pub fn new(chat_service: Arc<ChatService>) -> Self {
        Self { chat_service }
    }

    pub async fn draft_reply(&self, tenant_id: Uuid, conversation_id: Uuid, incoming_content: &str) -> Result<(), String> {
        let draft_content = format!("AI Draft Reply to: {}", incoming_content);

        self.chat_service
            .save_message(
                tenant_id,
                conversation_id,
                "bot", // Sender type bot
                None, // System bot has no sender id usually
                &draft_content,
            )
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
