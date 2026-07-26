use sqlx::PgPool;
use uuid::Uuid;
use super::models::{ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage};

pub struct ChatService {
    pool: PgPool,
}

impl ChatService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_inbox(
        &self,
        tenant_id: Uuid,
        name: String,
    ) -> Result<ChatInbox, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_inboxes (id, tenant_id, name)
            VALUES ($1, $2, $3)
            RETURNING id, tenant_id, name, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_channel(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        channel_type: String,
        config: serde_json::Value,
    ) -> Result<ChatChannel, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_channels (id, tenant_id, inbox_id, channel_type, config)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, inbox_id, channel_type, config, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(channel_type)
        .bind(config)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_contact(
        &self,
        tenant_id: Uuid,
        name: Option<String>,
        email: Option<String>,
        phone: Option<String>,
    ) -> Result<ChatContact, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_contacts (id, tenant_id, name, email, phone)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, name, email, phone, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
        .bind(email)
        .bind(phone)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn start_conversation(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        contact_id: Uuid,
        assignee_id: Option<Uuid>,
    ) -> Result<ChatConversation, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, assignee_id, status)
            VALUES ($1, $2, $3, $4, $5, 'open')
            RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .bind(assignee_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn send_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        sender_type: String,
        sender_id: Option<Uuid>,
        content: String,
    ) -> Result<ChatMessage, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, tenant_id, conversation_id, sender_type, sender_id, content, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(sender_type)
        .bind(sender_id)
        .bind(content)
        .fetch_one(&self.pool)
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use serde_json::json;

    #[test]
    fn test_chat_models_serialization_deserialization() {
        let inbox = ChatInbox {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            name: "Main Inbox".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let serialized = serde_json::to_string(&inbox).unwrap();
        let deserialized: ChatInbox = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.name, "Main Inbox");

        let channel = ChatChannel {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            inbox_id: Uuid::new_v4(),
            channel_type: "whatsapp".to_string(),
            config: json!({"phone_number": "+123456789"}),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let channel_serialized = serde_json::to_string(&channel).unwrap();
        let channel_deserialized: ChatChannel = serde_json::from_str(&channel_serialized).unwrap();
        assert_eq!(channel_deserialized.channel_type, "whatsapp");
        assert_eq!(channel_deserialized.config["phone_number"], "+123456789");

        let contact = ChatContact {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            name: Some("John Doe".to_string()),
            email: Some("john@example.com".to_string()),
            phone: Some("+123456".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let contact_serialized = serde_json::to_string(&contact).unwrap();
        let contact_deserialized: ChatContact = serde_json::from_str(&contact_serialized).unwrap();
        assert_eq!(contact_deserialized.name.as_deref(), Some("John Doe"));

        let conv = ChatConversation {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            inbox_id: Uuid::new_v4(),
            contact_id: Uuid::new_v4(),
            assignee_id: Some(Uuid::new_v4()),
            status: "open".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let conv_serialized = serde_json::to_string(&conv).unwrap();
        let conv_deserialized: ChatConversation = serde_json::from_str(&conv_serialized).unwrap();
        assert_eq!(conv_deserialized.status, "open");

        let msg = ChatMessage {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            conversation_id: Uuid::new_v4(),
            sender_type: "contact".to_string(),
            sender_id: Some(Uuid::new_v4()),
            content: "Hello!".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let msg_serialized = serde_json::to_string(&msg).unwrap();
        let msg_deserialized: ChatMessage = serde_json::from_str(&msg_serialized).unwrap();
        assert_eq!(msg_deserialized.content, "Hello!");
    }
}
