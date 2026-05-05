use sqlx::{postgres::PgPoolOptions, Row};

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tenant_isolation_rls() {
        let pool = PgPoolOptions::new()
            .connect_lazy("postgres://postgres:postgres@localhost/ohc")
            .unwrap();

        // Check if database is reachable before running actual test.
        // For tests using a shared external resource that might not be fully configured in all test execution environments yet,
        // we use this pattern based on project test design docs.
        let eager_check = sqlx::query("SELECT 1").execute(&pool).await.is_err();
        if eager_check {
            return;
        }

        // Setup test data for two tenants
        let tenant1_id = "test-tenant-1";
        let tenant2_id = "test-tenant-2";

        // Clean up any stale data first to prevent conflicts
        sqlx::query("RESET app.current_tenant").execute(&pool).await.unwrap_or_default();
        sqlx::query("ALTER TABLE tenants NO FORCE ROW LEVEL SECURITY").execute(&pool).await.unwrap_or_default();
        sqlx::query("ALTER TABLE customers NO FORCE ROW LEVEL SECURITY").execute(&pool).await.unwrap_or_default();
        sqlx::query("DELETE FROM customers WHERE id IN ('cust-1', 'cust-2')").execute(&pool).await.unwrap_or_default();
        sqlx::query("DELETE FROM tenants WHERE id IN ($1, $2)")
            .bind(tenant1_id).bind(tenant2_id).execute(&pool).await.unwrap_or_default();

        // Transaction block to safely apply DDL changes without polluting other concurrent tests
        let mut tx = pool.begin().await.unwrap();

        // Force RLS on the tables inside the transaction so the superuser is subject to policies
        sqlx::query("ALTER TABLE tenants FORCE ROW LEVEL SECURITY").execute(&mut *tx).await.unwrap_or_default();
        sqlx::query("ALTER TABLE customers FORCE ROW LEVEL SECURITY").execute(&mut *tx).await.unwrap_or_default();

        // Set session to system role temporarily to bypass RLS when inserting data
        sqlx::query("SET LOCAL app.current_tenant = 'system'").execute(&mut *tx).await.unwrap_or_default();

        sqlx::query("INSERT INTO tenants (id, name, industry, tier) VALUES ($1, 'Tenant 1', 'Tech', 'free') ON CONFLICT (id) DO NOTHING")
            .bind(tenant1_id).execute(&mut *tx).await.unwrap();
        sqlx::query("INSERT INTO tenants (id, name, industry, tier) VALUES ($1, 'Tenant 2', 'Tech', 'free') ON CONFLICT (id) DO NOTHING")
            .bind(tenant2_id).execute(&mut *tx).await.unwrap();

        sqlx::query("INSERT INTO customers (id, tenant_id, name, email) VALUES ('cust-1', $1, 'Alice', 'alice@example.com') ON CONFLICT (id) DO NOTHING")
            .bind(tenant1_id).execute(&mut *tx).await.unwrap();
        sqlx::query("INSERT INTO customers (id, tenant_id, name, email) VALUES ('cust-2', $1, 'Bob', 'bob@example.com') ON CONFLICT (id) DO NOTHING")
            .bind(tenant2_id).execute(&mut *tx).await.unwrap();

        // Set session to tenant 1
        sqlx::query("SET LOCAL app.current_tenant = $1")
            .bind(tenant1_id).execute(&mut *tx).await.unwrap();

        // Verify tenant 1 can only see their own customers
        let count1: i64 = sqlx::query("SELECT COUNT(*) FROM customers")
            .fetch_one(&mut *tx).await.unwrap().get(0);
        assert_eq!(count1, 1, "Tenant 1 should see exactly 1 customer (their own)");

        let tenant2_data_visible: i64 = sqlx::query("SELECT COUNT(*) FROM customers WHERE tenant_id = $1")
            .bind(tenant2_id).fetch_one(&mut *tx).await.unwrap().get(0);
        assert_eq!(tenant2_data_visible, 0, "Tenant 1 should return 0 rows when attempting to query tenant 2 data");

        // Set session to tenant 2
        sqlx::query("SET LOCAL app.current_tenant = $1")
            .bind(tenant2_id).execute(&mut *tx).await.unwrap();

        // Verify tenant 2 can only see their own customers
        let count2: i64 = sqlx::query("SELECT COUNT(*) FROM customers")
            .fetch_one(&mut *tx).await.unwrap().get(0);
        assert_eq!(count2, 1, "Tenant 2 should see exactly 1 customer (their own)");

        let tenant1_data_visible: i64 = sqlx::query("SELECT COUNT(*) FROM customers WHERE tenant_id = $1")
            .bind(tenant1_id).fetch_one(&mut *tx).await.unwrap().get(0);
        assert_eq!(tenant1_data_visible, 0, "Tenant 2 should return 0 rows when attempting to query tenant 1 data");

        // Let the transaction rollback naturally to avoid persisting the FORCE RLS and dummy data
        tx.rollback().await.unwrap_or_default();
    }
}
