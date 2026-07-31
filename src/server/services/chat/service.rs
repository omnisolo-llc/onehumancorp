use uuid::Uuid;
use super::models::{ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage};
use crate::db::{DB, DbStore};
use std::sync::Arc;

pub struct ChatService {
    db: Arc<DB>,
}

impl ChatService {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn create_inbox(
        &self,
        tenant_id: Uuid,
        name: String,
    ) -> Result<ChatInbox, sqlx::Error> {
        match &self.db.store {
            DbStore::Postgres => {
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
                .fetch_one(&self.db.pool)
                .await
            },
            DbStore::Sqlite(sqlite_pool) => {
                let id = Uuid::new_v4();
                sqlx::query("INSERT INTO chat_inboxes (id, tenant_id, name) VALUES (?, ?, ?)")
                    .bind(id.to_string())
                    .bind(tenant_id.to_string())
                    .bind(&name)
                    .execute(sqlite_pool)
                    .await?;
                sqlx::query_as("SELECT id, tenant_id, name, created_at, updated_at FROM chat_inboxes WHERE id = ?")
                    .bind(id.to_string())
                    .fetch_one(sqlite_pool)
                    .await
            }
        }
    }

    pub async fn create_channel(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        channel_type: String,
        config: serde_json::Value,
    ) -> Result<ChatChannel, sqlx::Error> {
        match &self.db.store {
            DbStore::Postgres => {
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
                .fetch_one(&self.db.pool)
                .await
            },
            DbStore::Sqlite(sqlite_pool) => {
                let id = Uuid::new_v4();
                sqlx::query("INSERT INTO chat_channels (id, tenant_id, inbox_id, channel_type, config) VALUES (?, ?, ?, ?, ?)")
                    .bind(id.to_string())
                    .bind(tenant_id.to_string())
                    .bind(inbox_id.to_string())
                    .bind(&channel_type)
                    .bind(config.to_string())
                    .execute(sqlite_pool)
                    .await?;
                sqlx::query_as("SELECT id, tenant_id, inbox_id, channel_type, config, created_at, updated_at FROM chat_channels WHERE id = ?")
                    .bind(id.to_string())
                    .fetch_one(sqlite_pool)
                    .await
            }
        }
    }

    pub async fn create_contact(
        &self,
        tenant_id: Uuid,
        name: Option<String>,
        email: Option<String>,
        phone: Option<String>,
    ) -> Result<ChatContact, sqlx::Error> {
        match &self.db.store {
            DbStore::Postgres => {
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
                .fetch_one(&self.db.pool)
                .await
            },
            DbStore::Sqlite(sqlite_pool) => {
                let id = Uuid::new_v4();
                sqlx::query("INSERT INTO chat_contacts (id, tenant_id, name, email, phone) VALUES (?, ?, ?, ?, ?)")
                    .bind(id.to_string())
                    .bind(tenant_id.to_string())
                    .bind(name)
                    .bind(email)
                    .bind(phone)
                    .execute(sqlite_pool)
                    .await?;
                sqlx::query_as("SELECT id, tenant_id, name, email, phone, created_at, updated_at FROM chat_contacts WHERE id = ?")
                    .bind(id.to_string())
                    .fetch_one(sqlite_pool)
                    .await
            }
        }
    }

    pub async fn start_conversation(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        contact_id: Uuid,
        assignee_id: Option<Uuid>,
    ) -> Result<ChatConversation, sqlx::Error> {
        match &self.db.store {
            DbStore::Postgres => {
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
                .fetch_one(&self.db.pool)
                .await
            },
            DbStore::Sqlite(sqlite_pool) => {
                let id = Uuid::new_v4();
                sqlx::query("INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, assignee_id, status) VALUES (?, ?, ?, ?, ?, 'open')")
                    .bind(id.to_string())
                    .bind(tenant_id.to_string())
                    .bind(inbox_id.to_string())
                    .bind(contact_id.to_string())
                    .bind(assignee_id.map(|u| u.to_string()))
                    .execute(sqlite_pool)
                    .await?;
                sqlx::query_as("SELECT id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at FROM chat_conversations WHERE id = ?")
                    .bind(id.to_string())
                    .fetch_one(sqlite_pool)
                    .await
            }
        }
    }

