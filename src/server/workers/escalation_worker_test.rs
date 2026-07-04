#[cfg(test)]
mod tests {
    use crate::workers::escalation_worker::EscalationWorker;
    use sqlx::PgPool;
    use std::env;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_escalation_worker_detects_violation() {
        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());

        let maybe_pool = PgPool::connect(&database_url).await;
        if maybe_pool.is_err() {
            return;
        }
        let pool = maybe_pool.unwrap();

        // Setup test data
        let tenant_id = Uuid::new_v4().to_string();
        let order_id = Uuid::new_v4().to_string();

        let _ = sqlx::query("INSERT INTO tenants (id, name) VALUES ($1, 'Test Tenant') ON CONFLICT DO NOTHING")
            .bind(&tenant_id)
            .execute(&pool)
            .await;

        // Insert an old pending order
        let old_time = chrono::Utc::now() - chrono::Duration::minutes(40);
        let _ = sqlx::query("INSERT INTO orders (id, tenant_id, status, created_at) VALUES ($1, $2, 'pending', $3)")
            .bind(&order_id)
            .bind(&tenant_id)
            .bind(old_time)
            .execute(&pool)
            .await;

        let worker = EscalationWorker::new(pool.clone(), 30);

        // Ensure no agent feed item exists before
        let count_before: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM agent_feed_items WHERE context_payload->>'order_id' = $1")
            .bind(&order_id)
            .fetch_one(&pool)
            .await
            .unwrap_or((0,));
        assert_eq!(count_before.0, 0);

        // Run one pass of the worker
        let result = worker.process_sla_violations().await;
        assert!(result.is_ok());

        // Verify that an agent feed item was created
        let count_after: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM agent_feed_items WHERE context_payload->>'order_id' = $1")
            .bind(&order_id)
            .fetch_one(&pool)
            .await
            .unwrap_or((0,));
        assert_eq!(count_after.0, 1);

        // Cleanup
        let _ = sqlx::query("DELETE FROM orders WHERE id = $1").bind(&order_id).execute(&pool).await;
        let _ = sqlx::query("DELETE FROM agent_feed_items WHERE context_payload->>'order_id' = $1").bind(&order_id).execute(&pool).await;
    }
}
