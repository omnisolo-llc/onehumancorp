use tokio::sync::broadcast;
#[allow(unused_imports)]
use std::sync::Arc;
use tokio::sync::Mutex;
use async_trait::async_trait;

#[derive(Clone, prost::Message)]
#[allow(dead_code)]
pub struct Message {
    #[prost(string, tag = "1")]
    pub topic: String,
    #[prost(bytes, tag = "2")]
    pub payload: Vec<u8>,
}

#[async_trait]
pub trait DistributedLock: Send + Sync {
    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String>;
    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String>;
}

#[async_trait]
#[allow(dead_code)]
pub trait Bus: Send + Sync {
    async fn publish(&self, msg: Message) -> Result<(), String>;
    async fn subscribe(&self, topic: String, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String>;
}

#[allow(dead_code)]
pub struct MemoryBus {
    subs: Mutex<std::collections::HashMap<String, broadcast::Sender<Message>>>,
    locks: Mutex<std::collections::HashMap<String, (String, std::time::Instant)>>,
}

#[allow(dead_code)]
impl MemoryBus {
    pub fn new() -> Self {
        MemoryBus {
            subs: Mutex::new(std::collections::HashMap::new()),
            locks: Mutex::new(std::collections::HashMap::new()),
        }
    }
}

#[async_trait]
impl Bus for MemoryBus {
    async fn publish(&self, msg: Message) -> Result<(), String> {
        let subs = self.subs.lock().await;
        if let Some(tx) = subs.get(&msg.topic) {
            let _ = tx.send(msg);
        }
        Ok(())
    }

    async fn subscribe(&self, topic: String, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let mut subs = self.subs.lock().await;
        let tx = subs.entry(topic.clone()).or_insert_with(|| {
            let (tx, _) = broadcast::channel(100);
            tx
        });
        
        let mut rx = tx.subscribe();
        
        let worker = tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                handler(msg);
            }
        });
        
        let cancel = Box::new(move || {
            worker.abort();
        });
        
        Ok(cancel)
    }
}

impl Default for MemoryBus {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DistributedLock for MemoryBus {
    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String> {
        let mut locks = self.locks.lock().await;
        let now = std::time::Instant::now();

        // Remove expired locks
        locks.retain(|_, (_, expires_at)| *expires_at > now);

        let expires_at = now + std::time::Duration::from_secs(ttl_seconds);
        if let Some((current_owner, _)) = locks.get(resource) {
            if current_owner == owner {
                locks.insert(resource.to_string(), (owner.to_string(), expires_at));
                return Ok(true);
            }
            return Ok(false);
        }

        locks.insert(resource.to_string(), (owner.to_string(), expires_at));
        Ok(true)
    }

    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
        let mut locks = self.locks.lock().await;
        if let Some((current_owner, _)) = locks.get(resource) {
            if current_owner == owner {
                locks.remove(resource);
            }
        }
        Ok(())
    }
}

#[allow(dead_code)]
pub struct RedisBus {
    client: redis::Client,
    publish_conn: tokio::sync::Mutex<redis::aio::MultiplexedConnection>,
}

#[allow(dead_code)]
impl RedisBus {
    pub async fn new(redis_url: &str) -> Result<Self, String> {
        let client = redis::Client::open(redis_url).map_err(|e| e.to_string())?;
        let publish_conn = client.get_multiplexed_tokio_connection().await.map_err(|e| e.to_string())?;

        Ok(RedisBus {
            client,
            publish_conn: tokio::sync::Mutex::new(publish_conn),
        })
    }
}

#[async_trait]
impl Bus for RedisBus {
    async fn publish(&self, msg: Message) -> Result<(), String> {
        let mut conn = self.publish_conn.lock().await;
        use prost::Message as ProstMessage;
        let mut buf = Vec::new();
        msg.encode(&mut buf).unwrap();

        let mut retries = 0;
        loop {
            match redis::cmd("PUBLISH")
                .arg(&msg.topic)
                .arg(&buf)
                .query_async::<()>(&mut *conn)
                .await {
                    Ok(_) => return Ok(()),
                    Err(e) => {
                        if retries >= 3 {
                            return Err(e.to_string());
                        }
                        retries += 1;
                        tokio::time::sleep(tokio::time::Duration::from_millis(100 * retries)).await;
                    }
                }
        }
    }

