#[cfg(test)]
mod security_isolation_tests {
    use sqlx::{PgPool, Executor, Row};
    use std::env;

    async fn setup_test_db() -> PgPool {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set for security isolation tests");
        PgPool::connect(&database_url).await.expect("Failed to connect to test DB")
    }

    #[tokio::test]
    async fn test_rls_tenant_isolation_leakage() {
        if env::var("DATABASE_URL").is_err() {
            return;
        }
        let pool = setup_test_db().await;

        // 1. Setup: Create two tenants and some data
        let tenant_a = "tenant-alpha";
        let tenant_b = "tenant-beta";

        // We use a transaction and SET LOCAL to simulate different tenant contexts

        // Insert data for Tenant A
        {
            let mut tx = pool.begin().await.unwrap();
            tx.execute("SET LOCAL ROLE ohc_bypassrls").await.unwrap();
            tx.execute(sqlx::query("INSERT INTO tenants (id, name) VALUES ($1, 'Alpha Corp') ON CONFLICT DO NOTHING").bind(tenant_a)).await.unwrap();
            tx.execute(sqlx::query("INSERT INTO products (id, tenant_id, title) VALUES ('prod-a', $1, 'Secret A') ON CONFLICT DO NOTHING").bind(tenant_a)).await.unwrap();
            tx.commit().await.unwrap();
        }

        // Insert data for Tenant B
        {
            let mut tx = pool.begin().await.unwrap();
            tx.execute("SET LOCAL ROLE ohc_bypassrls").await.unwrap();
            tx.execute(sqlx::query("INSERT INTO tenants (id, name) VALUES ($1, 'Beta Corp') ON CONFLICT DO NOTHING").bind(tenant_b)).await.unwrap();
            tx.execute(sqlx::query("INSERT INTO products (id, tenant_id, title) VALUES ('prod-b', $1, 'Secret B') ON CONFLICT DO NOTHING").bind(tenant_b)).await.unwrap();
            tx.commit().await.unwrap();
        }

        // 2. Verify: Tenant A cannot see Tenant B's data
        {
            let mut tx = pool.begin().await.unwrap();
            tx.execute(format!("SET LOCAL app.current_tenant = '{}'", tenant_a).as_str()).await.unwrap();

            let rows = sqlx::query("SELECT title FROM products").fetch_all(&mut *tx).await.unwrap();
            assert_eq!(rows.len(), 1, "Tenant A should only see 1 product");
            let title: String = rows[0].get("title");
            assert_eq!(title, "Secret A");

            // Explicitly try to access B's product ID (should return nothing due to RLS)
            let row_opt = sqlx::query("SELECT title FROM products WHERE id = 'prod-b'").fetch_optional(&mut *tx).await.unwrap();
            assert!(row_opt.is_none(), "Tenant A should not be able to fetch Tenant B's product even by ID");

            tx.rollback().await.unwrap();
        }

        // 3. Verify: Tenant B cannot see Tenant A's data
        {
            let mut tx = pool.begin().await.unwrap();
            tx.execute(format!("SET LOCAL app.current_tenant = '{}'", tenant_b).as_str()).await.unwrap();

            let rows = sqlx::query("SELECT title FROM products").fetch_all(&mut *tx).await.unwrap();
            assert_eq!(rows.len(), 1, "Tenant B should only see 1 product");
            let title: String = rows[0].get("title");
            assert_eq!(title, "Secret B");

            tx.rollback().await.unwrap();
        }
    }
}
