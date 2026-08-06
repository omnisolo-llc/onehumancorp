use sqlx::{PgPool, Error};
use super::models::{Tenant, Inbox, Channel, Contact, Conversation, Message};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct InboxRepository {
    pool: PgPool,
}

impl InboxRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_tenant(&self, name: String) -> Result<Tenant, Error> {
        let id = Uuid::new_v4().to_string();
        sqlx::query_as::<_, Tenant>(
            r#"
            INSERT INTO tenants (id, name)
            VALUES ($1, $2)
            RETURNING id, name
            "#
        )
        .bind(id)
        .bind(name)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_tenant(&self, id: &str) -> Result<Tenant, Error> {
        sqlx::query_as::<_, Tenant>(
            r#"
            SELECT id, name FROM tenants WHERE id = $1
            "#
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_inbox(&self, tenant_id: String, name: String) -> Result<Inbox, Error> {
        let id = Uuid::new_v4().to_string();
        sqlx::query_as::<_, Inbox>(
            r#"
            INSERT INTO inboxes (id, tenant_id, name)
            VALUES ($1, $2, $3)
            RETURNING id, tenant_id, name
            "#
        )
        .bind(id)
        .bind(tenant_id)
        .bind(name)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_inbox(&self, tenant_id: &str, id: &str) -> Result<Inbox, Error> {
        sqlx::query_as::<_, Inbox>(
            r#"
            SELECT id, tenant_id, name FROM inboxes WHERE tenant_id = $1 AND id = $2
            "#
        )
        .bind(tenant_id)
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_channel(&self, tenant_id: String, inbox_id: String, provider_type: String, credentials: serde_json::Value) -> Result<Channel, Error> {
        let id = Uuid::new_v4().to_string();
        sqlx::query_as::<_, Channel>(
            r#"
            INSERT INTO channels (id, tenant_id, inbox_id, provider_type, credentials)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, inbox_id, provider_type, credentials
            "#
        )
        .bind(id)
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(provider_type)
        .bind(sqlx::types::Json(credentials))
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_channel(&self, tenant_id: &str, id: &str) -> Result<Channel, Error> {
        sqlx::query_as::<_, Channel>(
            r#"
            SELECT id, tenant_id, inbox_id, provider_type, credentials FROM channels WHERE tenant_id = $1 AND id = $2
            "#
        )
        .bind(tenant_id)
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_contact(&self, tenant_id: String, name: String, identifier: String) -> Result<Contact, Error> {
        let id = Uuid::new_v4().to_string();
        sqlx::query_as::<_, Contact>(
            r#"
            INSERT INTO contacts (id, tenant_id, name, identifier)
            VALUES ($1, $2, $3, $4)
            RETURNING id, tenant_id, name, identifier
            "#
        )
        .bind(id)
        .bind(tenant_id)
        .bind(name)
        .bind(identifier)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_contact(&self, tenant_id: &str, id: &str) -> Result<Contact, Error> {
        sqlx::query_as::<_, Contact>(
            r#"
            SELECT id, tenant_id, name, identifier FROM contacts WHERE tenant_id = $1 AND id = $2
            "#
        )
        .bind(tenant_id)
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_conversation(&self, tenant_id: String, inbox_id: String, contact_id: String, status: String) -> Result<Conversation, Error> {
        let id = Uuid::new_v4().to_string();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        sqlx::query_as::<_, Conversation>(
            r#"
            INSERT INTO conversations (id, tenant_id, inbox_id, contact_id, status, created_at_unix, updated_at_unix)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, tenant_id, inbox_id, contact_id, status, created_at_unix, updated_at_unix
            "#
        )
        .bind(id)
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .bind(status)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_conversation(&self, tenant_id: &str, id: &str) -> Result<Conversation, Error> {
        sqlx::query_as::<_, Conversation>(
            r#"
            SELECT id, tenant_id, inbox_id, contact_id, status, created_at_unix, updated_at_unix FROM conversations WHERE tenant_id = $1 AND id = $2
            "#
        )
        .bind(tenant_id)
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_message(&self, tenant_id: String, conversation_id: String, content: String, sender_type: String, sender_id: String) -> Result<Message, Error> {
        let id = Uuid::new_v4().to_string();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        sqlx::query_as::<_, Message>(
            r#"
            INSERT INTO messages (id, tenant_id, conversation_id, content, sender_type, sender_id, created_at_unix, updated_at_unix)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, tenant_id, conversation_id, content, sender_type, sender_id, created_at_unix, updated_at_unix
            "#
        )
        .bind(id)
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(content)
        .bind(sender_type)
        .bind(sender_id)
        .bind(now)
        .bind(now)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_message(&self, tenant_id: &str, id: &str) -> Result<Message, Error> {
        sqlx::query_as::<_, Message>(
            r#"
            SELECT id, tenant_id, conversation_id, content, sender_type, sender_id, created_at_unix, updated_at_unix FROM messages WHERE tenant_id = $1 AND id = $2
            "#
        )
        .bind(tenant_id)
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }
}
