use std::sync::Arc;
use uuid::Uuid;
use sqlx::Row;
use crate::db::DB;
use super::models::{ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage};

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
            crate::db::DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await?;
                ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id.to_string()).await?;
                let _ = sqlx::query("SELECT set_config('app.current_tenant_id', $1, true);")
                    .bind(tenant_id.to_string())
                    .execute(&mut *tx)
                    .await;

                let res = sqlx::query_as::<_, ChatInbox>(
                    r#"
                    INSERT INTO chat_inboxes (id, tenant_id, name)
                    VALUES ($1, $2, $3)
                    RETURNING id, tenant_id, name, created_at, updated_at
                    "#
                )
                .bind(Uuid::new_v4())
                .bind(tenant_id)
                .bind(name)
                .fetch_one(&mut *tx)
                .await?;

                tx.commit().await?;
                Ok(res)
            }
            crate::db::DbStore::Sqlite(pool) => {
                let inbox_id = Uuid::new_v4();
                sqlx::query(
                    r#"
                    INSERT INTO chat_inboxes (id, tenant_id, name)
                    VALUES (?, ?, ?)
                    "#
                )
                .bind(inbox_id.to_string())
                .bind(tenant_id.to_string())
                .bind(&name)
                .execute(pool)
                .await?;

                Ok(ChatInbox {
                    id: inbox_id,
                    tenant_id,
                    name,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                })
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
            crate::db::DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await?;
                ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id.to_string()).await?;
                let _ = sqlx::query("SELECT set_config('app.current_tenant_id', $1, true);")
                    .bind(tenant_id.to_string())
                    .execute(&mut *tx)
                    .await;

                let res = sqlx::query_as::<_, ChatChannel>(
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
                .fetch_one(&mut *tx)
                .await?;

                tx.commit().await?;
                Ok(res)
            }
            crate::db::DbStore::Sqlite(pool) => {
                let channel_id = Uuid::new_v4();
                sqlx::query(
                    r#"
                    INSERT INTO chat_channels (id, tenant_id, inbox_id, channel_type, config)
                    VALUES (?, ?, ?, ?, ?)
                    "#
                )
                .bind(channel_id.to_string())
                .bind(tenant_id.to_string())
                .bind(inbox_id.to_string())
                .bind(&channel_type)
                .bind(config.to_string())
                .execute(pool)
                .await?;

                Ok(ChatChannel {
                    id: channel_id,
                    tenant_id,
                    inbox_id,
                    channel_type,
                    config,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                })
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
            crate::db::DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await?;
                ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id.to_string()).await?;
                let _ = sqlx::query("SELECT set_config('app.current_tenant_id', $1, true);")
                    .bind(tenant_id.to_string())
                    .execute(&mut *tx)
                    .await;

                let res = sqlx::query_as::<_, ChatContact>(
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
                .fetch_one(&mut *tx)
                .await?;

                tx.commit().await?;
                Ok(res)
            }
            crate::db::DbStore::Sqlite(pool) => {
                let contact_id = Uuid::new_v4();
                sqlx::query(
                    r#"
                    INSERT INTO chat_contacts (id, tenant_id, name, email, phone)
                    VALUES (?, ?, ?, ?, ?)
                    "#
                )
                .bind(contact_id.to_string())
                .bind(tenant_id.to_string())
                .bind(&name)
                .bind(&email)
                .bind(&phone)
                .execute(pool)
                .await?;

                Ok(ChatContact {
                    id: contact_id,
                    tenant_id,
                    name,
                    email,
                    phone,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                })
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
            crate::db::DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await?;
                ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id.to_string()).await?;
                let _ = sqlx::query("SELECT set_config('app.current_tenant_id', $1, true);")
                    .bind(tenant_id.to_string())
                    .execute(&mut *tx)
                    .await;

                // Verify inbox and contact belong to the same tenant
                let inbox_exists: Option<Uuid> = sqlx::query_scalar("SELECT id FROM chat_inboxes WHERE id = $1")
                    .bind(inbox_id)
                    .fetch_optional(&mut *tx)
                    .await?;

                if inbox_exists.is_none() {
                    return Err(sqlx::Error::RowNotFound);
                }

                let contact_exists: Option<Uuid> = sqlx::query_scalar("SELECT id FROM chat_contacts WHERE id = $1")
                    .bind(contact_id)
                    .fetch_optional(&mut *tx)
                    .await?;

                if contact_exists.is_none() {
                    return Err(sqlx::Error::RowNotFound);
                }

                let res = sqlx::query_as::<_, ChatConversation>(
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
                .fetch_one(&mut *tx)
                .await?;

                tx.commit().await?;
                Ok(res)
            }
            crate::db::DbStore::Sqlite(pool) => {
                let mut tx = pool.begin().await?;

                // Verify inbox exists
                let inbox_exists_row = sqlx::query("SELECT id FROM chat_inboxes WHERE id = ? AND tenant_id = ?")
                    .bind(inbox_id.to_string())
                    .bind(tenant_id.to_string())
                    .fetch_optional(&mut *tx)
                    .await?;

                if inbox_exists_row.is_none() {
                    return Err(sqlx::Error::RowNotFound);
                }

                // Verify contact exists
                let contact_exists_row = sqlx::query("SELECT id FROM chat_contacts WHERE id = ? AND tenant_id = ?")
                    .bind(contact_id.to_string())
                    .bind(tenant_id.to_string())
                    .fetch_optional(&mut *tx)
                    .await?;

                if contact_exists_row.is_none() {
                    return Err(sqlx::Error::RowNotFound);
                }

                let conversation_id = Uuid::new_v4();
                sqlx::query(
                    r#"
                    INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, assignee_id, status)
                    VALUES (?, ?, ?, ?, ?, 'open')
                    "#
                )
                .bind(conversation_id.to_string())
                .bind(tenant_id.to_string())
                .bind(inbox_id.to_string())
                .bind(contact_id.to_string())
                .bind(assignee_id.map(|id| id.to_string()))
                .execute(&mut *tx)
                .await?;

                let conversation = ChatConversation {
                    id: conversation_id,
                    tenant_id,
                    inbox_id,
                    contact_id,
                    assignee_id,
                    status: "open".to_string(),
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                };

                tx.commit().await?;
                Ok(conversation)
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
            crate::db::DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await?;
                ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id.to_string()).await?;
                let _ = sqlx::query("SELECT set_config('app.current_tenant_id', $1, true);")
                    .bind(tenant_id.to_string())
                    .execute(&mut *tx)
                    .await;

                // Verify conversation exists
                let conversation_exists: Option<Uuid> = sqlx::query_scalar("SELECT id FROM chat_conversations WHERE id = $1")
                    .bind(conversation_id)
                    .fetch_optional(&mut *tx)
                    .await?;

                if conversation_exists.is_none() {
                    return Err(sqlx::Error::RowNotFound);
                }

                let res = sqlx::query_as::<_, ChatMessage>(
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
                .fetch_one(&mut *tx)
                .await?;

                tx.commit().await?;
                Ok(res)
            }
            crate::db::DbStore::Sqlite(pool) => {
                let mut tx = pool.begin().await?;

                // Verify conversation exists
                let conversation_exists_row = sqlx::query("SELECT id FROM chat_conversations WHERE id = ? AND tenant_id = ?")
                    .bind(conversation_id.to_string())
                    .bind(tenant_id.to_string())
                    .fetch_optional(&mut *tx)
                    .await?;

                if conversation_exists_row.is_none() {
                    return Err(sqlx::Error::RowNotFound);
                }

                let message_id = Uuid::new_v4();
                sqlx::query(
                    r#"
                    INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content)
                    VALUES (?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(message_id.to_string())
                .bind(tenant_id.to_string())
                .bind(conversation_id.to_string())
                .bind(&sender_type)
                .bind(sender_id.map(|id| id.to_string()))
                .bind(&content)
                .execute(&mut *tx)
                .await?;

                let message = ChatMessage {
                    id: message_id,
                    tenant_id,
                    conversation_id,
                    sender_type,
                    sender_id,
                    content,
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                };

                tx.commit().await?;
                Ok(message)
            }
        }
    }

    pub async fn get_inboxes(
        &self,
        tenant_id: Uuid,
    ) -> Result<Vec<ChatInbox>, sqlx::Error> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await?;
                ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id.to_string()).await?;
                let _ = sqlx::query("SELECT set_config('app.current_tenant_id', $1, true);")
                    .bind(tenant_id.to_string())
                    .execute(&mut *tx)
                    .await;

                let res = sqlx::query_as(
                    r#"
                    SELECT id, tenant_id, name, created_at, updated_at
                    FROM chat_inboxes
                    WHERE tenant_id = $1
                    ORDER BY created_at DESC
                    "#
                )
                .bind(tenant_id)
                .fetch_all(&mut *tx)
                .await?;

                tx.commit().await?;
                Ok(res)
            }
            crate::db::DbStore::Sqlite(pool) => {
                let rows = sqlx::query(
                    r#"
                    SELECT id, tenant_id, name, created_at, updated_at
                    FROM chat_inboxes
                    WHERE tenant_id = ?
                    ORDER BY created_at DESC
                    "#
                )
                .bind(tenant_id.to_string())
                .fetch_all(pool)
                .await?;

                let inboxes = rows.into_iter().map(|row| {
                    let created_at = row.get::<Option<String>, _>("created_at")
                        .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|dt| dt.with_timezone(&chrono::Utc)))
                        .unwrap_or_else(chrono::Utc::now);
                    let updated_at = row.get::<Option<String>, _>("updated_at")
                        .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|dt| dt.with_timezone(&chrono::Utc)))
                        .unwrap_or_else(chrono::Utc::now);

                    ChatInbox {
                        id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap_or_default(),
                        tenant_id: Uuid::parse_str(row.get::<&str, _>("tenant_id")).unwrap_or_default(),
                        name: row.get::<String, _>("name"),
                        created_at,
                        updated_at,
                    }
                }).collect();

                Ok(inboxes)
            }
        }
    }

    pub async fn get_conversations(
        &self,
        tenant_id: Uuid,
    ) -> Result<Vec<ChatConversation>, sqlx::Error> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await?;
                ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id.to_string()).await?;
                let _ = sqlx::query("SELECT set_config('app.current_tenant_id', $1, true);")
                    .bind(tenant_id.to_string())
                    .execute(&mut *tx)
                    .await;

                let res = sqlx::query_as(
                    r#"
                    SELECT id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at
                    FROM chat_conversations
                    WHERE tenant_id = $1
                    ORDER BY created_at DESC
                    "#
                )
                .bind(tenant_id)
                .fetch_all(&mut *tx)
                .await?;

                tx.commit().await?;
                Ok(res)
            }
            crate::db::DbStore::Sqlite(pool) => {
                let rows = sqlx::query(
                    r#"
                    SELECT id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at
                    FROM chat_conversations
                    WHERE tenant_id = ?
                    ORDER BY created_at DESC
                    "#
                )
                .bind(tenant_id.to_string())
                .fetch_all(pool)
                .await?;

                let conversations = rows.into_iter().map(|row| {
                    let created_at = row.get::<Option<String>, _>("created_at")
                        .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|dt| dt.with_timezone(&chrono::Utc)))
                        .unwrap_or_else(chrono::Utc::now);
                    let updated_at = row.get::<Option<String>, _>("updated_at")
                        .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|dt| dt.with_timezone(&chrono::Utc)))
                        .unwrap_or_else(chrono::Utc::now);

                    ChatConversation {
                        id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap_or_default(),
                        tenant_id: Uuid::parse_str(row.get::<&str, _>("tenant_id")).unwrap_or_default(),
                        inbox_id: Uuid::parse_str(row.get::<&str, _>("inbox_id")).unwrap_or_default(),
                        contact_id: Uuid::parse_str(row.get::<&str, _>("contact_id")).unwrap_or_default(),
                        assignee_id: row.get::<Option<String>, _>("assignee_id").and_then(|id| Uuid::parse_str(&id).ok()),
                        status: row.get::<String, _>("status"),
                        created_at,
                        updated_at,
                    }
                }).collect();

                Ok(conversations)
            }
        }
    }

    pub async fn get_messages(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
    ) -> Result<Vec<ChatMessage>, sqlx::Error> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await?;
                ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id.to_string()).await?;
                let _ = sqlx::query("SELECT set_config('app.current_tenant_id', $1, true);")
                    .bind(tenant_id.to_string())
                    .execute(&mut *tx)
                    .await;

                // Verify conversation exists and belongs to this tenant
                let conversation_exists: Option<Uuid> = sqlx::query_scalar("SELECT id FROM chat_conversations WHERE id = $1")
                    .bind(conversation_id)
                    .fetch_optional(&mut *tx)
                    .await?;

                if conversation_exists.is_none() {
                    return Err(sqlx::Error::RowNotFound);
                }

                let res = sqlx::query_as(
                    r#"
                    SELECT id, tenant_id, conversation_id, sender_type, sender_id, content, created_at, updated_at
                    FROM chat_messages
                    WHERE tenant_id = $1 AND conversation_id = $2
                    ORDER BY created_at ASC
                    "#
                )
                .bind(tenant_id)
                .bind(conversation_id)
                .fetch_all(&mut *tx)
                .await?;

                tx.commit().await?;
                Ok(res)
            }
            crate::db::DbStore::Sqlite(pool) => {
                let mut tx = pool.begin().await?;

                // Verify conversation exists and belongs to this tenant
                let conversation_exists_row = sqlx::query("SELECT id FROM chat_conversations WHERE id = ? AND tenant_id = ?")
                    .bind(conversation_id.to_string())
                    .bind(tenant_id.to_string())
                    .fetch_optional(&mut *tx)
                    .await?;

                if conversation_exists_row.is_none() {
                    return Err(sqlx::Error::RowNotFound);
                }

                let rows = sqlx::query(
                    r#"
                    SELECT id, tenant_id, conversation_id, sender_type, sender_id, content, created_at, updated_at
                    FROM chat_messages
                    WHERE tenant_id = ? AND conversation_id = ?
                    ORDER BY created_at ASC
                    "#
                )
                .bind(tenant_id.to_string())
                .bind(conversation_id.to_string())
                .fetch_all(&mut *tx)
                .await?;

                let messages = rows.into_iter().map(|row| {
                    let created_at = row.get::<Option<String>, _>("created_at")
                        .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|dt| dt.with_timezone(&chrono::Utc)))
                        .unwrap_or_else(chrono::Utc::now);
                    let updated_at = row.get::<Option<String>, _>("updated_at")
                        .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|dt| dt.with_timezone(&chrono::Utc)))
                        .unwrap_or_else(chrono::Utc::now);

                    ChatMessage {
                        id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap_or_default(),
                        tenant_id: Uuid::parse_str(row.get::<&str, _>("tenant_id")).unwrap_or_default(),
                        conversation_id: Uuid::parse_str(row.get::<&str, _>("conversation_id")).unwrap_or_default(),
                        sender_type: row.get::<String, _>("sender_type"),
                        sender_id: row.get::<Option<String>, _>("sender_id").and_then(|id| Uuid::parse_str(&id).ok()),
                        content: row.get::<String, _>("content"),
                        created_at,
                        updated_at,
                    }
                }).collect();

                tx.commit().await?;
                Ok(messages)
            }
        }
    }

    pub async fn get_contacts(
        &self,
        tenant_id: Uuid,
    ) -> Result<Vec<ChatContact>, sqlx::Error> {
        match &self.db.store {
            crate::db::DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await?;
                ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id.to_string()).await?;
                let _ = sqlx::query("SELECT set_config('app.current_tenant_id', $1, true);")
                    .bind(tenant_id.to_string())
                    .execute(&mut *tx)
                    .await;

                let res = sqlx::query_as(
                    r#"
                    SELECT id, tenant_id, name, email, phone, created_at, updated_at
                    FROM chat_contacts
                    WHERE tenant_id = $1
                    ORDER BY created_at DESC
                    "#
                )
                .bind(tenant_id)
                .fetch_all(&mut *tx)
                .await?;

                tx.commit().await?;
                Ok(res)
            }
            crate::db::DbStore::Sqlite(pool) => {
                let rows = sqlx::query(
                    r#"
                    SELECT id, tenant_id, name, email, phone, created_at, updated_at
                    FROM chat_contacts
                    WHERE tenant_id = ?
                    ORDER BY created_at DESC
                    "#
                )
                .bind(tenant_id.to_string())
                .fetch_all(pool)
                .await?;

                let contacts = rows.into_iter().map(|row| {
                    let created_at = row.get::<Option<String>, _>("created_at")
                        .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|dt| dt.with_timezone(&chrono::Utc)))
                        .unwrap_or_else(chrono::Utc::now);
                    let updated_at = row.get::<Option<String>, _>("updated_at")
                        .and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|dt| dt.with_timezone(&chrono::Utc)))
                        .unwrap_or_else(chrono::Utc::now);

                    ChatContact {
                        id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap_or_default(),
                        tenant_id: Uuid::parse_str(row.get::<&str, _>("tenant_id")).unwrap_or_default(),
                        name: row.get::<Option<String>, _>("name"),
                        email: row.get::<Option<String>, _>("email"),
                        phone: row.get::<Option<String>, _>("phone"),
                        created_at,
                        updated_at,
                    }
                }).collect();

                Ok(contacts)
            }
        }
    }
}
