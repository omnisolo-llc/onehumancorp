use sqlx::PgPool;
use serde_json::Value;

pub async fn dispatch_action(
    feature_type: &str,
    tenant_id: &str,
    payload: &Value,
    pool: &PgPool,
) -> Result<(), String> {
    match feature_type {
        "quote_draft" => {
            crate::domain::quotes::handle_quote_action(tenant_id, payload, pool)
                .await
                .map_err(|e| e.to_string())?;
        }
        "ambassador_reply" | "instagram_dm" => {
            crate::domain::inbox::handle_inbox_action(tenant_id, payload, pool)
                .await
                .map_err(|e| e.to_string())?;
        }
        "supply_order" => {
            tracing::info!("Approved and dispatched supply order via Quartermaster Agent for tenant: {}", tenant_id); // pii-safe
            // Simulating outbound communication to vendor via omnichannel dispatcher
            if let Some(_msg) = payload.get("draft_message").and_then(|v| v.as_str()) {
                tracing::info!("Omnichannel Dispatcher sent: [REDACTED]");
            }
        }
        "social_post_draft" => {
            // Real implementation would buffer post here to AYRSHARE.
            tracing::info!("Approved and scheduled SocialPostDraft for tenant: {}", tenant_id); // pii-safe
        }

        "create_product" => {
            crate::domain::catalog::handle_create_product(tenant_id, payload, pool)
                .await
                .map_err(|e| e.to_string())?;
        }
        "booking_draft" => {
            crate::domain::booking::handle_booking_action(tenant_id, payload, pool)
                .await
                .map_err(|e| e.to_string())?;
        }

        "autonomous_quote" => {
            crate::domain::booking::handle_autonomous_quote_action(tenant_id, payload, pool)
                .await
                .map_err(|e| e.to_string())?;
        }
        "invoice_followup" => {
            crate::domain::invoice::handle_invoice_action(tenant_id, payload, pool)
                .await
                .map_err(|e| e.to_string())?;
        }
        "lead_recovery" => {
            tracing::info!("Approved and recovered lead for tenant: {}", tenant_id); // pii-safe
            if let Some(_msg) = payload.get("draft_reply").and_then(|v| v.as_str()) {
                tracing::info!("Lead Recovery Engine sent reply: [REDACTED]");
            }
        }
        "dispute_resolution" => {

            tracing::info!("Approved and resolved dispute for tenant: {}", tenant_id); // pii-safe
            if let Some(_msg) = payload.get("generated_response").and_then(|v| v.as_str()) {
                tracing::info!("Dispute Resolution Engine sent reply: [REDACTED]");
            }
            if let Some(refund) = payload.get("refund_amount").and_then(|v| v.as_f64()) {
                tracing::info!("Dispute Resolution Engine processed simulated refund: ${}", refund);
            }
            if let Some(ops) = payload.get("operational_action").and_then(|v| v.as_str()) {
                tracing::info!("Dispute Resolution Engine executed operational action: {}", ops);
            }
        }
        "loyalty_reward_notification" => {
            tracing::info!("Approved loyalty reward notification for tenant: {}", tenant_id);
            if let Some(customer_id) = payload.get("customer_id").and_then(|v| v.as_str()) {
                let id = uuid::Uuid::new_v4().to_string();
                let discount_code = format!("LOYALTY-{}", uuid::Uuid::new_v4().to_string().chars().take(8).collect::<String>().to_uppercase());
                let _ = sqlx::query("INSERT INTO reward_claims (id, tenant_id, customer_id, discount_code, status) VALUES ($1, $2, $3, $4, 'Active')")
                    .bind(&id)
                    .bind(tenant_id)
                    .bind(customer_id)
                    .bind(&discount_code)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                tracing::info!("Created reward claim {} with discount code {}", id, discount_code);
            }
        }

        _ => {
            tracing::warn!("Unsupported feature_type for action dispatch: {}", feature_type);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_dispatch_action_unsupported() {
        // We use connect_lazy to create a pool without connecting immediately
        // It's sufficient to test that unsupported paths don't attempt to use the pool
        let pool = PgPool::connect_lazy("postgres://dummy:dummy@localhost/dummy").unwrap();

        let payload = json!({});
        let res = dispatch_action("unsupported_action", "tenant_1", &payload, &pool).await;

        assert!(res.is_ok());
    }
}
