use async_trait::async_trait;
use std::time::Duration;
use uuid::Uuid;
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use crate::ohc::orchestration::{MeshEvent, TeammateMeshEvent};
use prost::Message;

/// Protocol for the Teammate Mesh Communication Layer.
/// Works in both Cloud mode (Redis Pub/Sub) and Standalone mode (local IPC).
#[async_trait]
pub trait HybridTransport: Send + Sync {
    async fn publish(&self, channel: &str, message: &TeammateMeshEvent) -> Result<(), String>;
    async fn subscribe(&self, channel: &str) -> Result<tokio::sync::broadcast::Receiver<TeammateMeshEvent>, String>;
}

/// Abstract distributed locking scheme.
/// Prevents conflicts when multiple parts of the swarm access the same tenant resource simultaneously.
#[async_trait]
pub trait HybridLock: Send + Sync {
    async fn acquire(&self, key: &str, timeout: Duration, expiration: Duration) -> Result<String, String>;
    async fn release(&self, key: &str, lock_id: &str) -> Result<(), String>;
}

/// Handoff protocol for when a business owner switches between Cloud and Standalone environments.
#[async_trait]
pub trait StateHandoff: Send + Sync {
    async fn sync_state(&self, tenant_id: &str, payload: &[u8]) -> Result<(), String>;
    async fn resolve_conflict(&self, local_version: i64, remote_version: i64) -> Result<String, String>;
}

// ---------------------------------------------------------
// STANDALONE MODE IMPLEMENTATIONS
// ---------------------------------------------------------

/// Local memory-based transport for Standalone mode.
pub struct StandaloneTransport {
    subs: Arc<Mutex<HashMap<String, tokio::sync::broadcast::Sender<TeammateMeshEvent>>>>,
}

