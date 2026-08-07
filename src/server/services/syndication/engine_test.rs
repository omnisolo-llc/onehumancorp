#[cfg(test)]
mod tests {
    use crate::services::syndication::engine::SyndicationEngine;
    use sqlx::PgPool;

    #[tokio::test]
    async fn test_syndication_flow() {
        let pool = PgPool::connect("postgres://ohc:ohc@localhost:5432/ohc").await.unwrap();
        let engine = SyndicationEngine::new(pool.clone());
        let tenant_id = "test-tenant-sync";

        // Seed test product
        let mut tx = pool.begin().await.unwrap();
        sqlx::query("INSERT INTO products (id, title, price, inventory_count, available_quantity, tenant_id) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT DO NOTHING")
            .bind("prod-sync-1")
            .bind("Test Product")
            .bind(100)
            .bind(10)
            .bind(10)
            .bind(tenant_id)
            .execute(&mut *tx).await.unwrap();
        tx.commit().await.unwrap();

        // 1. Ingest product
        let res = engine.ingest_product(tenant_id, "prod-sync-1", vec!["META_IG", "TIKTOK"]).await;
        assert!(res.is_ok());

        // 2. Background sync
        let res = engine.simulate_background_sync(tenant_id).await;
        assert!(res.is_ok());

        let mut tx = pool.begin().await.unwrap();
        sqlx::query("SET LOCAL app.current_tenant = $1").bind(tenant_id).execute(&mut *tx).await.unwrap();
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM platform_listings WHERE product_id = 'prod-sync-1' AND sync_status = 'ACTIVE'")
            .fetch_one(&mut *tx).await.unwrap();
        assert_eq!(count.0, 2);
        tx.commit().await.unwrap();

        // 3. Inventory deduction
        let res = engine.process_inventory_deduction(tenant_id, "prod-sync-1", 1, "pos").await;
        assert!(res.is_ok());

        let mut tx = pool.begin().await.unwrap();
        sqlx::query("SET LOCAL app.current_tenant = $1").bind(tenant_id).execute(&mut *tx).await.unwrap();
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM platform_listings WHERE product_id = 'prod-sync-1' AND sync_status = 'PENDING'")
            .fetch_one(&mut *tx).await.unwrap();
        assert_eq!(count.0, 2);
        tx.commit().await.unwrap();

        // Clean up
        let mut tx = pool.begin().await.unwrap();
        sqlx::query("DELETE FROM platform_listings WHERE product_id = 'prod-sync-1'").execute(&mut *tx).await.unwrap();
        sqlx::query("DELETE FROM inventory_ledger WHERE product_id = 'prod-sync-1'").execute(&mut *tx).await.unwrap();
        tx.commit().await.unwrap();
    }
}
