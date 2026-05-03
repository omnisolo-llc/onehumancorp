#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::db::{DB, DbStore};
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::str::FromStr;
    use crate::orchestration::power_sync::PowerSyncOrchestrator;

    #[tokio::test]
    async fn test_power_sync_orchestrator_push_pull() {
        // Setup an in-memory SQLite DB
        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:")
            .unwrap()
            .create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .connect_with(conn_opts)
            .await
            .unwrap();

        // Initialize schema for agent_missions
        let schema = "
            CREATE TABLE IF NOT EXISTS agent_missions (
                id TEXT PRIMARY KEY,
                status TEXT NOT NULL,
                payload TEXT NOT NULL,
                created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
                organization_id TEXT NOT NULL DEFAULT 'system',
                cloud_mission_id TEXT,
                sync_error TEXT,
                last_synced_at DATETIME,
                synced_to_cloud BOOLEAN DEFAULT 0,
                _sync_status TEXT DEFAULT 'pending',
                version INTEGER DEFAULT 1
            );
        ";
        sqlx::query(schema).execute(&pool).await.unwrap();

        // Insert a dummy mission
        sqlx::query("INSERT INTO agent_missions (id, status, payload, organization_id, _sync_status) VALUES (?, ?, ?, ?, ?)")
            .bind("dummy_mission_1")
            .bind("pending")
            .bind("{}")
            .bind("system")
            .bind("pending")
            .execute(&pool)
            .await
            .unwrap();

        // Create dummy DB structure wrapped with our DbStore::Sqlite
        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new()
                .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("RESET app.current_tenant").await?; Ok(true) }) })
                .connect_lazy("postgres://localhost/dummy").unwrap(),
            store: DbStore::Sqlite(pool.clone()),
        });

        // Normally, the orchestrator connects to a real REST endpoint.
        // In this test, we can just instantiate it to ensure it correctly resolves
        // pending items without panicking, up to the point of network failure since
        // the endpoint is localhost:0 which is not a real server.
        let orchestrator = PowerSyncOrchestrator::new(db, "http://127.0.0.1:0".to_string());

        let res = orchestrator.push_sync().await;

        // We expect it to fail gracefully because 127.0.0.1:0 is not running our REST server
        assert!(res.is_err(), "Expected push_sync to return a network error but it succeeded");
        let err_msg = res.unwrap_err();
        assert!(err_msg.contains("Connection refused") || err_msg.contains("error sending request"), "Unexpected error: {}", err_msg);

        let res_pull = orchestrator.pull_sync().await;
        assert!(res_pull.is_err(), "Expected pull_sync to return a network error but it succeeded");
    }
}
