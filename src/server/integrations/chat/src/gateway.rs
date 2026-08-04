use crate::models::*;
use sqlx::PgPool;
use uuid::Uuid;

pub struct OmnichannelGateway {
    pool: PgPool,
}

impl OmnichannelGateway {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn process_webhook(&self, tenant_id: Uuid, payload: WebhookPayload) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        let contact_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO chat_contacts (id, tenant_id, name, phone) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING"
        )
        .bind(contact_id)
        .bind(tenant_id)
        .bind(&payload.sender_id) // Placeholder for name
        .bind(&payload.sender_id)
        .execute(&mut *tx)
        .await?;

        // Simplified for testing: assume an inbox exists or create a default one
        let inbox_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO chat_inboxes (id, tenant_id, name) VALUES ($1, $2, 'Default Inbox') ON CONFLICT DO NOTHING"
        )
        .bind(inbox_id)
        .bind(tenant_id)
        .execute(&mut *tx)
        .await?;

        let conversation_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, status) VALUES ($1, $2, $3, $4, 'open')"
        )
        .bind(conversation_id)
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .execute(&mut *tx)
        .await?;

        let message_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, content) VALUES ($1, $2, $3, 'contact', $4)"
        )
        .bind(message_id)
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(&payload.content)
        .execute(&mut *tx)
        .await?;

        let draft_id = Uuid::new_v4();
        let drafted_response = format!("Auto-drafted reply to: {}", payload.content);

        // create work_item for the agent_draft
        let work_item_id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO work_item (id, tenant_id, customer_id, source, status) VALUES ($1, $2, $3, $4, 'OPEN')"
        )
        .bind(work_item_id)
        .bind(tenant_id)
        .bind(contact_id) // Assuming chat_contact can be mapped to work_item customer_id or bypassing for now
        .bind(&payload.channel)
        .execute(&mut *tx)
        .await?;

        sqlx::query(
            "INSERT INTO agent_draft (id, work_item_id, response, status) VALUES ($1, $2, $3, 'DRAFT')"
        )
        .bind(draft_id)
        .bind(work_item_id)
        .bind(drafted_response)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }
}
