#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use crate::db::{DB, DbStore};
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::str::FromStr;
    use crate::services::sync::power_sync_orchestrator::PowerSyncOrchestrator;

    #[tokio::test]
    async fn test_power_sync_orchestrator_push() {
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

        // Normally, the orchestrator connects to a real gRPC endpoint.
        // In this test, we can just instantiate it to ensure it correctly resolves
        // pending items without panicking, up to the point of network failure since
        // the endpoint is localhost:0 which is not a real gRPC server.
        let orchestrator = PowerSyncOrchestrator::new(db, "http://127.0.0.1:0".to_string());

        let res = orchestrator.push_sync().await;

        // We expect it to fail gracefully because 127.0.0.1:0 is not running our gRPC server
        assert!(res.is_err(), "Expected push_sync to return a network error but it succeeded");
        let err_msg = res.unwrap_err();
        assert!(err_msg.contains("Connection refused") || err_msg.contains("transport error"), "Unexpected error: {}", err_msg);
    }

    #[tokio::test]
    async fn test_power_sync_orchestrator_e2e() {
        use crate::ohc::orchestration::sync_service_server::SyncServiceServer;
        use crate::services::sync::service::MySyncService;
        use tonic::transport::Server;
        use std::net::SocketAddr;
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());

        if !database_url.contains("test") {
            return;
        }

        let cloud_pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("RESET app.current_tenant").await?; Ok(true) }) })
            .before_acquire(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("SET app.current_tenant = 'system'").await?; Ok(true) }) })
            .connect_lazy(&database_url).unwrap();

        let sync_service = MySyncService::new(cloud_pool.clone());

        let server = Server::builder()
            .add_service(SyncServiceServer::new(sync_service));

        let serve_future = server.serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener));
        tokio::spawn(serve_future);

        let conn_opts = SqliteConnectOptions::from_str("sqlite::memory:")
            .unwrap()
            .create_if_missing(true);

        let local_pool = SqlitePoolOptions::new()
            .connect_with(conn_opts)
            .await
            .unwrap();

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
        sqlx::query(schema).execute(&local_pool).await.unwrap();

        let test_mission_id = "test_e2e_local_to_cloud_mission";

        sqlx::query("INSERT INTO agent_missions (id, status, payload, organization_id, _sync_status) VALUES (?, ?, ?, ?, ?)")
            .bind(test_mission_id)
            .bind("COMPLETED")
            .bind("{}")
            .bind("system")
            .bind("pending")
            .execute(&local_pool)
            .await
            .unwrap();

        let db = Arc::new(DB {
            pool: cloud_pool.clone(),
            store: DbStore::Sqlite(local_pool.clone()),
        });

        let orchestrator = PowerSyncOrchestrator::new(db, format!("http://{}", addr));

        let res = orchestrator.push_sync().await;

        assert!(res.is_ok(), "push_sync should succeed to our real test server");

        use sqlx::Row;
        let row = sqlx::query("SELECT _sync_status FROM agent_missions WHERE id = ?")
            .bind(test_mission_id)
            .fetch_one(&local_pool)
            .await
            .unwrap();
        let status: String = row.get("_sync_status");
        assert_eq!(status, "synced");

        let cloud_row = sqlx::query("SELECT status, _sync_status FROM agent_missions WHERE id = $1")
            .bind(test_mission_id)
            .fetch_one(&cloud_pool)
            .await
            .unwrap();
        let cloud_status: String = cloud_row.get("status");
        let cloud_sync_status: String = cloud_row.get("_sync_status");
        assert_eq!(cloud_status, "COMPLETED");
        assert_eq!(cloud_sync_status, "synced");
    }
}