    async fn subscribe(&self, topic: String, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        use futures_util::StreamExt;

        let mut pubsub = self.client.get_async_pubsub().await.map_err(|e| e.to_string())?;
        pubsub.subscribe(&topic).await.map_err(|e| e.to_string())?;
        let mut stream = pubsub.into_on_message();

        let worker = tokio::spawn(async move {
            while let Some(msg) = stream.next().await {
                if let Ok(buf) = msg.get_payload::<Vec<u8>>() {
                    use prost::Message as ProstMessage;
                    let m = Message::decode(&buf[..]).unwrap_or_else(|_| Message { topic: topic.clone(), payload: vec![] });
                    handler(m);
                }
            }
        });

        let cancel = Box::new(move || {
            worker.abort();
        });

        Ok(cancel)
    }
}

#[allow(dead_code)]
pub struct IpcBus {
    pool: sqlx::SqlitePool,
    subs: std::sync::Arc<tokio::sync::Mutex<std::collections::HashMap<String, tokio::sync::broadcast::Sender<Message>>>>,
}

#[allow(dead_code)]
impl IpcBus {
    pub async fn new(db_url: &str) -> Result<Self, String> {
        use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
        let options: SqliteConnectOptions = db_url.parse().map_err(|e| format!("Invalid db url: {}", e))?;
        let options = options.create_if_missing(true);
        let pool = SqlitePoolOptions::new().connect_with(options).await.map_err(|e| e.to_string())?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS bus_checkpoints (
                subscriber_id TEXT PRIMARY KEY,
                last_id INTEGER NOT NULL
            );"
        )
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS bus_messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                topic TEXT NOT NULL,
                payload BLOB NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );"
        )
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS bus_locks (
                resource TEXT PRIMARY KEY,
                owner TEXT NOT NULL,
                expires_at INTEGER NOT NULL
            );"
        )
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

        let subs = std::sync::Arc::new(tokio::sync::Mutex::new(std::collections::HashMap::new()));
        let bus = IpcBus {
            pool: pool.clone(),
            subs: subs.clone(),
        };

        bus.start_worker().await;

        let cleanup_pool = pool.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(3600)).await;
                let _ = sqlx::query("DELETE FROM bus_messages WHERE created_at < datetime('now', '-1 day')")
                    .execute(&cleanup_pool)
                    .await;
            }
        });

        Ok(bus)
    }

    pub async fn start_worker(&self) {
        let pool = self.pool.clone();
        let subs = self.subs.clone();

        tokio::spawn(async move {
            let subscriber_id = "standalone_node".to_string();
            let mut last_id: i64 = sqlx::query_scalar("SELECT last_id FROM bus_checkpoints WHERE subscriber_id = ?")
                .bind(&subscriber_id)
                .fetch_optional(&pool)
                .await
                .unwrap_or(Some(0))
                .unwrap_or(0);

            loop {
                let rows: Result<Vec<(i64, String, Vec<u8>)>, _> = sqlx::query_as(
                    "SELECT id, topic, payload FROM bus_messages WHERE id > ? ORDER BY id ASC"
                )
                .bind(last_id)
                .fetch_all(&pool)
                .await;

                if let Ok(results) = rows {
                    let s = subs.lock().await;
                    for (id, topic, payload_buf) in &results {
                        last_id = *id;
                        if let Some(tx) = s.get(topic) {
                            use prost::Message as ProstMessage;
                            let m = Message::decode(&payload_buf[..]).unwrap_or_else(|_| Message { topic: topic.clone(), payload: vec![] });
                            let _ = tx.send(m);
                        }
                    }
                    if !results.is_empty() {
                        let _ = sqlx::query("INSERT INTO bus_checkpoints (subscriber_id, last_id) VALUES (?, ?) ON CONFLICT(subscriber_id) DO UPDATE SET last_id = excluded.last_id")
                            .bind(&subscriber_id)
                            .bind(last_id)
                            .execute(&pool)
                            .await;
                    }
                }

                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            }
        });
    }
}

#[async_trait]
impl Bus for IpcBus {
    async fn publish(&self, msg: Message) -> Result<(), String> {
        use prost::Message as ProstMessage;
        let mut payload = Vec::new();
        msg.encode(&mut payload).unwrap();

        let mut retries = 0;
        loop {
            match sqlx::query("INSERT INTO bus_messages (topic, payload) VALUES (?, ?)")
                .bind(&msg.topic)
                .bind(&payload)
                .execute(&self.pool)
                .await {
                    Ok(_) => return Ok(()),
                    Err(e) => {
                        if retries >= 3 {
                            return Err(e.to_string());
                        }
                        retries += 1;
                        tokio::time::sleep(tokio::time::Duration::from_millis(100 * retries)).await;
                    }
                }
        }
    }

