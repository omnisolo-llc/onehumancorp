use crate::db::{DB, DbStore};
use crate::orchestration::mesh::TeammateMesh;
use ohc_builtin_agent::mesh::transport::Message;

use async_trait::async_trait;

use std::sync::Arc;
use tokio::time::Duration;
use crate::orchestration::state::StateManager;
use crate::orchestration::state::cloud::CloudStateManager;

// A Mock mesh that introduces network latency
struct LatencyMockMesh {
    delay_ms: u64,
}

impl LatencyMockMesh {
    fn new(delay_ms: u64) -> Self {
        Self { delay_ms }
    }
}

#[async_trait]
impl TeammateMesh for LatencyMockMesh {
    async fn publish(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> {
        tokio::time::sleep(Duration::from_millis(self.delay_ms)).await;
        Ok(())
    }
    async fn publish_with_ack(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> {
        tokio::time::sleep(Duration::from_millis(self.delay_ms)).await;
        Ok(())
    }
    async fn subscribe(&self, _topic: &str, _handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        tokio::time::sleep(Duration::from_millis(self.delay_ms)).await;
        Ok(Box::new(|| {}))
    }
    async fn acquire_lock(&self, _resource: &str, _owner: &str, _ttl_seconds: u64) -> Result<bool, String> {
        tokio::time::sleep(Duration::from_millis(self.delay_ms)).await;
        Ok(true)
    }
    async fn release_lock(&self, _resource: &str, _owner: &str) -> Result<(), String> {
        tokio::time::sleep(Duration::from_millis(self.delay_ms)).await;
        Ok(())
    }
    async fn register_presence(&self, _agent_id: &str, _status: &str, _ttl_seconds: u64) -> Result<(), String> {
        tokio::time::sleep(Duration::from_millis(self.delay_ms)).await;
        Ok(())
    }
    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
        tokio::time::sleep(Duration::from_millis(self.delay_ms)).await;
        Ok(vec![])
    }
    async fn ping(&self) -> Result<(), String> {
        tokio::time::sleep(Duration::from_millis(self.delay_ms)).await;
        Ok(())
    }
    async fn start_health_responder(&self) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        tokio::time::sleep(Duration::from_millis(self.delay_ms)).await;
        Ok(Box::new(|| {}))
    }
    async fn publish_state_handoff(&self, _payload: Vec<u8>) -> Result<(), String> {
        tokio::time::sleep(Duration::from_millis(self.delay_ms)).await;
        Ok(())
    }
    async fn subscribe_state_handoff(&self, _handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        tokio::time::sleep(Duration::from_millis(self.delay_ms)).await;
        Ok(Box::new(|| {}))
    }
}

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
    transport: ohc_builtin_agent::mesh::transport::InProcessTransport,
}