impl StandaloneTransport {
    pub fn new() -> Self {
        StandaloneTransport {
            subs: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl HybridTransport for StandaloneTransport {
    async fn publish(&self, channel: &str, message: &TeammateMeshEvent) -> Result<(), String> {
        let mut subs = self.subs.lock().await;
        if let Some(tx) = subs.get(channel) {
            let _ = tx.send(message.clone());
        } else {
            let (tx, _) = tokio::sync::broadcast::channel(100);
            let _ = tx.send(message.clone());
            subs.insert(channel.to_string(), tx);
        }
        Ok(())
    }

    async fn subscribe(&self, channel: &str) -> Result<tokio::sync::broadcast::Receiver<TeammateMeshEvent>, String> {
        let mut subs = self.subs.lock().await;
        let tx = subs.entry(channel.to_string()).or_insert_with(|| {
            let (tx, _) = tokio::sync::broadcast::channel(100);
            tx
        });
        Ok(tx.subscribe())
    }
}

/// Local memory-based lock for Standalone mode.
pub struct StandaloneLock {
    locks: Arc<Mutex<HashMap<String, (String, std::time::Instant)>>>,
}

impl StandaloneLock {
    pub fn new() -> Self {
        StandaloneLock {
            locks: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl HybridLock for StandaloneLock {
    async fn acquire(&self, key: &str, timeout: Duration, expiration: Duration) -> Result<String, String> {
        let start = std::time::Instant::now();
        loop {
            if start.elapsed() > timeout {
                return Err("timeout acquiring lock".to_string());
            }

            let mut locks = self.locks.lock().await;

            let mut can_acquire = false;
            if let Some((_, expires_at)) = locks.get(key) {
                if std::time::Instant::now() > *expires_at {
                    can_acquire = true;
                }
            } else {
                can_acquire = true;
            }

            if can_acquire {
                let lock_id = Uuid::new_v4().to_string();
                locks.insert(key.to_string(), (lock_id.clone(), std::time::Instant::now() + expiration));
                return Ok(lock_id);
            }
            drop(locks);

            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }

    async fn release(&self, key: &str, lock_id: &str) -> Result<(), String> {
        let mut locks = self.locks.lock().await;
        if let Some((existing_lock_id, expires_at)) = locks.get(key) {
            if existing_lock_id == lock_id {
                locks.remove(key);
                return Ok(());
            }
            if std::time::Instant::now() > *expires_at {
               locks.remove(key);
               return Err("failed to release lock: lock expired".to_string());
            }
            return Err("failed to release lock: not owner".to_string());
        }
        Err("failed to release lock: lock not found".to_string())
    }
}

/// Standalone state handoff.
pub struct StandaloneHandoff;

impl StandaloneHandoff {
    pub fn new() -> Self {
        StandaloneHandoff
    }
}

#[async_trait]
impl StateHandoff for StandaloneHandoff {
    async fn sync_state(&self, _tenant_id: &str, _payload: &[u8]) -> Result<(), String> {
        // In a real implementation, this would sync with PowerSync/SQLite
        Ok(())
    }

    async fn resolve_conflict(&self, local_version: i64, remote_version: i64) -> Result<String, String> {
        if local_version > remote_version {
            Ok("local".to_string())
        } else if remote_version > local_version {
            Ok("remote".to_string())
        } else {
            Ok("tie_broken_by_remote".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_standalone_transport() {
        let transport = StandaloneTransport::new();
        let mut rx = transport.subscribe("test_channel").await.unwrap();

        let event = TeammateMeshEvent {
            agent_id: "agent_1".to_string(),
            action: "test_action".to_string(),
            status: "test_status".to_string(),
            payload: b"hello mesh".to_vec(),
        };
        transport.publish("test_channel", &event).await.unwrap();

        let msg = rx.recv().await.unwrap();
        assert_eq!(msg.payload, b"hello mesh");
    }

    #[tokio::test]
    async fn test_standalone_lock() {
        let lock = StandaloneLock::new();
        let lock_id = lock.acquire("resource_1", Duration::from_secs(1), Duration::from_secs(10)).await.unwrap();

        // Trying to acquire the same lock should timeout
        let res = lock.acquire("resource_1", Duration::from_millis(100), Duration::from_secs(10)).await;
        assert!(res.is_err());

        // Release the lock
        lock.release("resource_1", &lock_id).await.unwrap();

        // Testing expiration
        let lock_id3 = lock.acquire("resource_2", Duration::from_secs(1), Duration::from_millis(10)).await.unwrap();
        tokio::time::sleep(Duration::from_millis(50)).await;
        let lock_id4 = lock.acquire("resource_2", Duration::from_secs(1), Duration::from_secs(10)).await.unwrap();
        assert!(lock_id3 != lock_id4);


        // Now we can acquire it again
        let lock_id2 = lock.acquire("resource_1", Duration::from_secs(1), Duration::from_secs(10)).await.unwrap();
        assert!(lock_id != lock_id2);
    }

    #[tokio::test]
    async fn test_standalone_lock_release_invalid() {
        let lock = StandaloneLock::new();
        let _lock_id = lock.acquire("resource_1", Duration::from_secs(1), Duration::from_secs(10)).await.unwrap();

        let res = lock.release("resource_1", "wrong_id").await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_standalone_handoff() {
        let handoff = StandaloneHandoff::new();

        handoff.sync_state("tenant_123", b"data").await.unwrap();

        assert_eq!(handoff.resolve_conflict(2, 1).await.unwrap(), "local");
        assert_eq!(handoff.resolve_conflict(1, 2).await.unwrap(), "remote");
        assert_eq!(handoff.resolve_conflict(1, 1).await.unwrap(), "tie_broken_by_remote");
    }
}


// ---------------------------------------------------------
// CLOUD MODE IMPLEMENTATIONS
// ---------------------------------------------------------

use redis::AsyncCommands;

/// Redis Pub/Sub transport for Cloud mode.
pub struct RedisTransport {
    client: redis::Client,
    pubsub_conn: tokio::sync::Mutex<Option<redis::aio::PubSub>>,
}

impl RedisTransport {
    pub fn new(client: redis::Client) -> Self {
        RedisTransport {
            client,
            pubsub_conn: tokio::sync::Mutex::new(None),
        }
    }
}

#[async_trait]
impl HybridTransport for RedisTransport {
    async fn publish(&self, channel: &str, message: &TeammateMeshEvent) -> Result<(), String> {
        let mut con = self.client.get_multiplexed_async_connection().await.map_err(|e| e.to_string())?;

        let mut buf = Vec::new();
        message.encode(&mut buf).map_err(|e| e.to_string())?;

        con.publish::<_, _, ()>(channel, buf).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn subscribe(&self, channel: &str) -> Result<tokio::sync::broadcast::Receiver<TeammateMeshEvent>, String> {
        // Implementation detail for subscribing to Redis pub/sub and forwarding to broadcast channel
        let (tx, rx) = tokio::sync::broadcast::channel(100);

        let client_clone = self.client.clone();
        let channel_clone = channel.to_string();

        tokio::spawn(async move {
            let mut pubsub = match client_clone.get_async_pubsub().await {
                Ok(p) => p,
                Err(e) => {
                    tracing::error!("Failed to get async pubsub for {}: {}", channel_clone, e);
                    return;
                }
            };

            if let Err(e) = pubsub.subscribe(&channel_clone).await {
                tracing::error!("Failed to subscribe to {}: {}", channel_clone, e);
                return;
            }

            use tokio_stream::StreamExt;
            let mut stream = pubsub.on_message();

            while let Some(msg) = stream.next().await {
                if let Ok(payload) = msg.get_payload::<Vec<u8>>() {
                    if let Ok(event) = TeammateMeshEvent::decode(&payload[..]) {
                        let _ = tx.send(event);
                    }
                }
            }
        });

        Ok(rx)
    }
}

/// Redis Redlock for Cloud mode.
pub struct RedisLock {
    client: redis::Client,
}

impl RedisLock {
    pub fn new(client: redis::Client) -> Self {
        RedisLock {
            client,
        }
    }
}

#[async_trait]
impl HybridLock for RedisLock {
    async fn acquire(&self, key: &str, timeout: Duration, expiration: Duration) -> Result<String, String> {
        let mut con = self.client.get_async_connection().await.map_err(|e| e.to_string())?;
        let start = std::time::Instant::now();
        let lock_id = Uuid::new_v4().to_string();
        let lock_key = format!("ohc:lock:{}", key);

        loop {
            if start.elapsed() > timeout {
                return Err("timeout acquiring lock".to_string());
            }

            let res: Option<String> = redis::cmd("SET")
                .arg(&lock_key)
                .arg(&lock_id)
                .arg("NX")
                .arg("PX")
                .arg(expiration.as_millis() as u64)
                .query_async(&mut con)
                .await
                .map_err(|e| e.to_string())?;

            if res.is_some() {
                return Ok(lock_id);
            }

            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }

    async fn release(&self, key: &str, lock_id: &str) -> Result<(), String> {
        let mut con = self.client.get_async_connection().await.map_err(|e| e.to_string())?;
        let lock_key = format!("ohc:lock:{}", key);

        let script = redis::Script::new(r#"
            if redis.call("get", KEYS[1]) == ARGV[1] then
                return redis.call("del", KEYS[1])
            else
                return 0
            end
        "#);

        let res: i32 = script.key(&lock_key).arg(lock_id).invoke_async(&mut con).await.map_err(|e| e.to_string())?;

        if res == 1 {
            Ok(())
        } else {
            Err("failed to release lock: not owner or lock expired".to_string())
        }
    }
}

/// Cloud state handoff.
pub struct CloudHandoff {
    pool: sqlx::PgPool,
}

impl CloudHandoff {
    pub fn new(pool: sqlx::PgPool) -> Self {
        CloudHandoff {
            pool,
        }
    }
}

#[async_trait]
impl StateHandoff for CloudHandoff {
    async fn sync_state(&self, tenant_id: &str, payload: &[u8]) -> Result<(), String> {
        // Implement full Cloud handoff syncing logic here connecting with PostgreSQL
        let _payload_str = String::from_utf8_lossy(payload);

        // Example logic:
        let _query = "INSERT INTO state_handoff_log (tenant_id, payload, sync_status) VALUES ($1, $2, 'PENDING') ON CONFLICT DO NOTHING";

        Ok(())
    }

    async fn resolve_conflict(&self, local_version: i64, remote_version: i64) -> Result<String, String> {
        if local_version > remote_version {
            Ok("local".to_string())
        } else if remote_version > local_version {
            Ok("remote".to_string())
        } else {
            Ok("tie_broken_by_remote".to_string())
        }
    }
}
