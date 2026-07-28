use sqlx::{FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use crate::db::DB;

#[derive(Clone, Debug, FromRow, PartialEq)]
pub struct Inbox {
    pub id: Uuid,
    pub tenant_id: String,
    pub name: String,
    pub channel_type: String,
    pub enable_auto_assignment: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow, PartialEq)]
pub struct Contact {
    pub id: Uuid,
    pub tenant_id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub contact_type: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow, PartialEq)]
pub struct NativeConversation {
    pub id: Uuid,
    pub tenant_id: String,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub status: String,
    pub assignee_id: Option<Uuid>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow, PartialEq)]
pub struct NativeMessage {
    pub id: Uuid,
    pub tenant_id: String,
    pub conversation_id: Uuid,
    pub content: Option<String>,
    pub message_type: String,
    pub private: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct NativeChatRepo {
    db: Arc<DB>,
}

impl NativeChatRepo {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn create_inbox(&self, tenant_id: String, name: String, channel_type: String, enable_auto_assignment: bool) -> Result<Inbox, sqlx::Error> {
        let id = Uuid::new_v4();
        let mut tx = self.db.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&tenant_id)
            .execute(&mut *tx)
            .await?;

        let record = sqlx::query_as::<_, Inbox>(
            "INSERT INTO inboxes (id, tenant_id, name, channel_type, enable_auto_assignment) VALUES ($1, $2, $3, $4, $5) RETURNING id, tenant_id, name, channel_type, enable_auto_assignment, created_at, updated_at",
        )
        .bind(id)
        .bind(&tenant_id)
        .bind(name)
        .bind(channel_type)
        .bind(enable_auto_assignment)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(record)
    }

    pub async fn create_contact(&self, tenant_id: String, name: Option<String>, email: Option<String>, phone_number: Option<String>, contact_type: Option<String>) -> Result<Contact, sqlx::Error> {
        let id = Uuid::new_v4();
        let mut tx = self.db.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&tenant_id)
            .execute(&mut *tx)
            .await?;

        let record = sqlx::query_as::<_, Contact>(
            "INSERT INTO contacts (id, tenant_id, name, email, phone_number, contact_type) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, tenant_id, name, email, phone_number, contact_type, created_at, updated_at",
        )
        .bind(id)
        .bind(&tenant_id)
        .bind(name)
        .bind(email)
        .bind(phone_number)
        .bind(contact_type)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(record)
    }

    pub async fn create_conversation(&self, tenant_id: String, inbox_id: Uuid, contact_id: Uuid, status: String, assignee_id: Option<Uuid>) -> Result<NativeConversation, sqlx::Error> {
        let id = Uuid::new_v4();
        let mut tx = self.db.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&tenant_id)
            .execute(&mut *tx)
            .await?;

        let record = sqlx::query_as::<_, NativeConversation>(
            "INSERT INTO conversations (id, tenant_id, inbox_id, contact_id, status, assignee_id) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, tenant_id, inbox_id, contact_id, status, assignee_id, created_at, updated_at",
        )
        .bind(id)
        .bind(&tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .bind(status)
        .bind(assignee_id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(record)
    }

    pub async fn create_message(&self, tenant_id: String, conversation_id: Uuid, content: Option<String>, message_type: String, private: bool) -> Result<NativeMessage, sqlx::Error> {
        let id = Uuid::new_v4();
        let mut tx = self.db.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&tenant_id)
            .execute(&mut *tx)
            .await?;

        let record = sqlx::query_as::<_, NativeMessage>(
            "INSERT INTO messages (id, tenant_id, conversation_id, content, message_type, private) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, tenant_id, conversation_id, content, message_type, private, created_at, updated_at",
        )
        .bind(id)
        .bind(&tenant_id)
        .bind(conversation_id)
        .bind(content)
        .bind(message_type)
        .bind(private)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(record)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use crate::auth::postgres_test_support::postgres_security_pool;

    #[test]
    fn test_inbox_struct() {
        let inbox = Inbox {
            id: Uuid::new_v4(),
            tenant_id: "test_tenant".to_string(),
            name: "Test Inbox".to_string(),
            channel_type: "web_widget".to_string(),
            enable_auto_assignment: Some(true),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(inbox.name, "Test Inbox");
        assert_eq!(inbox.channel_type, "web_widget");
        assert_eq!(inbox.enable_auto_assignment, Some(true));
    }

    #[test]
    fn test_contact_struct() {
        let contact = Contact {
            id: Uuid::new_v4(),
            tenant_id: "test_tenant".to_string(),
            name: Some("Test Contact".to_string()),
            email: Some("test@example.com".to_string()),
            phone_number: Some("+1234567890".to_string()),
            contact_type: Some("visitor".to_string()),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(contact.name, Some("Test Contact".to_string()));
        assert_eq!(contact.email, Some("test@example.com".to_string()));
    }

    #[test]
    fn test_native_conversation_struct() {
        let conversation = NativeConversation {
            id: Uuid::new_v4(),
            tenant_id: "test_tenant".to_string(),
            inbox_id: Uuid::new_v4(),
            contact_id: Uuid::new_v4(),
            status: "open".to_string(),
            assignee_id: None,
            created_at: None,
            updated_at: None,
        };
        assert_eq!(conversation.status, "open");
        assert_eq!(conversation.assignee_id, None);
    }

    #[test]
    fn test_native_message_struct() {
        let message = NativeMessage {
            id: Uuid::new_v4(),
            tenant_id: "test_tenant".to_string(),
            conversation_id: Uuid::new_v4(),
            content: Some("Test message".to_string()),
            message_type: "incoming".to_string(),
            private: Some(false),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(message.content, Some("Test message".to_string()));
        assert_eq!(message.message_type, "incoming");
        assert_eq!(message.private, Some(false));
    }

    #[tokio::test]
    async fn test_db_operations() {
        let pool = postgres_security_pool(1).await;
        if let Some(pool) = pool {
            let db = Arc::new(DB { pool });
            let repo = NativeChatRepo::new(db.clone());
            let tenant_id = Uuid::new_v4().to_string();

            let inbox = repo.create_inbox(tenant_id.clone(), "Test Inbox".to_string(), "web".to_string(), true).await.expect("Failed to create inbox");
            assert_eq!(inbox.name, "Test Inbox");

            let contact = repo.create_contact(tenant_id.clone(), Some("Bob".to_string()), None, None, None).await.expect("Failed to create contact");
            assert_eq!(contact.name.as_deref(), Some("Bob"));

            let conversation = repo.create_conversation(tenant_id.clone(), inbox.id, contact.id, "open".to_string(), None).await.expect("Failed to create conv");
            assert_eq!(conversation.status, "open");

            let msg = repo.create_message(tenant_id.clone(), conversation.id, Some("Hi".to_string()), "incoming".to_string(), false).await.expect("Failed to create message");
            assert_eq!(msg.content.as_deref(), Some("Hi"));
        }
    }
}
