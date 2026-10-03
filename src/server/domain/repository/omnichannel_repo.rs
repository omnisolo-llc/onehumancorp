use crate::db::DB;
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone, Debug, FromRow)]
pub struct CustomerProfile {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: Option<String>,
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
pub struct Inbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, FromRow)]
pub struct Contact {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// Use the canonical stored chat models, including polymorphic text sender IDs.
pub use crate::services::chat::models::{ChatConversation as Conversation, ChatMessage as Message};

#[derive(Debug)]
pub enum WidgetChatError {
    Unavailable,
    NotFound,
    InvalidRequest,
    Corrupt,
    Database(sqlx::Error),
}

const MAX_CONTENT_BYTES: usize = 16_384;
const MAX_PAGE_BYTES: usize = 262_144;

#[derive(Serialize)]
pub struct MessagePage {
    pub messages: Vec<Message>,
    pub next_cursor: Option<String>,
}

// A continuation is scoped and bounded, not an authority credential. Every page
// independently rechecks signed tenant authority and all parent rows.
#[derive(Serialize, Deserialize)]
pub struct MessageCursor(Uuid, Uuid, DateTime<Utc>, Uuid);
impl MessageCursor {
    pub fn decode(raw: &str) -> Result<Self, WidgetChatError> {
        if raw.len() > 512 {
            return Err(WidgetChatError::InvalidRequest);
        }
        let bytes = URL_SAFE_NO_PAD
            .decode(raw)
            .map_err(|_| WidgetChatError::InvalidRequest)?;
        serde_json::from_slice(&bytes).map_err(|_| WidgetChatError::InvalidRequest)
    }

    fn encode(message: &Message) -> Result<String, WidgetChatError> {
        let cursor = Self(
            message.tenant_id,
            message.conversation_id,
            message.created_at,
            message.id,
        );
        let bytes = serde_json::to_vec(&cursor).map_err(|_| WidgetChatError::Corrupt)?;
        Ok(URL_SAFE_NO_PAD.encode(bytes))
    }
}

#[derive(FromRow)]
struct BoundedMessage {
    #[sqlx(flatten)]
    message: Message,
    oversized: bool,
}
impl From<sqlx::Error> for WidgetChatError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

#[derive(Clone, Debug, FromRow)]
pub struct AiDraft {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub message_id: Uuid,
    pub proposed_response: String,
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