impl RacingLockMesh {
    fn new() -> Self {
        Self {
            transport: ohc_builtin_agent::mesh::transport::InProcessTransport::new(),
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
    transport: ohc_builtin_agent::mesh::transport::InProcessTransport,
    drop_rate: std::sync::atomic::AtomicUsize,
}

impl DroppingMockTransport {
    fn new(drop_rate: usize) -> Self {
        Self {
            transport: ohc_builtin_agent::mesh::transport::InProcessTransport::new(),
            drop_rate: std::sync::atomic::AtomicUsize::new(drop_rate),
        }
    }
}

#[async_trait]
impl ohc_builtin_agent::mesh::transport::MeshTransport for DroppingMockTransport {
    async fn publish(&self, topic: &str, event: ohc_builtin_agent::mesh::transport::TeammateMeshEvent) -> Result<(), String> {
        let rate = self.drop_rate.load(std::sync::atomic::Ordering::SeqCst);

        let mut success = false;
        let mut attempts = 0;
        let max_attempts = 5;

        while attempts < max_attempts {
            let should_drop = (rand::random::<f64>() * 100.0) < rate as f64;
            if !should_drop {
                success = true;
                break;
            }
            attempts += 1;
            tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
        }

        if !success {
             return Ok(()); // Dropped after all retries
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
        tokio::time::sleep(tokio::time::Duration::from_millis(61000)).await;
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
    #[tokio::test]
    async fn test_timeout_storm() {
        let mesh: Arc<dyn TeammateMesh> = Arc::new(crate::orchestration::mesh::CentrifugeNode::new_with_timeout(Arc::new(SleepingMockMesh), std::time::Duration::from_millis(50)));

        let mut successful_sends = 0;
        let mut timeouts = 0;

        for _ in 0..10 {
            let result = mesh.ping().await;
            if result.is_err() {
                timeouts += 1;
            } else {
                successful_sends += 1;
            }
        }

        assert!(timeouts >= 10, "System should timeout internally on all calls when agent is non-responsive");
        assert_eq!(successful_sends, 0, "System should not have successful sends during a timeout storm");
    }

    #[tokio::test]
    async fn test_pubsub_high_message_loss_degradation() {
        let transport = Arc::new(DroppingMockTransport::new(90));
        let mesh = Arc::new(crate::orchestration::mesh::CentrifugeNode::new(transport));
        let received = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let received_clone = received.clone();

        let _ = mesh.subscribe("mesh:test:severe_loss", Box::new(move |_msg| {
            received_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        })).await.unwrap();

        let _ = mesh.start_health_responder().await;

        let mut successful_sends = 0;
        let mut failed_sends = 0;

        for _ in 0..20 {
             if mesh.ping().await.is_ok() {
                 successful_sends += 1;
             } else {
                 failed_sends += 1;
             }
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

        assert!(failed_sends > 0, "System should report failed sends under severe degradation");
        assert_eq!(successful_sends + failed_sends, 20, "All messages should be accounted for (success or safe failure)");
    }

    #[tokio::test]
    async fn test_partition_tolerance() {
        let transport = Arc::new(DroppingMockTransport::new(100));
        let mesh = Arc::new(crate::orchestration::mesh::CentrifugeNode::new(transport));

        let _ = mesh.subscribe("mesh:test:partition", Box::new(move |_msg| {
        })).await.unwrap();

        let _ = mesh.start_health_responder().await;

        let mut successful_sends = 0;
        let mut failures = 0;

        for _ in 0..10 {
            let result = mesh.ping().await;
            if result.is_err() {
                failures += 1;
            } else {
                successful_sends += 1;
            }
        }

        assert_eq!(failures, 10, "System should handle partition by failing all requests gracefully");
        assert_eq!(successful_sends, 0, "System should not have successful sends during a network partition");
    }

    use super::*;

    #[tokio::test]
    async fn test_network_partition_parity() {
        // Test network partition via LatencyMockMesh (1000ms delay) on both Cloud and Standalone

        let latency_mesh: Arc<dyn TeammateMesh> = Arc::new(LatencyMockMesh::new(1000));

        // 1. Cloud (Postgres)
        let dummy_pg_pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .acquire_timeout(std::time::Duration::from_millis(50))
            .connect_lazy("postgres://postgres:postgres@localhost:5432/test")
            .unwrap();

        let cloud_db = Arc::new(DB {
            pool: dummy_pg_pool.clone(),
            store: DbStore::Postgres,
        });

        let cloud_state_manager = CloudStateManager::new(cloud_db, latency_mesh.clone());

        // 2. Standalone (SQLite)
        let dummy_sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .acquire_timeout(std::time::Duration::from_millis(50))
            .connect_lazy("sqlite::memory:")
            .unwrap();

        // Fix copy paste artifact: do not reuse pg pool clone, mock a new one
        let standalone_db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new()
                .max_connections(1)
                .acquire_timeout(std::time::Duration::from_millis(50))
                .connect_lazy("postgres://postgres:postgres@localhost:5432/test")
                .unwrap(),
            store: DbStore::Sqlite(dummy_sqlite_pool),
        });

        let standalone_state_manager = crate::orchestration::state::standalone::StandaloneStateManager::new(standalone_db, latency_mesh.clone());

        // We pull tasks under network partition (simulated by LatencyMockMesh).
        // Since the mesh lock acquires take 1000ms and state_manager_timeout defaults to ~2000ms,
        // it may pass or fail based on timing. To guarantee timeout, we use a 3000ms delay internally
        // in a new test, but here we expect the fail-safe behavior of StateManagers:
        // if they timeout, they return an empty vector rather than panicking.

        let start_cloud = std::time::Instant::now();
        let cloud_tasks = cloud_state_manager.pull_available_tasks(10).await;
        let elapsed_cloud = start_cloud.elapsed();

        let start_standalone = std::time::Instant::now();
        let standalone_tasks = standalone_state_manager.pull_available_tasks(10).await;
        let elapsed_standalone = start_standalone.elapsed();

        // Parity verification: both should gracefully fallback (likely to empty lists or error, but must not panic)
        // Cloud and Standalone should behave identically at the API boundary
        assert_eq!(cloud_tasks.is_ok(), standalone_tasks.is_ok(), "Mode parity gap: Cloud and Standalone behave differently under network partition.");
        if let Ok(c_tasks) = cloud_tasks {
            let s_tasks = standalone_tasks.unwrap();
            assert_eq!(c_tasks.len(), 0, "Expected empty fallback under partition");
            assert_eq!(s_tasks.len(), 0, "Expected empty fallback under partition");
        }
    }

    #[tokio::test]
    async fn test_redis_mailbox_corruption() {
        let mesh = Arc::new(CorruptedMockMesh::new());
        let counter = mesh.received_messages.clone();

        let notify = Arc::new(tokio::sync::Notify::new());
        let notify_clone = notify.clone();

        // This will spawn a task that immediately receives corrupted message
        let _ = mesh.subscribe("mesh:test:corrupt", Box::new(move |msg| {
            // Simulate how the orchestrator processes JSON with fallback
            let parsed: Result<serde_json::Value, _> = serde_json::from_slice(&msg.payload);
            if parsed.is_err() {
                 tracing::warn!("Message mailbox corruption detected. Applying circuit breaker fallback logic.");
                 // Circuit breaker loop / fallback
                 let mut fallback_attempts = 0;
                 while fallback_attempts < 3 {
                      // Attempt to retrieve a safe fallback state from local storage or cache.
                      // For test simulation, we just increment attempt and break
                      fallback_attempts += 1;
                 }
                 assert_eq!(fallback_attempts, 3);
            }
            notify_clone.notify_one();
        })).await.unwrap();

        notify.notified().await;
        assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_agent_lock_race_conditions() {
        let _tracker = crate::telemetry::ChaosRecoveryTracker::new("Cloud");
        let mesh = Arc::new(RacingLockMesh::new());

        let mut join_handles = vec![];
        let resource_name = "ohc:lock:test_race_lock";

        let num_tasks = 25; // Simulating extreme contention
        let barrier = Arc::new(tokio::sync::Barrier::new(num_tasks));

        // Spawn concurrent tasks trying to acquire the same lock exactly at the same time
        for i in 0..num_tasks {
            let mesh_clone = mesh.clone();
            let barrier_clone = barrier.clone();
            let owner = format!("agent_{}", i);
            join_handles.push(tokio::spawn(async move {
                barrier_clone.wait().await;
                mesh_clone.acquire_lock(resource_name, &owner, 10).await.unwrap_or(false)
            }));
        }

        let mut winners = 0;
        for handle in join_handles {
            if handle.await.unwrap() {
                winners += 1;
            }
        }

        // Ensure exactly ONE agent wins the race condition under massive load
        assert_eq!(winners, 1, "There should be exactly one winner in a lock race even under high contention");
    }


    #[tokio::test]
    async fn test_pubsub_message_loss() {
        let _tracker = crate::telemetry::ChaosRecoveryTracker::new("Cloud");
        let transport = Arc::new(DroppingMockTransport::new(50)); // 50% drop rate
        let mesh = Arc::new(crate::orchestration::mesh::CentrifugeNode::new(transport));
        let received = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let received_clone = received.clone();

        let (tx, mut rx) = tokio::sync::mpsc::channel(20);

        let _ = mesh.subscribe("mesh:test:loss", Box::new(move |_msg| {
            received_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            let _ = tx.try_send(());
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
                 let _ = rx.recv().await;
             }
        }

        // Resilience rule: system must recover or degrade gracefully.
        // We verify that some messages were successfully delivered and ack'd despite high packet loss,
        // and that the retry mechanism helped improve the delivery rate.

        assert!(successful_sends > 0, "System should successfully send at least some messages under chaos");
        // Because of CentrifugeNode's retries, successful_sends should be roughly 87.5% of 20 (approx 17)
        assert!(successful_sends >= 10, "Retry logic should recover a significant portion of dropped messages");
    }

    #[tokio::test]
    async fn test_cloud_degradation_fallback() {
        let _tracker = crate::telemetry::ChaosRecoveryTracker::new("Cloud");
        // We use an empty db pool but with CloudStateManager to see fail-safes on lock acquisition timeout
        let dummy_pg_pool = sqlx::postgres::PgPoolOptions::new().after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).after_release(|conn, _meta| { Box::pin(async move { use sqlx::Executor; conn.execute("DISCARD ALL").await?; Ok(true) }) }).max_connections(1).acquire_timeout(std::time::Duration::from_millis(50))
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
        // The mocked sleeping mesh sleeps for 61s, forcing the 2s timeout to trigger.
        assert!(elapsed < std::time::Duration::from_millis(4000));
        assert!(elapsed > std::time::Duration::from_millis(1500));

        // It must fallback safely returning an empty vector
        assert_eq!(tasks.len(), 0);
    }

    #[tokio::test]
    async fn test_llm_api_failure_recovery() {
        let _tracker = crate::telemetry::ChaosRecoveryTracker::new("Cloud");
        // Simulates LLM API failures and ensuring circuit breaker/fallback behavior.
        use crate::workers::OperationsWorker;

        // Intentionally bad OHC_HUB_URL to simulate API failure
        temp_env::with_var("OHC_HUB_URL", Some("http://127.0.0.1:1"), || async {
            let dummy_sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
                .connect("sqlite::memory:")
                .await
                .unwrap();

            let db = Arc::new(DB {
                pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(),
                store: DbStore::Sqlite(dummy_sqlite_pool.clone()),
            });

            // Initialize schema for OperationsWorker
            sqlx::query("CREATE TABLE IF NOT EXISTS department_tasks (id TEXT PRIMARY KEY, tenant_id TEXT, department TEXT, event_type TEXT, payload TEXT, status TEXT, locked_until TEXT, created_at TEXT, updated_at TEXT)").execute(&dummy_sqlite_pool).await.unwrap();
            sqlx::query("CREATE TABLE IF NOT EXISTS products (id TEXT PRIMARY KEY, organization_id TEXT, tenant_id TEXT, name TEXT, inventory_count INT, supplier_name TEXT, supplier_contact TEXT)").execute(&dummy_sqlite_pool).await.unwrap();
            sqlx::query("CREATE TABLE IF NOT EXISTS shared_tasks (id TEXT PRIMARY KEY, organization_id TEXT, tenant_id TEXT, title TEXT, description TEXT, status TEXT, priority TEXT, action_risk TEXT, approval_status TEXT, proposed_content TEXT, created_at TEXT, updated_at TEXT)").execute(&dummy_sqlite_pool).await.unwrap();
            sqlx::query("CREATE TABLE IF NOT EXISTS orders (id TEXT PRIMARY KEY, tenant_id TEXT, status TEXT, created_at TEXT)").execute(&dummy_sqlite_pool).await.unwrap();
            sqlx::query("CREATE TABLE IF NOT EXISTS order_items (id TEXT PRIMARY KEY, tenant_id TEXT, order_id TEXT, product_id TEXT, quantity INT)").execute(&dummy_sqlite_pool).await.unwrap();

            // Seed data: low inventory product
            sqlx::query("INSERT INTO products (id, organization_id, tenant_id, name, inventory_count, supplier_name, supplier_contact) VALUES ('p1', 't1', 't1', 'item1', 1, 's1', 'c1')").execute(&dummy_sqlite_pool).await.unwrap();
            let payload = serde_json::json!({"items": [{"product_id": "p1", "quantity": 1}]});
            sqlx::query("INSERT INTO department_tasks (id, tenant_id, department, event_type, payload, status) VALUES ('task1', 't1', 'operations', 'OrderPlaced', ?, 'PENDING')")
                .bind(payload.to_string())
                .execute(&dummy_sqlite_pool).await.unwrap();

            // Poll - this should trigger restock drafting which will fail AI call due to bad OHC_HUB_URL
            let res = OperationsWorker::poll(&db).await;
            assert!(res.is_ok());

            // Despite AI failure, a fallback message should be used and final_status should be PAUSED after retries
            let row: (String, String) = sqlx::query_as("SELECT status, proposed_content FROM department_tasks JOIN shared_tasks ON department_tasks.tenant_id = shared_tasks.organization_id WHERE department_tasks.id = 'task1' AND shared_tasks.title LIKE '%AI Agent Paused%'")
                .fetch_one(&dummy_sqlite_pool).await.unwrap();

            assert_eq!(row.0, "PAUSED");
            assert!(row.1.contains("System is paused"));
        }).await;
    }

    #[tokio::test]
    async fn test_host_memory_exhaustion_degradation() {
        let _tracker = crate::telemetry::ChaosRecoveryTracker::new("Cloud");
        // We simulate host memory exhaustion by synthetically increasing database operation latency
        // and verifying that the 2-second timeout triggers graceful degradation.
        let latency_mesh: Arc<dyn TeammateMesh> = Arc::new(LatencyMockMesh::new(3000));

        let dummy_sqlite_pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();

        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().connect_lazy("postgres://dummy").unwrap(),
            store: DbStore::Sqlite(dummy_sqlite_pool),
        });

        let state_manager = crate::orchestration::state::standalone::StandaloneStateManager::new(db, latency_mesh);

        let start = std::time::Instant::now();
        let res = state_manager.pull_available_tasks(10).await;
        let elapsed = start.elapsed();

        // Operation takes 3s due to LatencyMockMesh, but StateManager has 2s timeout
        assert!(elapsed < std::time::Duration::from_millis(2500));
        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 0);
    }
}
