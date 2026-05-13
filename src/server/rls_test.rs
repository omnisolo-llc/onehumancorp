#[cfg(test)]
mod rls_coverage_tests {
    use crate::db::DB;
    use sqlx::Row;

    #[tokio::test]
    async fn test_rls_enabled_on_all_tenant_tables() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }
        let db = DB::new().await.unwrap();
        if db.is_sqlite() {
            return; // RLS is a Postgres-specific hardening feature
        }

        let query = r#"
            SELECT
                t.relname as table_name,
                t.relrowsecurity as rls_enabled
            FROM
                pg_class t
            JOIN
                pg_namespace n ON n.oid = t.relnamespace
            WHERE
                n.nspname = 'public'
                AND t.relkind = 'r'
                AND EXISTS (
                    SELECT 1 FROM information_schema.columns c
                    WHERE c.table_name = t.relname
                    AND (c.column_name = 'tenant_id' OR c.column_name = 'organization_id' OR c.column_name = 'org_id')
                );
        "#;

        let rows = sqlx::query(query).fetch_all(&db.pool).await.unwrap();

        let mut missing_rls = Vec::new();
        for row in rows {
            let table_name: String = row.get("table_name");
            let rls_enabled: bool = row.get("rls_enabled");
            if !rls_enabled {
                missing_rls.push(table_name);
            }
        }

        assert!(
            missing_rls.is_empty(),
            "The following multi-tenant tables are missing RLS: {:?}",
            missing_rls
        );
    }
}
