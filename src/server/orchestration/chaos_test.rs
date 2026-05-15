use crate::db::{DB, DbStore};
use crate::orchestration::mesh::TeammateMesh;
use ohc_builtin_agent::mesh::transport::Message;

use async_trait::async_trait;

use std::sync::Arc;
use tokio::time::Duration;
use crate::orchestration::state::StateManager;
use crate::orchestration::state::cloud::CloudStateManager;

// A Mock mesh that emits malformed payload
struct CorruptedMockMesh {
    received_messages: std::sync::Arc<std::sync::atomic::AtomicUsize>,
}

impl CorruptedMockMesh {
    fn new() -> Self {
        Self {
            received_messages: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        }
    }
}

#[async_trait]
impl TeammateMesh for CorruptedMockMesh {
    async fn publish(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> {
        Ok(())
    }
    async fn publish_with_ack(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> {
        Ok(())
    }
    async fn subscribe(&self, _topic: &str, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let counter = self.received_messages.clone();
        tokio::spawn(async move {
            let corrupted_msg = Message { agent_id: "sys".into(), action: "test".into(), status: "ok".into(),

                payload: vec![255, 255, 255, 255, 0, 1, 2, 3], // invalid utf8 / JSON
                msg_id: "corrupt_1".to_string(),

            };
            handler(corrupted_msg);
            counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        });
        Ok(Box::new(|| {}))
    }
    async fn acquire_lock(&self, _resource: &str, _owner: &str, _ttl_seconds: u64) -> Result<bool, String> {
        Ok(true)
    }
    async fn release_lock(&self, _resource: &str, _owner: &str) -> Result<(), String> {
        Ok(())
    }
    async fn register_presence(&self, _agent_id: &str, _status: &str, _ttl_seconds: u64) -> Result<(), String> { Ok(()) }
    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> { Ok(vec![]) }
    async fn ping(&self) -> Result<(), String> { Ok(()) }
    async fn start_health_responder(&self) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(|| {})) }
    async fn publish_state_handoff(&self, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
    async fn subscribe_state_handoff(&self, _handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(|| {})) }
}

struct RacingLockMesh {
    transport: ohc_builtin_agent::mesh::transport::MemoryTransport,
}

impl RacingLockMesh {
    fn new() -> Self {
        Self {
            transport: ohc_builtin_agent::mesh::transport::MemoryTransport::new(),
        }
    }
}

#[async_trait]
impl TeammateMesh for RacingLockMesh {
    async fn publish(&self, topic: &str, payload: Vec<u8>) -> Result<(), String> { self.transport.publish(topic, ohc_builtin_agent::mesh::transport::TeammateMeshEvent { agent_id: "sys".into(), action: topic.into(), status: "ok".into(), payload, msg_id: "m1".into() }).await }
    async fn publish_with_ack(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
    async fn subscribe(&self, topic: &str, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> { self.transport.subscribe(topic, handler).await }
    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String> {
        // Just use the internal memory transport to simulate a real Redis-backed cross-process lock
        self.transport.acquire_lock(resource, owner, ttl_seconds).await
    }
    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
        self.transport.release_lock(resource, owner).await
    }
    async fn register_presence(&self, _agent_id: &str, _status: &str, _ttl_seconds: u64) -> Result<(), String> { Ok(()) }
    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> { Ok(vec![]) }
    async fn ping(&self) -> Result<(), String> { Ok(()) }
    async fn start_health_responder(&self) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(|| {})) }
    async fn publish_state_handoff(&self, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
    async fn subscribe_state_handoff(&self, _handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(|| {})) }
}






// A mock transport that occasionally drops messages to test Pub/Sub message loss resilience
struct DroppingMockTransport {
    transport: ohc_builtin_agent::mesh::transport::MemoryTransport,
    drop_rate: std::sync::atomic::AtomicUsize,
}

impl DroppingMockTransport {
    fn new(drop_rate: usize) -> Self {
        Self {
            transport: ohc_builtin_agent::mesh::transport::MemoryTransport::new(),
            drop_rate: std::sync::atomic::AtomicUsize::new(drop_rate),
        }
    }
}

