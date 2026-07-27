use crate::domain::chat::{Conversation, Message};
use uuid::Uuid;

pub async fn create_message(tenant_id: Uuid, conversation_id: Uuid, payload: String) -> Result<Message, String> {
    Ok(Message {
        id: Uuid::new_v4(),
        tenant_id,
        conversation_id,
        payload,
    })
}

pub async fn fetch_conversations(tenant_id: Uuid, inbox_id: Uuid) -> Result<Vec<Conversation>, String> {
    Ok(vec![])
}
mod tests;
