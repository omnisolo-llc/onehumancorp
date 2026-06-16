use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::action::{ActionHandler, ActionIntent};

pub struct SalesHandler;

#[async_trait]
impl ActionHandler for SalesHandler {
    async fn execute(&self, pool: &PgPool, tenant_id: &str, intent: &ActionIntent) -> Result<(), String> {
        if let Some(quote_id_str) = intent.payload.get("quote_id").and_then(|v| v.as_str()) {
            let quote_id = Uuid::parse_str(quote_id_str).unwrap_or_default();
            tracing::info!("Approved quote draft: {}", quote_id);
            let result = sqlx::query("UPDATE quotes SET status = 'SENT', updated_at = NOW() WHERE id = $1 AND tenant_id = $2")
                .bind(quote_id)
                .bind(tenant_id)
                .execute(pool)
                .await;

            if let Err(e) = result {
                tracing::error!("Failed to update quotes: {}", e);
                return Err(e.to_string());
            }
        }
        Ok(())
    }
}