    async fn subscribe(&self, topic: String, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let mut s = self.subs.lock().await;
        let tx = s.entry(topic.clone()).or_insert_with(|| {
            let (tx, _) = tokio::sync::broadcast::channel(100);
            tx
        });

        let mut rx = tx.subscribe();

        let worker = tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                handler(msg);
            }
        });

        let cancel = Box::new(move || {
            worker.abort();
        });

        Ok(cancel)
    }
}

#[allow(dead_code)]
pub struct NatsBus {
    client: async_nats::Client,
    kv: async_nats::jetstream::kv::Store,
}

#[allow(dead_code)]
impl NatsBus {
    pub async fn new(nats_url: &str) -> Result<Self, String> {
        let client = async_nats::connect(nats_url).await.map_err(|e| e.to_string())?;
        let js = async_nats::jetstream::new(client.clone());
        let kv = match js.get_key_value("bus_locks").await {
            Ok(store) => store,
            Err(_) => js.create_key_value(async_nats::jetstream::kv::Config {
                bucket: "bus_locks".to_string(),
                history: 1,
                ..Default::default()
            }).await.map_err(|e| e.to_string())?
        };

        Ok(NatsBus {
            client,
            kv,
        })
    }
}

#[async_trait]
impl Bus for NatsBus {
    async fn publish(&self, msg: Message) -> Result<(), String> {
        use prost::Message as ProstMessage;
        let mut buf = Vec::new();
        msg.encode(&mut buf).unwrap();

        let mut retries = 0;
        loop {
            match self.client.publish(msg.topic.clone(), buf.clone().into()).await {
                Ok(_) => return Ok(()),
                Err(e) => {
                    if retries >= 3 {
                        return Err(e.to_string());
                    }
                    retries += 1;
                    tokio::time::sleep(tokio::time::Duration::from_millis(100 * retries)).await;
                }
            }
        }
    }

    async fn subscribe(&self, topic: String, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        use futures_util::StreamExt;

        let mut subscriber = self.client.subscribe(topic.clone()).await.map_err(|e| e.to_string())?;

        let worker = tokio::spawn(async move {
            while let Some(msg) = subscriber.next().await {
                use prost::Message as ProstMessage;
                let m = Message::decode(&msg.payload[..]).unwrap_or_else(|_| Message { topic: topic.clone(), payload: vec![] });
                handler(m);
            }
        });

        let cancel = Box::new(move || {
            worker.abort();
        });

        Ok(cancel)
    }
}

#[async_trait]
impl DistributedLock for NatsBus {
    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String> {
        let expires_at = chrono::Utc::now().timestamp() + ttl_seconds as i64;
        let payload = format!("{}:{}", owner, expires_at);

        if let Ok(Some(entry)) = self.kv.entry(resource).await {
            let entry_str = String::from_utf8_lossy(&entry.value);
            if let Some((stored_owner, stored_exp)) = entry_str.split_once(':') {
                if let Ok(exp) = stored_exp.parse::<i64>() {
                    if exp <= chrono::Utc::now().timestamp() || stored_owner == owner {
                        match self.kv.update(resource, payload.clone().into_bytes().into(), entry.revision).await {
                            Ok(_) => return Ok(true),
                            Err(_) => return Ok(false),
                        }
                    } else {
                        return Ok(false);
                    }
                }
            }
        }

        match self.kv.create(resource, payload.into_bytes().into()).await {
            Ok(_) => Ok(true),
            Err(e) => {
                if e.to_string().contains("wrong last sequence") {
                    Ok(false)
                } else {
                    Err(e.to_string())
                }
            }
        }
    }

    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
        if let Ok(Some(entry)) = self.kv.entry(resource).await {
            let entry_str = String::from_utf8_lossy(&entry.value);
            if let Some((stored_owner, _)) = entry_str.split_once(':') {
                if stored_owner == owner {
                    // Update with immediately expired lock to allow atomic replacement
                    let payload = format!("{}:0", owner);
                    let _ = self.kv.update(resource, payload.into_bytes().into(), entry.revision).await;
                }
            }
        }
        Ok(())
    }
}

