use sqlx::PgPool;
use uuid::Uuid;
use super::models::{ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage};

pub struct ChatService {
    pool: PgPool,
}

impl ChatService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_inbox(
        &self,
        tenant_id: Uuid,
        name: String,
    ) -> Result<ChatInbox, sqlx::Error> {
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
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_channel(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        channel_type: String,
        config: serde_json::Value,
    ) -> Result<ChatChannel, sqlx::Error> {
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
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_contact(
        &self,
        tenant_id: Uuid,
        name: Option<String>,
        email: Option<String>,
        phone: Option<String>,
    ) -> Result<ChatContact, sqlx::Error> {
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
        .fetch_one(&self.pool)
        .await
    }

    pub async fn start_conversation(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        contact_id: Uuid,
        assignee_id: Option<Uuid>,
    ) -> Result<ChatConversation, sqlx::Error> {
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
        .fetch_one(&self.pool)
        .await
    }

    pub async fn send_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        sender_type: String,
        sender_id: Option<Uuid>,
        content: String,
    ) -> Result<ChatMessage, sqlx::Error> {
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
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_macro(
        &self,
        tenant_id: Uuid,
        name: String,
        visibility: String,
        actions: serde_json::Value,
    ) -> Result<super::models::ChatMacro, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_macros (id, tenant_id, name, visibility, actions)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, name, visibility, actions, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
        .bind(visibility)
        .bind(actions)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_macros(
        &self,
        tenant_id: Uuid,
    ) -> Result<Vec<super::models::ChatMacro>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, tenant_id, name, visibility, actions, created_at, updated_at
            FROM chat_macros
            WHERE tenant_id = $1
            "#
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn create_canned_response(
        &self,
        tenant_id: Uuid,
        short_code: String,
        content: String,
    ) -> Result<super::models::ChatCannedResponse, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_canned_responses (id, tenant_id, short_code, content)
            VALUES ($1, $2, $3, $4)
            RETURNING id, tenant_id, short_code, content, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(short_code)
        .bind(content)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_canned_responses(
        &self,
        tenant_id: Uuid,
    ) -> Result<Vec<super::models::ChatCannedResponse>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, tenant_id, short_code, content, created_at, updated_at
            FROM chat_canned_responses
            WHERE tenant_id = $1
            "#
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn execute_macro(
        &self,
        tenant_id: Uuid,
        macro_id: Uuid,
        conversation_id: Uuid,
    ) -> Result<(), sqlx::Error> {
        let m: super::models::ChatMacro = sqlx::query_as(
            r#"
            SELECT id, tenant_id, name, visibility, actions, created_at, updated_at
            FROM chat_macros
            WHERE id = $1 AND tenant_id = $2
            "#
        )
        .bind(macro_id)
        .bind(tenant_id)
        .fetch_one(&self.pool)
        .await?;

        if let Some(actions) = m.actions.as_array() {
            for action in actions {
                if let Some(obj) = action.as_object() {
                    let action_name = obj.get("action_name").and_then(|v| v.as_str()).unwrap_or("");
                    match action_name {
                        "send_message" => {
                            if let Some(content) = obj.get("action_params").and_then(|v| v.as_array()).and_then(|a| a.first()).and_then(|v| v.as_str()) {
                                self.send_message(
                                    tenant_id,
                                    conversation_id,
                                    "agent".to_string(),
                                    None, // optionally we can capture the agent id
                                    content.to_string(),
                                ).await?;
                            }
                        }
                        "assign_agent" => {
                            if let Some(agent_id_str) = obj.get("action_params").and_then(|v| v.as_array()).and_then(|a| a.first()).and_then(|v| v.as_str()) {
                                if let Ok(agent_id) = Uuid::parse_str(agent_id_str) {
                                     sqlx::query(
                                        r#"
                                        UPDATE chat_conversations
                                        SET assignee_id = $1, updated_at = NOW()
                                        WHERE id = $2 AND tenant_id = $3
                                        "#
                                    )
                                    .bind(agent_id)
                                    .bind(conversation_id)
                                    .bind(tenant_id)
                                    .execute(&self.pool)
                                    .await?;
                                }
                            }
                        }
                        "change_status" => {
                            if let Some(status) = obj.get("action_params").and_then(|v| v.as_array()).and_then(|a| a.first()).and_then(|v| v.as_str()) {
                                 sqlx::query(
                                    r#"
                                    UPDATE chat_conversations
                                    SET status = $1, updated_at = NOW()
                                    WHERE id = $2 AND tenant_id = $3
                                    "#
                                )
                                .bind(status)
                                .bind(conversation_id)
                                .bind(tenant_id)
                                .execute(&self.pool)
                                .await?;
                            }
                        }
                        "resolve_conversation" => {
                             sqlx::query(
                                r#"
                                UPDATE chat_conversations
                                SET status = 'resolved', updated_at = NOW()
                                WHERE id = $1 AND tenant_id = $2
                                "#
                            )
                            .bind(conversation_id)
                            .bind(tenant_id)
                            .execute(&self.pool)
                            .await?;
                        }
                        _ => {
                            // unsupported action, skip for now
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
