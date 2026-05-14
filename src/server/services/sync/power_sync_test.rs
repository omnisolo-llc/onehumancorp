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
                created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                organization_id TEXT NOT NULL DEFAULT 'system',
                cloud_mission_id TEXT,
                sync_error TEXT,
                last_synced_at TIMESTAMP,
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
                .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
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
    async fn test_power_sync_orchestrator_pull() {
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

        // Create dummy DB structure wrapped with our DbStore::Sqlite
        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new()
                .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
                .connect_lazy("postgres://localhost/dummy").unwrap(),
            store: DbStore::Sqlite(pool.clone()),
        });

        // Normally, the orchestrator connects to a real gRPC endpoint.
        // In this test, we can just instantiate it to ensure it correctly handles
        // network failure since the endpoint is localhost:0 which is not a real gRPC server.
        let orchestrator = PowerSyncOrchestrator::new(db, "http://127.0.0.1:0".to_string());

        let res = orchestrator.pull_sync().await;

        // We expect it to fail gracefully because 127.0.0.1:0 is not running our gRPC server
        assert!(res.is_err(), "Expected pull_sync to return a network error but it succeeded");
        let err_msg = res.unwrap_err();
        assert!(err_msg.contains("Connection refused") || err_msg.contains("transport error"), "Unexpected error: {}", err_msg);
    }

    #[tokio::test]
    async fn test_power_sync_e2e_flow() {
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

        // Create dummy DB structure wrapped with our DbStore::Sqlite
        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new()
                .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
                .connect_lazy("postgres://localhost/dummy").unwrap(),
            store: DbStore::Sqlite(pool.clone()),
        });

        // Start a mock gRPC server
        use tonic::{Request, Response, Status};
        use crate::ohc::orchestration::{PowerSyncPushRequest, PowerSyncPushResponse, PowerSyncPullRequest, PowerSyncPullResponse};
        use crate::ohc::orchestration::sync_service_server::{SyncService, SyncServiceServer};
        use tokio::sync::Mutex;

        struct MockSyncService {
            pushed_items: Arc<Mutex<Vec<String>>>,
        }

        #[tonic::async_trait]
        impl SyncService for MockSyncService {
            async fn hybrid_sync_missions(
                &self,
                _request: Request<crate::ohc::orchestration::HybridSyncMissionsRequest>,
            ) -> Result<Response<crate::ohc::orchestration::HybridSyncMissionsResponse>, Status> {
                Ok(Response::new(crate::ohc::orchestration::HybridSyncMissionsResponse {
                    status: "success".to_string(),
                    message: "mock".to_string(),
                    synced_count: 0,
                }))
            }

            async fn vector_sync(
                &self,
                _request: Request<crate::ohc::orchestration::VectorSyncRequest>,
            ) -> Result<Response<crate::ohc::orchestration::VectorSyncResponse>, Status> {
                Ok(Response::new(crate::ohc::orchestration::VectorSyncResponse {
                    status: "success".to_string(),
                    message: "mock".to_string(),
                }))
            }

            async fn power_sync_push(
                &self,
                request: Request<PowerSyncPushRequest>,
            ) -> Result<Response<PowerSyncPushResponse>, Status> {
                let mut items = self.pushed_items.lock().await;
                items.push(request.into_inner().payload);
                Ok(Response::new(PowerSyncPushResponse {
                    status: "ok".to_string(),
                }))
            }

            async fn power_sync_pull(
                &self,
                _request: Request<PowerSyncPullRequest>,
            ) -> Result<Response<PowerSyncPullResponse>, Status> {
                let payload = serde_json::json!([{
                    "table": "agent_missions",
                    "id": "cloud_mission_1",
                    "status": "COMPLETED",
                    "payload": "{\"data\":\"cloud\"}",
                    "organization_id": "system",
                    "updated_at": chrono::Utc::now().to_rfc3339(),
                    "version": 2
                }]).to_string();

                Ok(Response::new(PowerSyncPullResponse {
                    payload,
                }))
            }

            async fn sync_mcp_deltas(
                &self,
                _request: Request<crate::ohc::orchestration::SyncMcpDeltasRequest>,
            ) -> Result<Response<crate::ohc::orchestration::SyncMcpDeltasResponse>, Status> {
                Ok(Response::new(crate::ohc::orchestration::SyncMcpDeltasResponse {
                    status: "success".to_string(),
                    message: "mock".to_string(),
                    synced_count: 0,
                }))
            }

            async fn sync_escalation(
                &self,
                _request: Request<crate::ohc::orchestration::SyncEscalationRequest>,
            ) -> Result<Response<crate::ohc::orchestration::SyncEscalationResponse>, Status> {
                Ok(Response::new(crate::ohc::orchestration::SyncEscalationResponse {
                    status: "success".to_string(),
                    message: "mock".to_string(),
                    synced_count: 0,
                }))
            }
        }

        let pushed_items = Arc::new(Mutex::new(Vec::new()));
        let service = MockSyncService {
            pushed_items: pushed_items.clone(),
        };

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            tonic::transport::Server::builder()
                .add_service(SyncServiceServer::new(service))
                .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener))
                .await
                .unwrap();
        });

        let orchestrator = PowerSyncOrchestrator::new(db, format!("http://{}", addr));

        // Part 1: Test Push (Offline local write reaches the cloud)
        sqlx::query("INSERT INTO agent_missions (id, status, payload, organization_id, _sync_status) VALUES (?, ?, ?, ?, ?)")
            .bind("local_mission_1")
            .bind("pending")
            .bind("{\"data\":\"local\"}")
            .bind("system")
            .bind("pending")
            .execute(&pool)
            .await
            .unwrap();

        let res = orchestrator.push_sync().await;
        assert!(res.is_ok(), "push_sync failed: {:?}", res);

        // Verify it was pushed
        let items = pushed_items.lock().await;
        assert_eq!(items.len(), 1);
        assert!(items[0].contains("local_mission_1"));
        drop(items);

        // Verify local status was updated to synced
        use sqlx::Row;
        let row = sqlx::query("SELECT _sync_status FROM agent_missions WHERE id = 'local_mission_1'")
            .fetch_one(&pool)
            .await
            .unwrap();
        let sync_status: String = row.get("_sync_status");
        assert_eq!(sync_status, "synced");

        // Part 2: Test Pull (Cloud write reaches offline local)
        let res2 = orchestrator.pull_sync().await;
        assert!(res2.is_ok(), "pull_sync failed: {:?}", res2);

        // Verify it was saved locally
        let row2 = sqlx::query("SELECT status, payload FROM agent_missions WHERE id = 'cloud_mission_1'")
            .fetch_one(&pool)
            .await
            .unwrap();
        let status: String = row2.get("status");
        let payload: String = row2.get("payload");
        assert_eq!(status, "COMPLETED");
        assert_eq!(payload, "{\"data\":\"cloud\"}");
    }
}
