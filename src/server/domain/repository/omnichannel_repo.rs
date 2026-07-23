use sqlx::{FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use crate::db::DB;

#[derive(Clone, Debug, FromRow)]
pub struct CustomerProfile {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct Contact {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub identifier: String,
    pub email: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct Inbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub enable_auto_assignment: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct ChannelAdapter {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Option<Uuid>,
    pub r#type: String,
    pub config: Option<sqlx::types::Json<serde_json::Value>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub content: String,
    pub sender_type: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct WorkItem {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub customer_id: Uuid,
    pub source: String,
    pub payload: Option<sqlx::types::Json<serde_json::Value>>,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct AgentDraft {
    pub id: Uuid,
    pub work_item_id: Uuid,
    pub response: String,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct OmniChannelRepo {
    db: Arc<DB>,
}

impl OmniChannelRepo {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn create_customer_profile(&self, tenant_id: Uuid, name: Option<String>) -> Result<CustomerProfile, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, CustomerProfile>(
            "INSERT INTO customer_profile (id, tenant_id, name) VALUES ($1, $2, $3) RETURNING id, tenant_id, name, created_at, updated_at",
        )
        .bind(id)
        .bind(tenant_id)
        .bind(name)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_work_item(&self, tenant_id: Uuid, customer_id: Uuid, source: String, payload: serde_json::Value) -> Result<WorkItem, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, WorkItem>(
            "INSERT INTO work_item (id, tenant_id, customer_id, source, payload, status) VALUES ($1, $2, $3, $4, $5, 'PENDING') RETURNING id, tenant_id, customer_id, source, payload as \"payload: sqlx::types::Json<serde_json::Value>\", status, created_at, updated_at",
        )
        .bind(id)
        .bind(tenant_id)
        .bind(customer_id)
        .bind(source)
        .bind(sqlx::types::Json(payload))
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_agent_draft(&self, work_item_id: Uuid, response: String) -> Result<AgentDraft, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, AgentDraft>(
            "INSERT INTO agent_draft (id, work_item_id, response, status) VALUES ($1, $2, $3, 'DRAFT') RETURNING id, work_item_id, response, status, created_at, updated_at",
        )
        .bind(id)
        .bind(work_item_id)
        .bind(response)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_contact(&self, tenant_id: Uuid, identifier: String, email: Option<String>) -> Result<Contact, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = match &self.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query_as::<_, Contact>(
                    "INSERT INTO contacts (id, tenant_id, identifier, email) VALUES ($1, $2, $3, $4) ON CONFLICT (tenant_id, identifier) DO UPDATE SET email = COALESCE(EXCLUDED.email, contacts.email) RETURNING id, tenant_id, identifier, email, created_at, updated_at",
                )
                .bind(id)
                .bind(tenant_id)
                .bind(identifier)
                .bind(email)
                .fetch_one(&self.db.pool)
                .await?
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as::<_, Contact>(
                    "INSERT INTO contacts (id, tenant_id, identifier, email) VALUES (?, ?, ?, ?) ON CONFLICT (tenant_id, identifier) DO UPDATE SET email = COALESCE(EXCLUDED.email, contacts.email) RETURNING id, tenant_id, identifier, email, created_at, updated_at",
                )
                .bind(id)
                .bind(tenant_id)
                .bind(identifier)
                .bind(email)
                .fetch_one(sqlite_pool)
                .await?
            }
        };
        Ok(record)
    }

    pub async fn get_contact(&self, tenant_id: Uuid, identifier: String) -> Result<Option<Contact>, sqlx::Error> {
        let record = match &self.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query_as::<_, Contact>(
                    "SELECT id, tenant_id, identifier, email, created_at, updated_at FROM contacts WHERE tenant_id = $1 AND identifier = $2",
                )
                .bind(tenant_id)
                .bind(identifier)
                .fetch_optional(&self.db.pool)
                .await?
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as::<_, Contact>(
                    "SELECT id, tenant_id, identifier, email, created_at, updated_at FROM contacts WHERE tenant_id = ? AND identifier = ?",
                )
                .bind(tenant_id)
                .bind(identifier)
                .fetch_optional(sqlite_pool)
                .await?
            }
        };
        Ok(record)
    }

    pub async fn create_inbox(&self, tenant_id: Uuid, name: String, enable_auto_assignment: bool) -> Result<Inbox, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = match &self.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query_as::<_, Inbox>(
                    "INSERT INTO inboxes (id, tenant_id, name, enable_auto_assignment) VALUES ($1, $2, $3, $4) RETURNING id, tenant_id, name, enable_auto_assignment, created_at, updated_at",
                )
                .bind(id)
                .bind(tenant_id)
                .bind(name)
                .bind(enable_auto_assignment)
                .fetch_one(&self.db.pool)
                .await?
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as::<_, Inbox>(
                    "INSERT INTO inboxes (id, tenant_id, name, enable_auto_assignment) VALUES (?, ?, ?, ?) RETURNING id, tenant_id, name, enable_auto_assignment, created_at, updated_at",
                )
                .bind(id)
                .bind(tenant_id)
                .bind(name)
                .bind(enable_auto_assignment)
                .fetch_one(sqlite_pool)
                .await?
            }
        };
        Ok(record)
    }

    pub async fn get_inboxes(&self, tenant_id: Uuid) -> Result<Vec<Inbox>, sqlx::Error> {
        let records = match &self.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query_as::<_, Inbox>(
                    "SELECT id, tenant_id, name, enable_auto_assignment, created_at, updated_at FROM inboxes WHERE tenant_id = $1",
                )
                .bind(tenant_id)
                .fetch_all(&self.db.pool)
                .await?
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as::<_, Inbox>(
                    "SELECT id, tenant_id, name, enable_auto_assignment, created_at, updated_at FROM inboxes WHERE tenant_id = ?",
                )
                .bind(tenant_id)
                .fetch_all(sqlite_pool)
                .await?
            }
        };
        Ok(records)
    }

    pub async fn create_channel_adapter(&self, tenant_id: Uuid, inbox_id: Option<Uuid>, adapter_type: String, config: serde_json::Value) -> Result<ChannelAdapter, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = match &self.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query_as::<_, ChannelAdapter>(
                    "INSERT INTO channel_adapters (id, tenant_id, inbox_id, type, config) VALUES ($1, $2, $3, $4, $5) RETURNING id, tenant_id, inbox_id, type, config as \"config: sqlx::types::Json<serde_json::Value>\", created_at, updated_at",
                )
                .bind(id)
                .bind(tenant_id)
                .bind(inbox_id)
                .bind(adapter_type)
                .bind(sqlx::types::Json(config))
                .fetch_one(&self.db.pool)
                .await?
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as::<_, ChannelAdapter>(
                    "INSERT INTO channel_adapters (id, tenant_id, inbox_id, type, config) VALUES (?, ?, ?, ?, ?) RETURNING id, tenant_id, inbox_id, type, config as \"config: sqlx::types::Json<serde_json::Value>\", created_at, updated_at",
                )
                .bind(id)
                .bind(tenant_id)
                .bind(inbox_id)
                .bind(adapter_type)
                .bind(sqlx::types::Json(config))
                .fetch_one(sqlite_pool)
                .await?
            }
        };
        Ok(record)
    }

    pub async fn create_conversation(&self, tenant_id: Uuid, inbox_id: Uuid, contact_id: Uuid, status: String) -> Result<Conversation, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = match &self.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query_as::<_, Conversation>(
                    "INSERT INTO conversations (id, tenant_id, inbox_id, contact_id, status) VALUES ($1, $2, $3, $4, $5) RETURNING id, tenant_id, inbox_id, contact_id, status, created_at, updated_at",
                )
                .bind(id)
                .bind(tenant_id)
                .bind(inbox_id)
                .bind(contact_id)
                .bind(status)
                .fetch_one(&self.db.pool)
                .await?
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as::<_, Conversation>(
                    "INSERT INTO conversations (id, tenant_id, inbox_id, contact_id, status) VALUES (?, ?, ?, ?, ?) RETURNING id, tenant_id, inbox_id, contact_id, status, created_at, updated_at",
                )
                .bind(id)
                .bind(tenant_id)
                .bind(inbox_id)
                .bind(contact_id)
                .bind(status)
                .fetch_one(sqlite_pool)
                .await?
            }
        };
        Ok(record)
    }

    pub async fn get_conversations(&self, tenant_id: Uuid, inbox_id: Option<Uuid>) -> Result<Vec<Conversation>, sqlx::Error> {
        let records = match &self.db.store {
            crate::db::DbStore::Postgres => {
                if let Some(i_id) = inbox_id {
                    sqlx::query_as::<_, Conversation>(
                        "SELECT id, tenant_id, inbox_id, contact_id, status, created_at, updated_at FROM conversations WHERE tenant_id = $1 AND inbox_id = $2",
                    )
                    .bind(tenant_id)
                    .bind(i_id)
                    .fetch_all(&self.db.pool)
                    .await?
                } else {
                    sqlx::query_as::<_, Conversation>(
                        "SELECT id, tenant_id, inbox_id, contact_id, status, created_at, updated_at FROM conversations WHERE tenant_id = $1",
                    )
                    .bind(tenant_id)
                    .fetch_all(&self.db.pool)
                    .await?
                }
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                if let Some(i_id) = inbox_id {
                    sqlx::query_as::<_, Conversation>(
                        "SELECT id, tenant_id, inbox_id, contact_id, status, created_at, updated_at FROM conversations WHERE tenant_id = ? AND inbox_id = ?",
                    )
                    .bind(tenant_id)
                    .bind(i_id)
                    .fetch_all(sqlite_pool)
                    .await?
                } else {
                    sqlx::query_as::<_, Conversation>(
                        "SELECT id, tenant_id, inbox_id, contact_id, status, created_at, updated_at FROM conversations WHERE tenant_id = ?",
                    )
                    .bind(tenant_id)
                    .fetch_all(sqlite_pool)
                    .await?
                }
            }
        };
        Ok(records)
    }

    pub async fn get_conversation_by_contact(&self, tenant_id: Uuid, inbox_id: Uuid, contact_id: Uuid) -> Result<Option<Conversation>, sqlx::Error> {
        let record = match &self.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query_as::<_, Conversation>(
                    "SELECT id, tenant_id, inbox_id, contact_id, status, created_at, updated_at FROM conversations WHERE tenant_id = $1 AND inbox_id = $2 AND contact_id = $3 LIMIT 1",
                )
                .bind(tenant_id)
                .bind(inbox_id)
                .bind(contact_id)
                .fetch_optional(&self.db.pool)
                .await?
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as::<_, Conversation>(
                    "SELECT id, tenant_id, inbox_id, contact_id, status, created_at, updated_at FROM conversations WHERE tenant_id = ? AND inbox_id = ? AND contact_id = ? LIMIT 1",
                )
                .bind(tenant_id)
                .bind(inbox_id)
                .bind(contact_id)
                .fetch_optional(sqlite_pool)
                .await?
            }
        };
        Ok(record)
    }

    pub async fn create_message(&self, tenant_id: Uuid, conversation_id: Uuid, content: String, sender_type: String) -> Result<Message, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = match &self.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query_as::<_, Message>(
                    "INSERT INTO messages (id, tenant_id, conversation_id, content, sender_type) VALUES ($1, $2, $3, $4, $5) RETURNING id, tenant_id, conversation_id, content, sender_type, created_at, updated_at",
                )
                .bind(id)
                .bind(tenant_id)
                .bind(conversation_id)
                .bind(content)
                .bind(sender_type)
                .fetch_one(&self.db.pool)
                .await?
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as::<_, Message>(
                    "INSERT INTO messages (id, tenant_id, conversation_id, content, sender_type) VALUES (?, ?, ?, ?, ?) RETURNING id, tenant_id, conversation_id, content, sender_type, created_at, updated_at",
                )
                .bind(id)
                .bind(tenant_id)
                .bind(conversation_id)
                .bind(content)
                .bind(sender_type)
                .fetch_one(sqlite_pool)
                .await?
            }
        };
        Ok(record)
    }

    pub async fn get_messages(&self, tenant_id: Uuid, conversation_id: Uuid) -> Result<Vec<Message>, sqlx::Error> {
        let records = match &self.db.store {
            crate::db::DbStore::Postgres => {
                sqlx::query_as::<_, Message>(
                    "SELECT id, tenant_id, conversation_id, content, sender_type, created_at, updated_at FROM messages WHERE tenant_id = $1 AND conversation_id = $2 ORDER BY created_at ASC",
                )
                .bind(tenant_id)
                .bind(conversation_id)
                .fetch_all(&self.db.pool)
                .await?
            },
            crate::db::DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as::<_, Message>(
                    "SELECT id, tenant_id, conversation_id, content, sender_type, created_at, updated_at FROM messages WHERE tenant_id = ? AND conversation_id = ? ORDER BY created_at ASC",
                )
                .bind(tenant_id)
                .bind(conversation_id)
                .fetch_all(sqlite_pool)
                .await?
            }
        };
        Ok(records)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    async fn setup_test_db() -> Arc<DB> {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS contacts (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                identifier TEXT NOT NULL,
                email TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(tenant_id, identifier)
            );
            CREATE TABLE IF NOT EXISTS inboxes (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                name TEXT NOT NULL,
                enable_auto_assignment BOOLEAN DEFAULT false,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS channel_adapters (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                inbox_id TEXT,
                type TEXT NOT NULL,
                config JSON,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS conversations (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                inbox_id TEXT NOT NULL,
                contact_id TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'open',
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                conversation_id TEXT NOT NULL,
                content TEXT NOT NULL,
                sender_type TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        Arc::new(DB {
            pool: sqlx::PgPool::connect_lazy("postgres://dummy").unwrap(),
            store: crate::db::DbStore::Sqlite(pool),
        })
    }

    #[tokio::test]
    async fn test_omnichannel_repo_full_flow() {
        let db = setup_test_db().await;
        let repo = OmniChannelRepo::new(db);
        let tenant_id = Uuid::new_v4();

        // 1. Create and get Contact
        let contact = repo.create_contact(tenant_id, "user123".to_string(), Some("test@example.com".to_string())).await.unwrap();
        assert_eq!(contact.identifier, "user123");
        assert_eq!(contact.email.as_deref(), Some("test@example.com"));

        let fetched_contact = repo.get_contact(tenant_id, "user123".to_string()).await.unwrap().unwrap();
        assert_eq!(fetched_contact.id, contact.id);

        // 1.b Test upsert contact
        let updated_contact = repo.create_contact(tenant_id, "user123".to_string(), Some("new@example.com".to_string())).await.unwrap();
        assert_eq!(updated_contact.id, contact.id);
        assert_eq!(updated_contact.email.as_deref(), Some("new@example.com"));

        // 2. Create and get Inbox
        let inbox = repo.create_inbox(tenant_id, "Main Inbox".to_string(), true).await.unwrap();
        assert_eq!(inbox.name, "Main Inbox");
        assert_eq!(inbox.enable_auto_assignment, Some(true));

        let inboxes = repo.get_inboxes(tenant_id).await.unwrap();
        assert_eq!(inboxes.len(), 1);
        assert_eq!(inboxes[0].id, inbox.id);

        // 3. Create Channel Adapter
        let adapter = repo.create_channel_adapter(tenant_id, Some(inbox.id), "whatsapp".to_string(), serde_json::json!({"token": "123"})).await.unwrap();
        assert_eq!(adapter.r#type, "whatsapp");

        // 4. Create and get Conversation
        let convo = repo.create_conversation(tenant_id, inbox.id, contact.id, "open".to_string()).await.unwrap();
        assert_eq!(convo.status, "open");

        let convos = repo.get_conversations(tenant_id, Some(inbox.id)).await.unwrap();
        assert_eq!(convos.len(), 1);
        assert_eq!(convos[0].id, convo.id);

        let convos_all = repo.get_conversations(tenant_id, None).await.unwrap();
        assert_eq!(convos_all.len(), 1);

        let fetched_convo = repo.get_conversation_by_contact(tenant_id, inbox.id, contact.id).await.unwrap().unwrap();
        assert_eq!(fetched_convo.id, convo.id);

        // 5. Create and get Message
        let msg = repo.create_message(tenant_id, convo.id, "Hello world".to_string(), "contact".to_string()).await.unwrap();
        assert_eq!(msg.content, "Hello world");
        assert_eq!(msg.sender_type, "contact");

        let messages = repo.get_messages(tenant_id, convo.id).await.unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].id, msg.id);
    }
}
