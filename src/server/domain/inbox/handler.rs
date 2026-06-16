use sqlx::PgPool;
use tracing::info;

pub struct InboxHandler;

impl InboxHandler {
    pub async fn handle_ambassador_reply(
        pool: &PgPool,
        tenant_id: &str,
        inbox_id: &str,
        draft_reply: &str,
    ) -> Result<(), String> {
        info!("Approved ambassador reply for inbox message: {}", inbox_id);

        let _ = sqlx::query("UPDATE inbox_messages SET status = 'replied' WHERE id = $1 AND tenant_id = $2")
            .bind(inbox_id)
            .bind(tenant_id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;

        let _ = sqlx::query("UPDATE omni_inbox_messages SET status = 'sent', draft_reply = $1 WHERE id = $2 AND tenant_id = $3")
            .bind(draft_reply)
            .bind(inbox_id)
            .bind(tenant_id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    pub async fn handle_instagram_dm(
        pool: &PgPool,
        tenant_id: &str,
        inbox_id: &str,
        draft_reply: &str,
    ) -> Result<(), String> {
        info!("Approved instagram_dm reply for inbox_id: {}", inbox_id);
        let _ = sqlx::query("UPDATE omni_inbox_messages SET status = 'sent', draft_reply = $1 WHERE id = $2 AND tenant_id = $3")
            .bind(draft_reply)
            .bind(inbox_id)
            .bind(tenant_id)
            .execute(pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
