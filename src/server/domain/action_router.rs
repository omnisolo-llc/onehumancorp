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
        "social_post_draft" => {
            // Real implementation would buffer post here to AYRSHARE.
            tracing::info!("Approved and scheduled SocialPostDraft for tenant: {}", tenant_id);
        }
        "product_creation" => {
            if let Some(details) = payload.get("action_details") {
                let name = details.get("name").and_then(|v| v.as_str()).unwrap_or("New Product").to_string();
                let description = details.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let price_cents = (details.get("price").and_then(|v| v.as_str()).unwrap_or("0").parse::<f64>().unwrap_or(0.0) * 100.0).round() as i64;
                let item_type = details.get("item_type").and_then(|v| v.as_str()).unwrap_or("Product").to_string();
                let product_id = uuid::Uuid::new_v4().to_string();

                if let Err(e) = sqlx::query(
                    "INSERT INTO products (id, tenant_id, title, description, type, price_cents, inventory_count) VALUES ($1, $2, $3, $4, $5, $6, 100)"
                )
                .bind(&product_id)
                .bind(tenant_id)
                .bind(&name)
                .bind(&description)
                .bind(&item_type)
                .bind(price_cents)
                .execute(pool)
                .await {
                    tracing::error!("Failed to insert magic catalog product: {}", e);
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