#[async_trait]
impl ohc_builtin_agent::mesh::transport::MeshTransport for DroppingMockTransport {
    async fn publish(&self, topic: &str, event: ohc_builtin_agent::mesh::transport::TeammateMeshEvent) -> Result<(), String> {
        let rate = self.drop_rate.load(std::sync::atomic::Ordering::SeqCst);
        let should_drop = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as usize) % 100 < rate;
        if should_drop {
            // Simulate dropping the message
            return Ok(());
        }
        self.transport.publish(topic, event).await
    }
    async fn subscribe(&self, topic: &str, handler: Box<dyn Fn(ohc_builtin_agent::mesh::transport::Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        self.transport.subscribe(topic, handler).await
    }
    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String> { Ok(true) }
    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> { Ok(()) }
    async fn register_presence(&self, _agent_id: &str, _status: &str, _ttl_seconds: u64) -> Result<(), String> { Ok(()) }
    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> { Ok(vec![]) }
}

struct SleepingMockMesh;

#[async_trait]
impl TeammateMesh for SleepingMockMesh {
    async fn publish(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
    async fn publish_with_ack(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
    async fn subscribe(&self, _topic: &str, _handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(|| {})) }
    async fn acquire_lock(&self, _resource: &str, _owner: &str, _ttl_seconds: u64) -> Result<bool, String> {
        tokio::time::sleep(tokio::time::Duration::from_millis(2500)).await;
        Ok(true)
    }
    async fn release_lock(&self, _resource: &str, _owner: &str) -> Result<(), String> { Ok(()) }

    async fn register_presence(&self, _agent_id: &str, _status: &str, _ttl_seconds: u64) -> Result<(), String> { Ok(()) }
    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> { Ok(vec![]) }
    async fn ping(&self) -> Result<(), String> { Ok(()) }
    async fn start_health_responder(&self) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(|| {})) }
    async fn publish_state_handoff(&self, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
    async fn subscribe_state_handoff(&self, _handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(|| {})) }
}


#[cfg(test)]
mod chaos_tests {
    use super::*;

    #[tokio::test]
    async fn test_redis_mailbox_corruption() {
        let mesh = Arc::new(CorruptedMockMesh::new());
        let counter = mesh.received_messages.clone();

        // This will spawn a task that immediately receives corrupted message
        let _ = mesh.subscribe("mesh:test:corrupt", Box::new(|msg| {
            // Simulate how the orchestrator processes JSON
            let _parsed: Result<serde_json::Value, _> = serde_json::from_slice(&msg.payload);
            // It should gracefully error out without panicking
            assert!(_parsed.is_err());
        })).await.unwrap();

        tokio::time::sleep(Duration::from_millis(100)).await;
        assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_agent_lock_race_conditions() {
        let mesh = Arc::new(RacingLockMesh::new());

        let mut join_handles = vec![];
        let resource_name = "ohc:lock:test_race_lock";

        // Spawn 100 concurrent tasks trying to acquire the same lock
        for i in 0..100 {
            let mesh_clone = mesh.clone();
            let owner = format!("agent_{}", i);
            join_handles.push(tokio::spawn(async move {
                mesh_clone.acquire_lock(resource_name, &owner, 10).await.unwrap_or(false)
            }));
        }

        let mut winners = 0;
        for handle in join_handles {
            if handle.await.unwrap() {
                winners += 1;
            }
        }

        // Ensure exactly ONE agent wins the race condition
        assert_eq!(winners, 1, "There should be exactly one winner in a lock race");
    }


    #[tokio::test]
    async fn test_agent_lock_corruption_handling() {
        use crate::orchestration::state::StateManager;

        let custom_tmp = std::env::temp_dir().join(format!(".agent-lock-chaos-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&custom_tmp).unwrap();

        let resource_name = "test_chaos_lock_harness";
        let lock_path = custom_tmp.join(format!("ohc_mesh_lock_{}", resource_name));
        std::fs::write(&lock_path, "locked:9999999999").unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&lock_path).unwrap().permissions();
            perms.set_mode(0o400); // Read only
            std::fs::set_permissions(&lock_path, perms.clone()).unwrap();

            let mut dir_perms = std::fs::metadata(&custom_tmp).unwrap().permissions();
            dir_perms.set_mode(0o500); // Prevent directory modification (read and execute only)
            std::fs::set_permissions(&custom_tmp, dir_perms).unwrap();
        }

        struct CorruptedTmpTransport {
            tmp_dir: std::path::PathBuf,
        }

        #[async_trait::async_trait]
        impl ohc_builtin_agent::mesh::transport::MeshTransport for CorruptedTmpTransport {
            async fn publish(&self, _topic: &str, _message: ohc_builtin_agent::mesh::transport::Message) -> Result<(), String> { Ok(()) }
            async fn subscribe(&self, _topic: &str, _handler: Box<dyn Fn(ohc_builtin_agent::mesh::transport::Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(|| {})) }

            async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String> {
                let lock_path = self.tmp_dir.join(format!("ohc_mesh_lock_{}", resource));
                let expires_at = chrono::Utc::now().timestamp_millis() + (ttl_seconds * 1000) as i64;
                let payload = format!("{}:{}", owner, expires_at);

                match std::fs::OpenOptions::new().write(true).create_new(true).open(&lock_path) {
                    Ok(mut f) => {
                        use std::io::Write;
                        let _ = f.write_all(payload.as_bytes());
                        Ok(true)
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists || e.kind() == std::io::ErrorKind::PermissionDenied => {
                        if let Ok(owner_bytes) = std::fs::read(&lock_path) {
                            let current_data = String::from_utf8_lossy(&owner_bytes).into_owned();
                            if let Some((stored_owner, stored_exp)) = current_data.split_once(':') {
                                if let Ok(exp) = stored_exp.parse::<i64>() {
                                    if stored_owner == owner || exp <= chrono::Utc::now().timestamp_millis() {
                                        let _ = std::fs::remove_file(&lock_path); // This will fail due to permissions
                                        if let Ok(mut f) = std::fs::OpenOptions::new().write(true).create_new(true).open(&lock_path) {
                                            use std::io::Write;
                                            let _ = f.write_all(payload.as_bytes());
                                            return Ok(true);
                                        }
                                    }
                                }
                            }
                        }
                        // Handle the permission error gracefully by failing to acquire the lock
                        Ok(false)
                    }
                    Err(e) => Err(e.to_string()),
                }
            }
            async fn release_lock(&self, _resource: &str, _owner: &str) -> Result<(), String> { Ok(()) }
            async fn register_presence(&self, _agent_id: &str, _status: &str, _ttl_seconds: u64) -> Result<(), String> { Ok(()) }
            async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> { Ok(vec![]) }
        }

        let mesh_transport = std::sync::Arc::new(CorruptedTmpTransport { tmp_dir: custom_tmp.clone() });
        let mesh = std::sync::Arc::new(crate::orchestration::mesh::CentrifugeNode::new(mesh_transport));

        // Use CloudStateManager since Standalone was tested in `chaos.rs`
        let dummy_pg_pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) })
            .connect_lazy("postgres://postgres:postgres@localhost:5432/test")
            .unwrap();
        let db = std::sync::Arc::new(DB { pool: dummy_pg_pool, store: DbStore::Postgres });

