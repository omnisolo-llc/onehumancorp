#[cfg(test)]
mod isolation_tests_v2 {
    use crate::db::{DB, DbStore};
    use crate::utils::auth_utils::set_org_context;
    use sqlx::Row;

    #[tokio::test]
    async fn test_rls_enforcement_cross_tenant_leak() {
        let database_url = match std::env::var("DATABASE_URL") {
            Ok(url) => url,
            Err(_) => return, // Skip if no DB
        };

        if database_url.starts_with("sqlite") {
            return; // Postgres-specific RLS test
        }

        let db = DB::new().await.expect("Failed to connect to DB");

        // 1. Setup: Create two tenants and some data
        let mut tx = db.pool.begin().await.unwrap();
        // Bypass RLS to setup test data
        sqlx::query("SET LOCAL ROLE ohc_bypassrls").execute(&mut *tx).await.unwrap();

        sqlx::query("INSERT INTO tenants (tenant_id, business_name) VALUES ('tenant_1', 'Tenant One'), ('tenant_2', 'Tenant Two') ON CONFLICT DO NOTHING").execute(&mut *tx).await.unwrap();

        sqlx::query("INSERT INTO shared_tasks (id, tenant_id, title, status) VALUES ('task_1', 'tenant_1', 'Secret 1', 'PENDING') ON CONFLICT DO NOTHING").execute(&mut *tx).await.unwrap();
        sqlx::query("INSERT INTO shared_tasks (id, tenant_id, title, status) VALUES ('task_2', 'tenant_2', 'Secret 2', 'PENDING') ON CONFLICT DO NOTHING").execute(&mut *tx).await.unwrap();

        tx.commit().await.unwrap();

        // 2. Test Isolation for Tenant 1
        let mut conn = db.pool.acquire().await.unwrap();
        set_org_context(&mut *conn, "tenant_1").await.unwrap();

        let rows = sqlx::query("SELECT title FROM shared_tasks").fetch_all(&mut *conn).await.unwrap();
        assert_eq!(rows.len(), 1, "Tenant 1 should only see its own tasks");
        assert_eq!(rows[0].get::<String, _>("title"), "Secret 1");

        // 3. Test Isolation for Tenant 2
        let mut conn2 = db.pool.acquire().await.unwrap();
        set_org_context(&mut *conn2, "tenant_2").await.unwrap();

        let rows2 = sqlx::query("SELECT title FROM shared_tasks").fetch_all(&mut *conn2).await.unwrap();
        assert_eq!(rows2.len(), 1, "Tenant 2 should only see its own tasks");
        assert_eq!(rows2[0].get::<String, _>("title"), "Secret 2");

        // 4. Test "system" isolation (should not see anything unless bypassed)
        let mut conn3 = db.pool.acquire().await.unwrap();
        set_org_context(&mut *conn3, "system").await.unwrap();
        let rows3 = sqlx::query("SELECT title FROM shared_tasks").fetch_all(&mut *conn3).await.unwrap();
        assert_eq!(rows3.len(), 0, "System context should not see tenant data by default");
    }

    #[tokio::test]
    async fn test_standalone_file_permissions_enforcement() {
        if std::env::var("OHC_STANDALONE").unwrap_or_default() != "true" {
            return;
        }

        let db_path = "ohc-standalone.db";
        if !std::path::Path::new(db_path).exists() {
            return;
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let meta = std::fs::metadata(db_path).unwrap();
            let mode = meta.permissions().mode();
            assert_eq!(mode & 0o777, 0o600, "Standalone SQLite DB must have 0600 permissions");
        }
    }
}
