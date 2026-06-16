#[cfg(test)]
mod db_privacy_tests {
    use sqlx::{Postgres, Executor};

    #[tokio::test]
    async fn test_postgresql_rls_coverage() {
        let db_url = std::env::var("OHC_DATABASE_URL");
        if db_url.is_err() {
            return;
        }
        let db_url = db_url.unwrap();
        if db_url.contains("sqlite") {
             return;
        }

        let pool = sqlx::PgPool::connect(&db_url).await.expect("Failed to connect to Postgres");

        let tables_with_tenant_cols: Vec<String> = sqlx::query_scalar(
            r#"
            SELECT DISTINCT table_name
            FROM information_schema.columns
            WHERE table_schema = 'public'
              AND column_name IN ('tenant_id', 'organization_id')
            "#
        )
        .fetch_all(&pool)
        .await
        .expect("Failed to query information_schema");

        for table in tables_with_tenant_cols {
            let is_rls_enabled: bool = sqlx::query_scalar(
                "SELECT relrowsecurity FROM pg_class WHERE oid = $1::regclass"
            )
            .bind(&table)
            .fetch_one(&pool)
            .await
            .expect(&format!("Failed to check RLS status for table {}", table));

            assert!(is_rls_enabled, "RLS must be enabled for table '{}'", table);

            let policy_count: i64 = sqlx::query_scalar(
                "SELECT count(*) FROM pg_policies WHERE schemaname = 'public' AND tablename = $1"
            )
            .bind(&table)
            .fetch_one(&pool)
            .await
            .expect(&format!("Failed to count policies for table {}", table));

            assert!(policy_count > 0, "At least one RLS policy must be defined for table '{}'", table);

            let is_force_rls_enabled: bool = sqlx::query_scalar(
                "SELECT relforcerowsecurity FROM pg_class WHERE oid = $1::regclass"
            )
            .bind(&table)
            .fetch_one(&pool)
            .await
            .expect(&format!("Failed to check FORCE RLS status for table {}", table));

            assert!(is_force_rls_enabled, "FORCE ROW LEVEL SECURITY must be enabled for table '{}'", table);
        }
    }

