use async_trait::async_trait;
use sqlx::PgPool;
use crate::domain::action::{ActionHandler, ActionIntent};

pub struct InboxHandler;

#[async_trait]
impl ActionHandler for InboxHandler {
    async fn execute(&self, pool: &PgPool, tenant_id: &str, intent: &ActionIntent) -> Result<(), String> {
        if let Some(inbox_id) = intent.payload.get("inbox_message_id").and_then(|v| v.as_str()) {
            tracing::info!("Approved ambassador reply for inbox message: {}", inbox_id);
            let result1 = sqlx::query("UPDATE inbox_messages SET status = 'replied' WHERE id = $1 AND tenant_id = $2")
                .bind(inbox_id)
                .bind(tenant_id)
                .execute(pool)
                .await;

            if let Err(e) = result1 {
                tracing::error!("Failed to update inbox_messages: {}", e);
                return Err(e.to_string());
            }

            let draft_reply = intent.payload.get("draft_reply").and_then(|v| v.as_str()).unwrap_or("");
            let result2 = sqlx::query("UPDATE omni_inbox_messages SET status = 'sent', draft_reply = $1 WHERE id = $2 AND tenant_id = $3")
                .bind(draft_reply)
                .bind(inbox_id)
                .bind(tenant_id)
                .execute(pool)
                .await;

            if let Err(e) = result2 {
                tracing::error!("Failed to update omni_inbox_messages: {}", e);
                return Err(e.to_string());
            }
        }
        Ok(())
    }
}
