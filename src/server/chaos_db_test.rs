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
            pool: sqlx::postgres::PgPoolOptions::new().acquire_timeout(std::time::Duration::from_millis(10)).connect_lazy("postgres://dummy").unwrap(),
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

    #[tokio::test]
    async fn test_sqlite_transaction_isolation_parity() {
        // Parity Auditing: Identify and fix subtle functional discrepancies between
        // different database (e.g., SQLite and Postgres) implementations.
        // Compare query results for identical inputs. Test edge cases (NULL handling, transaction isolation).
        let db_id = uuid::Uuid::new_v4().to_string();
        let uri = format!("sqlite:file:{}?mode=memory&cache=shared", db_id);

        let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&uri)
            .await
            .unwrap();

        // Initialize schema with a nullable column to test NULL handling
        sqlx::query("CREATE TABLE IF NOT EXISTS isolation_test (id TEXT PRIMARY KEY, val TEXT);")
            .execute(&sqlite_pool)
            .await
            .unwrap();

        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().acquire_timeout(std::time::Duration::from_millis(10)).connect_lazy("postgres://dummy").unwrap(),
            store: DbStore::Sqlite(sqlite_pool.clone()),
        });

        // Insert a row with NULL
        db.execute_with_retry::<_, _, _, String>("insert_null", || async {
            if let DbStore::Sqlite(pool) = &db.store {
                sqlx::query("INSERT INTO isolation_test (id, val) VALUES (?, ?) \
                     ON CONFLICT(id) DO UPDATE SET val = excluded.val")
                    .bind("row1")
                    .bind::<Option<String>>(None)
                    .execute(pool)
                    .await
                    .map_err(|e| e.to_string())
            } else {
                panic!("Expected SQLite store");
            }
        }).await.unwrap();

        // Read the row back and verify NULL handling parity
        let val: Option<String> = sqlx::query_scalar("SELECT val FROM isolation_test WHERE id = 'row1'")
            .fetch_one(&sqlite_pool)
            .await
            .unwrap();

        assert_eq!(val, None, "NULL handling should be preserved accurately");

        // Concurrent isolation check
        let db_clone = db.clone();
        let handle = tokio::spawn(async move {
            db_clone.execute_with_retry::<_, _, _, String>("isolation_write", || async {
                if let DbStore::Sqlite(pool) = &db_clone.store {
                    // Try inserting another row while the main thread might be reading
                    sqlx::query("INSERT INTO isolation_test (id, val) VALUES (?, ?)")
                        .bind("row2")
                        .bind(Some("test_val"))
                        .execute(pool)
                        .await
                        .map_err(|e| e.to_string())
                } else {
                    panic!("Expected SQLite store");
                }
            }).await
        });

        // While the spawn is running, let's do a read
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM isolation_test")
            .fetch_one(&sqlite_pool)
            .await
            .unwrap();

        assert!(count >= 1, "Count should reflect at least the first inserted row");

        handle.await.unwrap().unwrap();

        let count_after: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM isolation_test")
            .fetch_one(&sqlite_pool)
            .await
            .unwrap();

        assert_eq!(count_after, 2, "Second insert should be visible now");
    }

    #[tokio::test(start_paused = true)]
    async fn test_sql_sync_lag() {
        // We simulate a long-running sync (lag) that times out.
        // We enforce the 60-second ML-Resilience rule here by simulating
        // a database operation that hangs for 65 seconds using tokio's
        // time pausing (so the test runs instantly).
        let db_id = uuid::Uuid::new_v4().to_string();
        let uri = format!("sqlite:file:{}?mode=memory&cache=shared", db_id);

        let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect_lazy(&uri)
            .unwrap();

        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(),
            store: DbStore::Sqlite(sqlite_pool.clone()),
        });

        // Test the actual execute_with_retry timeout logic
        let res: Result<(), String> = db.execute_with_retry("slow_query", || async {
            // Simulate a query that takes longer than the timeout
            tokio::time::sleep(std::time::Duration::from_secs(65)).await;
            Ok(())
        }).await;

        assert!(res.is_err(), "Sync operation should time out to prevent cascading failures");
        assert!(res.unwrap_err().to_string().contains("timed out"), "Must be explicitly timed out by ML-Resilience rule");
    }
}
