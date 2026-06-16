use sqlx::PgPool;

pub struct InboxHandler;

impl InboxHandler {
    pub async fn handle_ambassador_reply(pool: &PgPool, tenant_id: &str, inbox_id: &str, draft_reply: &str) -> Result<(), Box<dyn std::error::Error>> {
        tracing::info!("Approved ambassador reply for inbox message: {}", inbox_id);
        let _ = sqlx::query("UPDATE inbox_messages SET status = 'replied' WHERE id = $1 AND tenant_id = $2")
            .bind(inbox_id)
            .bind(tenant_id)
            .execute(pool)
            .await?;

        tracing::info!("Approved Ambassador draft reply for inbox_id: {}", inbox_id);
        let _ = sqlx::query("UPDATE omni_inbox_messages SET status = 'sent', draft_reply = $1 WHERE id = $2 AND tenant_id = $3")
            .bind(draft_reply)
            .bind(inbox_id)
            .bind(tenant_id)
            .execute(pool)
            .await?;

        Ok(())
    }
}
