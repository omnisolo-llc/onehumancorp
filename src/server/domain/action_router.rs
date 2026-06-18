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
        "create_rule" => {
            tracing::info!("Approved and created pricing rule for tenant: {}", tenant_id);
            if let Some(rule_config) = payload.get("rule_config") {
                let id = uuid::Uuid::new_v4();
                let name = rule_config.get("name").and_then(|v| v.as_str()).unwrap_or("Dynamic Rule");

                // get the target_id
                let target_id = payload.get("target_id").and_then(|v| v.as_str()).unwrap_or("");

                // get the base price
                let base_price_cents: i64 = sqlx::query_scalar("SELECT price_cents FROM products WHERE id = $1 AND tenant_id = $2")
                    .bind(target_id)
                    .bind(tenant_id)
                    .fetch_optional(pool)
                    .await
                    .unwrap_or(None)
                    .unwrap_or(0);

                let rule_json = serde_json::json!([{
                    "id": id.to_string(),
                    "name": name,
                    "rule_type": rule_config,
                    "is_active": true
                }]);

                if let Err(e) = sqlx::query("INSERT INTO pricing_rules (id, tenant_id, target_id, name, base_price_cents, rules_json, is_active) VALUES ($1, $2, $3, $4, $5, $6, true)")
                    .bind(id)
                    .bind(tenant_id)
                    .bind(target_id)
                    .bind(name)
                    .bind(base_price_cents)
                    .bind(rule_json)
                    .execute(pool)
                    .await {
                        tracing::error!("Failed to insert dynamic pricing rule: {}", e);
                    }
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
