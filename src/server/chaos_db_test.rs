#[cfg(test)]
mod chaos_db_tests {
    use crate::db::{DB, DbStore};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_high_concurrency_lock_contention_resilience() {
        // We use a shared-cache in-memory SQLite to simulate heavy write contention
        // across multiple threads, exercising DB::execute_with_retry
        let db_id = uuid::Uuid::new_v4().to_string();
        let uri = format!("sqlite:file:{}?mode=memory&cache=shared", db_id);

        let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(5) // Small pool to force contention
            .connect(&uri)
            .await
            .unwrap();

        // Initialize schema
        sqlx::query("CREATE TABLE IF NOT EXISTS chaos_test (id TEXT PRIMARY KEY, val TEXT);")
            .execute(&sqlite_pool)
            .await
            .unwrap();

        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(),
            store: DbStore::Sqlite(sqlite_pool.clone()),
        });

        let mut handles = vec![];
        for i in 0..50 {
            let db_clone = db.clone();
            handles.push(tokio::spawn(async move {
                // execute_with_retry requires the error type E to implement From<String>
                db_clone.execute_with_retry::<_, _, _, String>("chaos_write", || async {
                    if let DbStore::Sqlite(pool) = &db_clone.store {
                        sqlx::query("INSERT INTO chaos_test (id, val) VALUES (?, ?)")
                            .bind(format!("id_{}", i))
                            .bind("val")
                            .execute(pool)
                            .await
                            .map_err(|e| e.to_string())
                    } else {
                        panic!("Expected SQLite store");
                    }
                }).await
            }));
        }

        for h in handles {
            let res = h.await.unwrap();
            assert!(res.is_ok(), "Write should eventually succeed despite lock contention: {:?}", res.err());
        }

        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM chaos_test")
            .fetch_one(&sqlite_pool)
            .await
            .unwrap();
        assert_eq!(count, 50);
    }
}
