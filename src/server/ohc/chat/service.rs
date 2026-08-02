use uuid::Uuid;
use sqlx::{PgPool, Row, Transaction, Postgres};
use chrono::Utc;
use crate::ohc::chat::models::*;

pub struct ChatService {
    pool: PgPool,
}

impl ChatService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn begin_tenant_tx<'a>(&self, tenant_id: Uuid) -> Result<Transaction<'a, Postgres>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant_id', $1, true)")
            .bind(tenant_id.to_string())
            .execute(&mut *tx)
            .await?;
        Ok(tx)
    }

    pub async fn create_inbox(&self, tenant_id: Uuid, name: &str) -> Result<ChatInbox, sqlx::Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let mut tx = self.begin_tenant_tx(tenant_id).await?;

        sqlx::query(
            r#"
            INSERT INTO chat_inboxes (id, tenant_id, name, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            "#
        )
        .bind(id)
        .bind(tenant_id)
        .bind(name)
        .bind(now)
        .bind(now)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(ChatInbox {
            id,
            tenant_id,
            name: name.to_string(),
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn send_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        sender_type: &str,
        sender_id: Option<Uuid>,
        content: &str,
    ) -> Result<ChatMessage, sqlx::Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let mut tx = self.begin_tenant_tx(tenant_id).await?;

        sqlx::query(
            r#"
            INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#
        )
        .bind(id)
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(sender_type)
        .bind(sender_id)
        .bind(content)
        .bind(now)
        .bind(now)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(ChatMessage {
            id,
            tenant_id,
            conversation_id,
            sender_type: sender_type.to_string(),
            sender_id,
            content: content.to_string(),
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn create_contact(&self, tenant_id: Uuid, name: Option<&str>, email: Option<&str>, phone: Option<&str>) -> Result<ChatContact, sqlx::Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let mut tx = self.begin_tenant_tx(tenant_id).await?;

        sqlx::query(
            r#"
            INSERT INTO chat_contacts (id, tenant_id, name, email, phone, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#
        )
        .bind(id)
        .bind(tenant_id)
        .bind(name)
        .bind(email)
        .bind(phone)
        .bind(now)
        .bind(now)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(ChatContact {
            id,
            tenant_id,
            name: name.map(|s| s.to_string()),
            email: email.map(|s| s.to_string()),
            phone: phone.map(|s| s.to_string()),
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn create_conversation(&self, tenant_id: Uuid, inbox_id: Uuid, contact_id: Uuid) -> Result<ChatConversation, sqlx::Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        let mut tx = self.begin_tenant_tx(tenant_id).await?;

        sqlx::query(
            r#"
            INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, 'open', $5, $6)
            "#
        )
        .bind(id)
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .bind(now)
        .bind(now)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(ChatConversation {
            id,
            tenant_id,
            inbox_id,
            contact_id,
            assignee_id: None,
            status: "open".to_string(),
            created_at: now,
            updated_at: now,
        })
    }
}
