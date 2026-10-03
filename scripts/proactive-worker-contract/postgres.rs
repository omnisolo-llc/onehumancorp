#[cfg(test)]
mod postgres_tests {
    use crate::storage;
    async fn scan(pool: &sqlx::PgPool, tenant: &str) -> Result<storage::ScanCounts, sqlx::Error> {
        let mut tx = pool.begin().await?;
        sqlx::query("SELECT set_config('app.current_tenant',$1,true)")
            .bind(tenant)
            .execute(&mut *tx)
            .await?;
        let result = storage::scan_postgres(&mut tx, tenant).await?;
        tx.commit().await?;
        Ok(result)
    }
    #[tokio::test]
    async fn compiled_production_pg_storage_scopes_facts_and_enforces_concurrent_dedupe() {
        let url = crate::test_database_url();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(4)
            .connect(&url)
            .await
            .unwrap();
        sqlx::raw_sql("CREATE TABLE raw_materials(id TEXT PRIMARY KEY,tenant_id TEXT NOT NULL,name TEXT,current_quantity INTEGER,reorder_threshold INTEGER,created_at TIMESTAMPTZ,updated_at TIMESTAMPTZ);CREATE TABLE agent_feed_items(id TEXT PRIMARY KEY,tenant_id TEXT NOT NULL,event_source TEXT,context_payload JSONB,proposed_action JSONB,lifecycle_state TEXT,created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP)").execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO raw_materials VALUES('a-source','a','Recorded flour',2,5,'2026-01-01','2026-01-01'),('b-source','b','Other tenant',1,8,'2026-01-01','2026-01-01'),('unknown','a','Unknown revision',1,5,NULL,NULL),('healthy','a','Healthy',8,5,'2026-01-01','2026-01-01'),('unconfigured','a','No rule',1,0,'2026-01-01','2026-01-01')").execute(&pool).await.unwrap();
        let (one, two) = tokio::join!(scan(&pool, "a"), scan(&pool, "a"));
        assert_eq!(one.unwrap().created + two.unwrap().created, 1);
        assert_eq!(
            sqlx::query_scalar::<_, i64>("SELECT count(*) FROM agent_feed_items")
                .fetch_one(&pool)
                .await
                .unwrap(),
            1
        );
        assert_eq!(scan(&pool, "a").await.unwrap().created, 0);
        assert_eq!(scan(&pool, "b").await.unwrap().created, 1);
        let raw: String = sqlx::query_scalar(
            "SELECT context_payload::text FROM agent_feed_items WHERE tenant_id='a'",
        )
        .fetch_one(&pool)
        .await
        .unwrap();
        let evidence: serde_json::Value = serde_json::from_str(&raw).unwrap();
        assert_eq!(evidence["source_id"], "a-source");
        assert_eq!(evidence["quantity"], 2);
        assert_eq!(evidence["threshold"], 5);
        sqlx::query("UPDATE raw_materials SET current_quantity=9,updated_at='2026-01-02' WHERE tenant_id='a'").execute(&pool).await.unwrap();
        let retired = scan(&pool, "a").await.unwrap();
        assert_eq!(retired.retired, 1);
        assert_eq!(retired.created, 0);
        assert_eq!(sqlx::query_scalar::<_,i64>("SELECT count(*) FROM agent_feed_items WHERE tenant_id='b' AND lifecycle_state='PENDING_APPROVAL'").fetch_one(&pool).await.unwrap(),1);
        sqlx::query("UPDATE raw_materials SET current_quantity=2,updated_at='2026-01-03' WHERE id='a-source'").execute(&pool).await.unwrap();
        assert_eq!(scan(&pool, "a").await.unwrap().created, 1);
        sqlx::query("UPDATE agent_feed_items SET lifecycle_state='APPROVED' WHERE tenant_id='a' AND lifecycle_state='PENDING_APPROVAL'").execute(&pool).await.unwrap();
        sqlx::query("DELETE FROM raw_materials WHERE tenant_id='a'")
            .execute(&pool)
            .await
            .unwrap();
        assert_eq!(scan(&pool, "a").await.unwrap().retired, 0);
        assert_eq!(sqlx::query_scalar::<_,i64>("SELECT count(*) FROM agent_feed_items WHERE tenant_id='a' AND lifecycle_state='APPROVED'").fetch_one(&pool).await.unwrap(),1);
        pool.close().await;
    }
}
