use uuid::Uuid;
use super::models::{ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage};
use crate::db::{DB, DbStore};

pub struct ChatService<'a> {
    db: &'a DB,
}

impl<'a> ChatService<'a> {
    pub fn new(db: &'a DB) -> Self {
        Self { db }
    }

    pub async fn get_or_create_inbox(&self, tenant_id: &str, name: &str) -> Result<String, sqlx::Error> {
        match &self.db.store {
            DbStore::Postgres => {
                let existing: Option<String> = sqlx::query_scalar("SELECT id::text FROM chat_inboxes WHERE tenant_id = $1::uuid AND name = $2 LIMIT 1")
                    .bind(tenant_id)
                    .bind(name)
                    .fetch_optional(&self.db.pool)
                    .await?;
                if let Some(id) = existing {
                    return Ok(id);
                }
                let new_id = Uuid::new_v4().to_string();
                sqlx::query("INSERT INTO chat_inboxes (id, tenant_id, name) VALUES ($1::uuid, $2::uuid, $3)")
                    .bind(&new_id)
                    .bind(tenant_id)
                    .bind(name)
                    .execute(&self.db.pool)
                    .await?;
                Ok(new_id)
            }
            DbStore::Sqlite(sqlite_pool) => {
                let existing: Option<String> = sqlx::query_scalar("SELECT id FROM chat_inboxes WHERE tenant_id = ? AND name = ? LIMIT 1")
                    .bind(tenant_id)
                    .bind(name)
                    .fetch_optional(sqlite_pool)
                    .await?;
                if let Some(id) = existing {
                    return Ok(id);
                }
                let new_id = Uuid::new_v4().to_string();
                sqlx::query("INSERT INTO chat_inboxes (id, tenant_id, name) VALUES (?, ?, ?)")
                    .bind(&new_id)
                    .bind(tenant_id)
                    .bind(name)
                    .execute(sqlite_pool)
                    .await?;
                Ok(new_id)
            }
        }
    }

    pub async fn get_or_create_contact_by_phone(&self, tenant_id: &str, phone: &str) -> Result<String, sqlx::Error> {
        match &self.db.store {
            DbStore::Postgres => {
                let existing: Option<String> = sqlx::query_scalar("SELECT id::text FROM chat_contacts WHERE tenant_id = $1::uuid AND phone = $2 LIMIT 1")
                    .bind(tenant_id)
                    .bind(phone)
                    .fetch_optional(&self.db.pool)
                    .await?;
                if let Some(id) = existing {
                    return Ok(id);
                }
                let new_id = Uuid::new_v4().to_string();
                sqlx::query("INSERT INTO chat_contacts (id, tenant_id, phone) VALUES ($1::uuid, $2::uuid, $3)")
                    .bind(&new_id)
                    .bind(tenant_id)
                    .bind(phone)
                    .execute(&self.db.pool)
                    .await?;
                Ok(new_id)
            }
            DbStore::Sqlite(sqlite_pool) => {
                let existing: Option<String> = sqlx::query_scalar("SELECT id FROM chat_contacts WHERE tenant_id = ? AND phone = ? LIMIT 1")
                    .bind(tenant_id)
                    .bind(phone)
                    .fetch_optional(sqlite_pool)
                    .await?;
                if let Some(id) = existing {
                    return Ok(id);
                }
                let new_id = Uuid::new_v4().to_string();
                sqlx::query("INSERT INTO chat_contacts (id, tenant_id, phone) VALUES (?, ?, ?)")
                    .bind(&new_id)
                    .bind(tenant_id)
                    .bind(phone)
                    .execute(sqlite_pool)
                    .await?;
                Ok(new_id)
            }
        }
    }

    pub async fn get_or_create_conversation(&self, tenant_id: &str, inbox_id: &str, contact_id: &str) -> Result<String, sqlx::Error> {
        match &self.db.store {
            DbStore::Postgres => {
                let existing: Option<String> = sqlx::query_scalar("SELECT id::text FROM chat_conversations WHERE tenant_id = $1::uuid AND inbox_id = $2::uuid AND contact_id = $3::uuid LIMIT 1")
                    .bind(tenant_id)
                    .bind(inbox_id)
                    .bind(contact_id)
                    .fetch_optional(&self.db.pool)
                    .await?;
                if let Some(id) = existing {
                    return Ok(id);
                }
                let new_id = Uuid::new_v4().to_string();
                sqlx::query("INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, status) VALUES ($1::uuid, $2::uuid, $3::uuid, $4::uuid, 'open')")
                    .bind(&new_id)
                    .bind(tenant_id)
                    .bind(inbox_id)
                    .bind(contact_id)
                    .execute(&self.db.pool)
                    .await?;
                Ok(new_id)
            }
            DbStore::Sqlite(sqlite_pool) => {
                let existing: Option<String> = sqlx::query_scalar("SELECT id FROM chat_conversations WHERE tenant_id = ? AND inbox_id = ? AND contact_id = ? LIMIT 1")
                    .bind(tenant_id)
                    .bind(inbox_id)
                    .bind(contact_id)
                    .fetch_optional(sqlite_pool)
                    .await?;
                if let Some(id) = existing {
                    return Ok(id);
                }
                let new_id = Uuid::new_v4().to_string();
                sqlx::query("INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, status) VALUES (?, ?, ?, ?, 'open')")
                    .bind(&new_id)
                    .bind(tenant_id)
                    .bind(inbox_id)
                    .bind(contact_id)
                    .execute(sqlite_pool)
                    .await?;
                Ok(new_id)
            }
        }
    }

    pub async fn send_message(
        &self,
        tenant_id: &str,
        conversation_id: &str,
        sender_type: &str,
        sender_id: Option<&str>,
        content: &str,
    ) -> Result<String, sqlx::Error> {
        let new_id = Uuid::new_v4().to_string();
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(
                    "INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content) VALUES ($1::uuid, $2::uuid, $3::uuid, $4, $5::uuid, $6)"
                )
                .bind(&new_id)
                .bind(tenant_id)
                .bind(conversation_id)
                .bind(sender_type)
                .bind(sender_id)
                .bind(content)
                .execute(&self.db.pool)
                .await?;
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query(
                    "INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content) VALUES (?, ?, ?, ?, ?, ?)"
                )
                .bind(&new_id)
                .bind(tenant_id)
                .bind(conversation_id)
                .bind(sender_type)
                .bind(sender_id)
                .bind(content)
                .execute(sqlite_pool)
                .await?;
            }
        }
        Ok(new_id)
    }
}
