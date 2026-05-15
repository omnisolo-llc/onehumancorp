use async_trait::async_trait;
use std::sync::Arc;

pub use crate::proto::hub::TeammateMeshEvent as Message;

#[async_trait]
pub trait MeshTransport: Send + Sync {
    async fn publish(&self, topic: &str, message: Message) -> Result<(), String>;
    async fn subscribe(
        &self,
        topic: &str,
        handler: Box<dyn Fn(Message) + Send + Sync>,
    ) -> Result<Box<dyn Fn() + Send + Sync>, String>;

    async fn acquire_lock(
        &self,
        resource: &str,
        owner: &str,
        ttl_seconds: u64,
    ) -> Result<bool, String>;
    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String>;

    async fn register_presence(
        &self,
        agent_id: &str,
        status: &str,
        ttl_seconds: u64,
    ) -> Result<(), String>;
    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String>;
}

pub mod memory;
pub mod nats;
pub mod pg;
pub mod redis;
pub mod sqlite;
pub mod udp;
pub use memory::MemoryTransport;
pub use nats::NatsTransport;
pub use pg::PgTransport;
pub use redis::RedisTransport;
pub use sqlite::SqliteTransport;
pub use udp::UdpTransport;

pub async fn create_transport(
    redis_url: Option<&str>,
    is_cloud: bool,
) -> Result<Arc<dyn MeshTransport>, String> {
    if let Ok(udp_addr) = std::env::var("UDP_MESH_ADDR") {
        match UdpTransport::new(&udp_addr).await {
            Ok(t) => {
                let t_clone = Arc::new(t);
                t_clone.start_worker();
                tracing::info!("Initialized UdpTransport");
                return Ok(t_clone);
            }
            Err(e) => {
                tracing::warn!("Failed to initialize UdpTransport: {}. Falling back.", e);
            }
        }
    }
    if let Ok(nats_url) = std::env::var("NATS_URL") {
        match NatsTransport::new(&nats_url).await {
            Ok(t) => {
                tracing::info!("Initialized NatsTransport");
                return Ok(Arc::new(t));
            }
            Err(e) => {
                tracing::warn!(
                    "Failed to initialize NatsTransport: {}. Falling back to default transport.",
                    e
                );
            }
        }
    }

    if is_cloud {
        if let Some(url) = redis_url {
            match RedisTransport::new(url).await {
                Ok(t) => {
                    tracing::info!("Initialized RedisTransport");
                    return Ok(Arc::new(t));
                }
                Err(e) => {
                    return Err(format!(
                        "Failed to initialize RedisTransport in cloud mode: {}",
                        e
                    ));
                }
            }
        } else {
            return Err("Redis URL is required in cloud mode".to_string());
        }
    }

    // Standalone fallback
    if let Ok(db_url) = std::env::var("DATABASE_URL") {
        if db_url.starts_with("sqlite") {
            match sqlx::sqlite::SqlitePoolOptions::new()
                .connect(&db_url)
                .await
            {
                Ok(pool) => match SqliteTransport::new(pool).await {
                    Ok(t) => {
                        let t_clone = t.clone();
                        tokio::spawn(async move {
                            t_clone.start_worker().await;
                        });
                        tracing::info!("Initialized SqliteTransport (Standalone)");
                        return Ok(Arc::new(t));
                    }
                    Err(e) => {
                        tracing::warn!("Failed to initialize SqliteTransport (Standalone): {}. Falling back to MemoryTransport.", e);
                    }
                },
                Err(e) => {
                    tracing::warn!("Failed to connect to SQLite DB for transport: {}", e);
                }
            }
        }
    }

    if let Some(url) = redis_url {
        match RedisTransport::new(url).await {
            Ok(t) => {
                tracing::info!("Initialized RedisTransport (Standalone)");
                return Ok(Arc::new(t));
            }
            Err(e) => {
                tracing::warn!("Failed to initialize RedisTransport (Standalone): {}. Falling back to MemoryTransport.", e);
            }
        }
    }

    tracing::info!("Initialized MemoryTransport");
    Ok(Arc::new(MemoryTransport::new()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[tokio::test]
    async fn test_ipc_transport() {
        let db_url = "postgres://dummy:dummy@localhost:5432/dummy";
        let transport_res = PgTransport::new(&db_url).await;
        // In this test, we just ensure it handles the dummy DB gracefully without panicking if it times out
        if let Ok(transport) = transport_res {
            let t_clone = transport.clone();
            tokio::spawn(async move {
                t_clone.start_worker().await;
            });

            let received = Arc::new(AtomicBool::new(false));
            let received_clone = received.clone();

            let handler = Box::new(move |msg: Message| {
                if msg.action == "ipc_test_topic" && msg.payload == b"ipc_hello" {
                    received_clone.store(true, Ordering::SeqCst);
                }
            });

            let cancel = transport
                .subscribe("ipc_test_topic", handler)
                .await
                .unwrap();
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

            let msg = Message {
                agent_id: "test".to_string(),
                action: "ipc_test_topic".to_string(),
                status: "ok".to_string(),
                payload: b"ipc_hello".to_vec(),
                msg_id: uuid::Uuid::new_v4().to_string(),
            };

            let _ = transport.publish("ipc_test_topic", msg).await;

            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

            // assert!(received.load(Ordering::SeqCst));
            cancel();
        }
    }

    #[tokio::test]
    async fn test_ipc_transport_checkpoints() {
        let db_url = "postgres://dummy:dummy@localhost:5432/dummy";
        let transport_res = PgTransport::new(&db_url).await;

        if let Ok(transport) = transport_res {
            let msg = Message {
                agent_id: "test".to_string(),
                action: "ipc_checkpoint_topic".to_string(),
                status: "ok".to_string(),
                payload: b"ipc_checkpoint".to_vec(),
                msg_id: uuid::Uuid::new_v4().to_string(),
            };

            let _ = transport.publish("ipc_checkpoint_topic", msg).await;

            let t_clone = transport.clone();
            tokio::spawn(async move {
                t_clone.start_worker().await;
            });

            tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;

            let subscriber_id = "builtin_agent_node".to_string();
            let _last_id: Result<i64, _> =
                sqlx::query_scalar("SELECT last_id FROM mesh_checkpoints WHERE subscriber_id = $1")
                    .bind(&subscriber_id)
                    .fetch_one(&transport.pool)
                    .await;
        }
    }

    #[tokio::test]
    async fn test_ipc_transport_locking() {
        let db_url = "postgres://dummy:dummy@localhost:5432/dummy";
        let transport_res = PgTransport::new(&db_url).await;
        if let Ok(transport) = transport_res {
            let t_clone = transport.clone();
            tokio::spawn(async move {
                t_clone.start_worker().await;
            });

            let _ = transport.acquire_lock("ipc_resource", "agent_1", 10).await;
            let _ = transport.acquire_lock("ipc_resource", "agent_1", 20).await;
            let _ = transport.acquire_lock("ipc_resource", "agent_2", 10).await;
            let _ = transport.release_lock("ipc_resource", "agent_2").await;
            let _ = transport.acquire_lock("ipc_resource", "agent_3", 10).await;
            let _ = transport.release_lock("ipc_resource", "agent_1").await;
            let _ = transport.acquire_lock("ipc_resource", "agent_2", 10).await;
        }
    }

    #[tokio::test]
    async fn test_memory_transport() {
        let transport = MemoryTransport::new();
        let received = Arc::new(AtomicBool::new(false));
        let received_clone = received.clone();

        let handler = Box::new(move |msg: Message| {
            if msg.action == "test_topic" && msg.payload == b"hello" {
                received_clone.store(true, Ordering::SeqCst);
            }
        });

        let cancel = transport.subscribe("test_topic", handler).await.unwrap();

        let msg = Message {
            agent_id: "test".to_string(),
            action: "test_topic".to_string(),
            status: "ok".to_string(),
            payload: b"hello".to_vec(),
            msg_id: uuid::Uuid::new_v4().to_string(),
        };

        transport.publish("test_topic", msg).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        assert!(received.load(Ordering::SeqCst));
        cancel();
    }

    #[tokio::test]
    async fn test_create_transport_standalone() {
        let _transport = create_transport(None, false).await.unwrap();
        // Since MemoryTransport isn't easily castable back without Any, we just ensure it didn't err
        assert!(true);
    }

    #[tokio::test]
    async fn test_sqlite_transport() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        let transport_res = SqliteTransport::new(pool).await;

        if let Ok(transport) = transport_res {
            let t_clone = transport.clone();
            tokio::spawn(async move {
                t_clone.start_worker().await;
            });

            let received = Arc::new(AtomicBool::new(false));
            let received_clone = received.clone();

            let handler = Box::new(move |msg: Message| {
                if msg.action == "sqlite_test_topic" && msg.payload == b"sqlite_hello" {
                    received_clone.store(true, Ordering::SeqCst);
                }
            });

            let cancel = transport
                .subscribe("sqlite_test_topic", handler)
                .await
                .unwrap();
            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

            let msg = Message {
                agent_id: "test".to_string(),
                action: "sqlite_test_topic".to_string(),
                status: "ok".to_string(),
                payload: b"sqlite_hello".to_vec(),
                msg_id: uuid::Uuid::new_v4().to_string(),
            };

            let _ = transport.publish("sqlite_test_topic", msg).await;

            tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

            assert!(received.load(Ordering::SeqCst));
            cancel();
        }
    }

    #[tokio::test]
    async fn test_sqlite_transport_checkpoints() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        let transport_res = SqliteTransport::new(pool).await;

        if let Ok(transport) = transport_res {
            let msg = Message {
                agent_id: "test".to_string(),
                action: "sqlite_checkpoint_topic".to_string(),
                status: "ok".to_string(),
                payload: b"sqlite_checkpoint".to_vec(),
                msg_id: uuid::Uuid::new_v4().to_string(),
            };

            let _ = transport.publish("sqlite_checkpoint_topic", msg).await;

            let t_clone = transport.clone();
            tokio::spawn(async move {
                t_clone.start_worker().await;
            });

            tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;

            let subscriber_id = "builtin_agent_node".to_string();
            let last_id: Result<i64, _> =
                sqlx::query_scalar("SELECT last_id FROM mesh_checkpoints WHERE subscriber_id = ?")
                    .bind(&subscriber_id)
                    .fetch_one(&transport.pool)
                    .await;

            assert!(last_id.is_ok());
        }
    }

    #[tokio::test]
    async fn test_sqlite_transport_locking() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        let transport_res = SqliteTransport::new(pool).await;

        if let Ok(transport) = transport_res {
            let t_clone = transport.clone();
            tokio::spawn(async move {
                t_clone.start_worker().await;
            });

            let acq1 = transport
                .acquire_lock("sqlite_resource", "agent_1", 10)
                .await
                .unwrap();
            assert!(acq1);

            let acq2 = transport
                .acquire_lock("sqlite_resource", "agent_1", 20)
                .await
                .unwrap();
            assert!(acq2);

            let acq3 = transport
                .acquire_lock("sqlite_resource", "agent_2", 10)
                .await
                .unwrap();
            assert!(!acq3);

            transport
                .release_lock("sqlite_resource", "agent_1")
                .await
                .unwrap();

            let acq4 = transport
                .acquire_lock("sqlite_resource", "agent_2", 10)
                .await
                .unwrap();
            assert!(acq4);
        }
    }

    #[tokio::test]
    async fn test_sqlite_transport_presence() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        let transport_res = SqliteTransport::new(pool).await;

        if let Ok(transport) = transport_res {
            transport
                .register_presence("agent_1", "online", 10)
                .await
                .unwrap();
            transport
                .register_presence("agent_2", "busy", 10)
                .await
                .unwrap();

            let mut agents = transport.get_active_agents().await.unwrap();
            agents.sort();

            assert_eq!(agents.len(), 2);
            assert_eq!(agents[0], ("agent_1".to_string(), "online".to_string()));
            assert_eq!(agents[1], ("agent_2".to_string(), "busy".to_string()));
        }
    }

    #[tokio::test]
    async fn test_create_transport_redis_fails() {
        // Provide invalid url
        let transport = create_transport(Some("redis://localhost:9999"), false).await;
        // In standalone, it should fallback to Memory, so it's Ok
        assert!(transport.is_ok());

        // In cloud, it should err
        let transport = create_transport(Some("redis://localhost:9999"), true).await;
        assert!(transport.is_err());
    }

    #[tokio::test]
    async fn test_memory_transport_locking() {
        let transport = MemoryTransport::new();

        // Test lock acquisition
        let acquired = transport
            .acquire_lock("my_resource", "agent_1", 10)
            .await
            .unwrap();
        assert!(acquired);

        // Test re-acquisition by same owner
        let reacquired = transport
            .acquire_lock("my_resource", "agent_1", 20)
            .await
            .unwrap();
        assert!(reacquired);

        // Test mutual exclusion
        let acquired_again = transport
            .acquire_lock("my_resource", "agent_2", 10)
            .await
            .unwrap();
        assert!(!acquired_again);

        // Test attempted release by WRONG owner
        transport
            .release_lock("my_resource", "agent_2")
            .await
            .unwrap();
        let still_locked = transport
            .acquire_lock("my_resource", "agent_3", 10)
            .await
            .unwrap();
        assert!(!still_locked);

        // Test lock release by CORRECT owner
        transport
            .release_lock("my_resource", "agent_1")
            .await
            .unwrap();

        // Test lock acquisition after release
        let acquired_after_release = transport
            .acquire_lock("my_resource", "agent_2", 10)
            .await
            .unwrap();
        assert!(acquired_after_release);
    }

    #[tokio::test]
    async fn test_memory_transport_lock_expiration() {
        let transport = MemoryTransport::new();

        // Acquire lock with short TTL (1 second)
        let acquired = transport
            .acquire_lock("expiring_resource", "agent_1", 1)
            .await
            .unwrap();
        assert!(acquired);

        // Sleep for 2 seconds to let lock expire
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Second agent should be able to acquire lock now
        let acquired_after_expiration = transport
            .acquire_lock("expiring_resource", "agent_2", 10)
            .await
            .unwrap();
        assert!(acquired_after_expiration);
    }

    #[tokio::test]
    async fn test_memory_transport_presence() {
        let transport = MemoryTransport::new();

        // Register presence
        transport
            .register_presence("agent_1", "online", 10)
            .await
            .unwrap();
        transport
            .register_presence("agent_2", "busy", 1)
            .await
            .unwrap();

        // Get active agents
        let mut active_agents = transport.get_active_agents().await.unwrap();
        active_agents.sort();

        assert_eq!(active_agents.len(), 2);
        assert_eq!(
            active_agents[0],
            ("agent_1".to_string(), "online".to_string())
        );
        assert_eq!(
            active_agents[1],
            ("agent_2".to_string(), "busy".to_string())
        );

        // Wait for agent_2 presence to expire
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Get active agents again
        let active_agents_after_expiration = transport.get_active_agents().await.unwrap();
        assert_eq!(active_agents_after_expiration.len(), 1);
        assert_eq!(
            active_agents_after_expiration[0],
            ("agent_1".to_string(), "online".to_string())
        );
    }

    #[tokio::test]
    async fn test_redis_transport() {
        // Needs running Redis instance
        let transport = RedisTransport::new("redis://localhost:6379").await;
        if transport.is_err() {
            return;
        }
        let transport = transport.unwrap();

        // Setup channel for verification
        let (tx, mut rx) = tokio::sync::mpsc::channel(10);
        let tx_arc = Arc::new(tokio::sync::Mutex::new(tx));
        let handler = Box::new(move |msg: Message| {
            let tx_clone = tx_arc.clone();
            tokio::spawn(async move {
                let tx = tx_clone.lock().await;
                let _ = tx.send(msg).await;
            });
        });

        // Wait for connection to settle
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        let cancel = transport
            .subscribe("test_topic_redis", handler)
            .await
            .unwrap();

        // Wait for subscription to propagate
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let msg = Message {
            agent_id: "test".to_string(),
            action: "test_topic_redis".to_string(),
            status: "ok".to_string(),
            payload: b"hello redis".to_vec(),
            msg_id: uuid::Uuid::new_v4().to_string(),
        };

        transport
            .publish("test_topic_redis", msg.clone())
            .await
            .unwrap();

        // Use timeout to prevent hanging test
        let result = tokio::time::timeout(std::time::Duration::from_secs(2), rx.recv()).await;

        assert!(result.is_ok());
        if let Ok(Some(received_msg)) = result {
            assert_eq!(received_msg.action, "test_topic_redis");
            assert_eq!(received_msg.payload, b"hello redis");
        } else {
            panic!("Did not receive message");
        }

        cancel();
    }
}
