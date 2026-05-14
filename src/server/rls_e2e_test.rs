use crate::db::{init_db, DbStore, get_pg_pool};
use std::env;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_postgres_rls_e2e_isolation() {
    let mode = env::var("OHC_STANDALONE").unwrap_or_default();
    if mode == "true" {
        return; // Skip in SQLite standalone mode
    }

    env::set_var("DATABASE_URL", "postgres://postgres:postgres@localhost:5432/test?sslmode=disable");
    let store = init_db().await.unwrap();

    if let DbStore::Postgres = store {
        let pool = get_pg_pool().await;

        let tenant_a = uuid::Uuid::new_v4();
        let tenant_b = uuid::Uuid::new_v4();

        // Ensure RLS table exists via init_db above

        // Insert data bypassing RLS
        let mut tx = pool.begin().await.unwrap();
        sqlx::query("ALTER TABLE agents DISABLE ROW LEVEL SECURITY;").execute(&mut *tx).await.unwrap();
        sqlx::query("INSERT INTO agents (id, tenant_id, department, status) VALUES ($1, $2, 'sales', 'active')")
            .bind(uuid::Uuid::new_v4())
            .bind(tenant_a)
            .execute(&mut *tx).await.unwrap();

        sqlx::query("INSERT INTO agents (id, tenant_id, department, status) VALUES ($1, $2, 'support', 'active')")
            .bind(uuid::Uuid::new_v4())
            .bind(tenant_b)
            .execute(&mut *tx).await.unwrap();
        sqlx::query("ALTER TABLE agents ENABLE ROW LEVEL SECURITY;").execute(&mut *tx).await.unwrap();
        tx.commit().await.unwrap();

        // 1. Query as Tenant A
        let mut tx_a = pool.begin().await.unwrap();
        sqlx::query(&format!("SELECT set_config('app.current_tenant', '{}', true)", tenant_a)).execute(&mut *tx_a).await.unwrap();
        let count_a: (i64,) = sqlx::query_as("SELECT count(*) FROM agents").fetch_one(&mut *tx_a).await.unwrap();
        assert_eq!(count_a.0, 1, "Tenant A should see exactly 1 agent");

        // 2. Query as Tenant B
        let mut tx_b = pool.begin().await.unwrap();
        sqlx::query(&format!("SELECT set_config('app.current_tenant', '{}', true)", tenant_b)).execute(&mut *tx_b).await.unwrap();
        let count_b: (i64,) = sqlx::query_as("SELECT count(*) FROM agents").fetch_one(&mut *tx_b).await.unwrap();
        assert_eq!(count_b.0, 1, "Tenant B should see exactly 1 agent");

        // 3. Query without Tenant Context
        let mut tx_empty = pool.begin().await.unwrap();
        sqlx::query("SELECT set_config('app.current_tenant', '', true)").execute(&mut *tx_empty).await.unwrap();
        let count_empty: (i64,) = sqlx::query_as("SELECT count(*) FROM agents").fetch_one(&mut *tx_empty).await.unwrap();
        assert_eq!(count_empty.0, 0, "Empty context should see 0 agents");
    }
}
