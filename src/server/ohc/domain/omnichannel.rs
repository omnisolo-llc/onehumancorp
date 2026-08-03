use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::{FromRow, PgPool, Error};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq, Eq)]
pub struct Inbox {
    pub tenant_id: Uuid,
    pub id: Uuid,
    pub name: String,
    pub channel_type: String,
    pub auto_assignment_config: serde_json::Value,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq, Eq)]
pub struct Contact {
    pub tenant_id: Uuid,
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone_number: String,
    pub identifier: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq, Eq)]
pub struct Conversation {
    pub tenant_id: Uuid,
    pub id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub status: String,
    pub assignee_id: Option<Uuid>,
    pub unread_count: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq, Eq)]
pub struct Message {
    pub tenant_id: Uuid,
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub inbox_id: Uuid,
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
    pub content_type: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[async_trait::async_trait]
pub trait OmniChannelChatRepository {
    async fn create_inbox(&self, inbox: &Inbox) -> Result<(), Error>;
    async fn get_inbox(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<Inbox>, Error>;
    async fn list_inboxes(&self, tenant_id: Uuid) -> Result<Vec<Inbox>, Error>;

    async fn create_contact(&self, contact: &Contact) -> Result<(), Error>;
    async fn get_contact(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<Contact>, Error>;

    async fn create_conversation(&self, conversation: &Conversation) -> Result<(), Error>;
    async fn get_conversation(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<Conversation>, Error>;
    async fn list_conversations(&self, tenant_id: Uuid) -> Result<Vec<Conversation>, Error>;

    async fn create_message(&self, message: &Message) -> Result<(), Error>;
    async fn get_message(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<Message>, Error>;
    async fn list_messages(&self, tenant_id: Uuid, conversation_id: Uuid) -> Result<Vec<Message>, Error>;
}

pub struct SqlxOmniChannelChatRepository {
    pool: PgPool,
}

impl SqlxOmniChannelChatRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl OmniChannelChatRepository for SqlxOmniChannelChatRepository {
    async fn create_inbox(&self, inbox: &Inbox) -> Result<(), Error> {
        sqlx::query(
            r#"
            INSERT INTO omni_inboxes (id, tenant_id, name, channel_type, auto_assignment_config, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, COALESCE($6, NOW()), COALESCE($7, NOW()))
            "#,
        )
        .bind(inbox.id)
        .bind(inbox.tenant_id)
        .bind(&inbox.name)
        .bind(&inbox.channel_type)
        .bind(&inbox.auto_assignment_config)
        .bind(inbox.created_at)
        .bind(inbox.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_inbox(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<Inbox>, Error> {
        sqlx::query_as::<_, Inbox>(
            r#"
            SELECT id, tenant_id, name, channel_type, auto_assignment_config, created_at, updated_at
            FROM omni_inboxes
            WHERE id = $1 AND tenant_id = $2
            "#,
        )
        .bind(id)
        .bind(tenant_id)
        .fetch_optional(&self.pool)
        .await
    }

    async fn list_inboxes(&self, tenant_id: Uuid) -> Result<Vec<Inbox>, Error> {
        sqlx::query_as::<_, Inbox>(
            r#"
            SELECT id, tenant_id, name, channel_type, auto_assignment_config, created_at, updated_at
            FROM omni_inboxes
            WHERE tenant_id = $1
            "#,
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await
    }

    async fn create_contact(&self, contact: &Contact) -> Result<(), Error> {
        sqlx::query(
            r#"
            INSERT INTO omni_contacts (id, tenant_id, name, email, phone_number, identifier, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, COALESCE($7, NOW()), COALESCE($8, NOW()))
            "#,
        )
        .bind(contact.id)
        .bind(contact.tenant_id)
        .bind(&contact.name)
        .bind(&contact.email)
        .bind(&contact.phone_number)
        .bind(&contact.identifier)
        .bind(contact.created_at)
        .bind(contact.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_contact(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<Contact>, Error> {
        sqlx::query_as::<_, Contact>(
            r#"
            SELECT id, tenant_id, name, email, phone_number, identifier, created_at, updated_at
            FROM omni_contacts
            WHERE id = $1 AND tenant_id = $2
            "#,
        )
        .bind(id)
        .bind(tenant_id)
        .fetch_optional(&self.pool)
        .await
    }

    async fn create_conversation(&self, conversation: &Conversation) -> Result<(), Error> {
        sqlx::query(
            r#"
            INSERT INTO omni_conversations (id, tenant_id, inbox_id, contact_id, status, assignee_id, unread_count, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, COALESCE($8, NOW()), COALESCE($9, NOW()))
            "#,
        )
        .bind(conversation.id)
        .bind(conversation.tenant_id)
        .bind(conversation.inbox_id)
        .bind(conversation.contact_id)
        .bind(&conversation.status)
        .bind(conversation.assignee_id)
        .bind(conversation.unread_count)
        .bind(conversation.created_at)
        .bind(conversation.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_conversation(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<Conversation>, Error> {
        sqlx::query_as::<_, Conversation>(
            r#"
            SELECT id, tenant_id, inbox_id, contact_id, status, assignee_id, unread_count, created_at, updated_at
            FROM omni_conversations
            WHERE id = $1 AND tenant_id = $2
            "#,
        )
        .bind(id)
        .bind(tenant_id)
        .fetch_optional(&self.pool)
        .await
    }

    async fn list_conversations(&self, tenant_id: Uuid) -> Result<Vec<Conversation>, Error> {
        sqlx::query_as::<_, Conversation>(
            r#"
            SELECT id, tenant_id, inbox_id, contact_id, status, assignee_id, unread_count, created_at, updated_at
            FROM omni_conversations
            WHERE tenant_id = $1
            "#,
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await
    }

    async fn create_message(&self, message: &Message) -> Result<(), Error> {
        sqlx::query(
            r#"
            INSERT INTO omni_messages (id, tenant_id, conversation_id, inbox_id, sender_type, sender_id, content, content_type, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, COALESCE($9, NOW()), COALESCE($10, NOW()))
            "#,
        )
        .bind(message.id)
        .bind(message.tenant_id)
        .bind(message.conversation_id)
        .bind(message.inbox_id)
        .bind(&message.sender_type)
        .bind(message.sender_id)
        .bind(&message.content)
        .bind(&message.content_type)
        .bind(message.created_at)
        .bind(message.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_message(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<Message>, Error> {
        sqlx::query_as::<_, Message>(
            r#"
            SELECT id, tenant_id, conversation_id, inbox_id, sender_type, sender_id, content, content_type, created_at, updated_at
            FROM omni_messages
            WHERE id = $1 AND tenant_id = $2
            "#,
        )
        .bind(id)
        .bind(tenant_id)
        .fetch_optional(&self.pool)
        .await
    }

    async fn list_messages(&self, tenant_id: Uuid, conversation_id: Uuid) -> Result<Vec<Message>, Error> {
        sqlx::query_as::<_, Message>(
            r#"
            SELECT id, tenant_id, conversation_id, inbox_id, sender_type, sender_id, content, content_type, created_at, updated_at
            FROM omni_messages
            WHERE conversation_id = $1 AND tenant_id = $2
            "#,
        )
        .bind(conversation_id)
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    use uuid::Uuid;

    async fn get_test_pool() -> Option<PgPool> {
        let database_url = std::env::var("DATABASE_URL")
            .or_else(|_| std::env::var("OHC_DATABASE_URL"))
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());

        PgPool::connect(&database_url).await.ok()
    }

    #[tokio::test]
    async fn test_omnichannel_chat_system_flow_and_tenant_isolation() {
        let Some(pool) = get_test_pool().await else {
            println!("PostgreSQL not running/available for omnichannel test. Skipping.");
            return;
        };

        let repo = SqlxOmniChannelChatRepository::new(pool.clone());

        let tenant_a = Uuid::new_v4();
        let tenant_b = Uuid::new_v4();

        // Ensure tables exist and clear them
        let _ = sqlx::query("DELETE FROM omni_messages").execute(&pool).await;
        let _ = sqlx::query("DELETE FROM omni_conversations").execute(&pool).await;
        let _ = sqlx::query("DELETE FROM omni_contacts").execute(&pool).await;
        let _ = sqlx::query("DELETE FROM omni_inboxes").execute(&pool).await;

        // 1. Set context for tenant A and create entities
        let mut tx_a = pool.begin().await.unwrap();
        sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)")
            .bind(tenant_a.to_string())
            .execute(&mut *tx_a)
            .await
            .unwrap();

        // Insert Inbox for tenant A
        let inbox_a = Inbox {
            tenant_id: tenant_a,
            id: Uuid::new_v4(),
            name: "Maya's Baker Inbox".to_string(),
            channel_type: "instagram".to_string(),
            auto_assignment_config: serde_json::json!({"enabled": true}),
            created_at: None,
            updated_at: None,
        };
        sqlx::query(
            r#"
            INSERT INTO omni_inboxes (id, tenant_id, name, channel_type, auto_assignment_config)
            VALUES ($1, $2, $3, $4, $5)
            "#
        )
        .bind(inbox_a.id)
        .bind(inbox_a.tenant_id)
        .bind(&inbox_a.name)
        .bind(&inbox_a.channel_type)
        .bind(&inbox_a.auto_assignment_config)
        .execute(&mut *tx_a)
        .await
        .unwrap();

        // Insert Contact for tenant A
        let contact_a = Contact {
            tenant_id: tenant_a,
            id: Uuid::new_v4(),
            name: "Sarah Baker Customer".to_string(),
            email: "sarah@example.com".to_string(),
            phone_number: "+12345678".to_string(),
            identifier: "sarah_insta_handle".to_string(),
            created_at: None,
            updated_at: None,
        };
        sqlx::query(
            r#"
            INSERT INTO omni_contacts (id, tenant_id, name, email, phone_number, identifier)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#
        )
        .bind(contact_a.id)
        .bind(contact_a.tenant_id)
        .bind(&contact_a.name)
        .bind(&contact_a.email)
        .bind(&contact_a.phone_number)
        .bind(&contact_a.identifier)
        .execute(&mut *tx_a)
        .await
        .unwrap();

        // Insert Conversation for tenant A
        let conv_a = Conversation {
            tenant_id: tenant_a,
            id: Uuid::new_v4(),
            inbox_id: inbox_a.id,
            contact_id: contact_a.id,
            status: "open".to_string(),
            assignee_id: None,
            unread_count: 0,
            created_at: None,
            updated_at: None,
        };
        sqlx::query(
            r#"
            INSERT INTO omni_conversations (id, tenant_id, inbox_id, contact_id, status, assignee_id, unread_count)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#
        )
        .bind(conv_a.id)
        .bind(conv_a.tenant_id)
        .bind(conv_a.inbox_id)
        .bind(conv_a.contact_id)
        .bind(&conv_a.status)
        .bind(conv_a.assignee_id)
        .bind(conv_a.unread_count)
        .execute(&mut *tx_a)
        .await
        .unwrap();

        // Insert Message for tenant A
        let msg_a = Message {
            tenant_id: tenant_a,
            id: Uuid::new_v4(),
            conversation_id: conv_a.id,
            inbox_id: inbox_a.id,
            sender_type: "contact".to_string(),
            sender_id: None,
            content: "Hi Maya, I'd like to order a chocolate cake!".to_string(),
            content_type: "text".to_string(),
            created_at: None,
            updated_at: None,
        };
        sqlx::query(
            r#"
            INSERT INTO omni_messages (id, tenant_id, conversation_id, inbox_id, sender_type, sender_id, content, content_type)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#
        )
        .bind(msg_a.id)
        .bind(msg_a.tenant_id)
        .bind(msg_a.conversation_id)
        .bind(msg_a.inbox_id)
        .bind(&msg_a.sender_type)
        .bind(msg_a.sender_id)
        .bind(&msg_a.content)
        .bind(&msg_a.content_type)
        .execute(&mut *tx_a)
        .await
        .unwrap();

        tx_a.commit().await.unwrap();

        // 2. Set context for tenant B and query
        let mut tx_b = pool.begin().await.unwrap();
        sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)")
            .bind(tenant_b.to_string())
            .execute(&mut *tx_b)
            .await
            .unwrap();

        // Verify that tenant B cannot see tenant A's inbox
        let inboxes_for_b: Vec<Inbox> = sqlx::query_as("SELECT * FROM omni_inboxes")
            .fetch_all(&mut *tx_b)
            .await
            .unwrap();
        assert!(
            inboxes_for_b.is_empty(),
            "Tenant B must NOT be able to read Tenant A's inboxes due to RLS"
        );

        // Verify that tenant B cannot see tenant A's message
        let messages_for_b: Vec<Message> = sqlx::query_as("SELECT * FROM omni_messages")
            .fetch_all(&mut *tx_b)
            .await
            .unwrap();
        assert!(
            messages_for_b.is_empty(),
            "Tenant B must NOT be able to read Tenant A's messages due to RLS"
        );

        // Try to insert a row for tenant A under tenant B's transaction
        let bad_insert = sqlx::query(
            "INSERT INTO omni_messages (id, tenant_id, conversation_id, inbox_id, sender_type, sender_id, content, content_type) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(Uuid::new_v4())
        .bind(tenant_a)
        .bind(conv_a.id)
        .bind(inbox_a.id)
        .bind("agent")
        .bind::<Option<Uuid>>(None)
        .bind("Attacking write")
        .bind("text")
        .execute(&mut *tx_b)
        .await;

        assert!(
            bad_insert.is_err(),
            "Inserting data for a different tenant than current_tenant_id must fail due to RLS CHECK policy"
        );

        tx_b.rollback().await.unwrap();

        // 3. Verify Repository methods using tenant A's context
        let inbox_repo = Inbox {
            tenant_id: tenant_a,
            id: Uuid::new_v4(),
            name: "Maya's Custom Inbox".to_string(),
            channel_type: "whatsapp".to_string(),
            auto_assignment_config: serde_json::json!({"enabled": false}),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        repo.create_inbox(&inbox_repo).await.unwrap();

        let retrieved_inbox = repo.get_inbox(tenant_a, inbox_repo.id).await.unwrap().unwrap();
        assert_eq!(retrieved_inbox.name, inbox_repo.name);

        let list_inboxes = repo.list_inboxes(tenant_a).await.unwrap();
        assert!(!list_inboxes.is_empty());

        let contact_repo = Contact {
            tenant_id: tenant_a,
            id: Uuid::new_v4(),
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
            phone_number: "+111222333".to_string(),
            identifier: "john_doe_id".to_string(),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        repo.create_contact(&contact_repo).await.unwrap();

        let retrieved_contact = repo.get_contact(tenant_a, contact_repo.id).await.unwrap().unwrap();
        assert_eq!(retrieved_contact.name, contact_repo.name);

        let conv_repo = Conversation {
            tenant_id: tenant_a,
            id: Uuid::new_v4(),
            inbox_id: inbox_repo.id,
            contact_id: contact_repo.id,
            status: "pending".to_string(),
            assignee_id: None,
            unread_count: 2,
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        repo.create_conversation(&conv_repo).await.unwrap();

        let retrieved_conv = repo.get_conversation(tenant_a, conv_repo.id).await.unwrap().unwrap();
        assert_eq!(retrieved_conv.status, conv_repo.status);

        let list_convs = repo.list_conversations(tenant_a).await.unwrap();
        assert!(!list_convs.is_empty());

        let msg_repo = Message {
            tenant_id: tenant_a,
            id: Uuid::new_v4(),
            conversation_id: conv_repo.id,
            inbox_id: inbox_repo.id,
            sender_type: "agent".to_string(),
            sender_id: Some(Uuid::new_v4()),
            content: "Hello from agent!".to_string(),
            content_type: "text".to_string(),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now()),
        };
        repo.create_message(&msg_repo).await.unwrap();

        let retrieved_msg = repo.get_message(tenant_a, msg_repo.id).await.unwrap().unwrap();
        assert_eq!(retrieved_msg.content, msg_repo.content);

        let list_msgs = repo.list_messages(tenant_a, conv_repo.id).await.unwrap();
        assert!(!list_msgs.is_empty());
    }
}
