#[cfg(test)]
mod isolation_tests {
    use crate::db::DB;
    use crate::utils::auth_utils::set_org_context;
    use sqlx::Executor;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_rls_isolation_between_tenants() {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_default();

        if !database_url.starts_with("postgres") {
            // RLS is a Postgres feature in our stack, skip if no Postgres
            return;
        }

        let db = match DB::new().await {
            Ok(db) => db,
            Err(_) => return, // Skip if cannot connect to Postgres
        };

        // Ensure extensions and schema exist (though in CI they should)
        let _ = db.run_migrations().await;

        let org_a = format!("org-a-{}", Uuid::new_v4());
        let org_b = format!("org-b-{}", Uuid::new_v4());

        // 1. Insert data for Org A
        {
            let mut conn = db.pool.acquire().await.unwrap();
            set_org_context(&mut *conn, &org_a).await.unwrap();

            sqlx::query("INSERT INTO tasks (id, organization_id, title) VALUES ($1, $2, $3)")
                .bind(Uuid::new_v4().to_string())
                .bind(&org_a)
                .bind("Org A Task")
                .execute(&mut *conn)
                .await.unwrap();
        }

        // 2. Insert data for Org B
        {
            let mut conn = db.pool.acquire().await.unwrap();
            set_org_context(&mut *conn, &org_b).await.unwrap();

            sqlx::query("INSERT INTO tasks (id, organization_id, title) VALUES ($1, $2, $3)")
                .bind(Uuid::new_v4().to_string())
                .bind(&org_b)
                .bind("Org B Task")
                .execute(&mut *conn)
                .await.unwrap();
        }

        // 3. Verify Org A can only see its own data
        {
            let mut conn = db.pool.acquire().await.unwrap();
            set_org_context(&mut *conn, &org_a).await.unwrap();

            let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tasks")
                .fetch_one(&mut *conn)
                .await.unwrap();

            assert_eq!(count.0, 1, "Org A should only see 1 task");

            let titles: Vec<(String,)> = sqlx::query_as("SELECT title FROM tasks")
                .fetch_all(&mut *conn)
                .await.unwrap();
            assert_eq!(titles[0].0, "Org A Task");
        }

        // 4. Verify Org B can only see its own data
        {
            let mut conn = db.pool.acquire().await.unwrap();
            set_org_context(&mut *conn, &org_b).await.unwrap();

            let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tasks")
                .fetch_one(&mut *conn)
                .await.unwrap();

            assert_eq!(count.0, 1, "Org B should only see 1 task");

            let titles: Vec<(String,)> = sqlx::query_as("SELECT title FROM tasks")
                .fetch_all(&mut *conn)
                .await.unwrap();
            assert_eq!(titles[0].0, "Org B Task");
        }

        // 5. Verify system can see everything
        {
            let mut conn = db.pool.acquire().await.unwrap();
            set_org_context(&mut *conn, "system").await.unwrap();

            let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tasks WHERE organization_id IN ($1, $2)")
                .bind(&org_a)
                .bind(&org_b)
                .fetch_one(&mut *conn)
                .await.unwrap();

            assert!(count.0 >= 2, "System should see at least 2 tasks");
        }
    }

    #[tokio::test]
    async fn test_standalone_sqlite_permissions() {
        use std::fs;
        use std::path::Path;

        let db_path = "test-standalone.db";
        if Path::new(db_path).exists() {
            fs::remove_file(db_path).unwrap();
        }

        // Force standalone mode via environment
        // SAFETY: This is a test, and we are not running concurrently with other env access here.
        unsafe { std::env::set_var("DATABASE_URL", format!("sqlite://{}", db_path)) };

        let _db = DB::new().await.expect("Failed to initialize standalone DB");

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = fs::metadata(db_path).expect("Failed to read DB metadata");
            let mode = metadata.permissions().mode();
            assert_eq!(mode & 0o777, 0o600, "SQLite file must have 0600 permissions");
        }

        fs::remove_file(db_path).unwrap();
    }
}
