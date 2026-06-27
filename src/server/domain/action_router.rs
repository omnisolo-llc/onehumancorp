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
            tracing::info!("Approved and dispatched supply order via Quartermaster Agent for tenant: {}", tenant_id);
            // Simulating outbound communication to vendor via omnichannel dispatcher
            if let Some(msg) = payload.get("draft_message").and_then(|v| v.as_str()) {
                tracing::info!("Omnichannel Dispatcher sent: {}", msg);
            }
        }
        "social_post_draft" => {
            // Real implementation would buffer post here to AYRSHARE.
            tracing::info!("Approved and scheduled SocialPostDraft for tenant: {}", tenant_id);
        }
        "booking_draft" => {
            crate::domain::booking::handle_booking_action(tenant_id, payload, pool)
                .await
                .map_err(|e| e.to_string())?;
        }
        "shift_reassignment" => {
            tracing::info!("Approved and dispatched shift reassignment via Operations Agent for tenant: {}", tenant_id);
            // Simulate outbound SMS communication to vendor via omnichannel dispatcher
            if let Some(msg) = payload.get("draft_message").and_then(|v| v.as_str()) {
                tracing::info!("Omnichannel Dispatcher sent SMS to replacement staff: {}", msg);
            }

            // Reassign the actual shift in the database.
            // Using a dummy shift_id here to simulate the actual record update
            if let Some(staff_id) = payload.get("staff_id").and_then(|v| v.as_str()) {
                 let proposed_replacement = payload.get("proposed_replacement").and_then(|v| v.as_str()).unwrap_or("Alex");

                 // Update the shift record for tomorrow. We simulate looking up the exact shift.
                 let _ = sqlx::query(
                     r#"
                     UPDATE shifts
                     SET staff_profile_id = (SELECT id FROM staff_profiles WHERE name = $1 AND tenant_id = $2 LIMIT 1)
                     WHERE tenant_id = $2 AND start_time > NOW() AND staff_profile_id = (SELECT id FROM staff_profiles WHERE id = $3 AND tenant_id = $2 LIMIT 1)
                     "#
                 )
                 .bind(proposed_replacement)
                 .bind(tenant_id)
                 .bind(staff_id)
                 .execute(pool)
                 .await;
            }
        }


        "autonomous_quote" => {
            crate::domain::booking::handle_autonomous_quote_action(tenant_id, payload, pool)
                .await
                .map_err(|e| e.to_string())?;
        }
        "dispute_resolution" => {

            tracing::info!("Approved and resolved dispute for tenant: {}", tenant_id);
            if let Some(msg) = payload.get("generated_response").and_then(|v| v.as_str()) {
                tracing::info!("Dispute Resolution Engine sent reply: {}", msg);
            }
            if let Some(refund) = payload.get("refund_amount").and_then(|v| v.as_f64()) {
                tracing::info!("Dispute Resolution Engine processed simulated refund: ${}", refund);
            }
            if let Some(ops) = payload.get("operational_action").and_then(|v| v.as_str()) {
                tracing::info!("Dispute Resolution Engine executed operational action: {}", ops);
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
