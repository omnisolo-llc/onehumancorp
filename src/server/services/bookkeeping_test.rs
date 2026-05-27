#[cfg(test)]
mod tests {
    use crate::services::bookkeeping::BookkeepingService;
    use crate::db::{DB, DbStore};
    use std::sync::Arc;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn setup_test_db() -> Arc<DB> {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        sqlx::query("CREATE TABLE ledger_entries (id TEXT PRIMARY KEY, tenant_id TEXT, amount REAL, type TEXT, description TEXT, created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP)")
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query("CREATE TABLE financial_insights (id TEXT PRIMARY KEY, tenant_id TEXT, plain_text_summary TEXT, suggested_action TEXT, generated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP)")
            .execute(&pool)
            .await
            .unwrap();

        Arc::new(DB {
            pool: sqlx::PgPool::connect_lazy("postgres://localhost/test").unwrap(),
            store: DbStore::Sqlite(pool),
        })
    }

    #[tokio::test]
    async fn test_record_entry() {
        let db = setup_test_db().await;
        let service = BookkeepingService::new(db.clone());

        service.record_entry("tenant_1", 100.0, "sale", Some("Test Sale")).await.unwrap();

        if let DbStore::Sqlite(pool) = &db.store {
            let row: (f64,) = sqlx::query_as("SELECT amount FROM ledger_entries WHERE tenant_id = 'tenant_1'")
                .fetch_one(pool)
                .await
                .unwrap();
            assert_eq!(row.0, 100.0);
        }
    }

    #[tokio::test]
    async fn test_get_latest_insight_none() {
        let db = setup_test_db().await;
        let service = BookkeepingService::new(db.clone());

        let insight = service.get_latest_insight("tenant_1").await.unwrap();
        assert!(insight.is_none());
    }
}
