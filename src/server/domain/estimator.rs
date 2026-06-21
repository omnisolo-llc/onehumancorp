use sqlx::PgPool;
use serde_json::{Value, json};
use uuid::Uuid;

pub async fn handle_proposal_action(tenant_id: &str, payload: &Value, pool: &PgPool) -> Result<(), sqlx::Error> {
    if let Some(action) = payload.get("action").and_then(|v| v.as_str()) {
        if action == "approve" {
            if let Some(proposal_id) = payload.get("proposal_id").and_then(|v| v.as_str()) {
                tracing::info!("Approved interactive proposal draft: {}", proposal_id);
                sqlx::query("UPDATE interactive_proposals SET status = 'Sent', updated_at = NOW() WHERE id = $1 AND tenant_id = $2")
                    .bind(Uuid::parse_str(proposal_id).unwrap_or_default())
                    .bind(tenant_id)
                    .execute(pool)
                    .await?;
            }
        }
    }
    Ok(())
}

pub async fn parse_inquiry_to_proposal(tenant_id: &str, customer_id: Uuid, inquiry_text: &str, pool: &PgPool) -> Result<Uuid, sqlx::Error> {
    // Simulated Estimator Agent logic
    // In a real implementation, this would use RAG against pricing rules

    let proposal_id = Uuid::new_v4();
    let total_amount_cents: i64 = 15000; // $150.00
    let required_deposit_cents: i64 = 5000; // $50.00
    let message = format!("Based on your inquiry: '{}', here is the quote.", inquiry_text);

    // Create Draft Proposal
    sqlx::query(
        "INSERT INTO interactive_proposals (id, tenant_id, customer_id, status, total_amount_cents, required_deposit_cents, message)
         VALUES ($1, $2, $3, 'Draft', $4, $5, $6)"
    )
    .bind(proposal_id)
    .bind(tenant_id)
    .bind(customer_id)
    .bind(total_amount_cents)
    .bind(required_deposit_cents)
    .bind(message)
    .execute(pool)
    .await?;

    // Create sample line item
    sqlx::query(
        "INSERT INTO interactive_proposal_line_items (id, proposal_id, description, unit_price_cents, quantity)
         VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(Uuid::new_v4())
    .bind(proposal_id)
    .bind("Custom Service Base Fee")
    .bind(15000)
    .bind(1)
    .execute(pool)
    .await?;

    tracing::info!("Estimator Agent drafted proposal {} for tenant {}", proposal_id, tenant_id);

    Ok(proposal_id)
}
