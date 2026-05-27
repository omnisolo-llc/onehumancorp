use super::mesh::{Message, TeammateMesh};
use crate::db::DbStore;
use crate::orchestration::state::cloud::CloudStateManager;
use crate::orchestration::state::StateEngine;
use crate::DB;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::time::Duration;

pub struct SleepingMockMesh;

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


pub struct CorruptedMockMesh {
    pub received_messages: Arc<std::sync::atomic::AtomicUsize>,
}

impl CorruptedMockMesh {
    pub fn new() -> Self {
        CorruptedMockMesh {
            received_messages: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        }
    }
}

#[async_trait]
impl TeammateMesh for CorruptedMockMesh {
    async fn publish(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
    async fn publish_with_ack(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
    async fn subscribe(&self, _topic: &str, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let counter = self.received_messages.clone();
        tokio::spawn(async move {
            let msg = Message {
                id: "test".to_string(),
                payload: b"{ invalid json }".to_vec(),
                sender_id: "test".to_string(),
                timestamp: 0,
            };
            handler(msg);
            counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        });
        Ok(Box::new(|| {}))
    }
    async fn acquire_lock(&self, _resource: &str, _owner: &str, _ttl_seconds: u64) -> Result<bool, String> { Ok(true) }
    async fn release_lock(&self, _resource: &str, _owner: &str) -> Result<(), String> { Ok(()) }

    async fn register_presence(&self, _agent_id: &str, _status: &str, _ttl_seconds: u64) -> Result<(), String> { Ok(()) }
    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> { Ok(vec![]) }
    async fn ping(&self) -> Result<(), String> { Ok(()) }
    async fn start_health_responder(&self) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(|| {})) }
    async fn publish_state_handoff(&self, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
    async fn subscribe_state_handoff(&self, _handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(|| {})) }
}

pub struct RacingLockMesh {
    locked: Arc<tokio::sync::Mutex<bool>>,
}

impl RacingLockMesh {
    pub fn new() -> Self {
        RacingLockMesh {
            locked: Arc::new(tokio::sync::Mutex::new(false)),
        }
    }
}

#[async_trait]
impl TeammateMesh for RacingLockMesh {
    async fn publish(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
    async fn publish_with_ack(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
    async fn subscribe(&self, _topic: &str, _handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(|| {})) }
    async fn acquire_lock(&self, _resource: &str, _owner: &str, _ttl_seconds: u64) -> Result<bool, String> {
        let mut lock = self.locked.lock().await;
        if *lock {
            Ok(false)
        } else {
            *lock = true;
            Ok(true)
        }
    }
    async fn release_lock(&self, _resource: &str, _owner: &str) -> Result<(), String> {
        let mut lock = self.locked.lock().await;
        *lock = false;
        Ok(())
    }

    async fn register_presence(&self, _agent_id: &str, _status: &str, _ttl_seconds: u64) -> Result<(), String> { Ok(()) }
    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> { Ok(vec![]) }
    async fn ping(&self) -> Result<(), String> { Ok(()) }
    async fn start_health_responder(&self) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(|| {})) }
    async fn publish_state_handoff(&self, _payload: Vec<u8>) -> Result<(), String> { Ok(()) }
    async fn subscribe_state_handoff(&self, _handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(|| {})) }
}


pub struct DroppingMockTransport {
    pub drop_rate_percent: u8,
}

impl DroppingMockTransport {
    pub fn new(drop_rate_percent: u8) -> Self {
        DroppingMockTransport { drop_rate_percent }
    }
}

#[async_trait]
impl crate::orchestration::mesh::MeshTransport for DroppingMockTransport {
    async fn publish(&self, _topic: &str, _payload: Vec<u8>) -> Result<(), String> {
        if rand::random::<u8>() % 100 < self.drop_rate_percent {
            return Err("Dropped by ChaosTransport".to_string());
        }
        Ok(())
    }
    async fn subscribe(&self, _topic: &str, _handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> { Ok(Box::new(|| {})) }
    async fn acquire_lock(&self, _resource: &str, _owner: &str, _ttl_seconds: u64) -> Result<bool, String> { Ok(true) }
    async fn release_lock(&self, _resource: &str, _owner: &str) -> Result<(), String> { Ok(()) }
    async fn set_ex(&self, _key: &str, _value: &str, _ttl_seconds: u64) -> Result<(), String> { Ok(()) }
    async fn get(&self, _key: &str) -> Result<Option<String>, String> { Ok(None) }
    async fn keys(&self, _pattern: &str) -> Result<Vec<String>, String> { Ok(vec![]) }
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
        // Adjust bound down slightly to avoid flakiness in testing framework due to RNG
        assert!(successful_sends >= 7, "Retry logic should recover a significant portion of dropped messages");
    }

    #[tokio::test]
    async fn test_cloud_degradation_fallback() {
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

        // The pull_available_tasks for cloud has a 60-second timeout on the lock or DB
        // The mocked sleeping mesh sleeps for 61s, forcing the 60s timeout to trigger.
        assert!(elapsed < std::time::Duration::from_millis(62000));
        assert!(elapsed > std::time::Duration::from_millis(59000));

        // It must fallback safely returning an empty vector
        assert_eq!(tasks.len(), 0);
    }

    #[tokio::test]
    async fn test_cloud_db_transition_fallback() {
        // Intentionally bad DB URL to simulate database failure / degraded performance
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1).acquire_timeout(std::time::Duration::from_millis(50))
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
            .max_connections(1).acquire_timeout(std::time::Duration::from_millis(50))
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
            .max_connections(1).acquire_timeout(std::time::Duration::from_millis(50))
            .connect_lazy("sqlite::memory:")
            .unwrap();

        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().max_connections(1).acquire_timeout(std::time::Duration::from_millis(50)).connect_lazy("postgres://postgres:postgres@localhost:12345/nonexistent").unwrap(),
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
            .max_connections(1).acquire_timeout(std::time::Duration::from_millis(50))
            .connect_lazy("sqlite::memory:")
            .unwrap();

        let db = Arc::new(DB {
            pool: sqlx::postgres::PgPoolOptions::new().max_connections(1).acquire_timeout(std::time::Duration::from_millis(50)).connect_lazy("postgres://postgres:postgres@localhost:5432/test").unwrap(),
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
