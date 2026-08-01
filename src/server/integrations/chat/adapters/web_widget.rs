use std::sync::Arc;
use uuid::Uuid;

use crate::domain::ChatMessage;
use crate::services::ChatService;

pub struct WebWidgetAdapter {
    chat_service: Arc<ChatService>,
}

impl WebWidgetAdapter {
    pub fn new(chat_service: Arc<ChatService>) -> Self {
        Self { chat_service }
    }

    pub async fn handle_incoming_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        content: String,
        sender_id: Uuid,
    ) -> Result<ChatMessage, sqlx::Error> {
        let msg = self.chat_service.send_message(
            tenant_id,
            conversation_id,
            "contact".to_string(),
            Some(sender_id),
            content,
        ).await?;

        Ok(msg)
    }
}