        let state_manager = CloudStateManager::new(db, mesh.clone());

        // This operation attempts to acquire the lock via the state manager
        let result = state_manager.pull_available_tasks(10).await;

        // Since the lock acquisition gracefully returns false instead of panicking,
        // the state manager's internal timeout/error handling will catch it and return an empty list or error
        assert!(result.is_ok(), "The state manager should gracefully handle lock acquisition failures caused by .agent-lock corruption");
        if let Ok(tasks) = result {
            assert_eq!(tasks.len(), 0, "No tasks should be pulled if the lock couldn't be acquired");
        }

        // Cleanup
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut dir_perms = std::fs::metadata(&custom_tmp).unwrap().permissions();
            dir_perms.set_mode(0o755);
            std::fs::set_permissions(&custom_tmp, dir_perms).unwrap();

            let mut file_perms = std::fs::metadata(&lock_path).unwrap().permissions();
            file_perms.set_mode(0o644);
            std::fs::set_permissions(&lock_path, file_perms).unwrap();
        }

        std::fs::remove_dir_all(&custom_tmp).unwrap();
    }

    #[tokio::test]
    async fn test_pubsub_message_loss() {
        let transport = Arc::new(DroppingMockTransport::new(50)); // 50% drop rate
        let mesh = Arc::new(crate::orchestration::mesh::CentrifugeNode::new(transport));
        let received = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let received_clone = received.clone();

        let _ = mesh.subscribe("mesh:test:loss", Box::new(move |_msg| {
            received_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        })).await.unwrap();

        // Start health responder (simulates ack responder)
        let _ = mesh.start_health_responder().await;

        // Send 20 messages with publish_with_ack which simulates the resilience.
        // CentrifugeNode's publish_with_ack implements retries automatically!
        let mut successful_sends = 0;
        for _ in 0..20 {
             // In CentrifugeNode, publish_with_ack subscribes to ack topic, sends, and waits.
             // We can just use ping() which wraps publish_with_ack for health!
             if mesh.ping().await.is_ok() {
                 successful_sends += 1;
             }
        }

        tokio::time::sleep(Duration::from_millis(200)).await;

        // Resilience rule: system must recover or degrade gracefully.
        // We verify that some messages were successfully delivered and ack'd despite high packet loss,
        // and that the retry mechanism helped improve the delivery rate.

        assert!(successful_sends > 0, "System should successfully send at least some messages under chaos");
        // Because of CentrifugeNode's retries, successful_sends should be roughly 87.5% of 20 (approx 17)
        assert!(successful_sends >= 10, "Retry logic should recover a significant portion of dropped messages");
    }

    #[tokio::test]
    async fn test_cloud_degradation_fallback() {
        // We use an empty db pool but with CloudStateManager to see fail-safes on lock acquisition timeout
        let dummy_pg_pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).max_connections(1)
            .connect_lazy("postgres://postgres:postgres@localhost:5432/test")
            .unwrap();

        let db = Arc::new(DB {
            pool: dummy_pg_pool,
            store: DbStore::Postgres,
        });

        let mesh: Arc<dyn TeammateMesh> = Arc::new(SleepingMockMesh);
        let state_manager = CloudStateManager::new(db.clone(), mesh);

        let start = std::time::Instant::now();
        let tasks = state_manager.pull_available_tasks(10).await.unwrap_or(vec![]);
        let elapsed = start.elapsed();

        // The pull_available_tasks for cloud has a 2-second timeout on the lock or DB
        // The mocked sleeping mesh sleeps for 2.5s, forcing the 2s timeout to trigger.
        assert!(elapsed < std::time::Duration::from_millis(2200));
        assert!(elapsed > std::time::Duration::from_millis(1900));

        // It must fallback safely returning an empty vector
        assert_eq!(tasks.len(), 0);
    }

    #[tokio::test]
    async fn test_cloud_db_transition_fallback() {
        // Intentionally bad DB URL to simulate database failure / degraded performance
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect_lazy("postgres://postgres:postgres@localhost:12345/nonexistent")
            .unwrap();

        let db = Arc::new(DB {
            pool,
            store: DbStore::Postgres,
        });

        let mesh: Arc<dyn TeammateMesh> = Arc::new(SleepingMockMesh);
        let state_manager = CloudStateManager::new(db, mesh);

        let result = state_manager.transition_state("task1", "tenant1", "PENDING", "IN_PROGRESS", None, None).await;

        // Should fallback safely instead of panicking/blocking forever
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_cloud_db_pull_fallback() {
        // Intentionally bad DB URL to simulate database failure / degraded performance
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect_lazy("postgres://postgres:postgres@localhost:12345/nonexistent")
            .unwrap();

        let db = Arc::new(DB {
            pool,
            store: DbStore::Postgres,
        });

        let mesh: Arc<dyn TeammateMesh> = Arc::new(SleepingMockMesh);
        let state_manager = CloudStateManager::new(db, mesh);

        let tasks = state_manager.pull_available_tasks(10).await;

        // On connection failure (not timeout), it correctly propagates the error.
        assert!(tasks.is_err());
    }

    #[tokio::test]
    async fn test_standalone_db_transition_fallback() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect_lazy("sqlite::memory:")
            .unwrap();

        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().max_connections(1).connect_lazy("postgres://postgres:postgres@localhost:12345/nonexistent").unwrap(),
            store: DbStore::Sqlite(pool),
        });

        // The fallback is tested via a timeout on the inner lock block
        let mesh: Arc<dyn TeammateMesh> = Arc::new(SleepingMockMesh);
        let state_manager = crate::orchestration::state::standalone::StandaloneStateManager::new(db, mesh);

        let result = state_manager.transition_state("task1", "tenant1", "PENDING", "IN_PROGRESS", None, None).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_standalone_db_pull_fallback() {
        let dummy_sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect_lazy("sqlite::memory:")
            .unwrap();

        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().max_connections(1).connect_lazy("postgres://postgres:postgres@localhost:5432/test").unwrap(),
            store: DbStore::Sqlite(dummy_sqlite_pool),
        });

        let mesh: Arc<dyn TeammateMesh> = Arc::new(SleepingMockMesh);
        let state_manager = crate::orchestration::state::standalone::StandaloneStateManager::new(db.clone(), mesh);

        let tasks = state_manager.pull_available_tasks(10).await;

        // With SleepingMockMesh, this triggers the inner lock timeout.
        assert!(tasks.is_ok());
        assert_eq!(tasks.unwrap().len(), 0);
    }
}
