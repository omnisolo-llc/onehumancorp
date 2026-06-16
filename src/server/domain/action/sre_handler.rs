use async_trait::async_trait;
use sqlx::PgPool;
use crate::domain::action::{ActionHandler, ActionIntent};

pub struct SreHandler;

#[async_trait]
impl ActionHandler for SreHandler {
    async fn execute(&self, pool: &PgPool, tenant_id: &str, intent: &ActionIntent) -> Result<(), String> {
        if let Some(incident_id) = intent.payload.get("incident_id").and_then(|v| v.as_str()) {
            let result = sqlx::query("UPDATE incidents SET status = 'RESOLVED', updated_at = NOW() WHERE id = $1 AND tenant_id = $2")
                .bind(incident_id)
                .bind(tenant_id)
                .execute(pool)
                .await;

            if let Err(e) = result {
                tracing::error!("Failed to update incident resolution: {}", e);
                return Err(e.to_string());
            }
        }
        Ok(())
    }
}
