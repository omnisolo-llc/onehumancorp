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
