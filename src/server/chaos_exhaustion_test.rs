#[cfg(test)]
mod chaos_exhaustion_tests {
    use crate::db::{DB, DbStore};
    use std::sync::Arc;

    #[tokio::test(start_paused = true)]
    async fn test_graceful_degradation_host_exhaustion() {
        // Start heavy CPU and memory workload
        let mut heavy_handles = vec![];
        for _ in 0..50 {
            heavy_handles.push(tokio::spawn(async move {
                // allocate a large chunk of memory
                let mut data = vec![0u8; 1024 * 1024 * 2]; // 2MB each

                // tight CPU loop for a short while
                let start = std::time::Instant::now();
                while start.elapsed() < std::time::Duration::from_millis(150) {
                    for i in 0..data.len() {
                        data[i] = data[i].wrapping_add(1);
                    }
                }
            }));
        }

        // Run the DB operation concurrently with the heavy load
        let sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect_lazy("sqlite::memory:")
            .unwrap();

        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(),
            store: DbStore::Sqlite(sqlite_pool.clone()),
        });

        // Test normal operation success
        let db_res: Result<(), String> = db.execute_with_retry("exhaustion_test", || async {
            Ok(())
        }).await;

        // Test timeout degradation
        let timeout_res: Result<(), String> = db.execute_with_retry("exhaustion_timeout", || async {
             tokio::time::sleep(std::time::Duration::from_secs(65)).await;
             Err("Should not happen".to_string())
        }).await;

        for h in heavy_handles {
            let _ = h.await;
        }

        assert!(db_res.is_ok(), "Normal DB ops should complete successfully under heavy load");
        assert!(timeout_res.is_err(), "Timeouts should still function safely without cascading failure under heavy load");
        assert!(timeout_res.unwrap_err().contains("timed out"));
    }
}
