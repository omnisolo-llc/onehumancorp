use super::models::{self, Contact, Conversation, Inbox, Message};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use uuid::Uuid;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
}

#[derive(Deserialize)]
pub struct CreateInboxRequest {
    pub name: String,
}

pub struct ChatService;

impl ChatService {
    pub async fn create_inbox(db: &DatabaseConnection, tenant_id: Uuid, name: String) -> Result<Inbox, sea_orm::DbErr> {
        let now = Utc::now();
        let new_inbox = models::ActiveModel {
            id: Set(Uuid::new_v4()),
            tenant_id: Set(tenant_id),
            name: Set(name),
            created_at: Set(now),
            updated_at: Set(now),
        };
        new_inbox.insert(db).await
    }

    pub async fn create_contact(
        tenant_id: Uuid,
        name: String,
        email: Option<String>,
        phone_number: Option<String>,
        avatar_url: Option<String>,
    ) -> Result<Contact, String> {
        let now = Utc::now();
        Ok(Contact {
            id: Uuid::new_v4(),
            tenant_id,
            name,
            email,
            phone_number,
            avatar_url,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn create_conversation(
        tenant_id: Uuid,
        inbox_id: Uuid,
        contact_id: Uuid,
    ) -> Result<Conversation, String> {
        let now = Utc::now();
        Ok(Conversation {
            id: Uuid::new_v4(),
            tenant_id,
            inbox_id,
            contact_id,
            assignee_id: None,
            status: "open".to_string(),
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn send_message(
        tenant_id: Uuid,
        conversation_id: Uuid,
        sender_type: String,
        sender_id: Option<Uuid>,
        content: String,
        message_type: String,
    ) -> Result<Message, String> {
        let now = Utc::now();
        let message = Message {
            id: Uuid::new_v4(),
            tenant_id,
            conversation_id,
            sender_type,
            sender_id,
            content,
            message_type,
            created_at: now,
        };

        // In a real application, we would save to the DB and broadcast via ChatHub
        Ok(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_inbox() {
        use sea_orm::MockDatabase;
        let tenant_id = Uuid::new_v4();
        let db = MockDatabase::new(sea_orm::DatabaseBackend::Postgres)
            .append_query_results([
                vec![
                    models::Model {
                        id: Uuid::new_v4(),
                        tenant_id,
                        name: "Test Inbox".to_string(),
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                    }
                ]
            ])
            .into_connection();
        let res = ChatService::create_inbox(&db, tenant_id, "Test Inbox".to_string()).await;
        assert!(res.is_ok());
        let inbox = res.unwrap();
        assert_eq!(inbox.name, "Test Inbox");
        assert_eq!(inbox.tenant_id, tenant_id);
    }

    #[tokio::test]
    async fn test_create_contact() {
        let tenant_id = Uuid::new_v4();
        let res = ChatService::create_contact(
            tenant_id,
            "Jane Doe".to_string(),
            Some("jane@example.com".to_string()),
            None,
            None,
        )
        .await;
        assert!(res.is_ok());
        let contact = res.unwrap();
        assert_eq!(contact.name, "Jane Doe");
        assert_eq!(contact.email.unwrap(), "jane@example.com");
        assert_eq!(contact.tenant_id, tenant_id);
    }

    #[tokio::test]
    async fn test_create_conversation() {
        let tenant_id = Uuid::new_v4();
        let inbox_id = Uuid::new_v4();
        let contact_id = Uuid::new_v4();
        let res = ChatService::create_conversation(tenant_id, inbox_id, contact_id).await;
        assert!(res.is_ok());
        let conversation = res.unwrap();
        assert_eq!(conversation.inbox_id, inbox_id);
        assert_eq!(conversation.contact_id, contact_id);
        assert_eq!(conversation.tenant_id, tenant_id);
        assert_eq!(conversation.status, "open");
    }

    #[tokio::test]
    async fn test_send_message() {
        let tenant_id = Uuid::new_v4();
        let conversation_id = Uuid::new_v4();
        let res = ChatService::send_message(
            tenant_id,
            conversation_id,
            "Agent".to_string(),
            None,
            "Hello, how can I help you?".to_string(),
            "outgoing".to_string(),
        )
        .await;
        assert!(res.is_ok());
        let message = res.unwrap();
        assert_eq!(message.conversation_id, conversation_id);
        assert_eq!(message.content, "Hello, how can I help you?");
        assert_eq!(message.message_type, "outgoing");
        assert_eq!(message.tenant_id, tenant_id);
    }
}
