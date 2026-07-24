use sqlx::PgPool;
use uuid::Uuid;
use crate::ohc::inbox::{Conversation, OmniMessage, CreateMessageRequest};

#[derive(Clone)]
pub struct InboxDao {
    pub pool: PgPool,
}

impl InboxDao {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn fetch_conversations(&self, tenant_id: Uuid) -> Result<Vec<Conversation>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"
            SELECT id, tenant_id, contact_id, inbox_id, channel_id, status, ai_handoff_state,
                   snoozed_until_unix, contact_last_seen_at_unix, agent_last_seen_at_unix,
                   created_at_unix, updated_at_unix
            FROM conversations
            WHERE tenant_id = $1
            ORDER BY updated_at_unix DESC
            "#,
            tenant_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut conversations = Vec::new();
        for row in rows {
            conversations.push(Conversation {
                id: row.id.to_string(),
                tenant_id: row.tenant_id.to_string(),
                contact_id: row.contact_id.to_string(),
                inbox_id: row.inbox_id.to_string(),
                channel_id: row.channel_id.to_string(),
                status: row.status as i32,
                ai_handoff_state: row.ai_handoff_state as i32,
                assigned_agent_id: "".to_string(), // Omitted for simplicity
                snoozed_until_unix: row.snoozed_until_unix.unwrap_or(0),
                contact_last_seen_at_unix: row.contact_last_seen_at_unix.unwrap_or(0),
                agent_last_seen_at_unix: row.agent_last_seen_at_unix.unwrap_or(0),
                created_at_unix: row.created_at_unix,
                updated_at_unix: row.updated_at_unix,
            });
        }
        Ok(conversations)
    }

    pub async fn fetch_messages(&self, tenant_id: Uuid, conversation_id: Uuid) -> Result<Vec<OmniMessage>, sqlx::Error> {
        let rows = sqlx::query!(
            r#"
            SELECT id, tenant_id, conversation_id, sender_id, message_type, content_type,
                   original_content, translated_content, source_language, target_language,
                   content_attributes_json, draft_reply, status, created_at_unix, updated_at_unix
            FROM messages
            WHERE tenant_id = $1 AND conversation_id = $2
            ORDER BY created_at_unix ASC
            "#,
            tenant_id,
            conversation_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut messages = Vec::new();
        for row in rows {
            messages.push(OmniMessage {
                id: row.id.to_string(),
                tenant_id: row.tenant_id.to_string(),
                conversation_id: row.conversation_id.to_string(),
                sender_id: row.sender_id.map(|u| u.to_string()).unwrap_or_default(),
                message_type: row.message_type as i32,
                content_type: row.content_type as i32,
                original_content: row.original_content.unwrap_or_default(),
                translated_content: row.translated_content.unwrap_or_default(),
                source_language: row.source_language.unwrap_or_default(),
                target_language: row.target_language.unwrap_or_default(),
                content_attributes_json: row.content_attributes_json.map(|v| v.to_string()).unwrap_or_default(),
                draft_reply: row.draft_reply.unwrap_or_default(),
                status: row.status.unwrap_or_default(),
                created_at_unix: row.created_at_unix,
                updated_at_unix: row.updated_at_unix,
            });
        }
        Ok(messages)
    }

    pub async fn create_message(&self, tenant_id: Uuid, req: CreateMessageRequest) -> Result<OmniMessage, sqlx::Error> {
        let conversation_id = Uuid::parse_str(&req.conversation_id).unwrap_or_default();
        let sender_id = Uuid::parse_str(&req.sender_id).ok();

        let row = sqlx::query!(
            r#"
            INSERT INTO messages (
                tenant_id, conversation_id, sender_id, message_type, content_type,
                original_content, content_attributes_json, status, created_at_unix, updated_at_unix
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, 'sent', extract(epoch from now()), extract(epoch from now()))
            RETURNING id, tenant_id, conversation_id, sender_id, message_type, content_type,
                      original_content, translated_content, source_language, target_language,
                      content_attributes_json, draft_reply, status, created_at_unix, updated_at_unix
            "#,
            tenant_id,
            conversation_id,
            sender_id,
            req.message_type as i32,
            req.content_type as i32,
            req.content,
            sqlx::types::JsonValue::Null // Omitted detailed JSON parsing for now
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(OmniMessage {
            id: row.id.to_string(),
            tenant_id: row.tenant_id.to_string(),
            conversation_id: row.conversation_id.to_string(),
            sender_id: row.sender_id.map(|u| u.to_string()).unwrap_or_default(),
            message_type: row.message_type as i32,
            content_type: row.content_type as i32,
            original_content: row.original_content.unwrap_or_default(),
            translated_content: row.translated_content.unwrap_or_default(),
            source_language: row.source_language.unwrap_or_default(),
            target_language: row.target_language.unwrap_or_default(),
            content_attributes_json: row.content_attributes_json.map(|v| v.to_string()).unwrap_or_default(),
            draft_reply: row.draft_reply.unwrap_or_default(),
            status: row.status.unwrap_or_default(),
            created_at_unix: row.created_at_unix,
            updated_at_unix: row.updated_at_unix,
        })
    }

    pub async fn approve_draft(&self, tenant_id: Uuid, message_id: Uuid) -> Result<OmniMessage, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            UPDATE messages
            SET status = 'approved', updated_at_unix = extract(epoch from now())
            WHERE id = $1 AND tenant_id = $2
            RETURNING id, tenant_id, conversation_id, sender_id, message_type, content_type,
                      original_content, translated_content, source_language, target_language,
                      content_attributes_json, draft_reply, status, created_at_unix, updated_at_unix
            "#,
            message_id,
            tenant_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(OmniMessage {
            id: row.id.to_string(),
            tenant_id: row.tenant_id.to_string(),
            conversation_id: row.conversation_id.to_string(),
            sender_id: row.sender_id.map(|u| u.to_string()).unwrap_or_default(),
            message_type: row.message_type as i32,
            content_type: row.content_type as i32,
            original_content: row.original_content.unwrap_or_default(),
            translated_content: row.translated_content.unwrap_or_default(),
            source_language: row.source_language.unwrap_or_default(),
            target_language: row.target_language.unwrap_or_default(),
            content_attributes_json: row.content_attributes_json.map(|v| v.to_string()).unwrap_or_default(),
            draft_reply: row.draft_reply.unwrap_or_default(),
            status: row.status.unwrap_or_default(),
            created_at_unix: row.created_at_unix,
            updated_at_unix: row.updated_at_unix,
        })
    }
}