    pub async fn send_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        sender_type: String,
        sender_id: Option<Uuid>,
        content: String,
    ) -> Result<ChatMessage, sqlx::Error> {
        match &self.db.store {
            DbStore::Postgres => {
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
                .fetch_one(&self.db.pool)
                .await
            },
            DbStore::Sqlite(sqlite_pool) => {
                let id = Uuid::new_v4();
                sqlx::query("INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content) VALUES (?, ?, ?, ?, ?, ?)")
                    .bind(id.to_string())
                    .bind(tenant_id.to_string())
                    .bind(conversation_id.to_string())
                    .bind(&sender_type)
                    .bind(sender_id.map(|u| u.to_string()))
                    .bind(&content)
                    .execute(sqlite_pool)
                    .await?;
                sqlx::query_as("SELECT id, tenant_id, conversation_id, sender_type, sender_id, content, created_at, updated_at FROM chat_messages WHERE id = ?")
                    .bind(id.to_string())
                    .fetch_one(sqlite_pool)
                    .await
            }
        }
    }

    pub async fn get_inbox_by_name(
        &self,
        tenant_id: Uuid,
        name: &str,
    ) -> Result<Option<ChatInbox>, sqlx::Error> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_as(
                    r#"
                    SELECT * FROM chat_inboxes
                    WHERE tenant_id = $1 AND name = $2
                    LIMIT 1
                    "#
                )
                .bind(tenant_id)
                .bind(name)
                .fetch_optional(&self.db.pool)
                .await
            },
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as("SELECT id, tenant_id, name, created_at, updated_at FROM chat_inboxes WHERE tenant_id = ? AND name = ? LIMIT 1")
                    .bind(tenant_id.to_string())
                    .bind(name)
                    .fetch_optional(sqlite_pool)
                    .await
            }
        }
    }

    pub async fn get_channel_by_type(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        channel_type: &str,
    ) -> Result<Option<ChatChannel>, sqlx::Error> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_as(
                    r#"
                    SELECT * FROM chat_channels
                    WHERE tenant_id = $1 AND inbox_id = $2 AND channel_type = $3
                    LIMIT 1
                    "#
                )
                .bind(tenant_id)
                .bind(inbox_id)
                .bind(channel_type)
                .fetch_optional(&self.db.pool)
                .await
            },
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as("SELECT id, tenant_id, inbox_id, channel_type, config, created_at, updated_at FROM chat_channels WHERE tenant_id = ? AND inbox_id = ? AND channel_type = ? LIMIT 1")
                    .bind(tenant_id.to_string())
                    .bind(inbox_id.to_string())
                    .bind(channel_type)
                    .fetch_optional(sqlite_pool)
                    .await
            }
        }
    }

    pub async fn get_contact_by_identifier(
        &self,
        tenant_id: Uuid,
        identifier: &str,
    ) -> Result<Option<ChatContact>, sqlx::Error> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_as(
                    r#"
                    SELECT * FROM chat_contacts
                    WHERE tenant_id = $1 AND (email = $2 OR phone = $2)
                    LIMIT 1
                    "#
                )
                .bind(tenant_id)
                .bind(identifier)
                .fetch_optional(&self.db.pool)
                .await
            },
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as("SELECT id, tenant_id, name, email, phone, created_at, updated_at FROM chat_contacts WHERE tenant_id = ? AND (email = ? OR phone = ?) LIMIT 1")
                    .bind(tenant_id.to_string())
                    .bind(identifier)
                    .bind(identifier)
                    .fetch_optional(sqlite_pool)
                    .await
            }
        }
    }

    pub async fn get_conversation(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        contact_id: Uuid,
    ) -> Result<Option<ChatConversation>, sqlx::Error> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_as(
                    r#"
                    SELECT * FROM chat_conversations
                    WHERE tenant_id = $1 AND inbox_id = $2 AND contact_id = $3
                    LIMIT 1
                    "#
                )
                .bind(tenant_id)
                .bind(inbox_id)
                .bind(contact_id)
                .fetch_optional(&self.db.pool)
                .await
            },
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as("SELECT id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at FROM chat_conversations WHERE tenant_id = ? AND inbox_id = ? AND contact_id = ? LIMIT 1")
                    .bind(tenant_id.to_string())
                    .bind(inbox_id.to_string())
                    .bind(contact_id.to_string())
                    .fetch_optional(sqlite_pool)
                    .await
            }
        }
    }
}
