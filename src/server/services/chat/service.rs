use sqlx::{PgPool, Result as SqlxResult};
use uuid::Uuid;

use super::models::{ChatContact, ChatConversation, ChatInbox, ChatMessage};

#[derive(Clone)]
pub struct ChatService {
    pool: PgPool,
}

impl ChatService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_inbox(
        &self,
        tenant_id: &str,
        name: &str,
        channel_type: &str,
        config: serde_json::Value,
    ) -> SqlxResult<ChatInbox> {
        let id = Uuid::new_v4().to_string();
        sqlx::query_as::<_, ChatInbox>(
            r#"
            INSERT INTO chat_inboxes (id, tenant_id, name, channel_type, config)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, name, channel_type, config, is_active, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(tenant_id)
        .bind(name)
        .bind(channel_type)
        .bind(config)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_inboxes(&self, tenant_id: &str) -> SqlxResult<Vec<ChatInbox>> {
        sqlx::query_as::<_, ChatInbox>(
            r#"
            SELECT id, tenant_id, name, channel_type, config, is_active, created_at, updated_at
            FROM chat_inboxes
            WHERE tenant_id = $1
            "#,
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn create_or_update_contact(
        &self,
        tenant_id: &str,
        identifier: &str, // email or phone
        name: Option<&str>,
    ) -> SqlxResult<ChatContact> {
        let existing = sqlx::query_as::<_, ChatContact>(
            r#"
            SELECT id, tenant_id, name, email, phone_number, avatar_url, custom_attributes, created_at, updated_at
            FROM chat_contacts
            WHERE tenant_id = $1 AND (email = $2 OR phone_number = $2)
            LIMIT 1
            "#,
        )
        .bind(tenant_id)
        .bind(identifier)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(mut contact) = existing {
            if let Some(new_name) = name {
                contact.name = Some(new_name.to_string());
                sqlx::query(
                    r#"
                    UPDATE chat_contacts SET name = $1, updated_at = CURRENT_TIMESTAMP
                    WHERE id = $2
                    "#,
                )
                .bind(new_name)
                .bind(contact.id.clone())
                .execute(&self.pool)
                .await?;
            }
            Ok(contact)
        } else {
            let id = Uuid::new_v4().to_string();
            let is_email = identifier.contains('@');
            let email = if is_email { Some(identifier) } else { None };
            let phone = if !is_email { Some(identifier) } else { None };

            sqlx::query_as::<_, ChatContact>(
                r#"
                INSERT INTO chat_contacts (id, tenant_id, name, email, phone_number)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id, tenant_id, name, email, phone_number, avatar_url, custom_attributes, created_at, updated_at
                "#,
            )
            .bind(id)
            .bind(tenant_id)
            .bind(name)
            .bind(email)
            .bind(phone)
            .fetch_one(&self.pool)
            .await
        }
    }

    pub async fn create_conversation(
        &self,
        tenant_id: &str,
        inbox_id: &str,
        contact_id: &str,
    ) -> SqlxResult<ChatConversation> {
        let id = Uuid::new_v4().to_string();
        sqlx::query_as::<_, ChatConversation>(
            r#"
            INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id)
            VALUES ($1, $2, $3, $4)
            RETURNING id, tenant_id, inbox_id, contact_id, status, assignee_id, unread_count, custom_attributes, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn send_message(
        &self,
        tenant_id: &str,
        conversation_id: &str,
        sender_type: &str,
        sender_id: Option<&str>,
        content: &str,
    ) -> SqlxResult<ChatMessage> {
        let id = Uuid::new_v4().to_string();

        let mut tx = self.pool.begin().await?;

        let message = sqlx::query_as::<_, ChatMessage>(
            r#"
            INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, tenant_id, conversation_id, sender_type, sender_id, content, content_type, status, metadata, created_at, updated_at
            "#,
        )
        .bind(id)
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(sender_type)
        .bind(sender_id)
        .bind(content)
        .fetch_one(&mut *tx)
        .await?;

        // Update conversation unread count and updated_at
        sqlx::query(
            r#"
            UPDATE chat_conversations
            SET unread_count = unread_count + 1, updated_at = CURRENT_TIMESTAMP
            WHERE id = $1
            "#,
        )
        .bind(conversation_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(message)
    }

    pub async fn get_conversation_messages(
        &self,
        tenant_id: &str,
        conversation_id: &str,
    ) -> SqlxResult<Vec<ChatMessage>> {
        sqlx::query_as::<_, ChatMessage>(
            r#"
            SELECT id, tenant_id, conversation_id, sender_type, sender_id, content, content_type, status, metadata, created_at, updated_at
            FROM chat_messages
            WHERE tenant_id = $1 AND conversation_id = $2
            ORDER BY created_at ASC
            "#,
        )
        .bind(tenant_id)
        .bind(conversation_id)
        .fetch_all(&self.pool)
        .await
    }
}
