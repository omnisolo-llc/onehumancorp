use sqlx::PgPool;
use serde_json::Value;
use uuid::Uuid;

pub async fn handle_milestone_action(tenant_id: &str, payload: &Value, pool: &PgPool) -> Result<(), sqlx::Error> {
    if let Some(action) = payload.get("action").and_then(|v| v.as_str()) {
        if action == "approve_milestone" {
            if let Some(milestone_id) = payload.get("milestone_id").and_then(|v| v.as_str()) {
                tracing::info!("Approved milestone for billing: {}", milestone_id);

                // 1. Mark milestone complete
                sqlx::query("UPDATE milestones SET status = 'COMPLETED', updated_at = NOW() WHERE id = $1 AND tenant_id = $2")
                    .bind(Uuid::parse_str(milestone_id).unwrap_or_default())
                    .bind(tenant_id)
                    .execute(pool)
                    .await?;

                // 2. Mock sending the invoice (in reality we would call Stripe here)
                tracing::info!("Finance Agent drafting and queueing Stripe invoice for milestone {}", milestone_id);

                // 3. Mark invoice as SENT
                sqlx::query("UPDATE milestone_invoices SET status = 'SENT', updated_at = NOW() WHERE milestone_id = $1 AND tenant_id = $2")
                    .bind(Uuid::parse_str(milestone_id).unwrap_or_default())
                    .bind(tenant_id)
                    .execute(pool)
                    .await?;
            }
        }
    }
    Ok(())
}