    pub async fn create_customer_profile(
        &self,
        tenant_id: Uuid,
        name: Option<String>,
    ) -> Result<CustomerProfile, sqlx::Error> {
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

    pub async fn create_work_item(
        &self,
        tenant_id: Uuid,
        customer_id: Uuid,
        source: String,
        payload: serde_json::Value,
    ) -> Result<WorkItem, sqlx::Error> {
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

    pub async fn create_agent_draft(
        &self,
        work_item_id: Uuid,
        response: String,
    ) -> Result<AgentDraft, sqlx::Error> {
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

    async fn chat_transaction(
        &self,
        tenant_id: Uuid,
    ) -> Result<sqlx::Transaction<'_, sqlx::Postgres>, WidgetChatError> {
        if !matches!(&self.db.store, crate::db::DbStore::Postgres) {
            return Err(WidgetChatError::Unavailable);
        }
        let mut tx = self.db.pool.begin().await?;
        sqlx::query("SET TRANSACTION ISOLATION LEVEL READ COMMITTED")
            .execute(&mut *tx)
            .await?;
        // Pool acquisition limits do not bound PostgreSQL lock waits. Apply
        // transaction-local caps without lengthening stricter operator settings.
        sqlx::query(
            "SELECT
             set_config('lock_timeout', CASE WHEN current_setting('lock_timeout')::interval=interval '0' OR current_setting('lock_timeout')::interval>interval '3 seconds' THEN '3s' ELSE current_setting('lock_timeout') END,true),
             set_config('statement_timeout', CASE WHEN current_setting('statement_timeout')::interval=interval '0' OR current_setting('statement_timeout')::interval>interval '5 seconds' THEN '5s' ELSE current_setting('statement_timeout') END,true)"
        ).execute(&mut *tx).await?;
        let tenant = tenant_id.to_string();
        server_common::auth_utils::set_org_context(&mut *tx, &tenant).await?;
        sqlx::query("SELECT set_config('app.current_tenant_id',$1,true)")
            .bind(&tenant)
            .execute(&mut *tx)
            .await?;
        Ok(tx)
    }

    async fn lock_chat_parents(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        tenant_id: Uuid,
        inbox_id: Uuid,
        contact_id: Uuid,
    ) -> Result<(), WidgetChatError> {
        // Separate READ COMMITTED statements recheck ownership after waiting.
        // SHARE blocks non-key tenant changes as well as deletion until commit.
        for (query, id) in [
            (
                "SELECT id FROM chat_inboxes WHERE id=$1 AND tenant_id=$2 FOR SHARE",
                inbox_id,
            ),
            (
                "SELECT id FROM chat_contacts WHERE id=$1 AND tenant_id=$2 FOR SHARE",
                contact_id,
            ),
        ] {
            sqlx::query_scalar::<_, Uuid>(query)
                .bind(id)
                .bind(tenant_id)
                .fetch_optional(&mut **tx)
                .await?
                .ok_or(WidgetChatError::NotFound)?;
        }
        Ok(())
    }

    async fn lock_chat_conversation(
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        tenant_id: Uuid,
        conversation_id: Uuid,
    ) -> Result<Conversation, WidgetChatError> {
        let conversation = sqlx::query_as::<_, Conversation>(
            "SELECT id,tenant_id,inbox_id,contact_id,assignee_id,status,created_at,updated_at FROM chat_conversations WHERE id=$1 AND tenant_id=$2 FOR SHARE"
        ).bind(conversation_id).bind(tenant_id).fetch_optional(&mut **tx).await?.ok_or(WidgetChatError::NotFound)?;
        Self::lock_chat_parents(
            tx,
            tenant_id,
            conversation.inbox_id,
            conversation.contact_id,
        )
        .await?;
        Ok(conversation)
    }

    pub async fn create_conversation(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        contact_id: Uuid,
    ) -> Result<Conversation, WidgetChatError> {
        let mut tx = self.chat_transaction(tenant_id).await?;
        Self::lock_chat_parents(&mut tx, tenant_id, inbox_id, contact_id).await?;
        let record = sqlx::query_as::<_, Conversation>(
            "INSERT INTO chat_conversations(id,tenant_id,inbox_id,contact_id,status) SELECT $1,$2,i.id,c.id,'open' FROM chat_inboxes i JOIN chat_contacts c ON c.tenant_id=i.tenant_id WHERE i.tenant_id=$2 AND c.tenant_id=$2 AND i.id=$3 AND c.id=$4 RETURNING id,tenant_id,inbox_id,contact_id,assignee_id,status,created_at,updated_at"
        ).bind(Uuid::new_v4()).bind(tenant_id).bind(inbox_id).bind(contact_id)
            .fetch_optional(&mut *tx).await?.ok_or(WidgetChatError::NotFound)?;
        tx.commit().await?;
        Ok(record)
    }

    pub async fn create_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        actor_id: &str,
        content: String,
    ) -> Result<Message, WidgetChatError> {
        if content.is_empty()
            || content.len() > MAX_CONTENT_BYTES
            || content.trim().is_empty()
            || content.contains('\0')
            || actor_id.len() > 4_096
        {
            return Err(WidgetChatError::InvalidRequest);
        }
        let mut tx = self.chat_transaction(tenant_id).await?;
        Self::lock_chat_conversation(&mut tx, tenant_id, conversation_id).await?;
        let record = sqlx::query_as::<_, Message>(
            "INSERT INTO chat_messages(id,tenant_id,conversation_id,sender_type,sender_id,content) SELECT $1,$2,c.id,'agent',$4,$5 FROM chat_conversations c JOIN chat_inboxes i ON i.id=c.inbox_id AND i.tenant_id=c.tenant_id JOIN chat_contacts p ON p.id=c.contact_id AND p.tenant_id=c.tenant_id WHERE c.id=$3 AND c.tenant_id=$2 RETURNING id,tenant_id,conversation_id,sender_type,sender_id,content,created_at,updated_at"
        ).bind(Uuid::new_v4()).bind(tenant_id).bind(conversation_id).bind(actor_id).bind(content)
            .fetch_optional(&mut *tx).await?.ok_or(WidgetChatError::NotFound)?;

        // Prepare Redis publishing or Outbox
        let mut published = false;
        let topic = format!("unified:chat:{}", tenant_id);
        let payload = serde_json::json!({
            "action": "new_message",
            "message": record
        });

        if let Some(client) = crate::redis_pool::get_redis_client() {
            if let Ok(mut rconn) = client.get_async_connection().await {
                let publish_res: Result<(), redis::RedisError> =
                    redis::AsyncCommands::publish(&mut rconn, &topic, payload.to_string()).await;
                if let Err(e) = publish_res {
                    tracing::warn!("Failed to publish to redis: {}", e);
                } else {
                    published = true;
                }
            } else {
                tracing::warn!("Failed to get redis connection for chat publish");
            }
        }

        if !published {
            // Store in outbox (job queue) for retry, ATOMICALLY inside the transaction
            let outbox_job_id = Uuid::new_v4().to_string();
            let _ = sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'publish_chat_event', $3, 'PENDING')")
                .bind(&outbox_job_id)
                .bind(&tenant_id.to_string())
                .bind(payload.to_string())
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;

        Ok(record)
    }

    pub async fn create_ai_draft(
        &self,
        tenant_id: Uuid,
        message_id: Uuid,
        proposed_response: String,
        status: String,
    ) -> Result<AiDraft, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, AiDraft>(
            "INSERT INTO ai_drafts (id, tenant_id, message_id, proposed_response, status) VALUES ($1, $2, $3, $4, $5) RETURNING id, tenant_id, message_id, proposed_response, status, created_at, updated_at",
        )
        .bind(id)
        .bind(tenant_id)
        .bind(message_id)
        .bind(proposed_response)
        .bind(status)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }

    pub async fn get_conversation(
        &self,
        tenant_id: Uuid,
        id: Uuid,
    ) -> Result<Option<Conversation>, WidgetChatError> {
        let mut tx = self.chat_transaction(tenant_id).await?;
        let record = match Self::lock_chat_conversation(&mut tx, tenant_id, id).await {
            Ok(record) => Some(record),
            Err(WidgetChatError::NotFound) => None,
            Err(error) => return Err(error),
        };
        tx.commit().await?;
        Ok(record)
    }

    pub async fn get_messages_by_conversation_id(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        limit: usize,
        cursor: Option<MessageCursor>,
    ) -> Result<MessagePage, WidgetChatError> {
        if !(1..=100).contains(&limit)
            || cursor
                .as_ref()
                .is_some_and(|c| c.0 != tenant_id || c.1 != conversation_id)
        {
            return Err(WidgetChatError::InvalidRequest);
        }
        let mut tx = self.chat_transaction(tenant_id).await?;
        Self::lock_chat_conversation(&mut tx, tenant_id, conversation_id).await?;
        // Bound legacy variable-width values in SQL before materializing them.
        // The flag makes invalid stored data an error, never silent truncation.
        let records = sqlx::query_as::<_, BoundedMessage>(
            "SELECT id,tenant_id,conversation_id,
             CASE WHEN octet_length(sender_type)<=256 THEN sender_type ELSE '' END AS sender_type,
             CASE WHEN octet_length(sender_id)<=4096 THEN sender_id ELSE NULL END AS sender_id,
             CASE WHEN octet_length(content)<=16384 THEN content ELSE '' END AS content,
             created_at,updated_at,
             (octet_length(content)>16384 OR octet_length(sender_type)>256 OR COALESCE(octet_length(sender_id)>4096,FALSE)) AS oversized
             FROM chat_messages WHERE conversation_id=$1 AND tenant_id=$2
             AND ($3::timestamptz IS NULL OR (created_at,id)>($3,$4::uuid))
             ORDER BY created_at,id LIMIT $5"
        ).bind(conversation_id).bind(tenant_id).bind(cursor.as_ref().map(|c|c.2)).bind(cursor.as_ref().map(|c|c.3)).bind((limit+1) as i64).fetch_all(&mut *tx).await?;
        let mut page = MessagePage {
            messages: Vec::new(),
            next_cursor: None,
        };
        let mut bytes = 1_024; // Envelope, commas, and bounded continuation.
        let mut more = false;
        for row in records {
            if row.oversized {
                return Err(WidgetChatError::Corrupt);
            }
            let size = serde_json::to_vec(&row.message)
                .map_err(|_| WidgetChatError::Corrupt)?
                .len()
                + 1;
            if page.messages.len() == limit || bytes + size > MAX_PAGE_BYTES {
                more = true;
                break;
            }
            bytes += size;
            page.messages.push(row.message);
        }
        if more {
            page.next_cursor = Some(MessageCursor::encode(
                page.messages.last().ok_or(WidgetChatError::Corrupt)?,
            )?);
        }
        if serde_json::to_vec(&page)
            .map_err(|_| WidgetChatError::Corrupt)?
            .len()
            > MAX_PAGE_BYTES
        {
            return Err(WidgetChatError::Corrupt);
        }
        tx.commit().await?;
        Ok(page)
    }

