#[cfg(test)]
mod isolation_tests {
    use sqlx::PgPool;
    use crate::utils::auth_utils::set_org_context;
    use sqlx::Executor;

    async fn setup_test_db() -> PgPool {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/test".to_string());
        sqlx::PgPool::connect(&database_url).await.expect("Failed to connect to test DB")
    }

    #[tokio::test]
    async fn test_rls_isolation_enforcement() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }
        let pool = setup_test_db().await;

        // Clean up and setup test data
        pool.execute("DROP TABLE IF EXISTS test_isolation CASCADE").await.unwrap();
        pool.execute("CREATE TABLE test_isolation (id SERIAL PRIMARY KEY, organization_id TEXT NOT NULL, data TEXT)").await.unwrap();
        pool.execute("ALTER TABLE test_isolation ENABLE ROW LEVEL SECURITY").await.unwrap();
        pool.execute("CREATE POLICY test_isolation_policy ON test_isolation USING (organization_id = current_setting('app.current_tenant', true) OR current_setting('app.current_tenant', true) = 'system')").await.unwrap();

        pool.execute("INSERT INTO test_isolation (organization_id, data) VALUES ('org_a', 'data_a'), ('org_b', 'data_b')").await.unwrap();

        // Test Org A context
        {
            let mut conn = pool.acquire().await.unwrap();
            set_org_context(&mut *conn, "org_a").await.unwrap();
            let rows = sqlx::query_as::<_, (i32, String, String)>("SELECT * FROM test_isolation")
                .fetch_all(&mut *conn)
                .await
                .unwrap();
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0].1, "org_a");
        }

        // Test Org B context
        {
            let mut conn = pool.acquire().await.unwrap();
            set_org_context(&mut *conn, "org_b").await.unwrap();
            let rows = sqlx::query_as::<_, (i32, String, String)>("SELECT * FROM test_isolation")
                .fetch_all(&mut *conn)
                .await
                .unwrap();
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0].1, "org_b");
        }

        // Test System context
        {
            let mut conn = pool.acquire().await.unwrap();
            set_org_context(&mut *conn, "system").await.unwrap();
            let rows = sqlx::query_as::<_, (i32, String, String)>("SELECT * FROM test_isolation")
                .fetch_all(&mut *conn)
                .await
                .unwrap();
            assert_eq!(rows.len(), 2);
        }

        // Test unauthorized access (empty context)
        {
            let mut conn = pool.acquire().await.unwrap();
            // Reset context or use a new connection where app.current_tenant is not set (it defaults to '')
            sqlx::query("SELECT set_config('app.current_tenant', '', true)").execute(&mut *conn).await.unwrap();
            let rows = sqlx::query_as::<_, (i32, String, String)>("SELECT * FROM test_isolation")
                .fetch_all(&mut *conn)
                .await
                .unwrap();
            assert_eq!(rows.len(), 0);
        }
    }
}
