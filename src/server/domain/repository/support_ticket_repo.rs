use crate::domain::repository::models::{SupportTicket, TicketMessage};
use sqlx::{Pool, Postgres};

pub struct SupportTicketRepository {
    pool: Pool<Postgres>,
}

impl SupportTicketRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create_ticket(
        &self,
        tenant_id: &str,
        channel: &str,
        external_message_id: Option<&str>,
        customer_id: Option<&str>,
    ) -> Result<SupportTicket, sqlx::Error> {
        let ticket = sqlx::query_as!(
            SupportTicket,
            r#"
            INSERT INTO support_tickets (tenant_id, channel, external_message_id, customer_id, status)
            VALUES ($1, $2, $3, $4, 'open')
            RETURNING
                id::TEXT as "id!",
                tenant_id::TEXT as "tenant_id!",
                customer_id::TEXT as customer_id,
                channel,
                external_message_id,
                status,
                created_at,
                updated_at
            "#,
            tenant_id,
            channel,
            external_message_id,
            customer_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(ticket)
    }

    pub async fn add_message(
        &self,
        ticket_id: &str,
        sender_type: &str,
        content: &str,
        ai_confidence: Option<f64>,
    ) -> Result<TicketMessage, sqlx::Error> {
        let message = sqlx::query_as!(
            TicketMessage,
            r#"
            INSERT INTO ticket_messages (ticket_id, sender_type, content, ai_confidence)
            VALUES ($1, $2, $3, $4)
            RETURNING
                id::TEXT as "id!",
                ticket_id::TEXT as "ticket_id!",
                sender_type,
                content,
                ai_confidence::FLOAT8 as ai_confidence,
                created_at
            "#,
            ticket_id,
            sender_type,
            content,
            ai_confidence
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(message)
    }

    pub async fn list_open_tickets(&self, tenant_id: &str) -> Result<Vec<SupportTicket>, sqlx::Error> {
        let tickets = sqlx::query_as!(
            SupportTicket,
            r#"
            SELECT
                id::TEXT as "id!",
                tenant_id::TEXT as "tenant_id!",
                customer_id::TEXT as customer_id,
                channel,
                external_message_id,
                status,
                created_at,
                updated_at
            FROM support_tickets
            WHERE tenant_id = $1 AND status IN ('open', 'draft')
            ORDER BY created_at DESC
            "#,
            tenant_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(tickets)
    }

    pub async fn get_ticket_messages(&self, ticket_id: &str) -> Result<Vec<TicketMessage>, sqlx::Error> {
        let messages = sqlx::query_as!(
            TicketMessage,
            r#"
            SELECT
                id::TEXT as "id!",
                ticket_id::TEXT as "ticket_id!",
                sender_type,
                content,
                ai_confidence::FLOAT8 as ai_confidence,
                created_at
            FROM ticket_messages
            WHERE ticket_id = $1
            ORDER BY created_at ASC
            "#,
            ticket_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(messages)
    }

    pub async fn update_ticket_status(&self, ticket_id: &str, status: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE support_tickets
            SET status = $1, updated_at = NOW()
            WHERE id = $2::UUID
            "#,
            status,
            ticket_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