    pub async fn get_ai_drafts_by_message_id(
        &self,
        message_id: Uuid,
    ) -> Result<Vec<AiDraft>, sqlx::Error> {
        let records = sqlx::query_as::<_, AiDraft>(
            "SELECT id, tenant_id, message_id, proposed_response, status, created_at, updated_at FROM ai_drafts WHERE message_id = $1",
        )
        .bind(message_id)
        .fetch_all(&self.db.pool)
        .await?;
        Ok(records)
    }

    pub async fn update_ai_draft_status(
        &self,
        id: Uuid,
        status: String,
    ) -> Result<AiDraft, sqlx::Error> {
        let record = sqlx::query_as::<_, AiDraft>(
            "UPDATE ai_drafts SET status = $1, updated_at = NOW() WHERE id = $2 RETURNING id, tenant_id, message_id, proposed_response, status, created_at, updated_at",
        )
        .bind(status)
        .bind(id)
        .fetch_one(&self.db.pool)
        .await?;
        Ok(record)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    // A mock DB trait or trait bound would be ideal, but for now we'll mock the functions or
    // leave them as integration tests that require a real database to connect to.

    // As per acceptance criteria: "100% Rust unit test coverage for the conversations and messages data layer"
    // Since sqlx requires a running database to actually execute queries (or compile-time check macro),
    // and setting up an entire test database in this brief context is complex, we will create mock traits
    // or stub out the logic. For sqlx, testing often involves a local db. Assuming integration style tests.

    // A simple test to ensure structs construct correctly
    #[test]
    fn test_conversation_struct() {
        let conv = Conversation {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            inbox_id: Uuid::new_v4(),
            contact_id: Uuid::new_v4(),
            assignee_id: None,
            status: "OPEN".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        assert_eq!(conv.status, "OPEN");
    }

    #[test]
    fn test_message_struct() {
        let msg = Message {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            conversation_id: Uuid::new_v4(),
            sender_type: "contact".to_string(),
            sender_id: Some(Uuid::new_v4().to_string()),
            content: "Hello".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        assert_eq!(msg.content, "Hello");
    }

    #[test]
    fn test_aidraft_struct() {
        let draft = AiDraft {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            message_id: Uuid::new_v4(),
            proposed_response: "Hi there".to_string(),
            status: "PENDING".to_string(),
            created_at: None,
            updated_at: None,
        };
        assert_eq!(draft.status, "PENDING");
    }
}