#[async_trait]
impl DistributedLock for RedisBus {
    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String> {
        let mut conn = self.publish_conn.lock().await;
        let key = format!("lock:{}", resource);
        let script = redis::Script::new(r#"
            if redis.call("get", KEYS[1]) == ARGV[1] then
                redis.call("set", KEYS[1], ARGV[1], "EX", ARGV[2])
                return 1
            elseif redis.call("set", KEYS[1], ARGV[1], "NX", "EX", ARGV[2]) then
                return 1
            else
                return 0
            end
        "#);
        let res: i32 = script.key(&key).arg(owner).arg(ttl_seconds).invoke_async(&mut *conn).await.map_err(|e| e.to_string())?;
        Ok(res == 1)
    }

    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
        let mut conn = self.publish_conn.lock().await;
        let key = format!("lock:{}", resource);
        let script = redis::Script::new(r#"
            if redis.call("get", KEYS[1]) == ARGV[1] then
                return redis.call("del", KEYS[1])
            else
                return 0
            end
        "#);

        let _: i32 = script.key(&key).arg(owner).invoke_async(&mut *conn).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[async_trait]
impl DistributedLock for IpcBus {
    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String> {
        let expires_at = chrono::Utc::now().timestamp() + ttl_seconds as i64;
        let res = sqlx::query("INSERT INTO bus_locks (resource, owner, expires_at) VALUES (?, ?, ?) ON CONFLICT(resource) DO UPDATE SET owner = excluded.owner, expires_at = excluded.expires_at WHERE bus_locks.owner = excluded.owner OR bus_locks.expires_at < cast(strftime('%s', 'now') as integer)")
            .bind(resource)
            .bind(owner)
            .bind(expires_at)
            .execute(&self.pool)
            .await;

        match res {
            Ok(r) => Ok(r.rows_affected() > 0),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
        sqlx::query("DELETE FROM bus_locks WHERE resource = ? AND owner = ?")
            .bind(resource)
            .bind(owner)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[allow(dead_code)]
pub struct StateHandoffManager {
    bus: std::sync::Arc<dyn Bus>,
}

#[allow(dead_code)]
impl StateHandoffManager {
    pub fn new(bus: std::sync::Arc<dyn Bus>) -> Self {
        Self { bus }
    }

    pub async fn trigger_handoff(&self, payload: Vec<u8>) -> Result<(), String> {
        let msg = Message {
            topic: "system:state_handoff".to_string(),
            payload,
        };
        self.bus.publish(msg).await
    }
}

#[allow(dead_code)]
pub struct HealthMonitor {
    bus: std::sync::Arc<dyn Bus>,
}

#[allow(dead_code)]
impl HealthMonitor {
    pub fn new(bus: std::sync::Arc<dyn Bus>) -> Self {
        Self { bus }
    }

    pub async fn ping(&self) -> Result<(), String> {
        let msg = Message {
            topic: "system:health_ping".to_string(),
            payload: vec![],
        };
        self.bus.publish(msg).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    
    #[tokio::test]
    async fn test_memory_bus_pub_sub() {
        let bus = MemoryBus::new();
        let received = Arc::new(AtomicBool::new(false));
        let received_clone = received.clone();
        
        let handler = Box::new(move |msg: Message| {
            tracing::debug!("Received message: {:?}", msg);
            received_clone.store(true, Ordering::SeqCst);
        });
        
        let cancel = bus.subscribe("test_topic".to_string(), handler).await.unwrap();
        
        let msg = Message {
            topic: "test_topic".to_string(),
            payload: vec![],
        };
        
        bus.publish(msg).await.unwrap();
        
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        assert!(received.load(Ordering::SeqCst));
        
        cancel();
    }

    #[tokio::test]
    async fn test_ipc_bus_pub_sub() {
        let tmp_dir = std::env::var("TEST_TMPDIR").unwrap_or_else(|_| "/tmp".to_string());
        let db_path = format!("{}/test_ipc_bus_{}.sqlite", tmp_dir, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());
        let db_url = format!("sqlite://{}", db_path);

        let bus = IpcBus::new(&db_url).await.unwrap();

        let received = Arc::new(AtomicBool::new(false));
        let received_clone = received.clone();

        let handler = Box::new(move |msg: Message| {
            if msg.topic == "test_ipc_topic" {
                received_clone.store(true, Ordering::SeqCst);
            }
        });

        let cancel = bus.subscribe("test_ipc_topic".to_string(), handler).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        let msg = Message {
            topic: "test_ipc_topic".to_string(),
            payload: vec![],
        };

        bus.publish(msg).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

        assert!(received.load(Ordering::SeqCst));
        cancel();
    }

    #[tokio::test]
    async fn test_redis_bus_pub_sub() {
        let url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1".to_string());
        let bus = match RedisBus::new(&url).await {
            Ok(b) => b,
            Err(_) => return,
        };

        let received = Arc::new(AtomicBool::new(false));
        let received_clone = received.clone();

        let handler = Box::new(move |msg: Message| {
            if msg.topic == "test_redis_topic" {
                received_clone.store(true, Ordering::SeqCst);
            }
        });

        let cancel = bus.subscribe("test_redis_topic".to_string(), handler).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let msg = Message {
            topic: "test_redis_topic".to_string(),
            payload: vec![],
        };

        bus.publish(msg).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

        assert!(received.load(Ordering::SeqCst));
        cancel();
    }

    #[tokio::test]
    async fn test_health_monitor_ping() {
        let bus = std::sync::Arc::new(MemoryBus::new());
        let monitor = HealthMonitor::new(bus.clone());

        let received = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let received_clone = received.clone();

        let handler = Box::new(move |msg: Message| {
            if msg.topic == "system:health_ping" {
                received_clone.store(true, std::sync::atomic::Ordering::SeqCst);
            }
        });

        let cancel = bus.subscribe("system:health_ping".to_string(), handler).await.unwrap();

        monitor.ping().await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        assert!(received.load(std::sync::atomic::Ordering::SeqCst));
        cancel();
    }

    #[tokio::test]
    async fn test_state_handoff_trigger() {
        let bus = std::sync::Arc::new(MemoryBus::new());
        let manager = StateHandoffManager::new(bus.clone());

        let received = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let received_clone = received.clone();

        let payload_data = vec![1, 2, 3, 4];
        let payload_clone = payload_data.clone();

        let handler = Box::new(move |msg: Message| {
            if msg.topic == "system:state_handoff" && msg.payload == payload_clone {
                received_clone.store(true, std::sync::atomic::Ordering::SeqCst);
            }
        });

        let cancel = bus.subscribe("system:state_handoff".to_string(), handler).await.unwrap();

        manager.trigger_handoff(payload_data).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        assert!(received.load(std::sync::atomic::Ordering::SeqCst));
        cancel();
    }

    #[tokio::test]
    async fn test_memory_bus_distributed_lock() {
        let bus = MemoryBus::new();
        let resource = "test_resource";
        let owner1 = "owner1";
        let owner2 = "owner2";

        assert!(bus.acquire_lock(resource, owner1, 1).await.unwrap());
        assert!(!bus.acquire_lock(resource, owner2, 1).await.unwrap());
        assert!(bus.acquire_lock(resource, owner1, 1).await.unwrap());

        bus.release_lock(resource, owner1).await.unwrap();
        assert!(bus.acquire_lock(resource, owner2, 1).await.unwrap());
    }

    #[tokio::test]
    async fn test_ipc_bus_distributed_lock() {
        let tmp_dir = std::env::var("TEST_TMPDIR").unwrap_or_else(|_| "/tmp".to_string());
        let db_path = format!("{}/test_ipc_lock_{}.sqlite", tmp_dir, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());
        let db_url = format!("sqlite://{}", db_path);

        let bus = IpcBus::new(&db_url).await.unwrap();
        let resource = "test_ipc_resource";
        let owner1 = "owner1";
        let owner2 = "owner2";

        assert!(bus.acquire_lock(resource, owner1, 1).await.unwrap());
        assert!(!bus.acquire_lock(resource, owner2, 1).await.unwrap());

        // Allow lock to expire
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        assert!(bus.acquire_lock(resource, owner2, 1).await.unwrap());

        bus.release_lock(resource, owner2).await.unwrap();
        assert!(bus.acquire_lock(resource, owner1, 1).await.unwrap());

        // Re-acquire by same owner to extend
        assert!(bus.acquire_lock(resource, owner1, 1).await.unwrap());
    }

    #[tokio::test]
    async fn test_redis_bus_distributed_lock() {
        let url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1".to_string());
        let bus = match RedisBus::new(&url).await {
            Ok(b) => b,
            Err(_) => return,
        };
        let resource = "test_redis_resource";
        let owner1 = "owner1";
        let owner2 = "owner2";

        assert!(bus.acquire_lock(resource, owner1, 1).await.unwrap());
        assert!(!bus.acquire_lock(resource, owner2, 1).await.unwrap());
        assert!(bus.acquire_lock(resource, owner1, 1).await.unwrap());

        bus.release_lock(resource, owner1).await.unwrap();
        assert!(bus.acquire_lock(resource, owner2, 1).await.unwrap());
    }
}
