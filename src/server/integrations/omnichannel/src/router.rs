use crate::models::{contact, conversation, message};
use crate::repository::ChatRepository;
use crate::state_machine::{ConversationEvent, ConversationState, ConversationStateMachine};
use chrono::Utc;
use uuid::Uuid;

pub struct MessageRouter<R: ChatRepository> {
    repository: R,
}

impl<R: ChatRepository> MessageRouter<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn route_incoming_message(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        channel_identity: &str,
        content: &str,
    ) -> Result<message::Model, String> {
        // 1. Resolve contact
        let contact = match self
            .repository
            .find_contact_by_identity(tenant_id, channel_identity)
            .await?
        {
            Some(c) => c,
            None => {
                let new_contact = contact::Model {
                    id: Uuid::new_v4(),
                    tenant_id,
                    name: "Unknown".to_string(),
                    email: None,
                    phone: None,
                    channel_identity: channel_identity.to_string(),
                    created_at: Utc::now().into(),
                    updated_at: Utc::now().into(),
                };
                self.repository.create_contact(new_contact).await?
            }
        };

        // 2. Resolve conversation
        let mut conversation = match self
            .repository
            .find_active_conversation(tenant_id, contact.id)
            .await?
        {
            Some(conv) => conv,
            None => {
                let new_conv = conversation::Model {
                    id: Uuid::new_v4(),
                    tenant_id,
                    inbox_id,
                    contact_id: contact.id,
                    assignee_id: None,
                    status: "open".to_string(),
                    created_at: Utc::now().into(),
                    updated_at: Utc::now().into(),
                };
                self.repository.create_conversation(new_conv).await?
            }
        };

        // 3. Process state transition
        if let Ok(current_state) = conversation.status.parse::<ConversationState>() {
            if let Ok(new_state) = ConversationStateMachine::transition(
                &current_state,
                &ConversationEvent::IncomingMessage,
            ) {
                if new_state != current_state {
                    conversation.status = new_state.to_string();
                    conversation.updated_at = Utc::now().into();
                    let _ = self
                        .repository
                        .update_conversation(conversation.clone())
                        .await;
                }
            }
        }

        // 4. Create message
        let new_msg = message::Model {
            id: Uuid::new_v4(),
            tenant_id,
            conversation_id: conversation.id,
            sender_type: "contact".to_string(),
            sender_id: Some(contact.id),
            content_type: "text".to_string(),
            content: content.to_string(),
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
        };

        self.repository.create_message(new_msg).await
    }
}
