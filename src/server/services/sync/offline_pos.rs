use std::sync::Arc;

pub async fn process_offline_pos_queue(
    pool: &sqlx::PgPool,
    tenant_id: &str,
    mutations: &[serde_json::Value],
) -> Result<usize, String> {
    let mut processed_count = 0;

    for mutation in mutations {
        let typ = mutation.get("type").and_then(|v| v.as_str()).unwrap_or("");
        let id = mutation.get("id").and_then(|v| v.as_str()).unwrap_or("");

        // Basic CRDT resolution: In an offline POS environment, we always accept the client's append for Tap to Pay
        if typ == "tap_to_pay" {
            let amount = mutation.get("amount").and_then(|v| v.as_f64()).unwrap_or(0.0);
            let timestamp = mutation.get("timestamp").and_then(|v| v.as_str()).unwrap_or("");
            let idempotency_key = mutation.get("idempotency_key").and_then(|v| v.as_str()).unwrap_or("");

            tracing::info!(
                "Reconciling offline tap_to_pay transaction for tenant {}: amount={}, idempotency_key={}, timestamp={}",
                tenant_id, amount, idempotency_key, timestamp
            );

            // Record transaction in ledger (Mocked for CRDT sync processing)
            // sqlx::query("INSERT INTO transactions (...) VALUES (...) ON CONFLICT (idempotency_key) DO NOTHING")
            processed_count += 1;
        } else if typ == "inventory_toggle" {
            tracing::info!(
                "Reconciling offline inventory_toggle for tenant {}: id={}",
                tenant_id, id
            );
            processed_count += 1;
        } else {
            tracing::warn!("Unknown mutation type in offline POS queue: {}", typ);
            // Even if unknown, we might count it or ignore it. For now, just continue.
        }
    }

    Ok(processed_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_process_offline_pos_queue() {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .connect_lazy("postgres://localhost/dummy")
            .unwrap();

        let mutations = vec![
            serde_json::json!({
                "id": "txn_123",
                "type": "tap_to_pay",
                "amount": 45.0,
                "timestamp": "2023-10-01T12:00:00Z",
                "idempotency_key": "idemp_123"
            }),
            serde_json::json!({
                "id": "item_1",
                "type": "inventory_toggle",
                "timestamp": "2023-10-01T12:05:00Z"
            })
        ];

        let result = process_offline_pos_queue(&pool, "tenant_a", &mutations).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
    }
}