    #[tokio::test]
    async fn test_tenant_isolation_enforcement() {
        let db_url = std::env::var("OHC_DATABASE_URL");
        if db_url.is_err() { return; }
        let db_url = db_url.unwrap();
        if db_url.contains("sqlite") { return; }

        let pool = sqlx::PgPool::connect(&db_url).await.unwrap();

        let tenant_a = format!("tenant_a_{}", uuid::Uuid::new_v4());
        let tenant_b = format!("tenant_b_{}", uuid::Uuid::new_v4());

        let mut conn = pool.acquire().await.unwrap();

        sqlx::query("SELECT set_config('app.current_tenant', $1, true)").bind(&tenant_a).execute(&mut *conn).await.unwrap();
        sqlx::query("INSERT INTO products (id, tenant_id, title, type) VALUES ($1, $2, 'Product A', 'digital')")
            .bind(uuid::Uuid::new_v4().to_string())
            .bind(&tenant_a)
            .execute(&mut *conn).await.unwrap();

        sqlx::query("SELECT set_config('app.current_tenant', $1, true)").bind(&tenant_b).execute(&mut *conn).await.unwrap();
        sqlx::query("INSERT INTO products (id, tenant_id, title, type) VALUES ($1, $2, 'Product B', 'digital')")
            .bind(uuid::Uuid::new_v4().to_string())
            .bind(&tenant_b)
            .execute(&mut *conn).await.unwrap();

        sqlx::query("SELECT set_config('app.current_tenant', $1, true)").bind(&tenant_a).execute(&mut *conn).await.unwrap();
        let count_a: i64 = sqlx::query_scalar("SELECT count(*) FROM products").fetch_one(&mut *conn).await.unwrap();
        assert_eq!(count_a, 1);

        sqlx::query("SELECT set_config('app.current_tenant', $1, true)").bind(&tenant_b).execute(&mut *conn).await.unwrap();
        let count_b: i64 = sqlx::query_scalar("SELECT count(*) FROM products").fetch_one(&mut *conn).await.unwrap();
        assert_eq!(count_b, 1);

        sqlx::query("SELECT set_config('app.current_tenant', $1, true)").bind(&tenant_a).execute(&mut *conn).await.unwrap();
        sqlx::query("INSERT INTO customer_identities (id, tenant_id, customer_id, channel, channel_identity) VALUES ($1, $2, $3, $4, $5)")
            .bind(uuid::Uuid::new_v4().to_string()).bind(&tenant_a).bind("cust_a").bind("email").bind("alice@a.com")
            .execute(&mut *conn).await.unwrap();

        sqlx::query("SELECT set_config('app.current_tenant', $1, true)").bind(&tenant_b).execute(&mut *conn).await.unwrap();
        sqlx::query("INSERT INTO customer_identities (id, tenant_id, customer_id, channel, channel_identity) VALUES ($1, $2, $3, $4, $5)")
            .bind(uuid::Uuid::new_v4().to_string()).bind(&tenant_b).bind("cust_b").bind("email").bind("bob@b.com")
            .execute(&mut *conn).await.unwrap();

        sqlx::query("SELECT set_config('app.current_tenant', $1, true)").bind(&tenant_a).execute(&mut *conn).await.unwrap();
        sqlx::query("INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, status) VALUES ($1, $2, $3, $4, $5)")
            .bind(uuid::Uuid::new_v4().to_string()).bind(&tenant_a).bind("email").bind("Msg A").bind("unread")
            .execute(&mut *conn).await.unwrap();

        sqlx::query("SELECT set_config('app.current_tenant', $1, true)").bind(&tenant_b).execute(&mut *conn).await.unwrap();
        sqlx::query("INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, status) VALUES ($1, $2, $3, $4, $5)")
            .bind(uuid::Uuid::new_v4().to_string()).bind(&tenant_b).bind("email").bind("Msg B").bind("unread")
            .execute(&mut *conn).await.unwrap();

        sqlx::query("SELECT set_config('app.current_tenant', $1, true)").bind(&tenant_a).execute(&mut *conn).await.unwrap();
        let ci_count_a: i64 = sqlx::query_scalar("SELECT count(*) FROM customer_identities").fetch_one(&mut *conn).await.unwrap();
        assert_eq!(ci_count_a, 1);

        sqlx::query("SELECT set_config('app.current_tenant', $1, true)").bind(&tenant_b).execute(&mut *conn).await.unwrap();
        let ci_count_b: i64 = sqlx::query_scalar("SELECT count(*) FROM customer_identities").fetch_one(&mut *conn).await.unwrap();
        assert_eq!(ci_count_b, 1);

        sqlx::query("SELECT set_config('app.current_tenant', $1, true)").bind(&tenant_a).execute(&mut *conn).await.unwrap();
        let omni_count_a: i64 = sqlx::query_scalar("SELECT count(*) FROM omni_inbox_messages").fetch_one(&mut *conn).await.unwrap();
        assert_eq!(omni_count_a, 1);

        sqlx::query("SELECT set_config('app.current_tenant', $1, true)").bind(&tenant_b).execute(&mut *conn).await.unwrap();
        let omni_count_b: i64 = sqlx::query_scalar("SELECT count(*) FROM omni_inbox_messages").fetch_one(&mut *conn).await.unwrap();
        assert_eq!(omni_count_b, 1);

        sqlx::query("SET LOCAL ROLE ohc_bypassrls").execute(&mut *conn).await.unwrap();
        let count_all: i64 = sqlx::query_scalar("SELECT count(*) FROM products").fetch_one(&mut *conn).await.unwrap();
        assert!(count_all >= 2);
    }
}
