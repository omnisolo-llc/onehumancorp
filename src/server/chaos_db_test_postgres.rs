#[cfg(test)]
mod postgres_chaos_tests {
    use crate::db::{DB, DbStore};
    use std::sync::Arc;

    async fn setup_postgres_db() -> Option<Arc<DB>> {
        if let Ok(url) = std::env::var("OHC_DATABASE_URL") {
            if url.starts_with("postgres") {
                let pool = sqlx::postgres::PgPoolOptions::new()
                    .acquire_timeout(std::time::Duration::from_millis(100))
                    .connect(&url)
                    .await
                    .ok()?;

                let db = DB {
                    pool: pool.clone(),
                    store: DbStore::Postgres,
                };

                return Some(Arc::new(db));
            }
        }
        None
    }

    #[tokio::test]
    async fn test_postgres_transaction_isolation_parity() {
        let pg_db = setup_postgres_db().await;
        if pg_db.is_none() {
            tracing::info!("Skipping postgres parity test due to missing database");
            return;
        }
        let db = pg_db.unwrap();

        sqlx::query("CREATE TABLE IF NOT EXISTS isolation_test (id TEXT PRIMARY KEY, val TEXT);")
            .execute(&db.pool)
            .await
            .unwrap();

        // Clear table to be hermetic
        sqlx::query("DELETE FROM isolation_test")
            .execute(&db.pool)
            .await
            .unwrap();

        db.execute_with_retry::<_, _, _, String>("insert_null", || async {
            sqlx::query("INSERT INTO isolation_test (id, val) VALUES ($1, $2)")
                .bind("row1")
                .bind::<Option<String>>(None)
                .execute(&db.pool)
                .await
                .map_err(|e| e.to_string())
        }).await.unwrap();

        let val: Option<String> = sqlx::query_scalar("SELECT val FROM isolation_test WHERE id = 'row1'")
            .fetch_one(&db.pool)
            .await
            .unwrap();

        assert_eq!(val, None, "NULL handling should be preserved accurately");

        let db_clone = db.clone();
        let handle = tokio::spawn(async move {
            db_clone.execute_with_retry::<_, _, _, String>("isolation_write", || async {
                sqlx::query("INSERT INTO isolation_test (id, val) VALUES ($1, $2)")
                    .bind("row2")
                    .bind(Some("test_val"))
                    .execute(&db_clone.pool)
                    .await
                    .map_err(|e| e.to_string())
            }).await
        });

        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM isolation_test")
            .fetch_one(&db.pool)
            .await
            .unwrap();

        assert!(count >= 1, "Count should reflect at least the first inserted row");

        handle.await.unwrap().unwrap();

        let count_after: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM isolation_test")
            .fetch_one(&db.pool)
            .await
            .unwrap();

        assert_eq!(count_after, 2, "Second insert should be visible now");

        // Clean up
        sqlx::query("DELETE FROM isolation_test")
            .execute(&db.pool)
            .await
            .unwrap();
    }
}
