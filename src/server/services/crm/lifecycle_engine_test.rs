#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use server_lib::db::{DB, DbStore};
    use crm::repo::CrmRepository;
    use super::super::lifecycle_engine::LifecycleEngine;
    use sqlx::sqlite::SqlitePoolOptions;
    use uuid::Uuid;

    async fn setup_test_db() -> Arc<DB> {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS ohc_customer360 (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                email TEXT,
                phone TEXT,
                mood TEXT NOT NULL,
                preferences TEXT NOT NULL,
                created_at DATETIME,
                updated_at DATETIME
            )
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS ohc_interaction_timeline (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                customer_id TEXT NOT NULL,
                source TEXT NOT NULL,
                sentiment TEXT NOT NULL,
                occurred_at DATETIME
            )
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS ohc_loyalty_ledger (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                customer_id TEXT NOT NULL UNIQUE,
                points_balance INTEGER NOT NULL,
                tier_name TEXT NOT NULL,
                last_updated DATETIME
            )
            "#
        )
        .execute(&pool)
        .await
        .unwrap();

        Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://postgres:postgres@localhost:5432/test").unwrap(),
            store: DbStore::Sqlite(pool),
        })
    }

    #[tokio::test]
    async fn test_process_order_completed_and_vip_upgrade() {
        let db = setup_test_db().await;
        let repo = Arc::new(CrmRepository::new(db));
        let engine = LifecycleEngine::new(repo.clone());

        let tenant_id = "tenant_1";
        let customer_id = "cust_1";

        engine.process_order_completed(tenant_id, customer_id, 100.0).await.unwrap();

        let loyalty1 = repo.get_loyalty(tenant_id, customer_id).await.unwrap().unwrap();
        assert_eq!(loyalty1.points_balance, 100);
        assert_eq!(loyalty1.tier_name, "Frequent Buyer");

        let cust1 = repo.get_customer(tenant_id, customer_id).await.unwrap().unwrap();
        assert_eq!(cust1.mood, "Active");

        engine.process_order_completed(tenant_id, customer_id, 450.0).await.unwrap();

        let loyalty2 = repo.get_loyalty(tenant_id, customer_id).await.unwrap().unwrap();
        assert_eq!(loyalty2.points_balance, 550);
        assert_eq!(loyalty2.tier_name, "Top 5% Spender");

        let cust2 = repo.get_customer(tenant_id, customer_id).await.unwrap().unwrap();
        assert_eq!(cust2.mood, "VIP");

        let interactions = repo.get_interactions(tenant_id, customer_id).await.unwrap();
        assert_eq!(interactions.len(), 2);
    }

    #[tokio::test]
    async fn test_evaluate_at_risk_customers() {
        let db = setup_test_db().await;
        let repo = Arc::new(CrmRepository::new(db));
        let engine = LifecycleEngine::new(repo.clone());

        let tenant_id = "tenant_1";
        let customer_id = "cust_2";

        engine.process_order_completed(tenant_id, customer_id, 50.0).await.unwrap();

        engine.evaluate_at_risk_customers(tenant_id, customer_id, 10).await.unwrap();
        let cust1 = repo.get_customer(tenant_id, customer_id).await.unwrap().unwrap();
        assert_eq!(cust1.mood, "Active");

        engine.evaluate_at_risk_customers(tenant_id, customer_id, 22).await.unwrap();
        let cust2 = repo.get_customer(tenant_id, customer_id).await.unwrap().unwrap();
        assert_eq!(cust2.mood, "Needs Attention");
    }
}
