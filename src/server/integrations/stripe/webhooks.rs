use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use serde_json::Value;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StripeWebhookEvent {
    pub id: Option<String>,
    pub r#type: Option<String>,
    pub data: Option<StripeWebhookData>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StripeWebhookData {
    pub object: Option<Value>,
}

pub fn verify_signature(_payload: &str, _signature: &str, _secret: &str) -> bool {
    // In a real application, verify the Stripe webhook signature here.
    // This is a stub for the architecture implementation.
    true
}

pub async fn enqueue_webhook_event(
    pool: &PgPool,
    tenant_id: &str,
    event: &StripeWebhookEvent,
) -> Result<(), sqlx::Error> {
    let payload = serde_json::to_value(event).unwrap();
    let id = uuid::Uuid::new_v4().to_string();

    // Use ohc_job_queue (as seen in migration 014) to process webhooks
    sqlx::query(
        r#"
        INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status)
        VALUES ($1, $2, 'stripe_webhook', $3, 'PENDING')
        "#
    )
    .bind(&id)
    .bind(tenant_id)
    .bind(&payload)
    .execute(pool)
    .await?;

    Ok(())
}
