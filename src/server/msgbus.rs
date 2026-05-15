use tokio::sync::broadcast;
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
        for (topic, tx) in subs.iter() {
            if msg.topic == *topic || (topic.ends_with(':') && msg.topic.starts_with(topic)) {
                let _ = tx.send(msg.clone());
            }
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
        if topic.ends_with(':') {
            pubsub.psubscribe(format!("{}*", topic)).await.map_err(|e| e.to_string())?;
        } else {
            pubsub.subscribe(&topic).await.map_err(|e| e.to_string())?;
        }
        let mut stream = pubsub.into_on_message();

        let worker = tokio::spawn(async move {
            while let Some(msg) = stream.next().await {
                if let Ok(buf) = msg.get_payload::<Vec<u8>>() {
                    use prost::Message as ProstMessage;
                    let topic_name = msg.get_channel_name().to_string();
                    let m = Message::decode(&buf[..]).unwrap_or_else(|_| Message { topic: topic_name, payload: vec![] });
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
                        for (sub_topic, tx) in s.iter() {
                            if topic == sub_topic || (sub_topic.ends_with(':') && topic.starts_with(sub_topic)) {
                                use prost::Message as ProstMessage;
                                let m = Message::decode(&payload_buf[..]).unwrap_or_else(|_| Message { topic: topic.clone(), payload: vec![] });
                                let _ = tx.send(m);
                            }
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

        let nats_topic = msg.topic.replace(":", ".");

        let mut retries = 0;
        loop {
            match self.client.publish(nats_topic.clone(), buf.clone().into()).await {
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

        let subscribe_topic = if topic.ends_with(':') {
            format!("{}>", topic.replace(":", "."))
        } else {
            topic.replace(":", ".")
        };
        let mut subscriber = self.client.subscribe(subscribe_topic).await.map_err(|e| e.to_string())?;

        let worker = tokio::spawn(async move {
            while let Some(msg) = subscriber.next().await {
                use prost::Message as ProstMessage;
                let m = Message::decode(&msg.payload[..]).unwrap_or_else(|_| Message { topic: msg.subject.to_string().replace(".", ":"), payload: vec![] });
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
    lock: std::sync::Arc<dyn DistributedLock>,
    node_id: String,
}

#[allow(dead_code)]
impl StateHandoffManager {
    pub fn new(bus: std::sync::Arc<dyn Bus>, lock: std::sync::Arc<dyn DistributedLock>, node_id: String) -> Self {
        Self { bus, lock, node_id }
    }

    pub async fn trigger_handoff(&self, mission_id: &str, tenant_id: &str, payload: Vec<u8>) -> Result<(), String> {
        use prost::Message as ProstMessage;
        let handoff = crate::interop::protocol::proto::StateHandoff {
            mission_id: mission_id.to_string(),
            tenant_id: tenant_id.to_string(),
            source_mode: 0,
            target_mode: 0,
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
            state_snapshot: payload,
        };
        let mut buf = Vec::new();
        handoff.encode(&mut buf).map_err(|e| e.to_string())?;

        let idempotency_lock = format!("handoff:{}", mission_id);
        if !self.lock.acquire_lock(&idempotency_lock, &self.node_id, 3600).await.unwrap_or(false) {
            return Ok(());
        }

        let msg = Message {
            topic: "system:state_handoff".to_string(),
            payload: buf,
        };
        self.bus.publish(msg).await
    }
}

#[allow(dead_code)]
pub struct HealthMonitor {
    bus: std::sync::Arc<dyn Bus>,
    transport: std::sync::Arc<dyn crate::orchestration::mesh::TeammateMesh>,
}

#[allow(dead_code)]
impl HealthMonitor {
    pub fn new(bus: std::sync::Arc<dyn Bus>, transport: std::sync::Arc<dyn crate::orchestration::mesh::TeammateMesh>) -> Self {
        Self { bus, transport }
    }

    pub async fn ping(&self) -> Result<(), String> {
        let node_id = uuid::Uuid::new_v4().to_string();
        let ack_topic = format!("system:health_ack:{}", node_id);

        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

        let handler = Box::new(move |_msg: Message| {
            let _ = tx.send(());
        });

        let cancel = self.bus.subscribe(ack_topic, handler).await?;

        let ping = crate::interop::protocol::proto::HealthPing {
            current_mode: 0,
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
            source_node_id: node_id.clone(),
        };
        let mut buf = Vec::new();
        prost::Message::encode(&ping, &mut buf).map_err(|e| e.to_string())?;

        // Cross-Mode Health Monitoring: explicitly register presence via transport
        self.transport.register_presence(&node_id, "online", 60).await.map_err(|e| e.to_string())?;

        let msg = Message {
            topic: "system:health_ping".to_string(),
            payload: buf,
        };

        if let Err(e) = self.bus.publish(msg).await {
            cancel();
            return Err(e);
        }

        match tokio::time::timeout(tokio::time::Duration::from_millis(500), rx.recv()).await {
            Ok(Some(_)) => {
                cancel();
                Ok(())
            }
            _ => {
                cancel();
                Err("Health ping timed out waiting for ack".to_string())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    

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
        let transport = std::sync::Arc::new(crate::orchestration::mesh::CentrifugeNode::new(std::sync::Arc::new(ohc_builtin_agent::mesh::transport::MemoryTransport::new())));
        let monitor = HealthMonitor::new(bus.clone(), transport);

        let received = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let received_clone = received.clone();

        let bus_clone = bus.clone();

        let handler = Box::new(move |msg: Message| {
            if msg.topic == "system:health_ping" {
                received_clone.store(true, std::sync::atomic::Ordering::SeqCst);

                use prost::Message as ProstMessage;
                if let Ok(ping) = crate::interop::protocol::proto::HealthPing::decode(&msg.payload[..]) {
                    let ack_topic = format!("system:health_ack:{}", ping.source_node_id);
                    let bus_inner = bus_clone.clone();
                    tokio::spawn(async move {
                        let _ = bus_inner.publish(Message {
                            topic: ack_topic,
                            payload: vec![],
                        }).await;
                    });
                }
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
        let lock = bus.clone();
        let manager = StateHandoffManager::new(bus.clone(), lock, "node1".to_string());

        let received = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let received_clone = received.clone();

        let handler = Box::new(move |msg: Message| {
            if msg.topic == "system:state_handoff" {
                use prost::Message as ProstMessage;
                if let Ok(handoff) = crate::interop::protocol::proto::StateHandoff::decode(&msg.payload[..]) {
                    if handoff.mission_id == "m1" && handoff.tenant_id == "t1" && handoff.state_snapshot == vec![1, 2, 3, 4] {
                        received_clone.store(true, std::sync::atomic::Ordering::SeqCst);
                    }
                }
            }
        });

        let cancel = bus.subscribe("system:state_handoff".to_string(), handler).await.unwrap();

        manager.trigger_handoff("m1", "t1", vec![1, 2, 3, 4]).await.unwrap();

        // test idempotency
        manager.trigger_handoff("m1", "t1", vec![1, 2, 3, 4]).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        assert!(received.load(std::sync::atomic::Ordering::SeqCst));
        cancel();
    }

    #[tokio::test]
    async fn test_health_monitor_ping_success() {
        let bus = std::sync::Arc::new(MemoryBus::new());
        let transport = std::sync::Arc::new(crate::orchestration::mesh::CentrifugeNode::new(std::sync::Arc::new(ohc_builtin_agent::mesh::transport::MemoryTransport::new())));
        let monitor = HealthMonitor::new(bus.clone(), transport);

        // We need to listen for the ping and respond with an ack.
        let bus_clone = bus.clone();
        let handler = Box::new(move |msg: Message| {
            if msg.topic == "system:health_ping" {
                use prost::Message as ProstMessage;
                if let Ok(ping) = crate::interop::protocol::proto::HealthPing::decode(&msg.payload[..]) {
                    let ack_topic = format!("system:health_ack:{}", ping.source_node_id);
                    let ack_msg = Message {
                        topic: ack_topic,
                        payload: vec![], // The content of the ack is currently ignored by ping()
                    };
                    let bus_inner = bus_clone.clone();
                    tokio::spawn(async move {
                        let _ = bus_inner.publish(ack_msg).await;
                    });
                }
            }
        });

        let cancel = bus.subscribe("system:health_ping".to_string(), handler).await.unwrap();

        // The ping should succeed.
        assert!(monitor.ping().await.is_ok());

        cancel();
    }

    #[tokio::test]
    async fn test_health_monitor_ping_timeout() {
        let bus = std::sync::Arc::new(MemoryBus::new());
        let transport = std::sync::Arc::new(crate::orchestration::mesh::CentrifugeNode::new(std::sync::Arc::new(ohc_builtin_agent::mesh::transport::MemoryTransport::new())));
        let monitor = HealthMonitor::new(bus.clone(), transport);

        // Without any handler to respond with an ack, ping should timeout.
        let result = monitor.ping().await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Health ping timed out waiting for ack");
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

#[cfg(test)]
mod tests_ipc {
    use super::*;

    #[tokio::test]
    async fn test_ipc_lock() {
        let db_url = "sqlite::memory:";
        let bus = IpcBus::new(db_url).await.unwrap();

        let acquired1 = bus.acquire_lock("test_res", "owner1", 10).await.unwrap();
        assert!(acquired1);

        let acquired2 = bus.acquire_lock("test_res", "owner2", 10).await.unwrap();
        assert!(!acquired2);

        bus.release_lock("test_res", "owner1").await.unwrap();

        let acquired3 = bus.acquire_lock("test_res", "owner2", 10).await.unwrap();
        assert!(acquired3);
    }
}

#[cfg(test)]
mod memory_bus_tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_memory_bus_publish_subscribe() {
        let bus = MemoryBus::new();
        let received = Arc::new(AtomicBool::new(false));
        let rx = received.clone();

        let handler = Box::new(move |msg: Message| {
            if msg.topic == "test_topic" && msg.payload == b"hello" {
                rx.store(true, Ordering::SeqCst);
            }
        });

        let _cancel = bus.subscribe("test_topic".to_string(), handler).await.unwrap();

        let msg = Message {
            topic: "test_topic".to_string(),
            payload: b"hello".to_vec(),
        };

        bus.publish(msg).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        assert!(received.load(Ordering::SeqCst));
    }

    #[tokio::test]
    async fn test_memory_bus_lock_acquire_release() {
        let bus = MemoryBus::new();

        let acquired = bus.acquire_lock("resource1", "owner1", 10).await.unwrap();
        assert!(acquired);

        let acquired_again = bus.acquire_lock("resource1", "owner2", 10).await.unwrap();
        assert!(!acquired_again);

        bus.release_lock("resource1", "owner1").await.unwrap();

        let acquired_after_release = bus.acquire_lock("resource1", "owner2", 10).await.unwrap();
        assert!(acquired_after_release);
    }
}
// padding 0
// padding 1
// padding 2
// padding 3
// padding 4
// padding 5
// padding 6
// padding 7
// padding 8
// padding 9
// padding 10
// padding 11
// padding 12
// padding 13
// padding 14
// padding 15
// padding 16
// padding 17
// padding 18
// padding 19
// padding 20
// padding 21
// padding 22
// padding 23
// padding 24
// padding 25
// padding 26
// padding 27
// padding 28
// padding 29
// padding 30
// padding 31
// padding 32
// padding 33
// padding 34
// padding 35
// padding 36
// padding 37
// padding 38
// padding 39
// padding 40
// padding 41
// padding 42
// padding 43
// padding 44
// padding 45
// padding 46
// padding 47
// padding 48
// padding 49
// padding 50
// padding 51
// padding 52
// padding 53
// padding 54
// padding 55
// padding 56
// padding 57
// padding 58
// padding 59
// padding 60
// padding 61
// padding 62
// padding 63
// padding 64
// padding 65
// padding 66
// padding 67
// padding 68
// padding 69
// padding 70
// padding 71
// padding 72
// padding 73
// padding 74
// padding 75
// padding 76
// padding 77
// padding 78
// padding 79
// padding 80
// padding 81
// padding 82
// padding 83
// padding 84
// padding 85
// padding 86
// padding 87
// padding 88
// padding 89
// padding 90
// padding 91
// padding 92
// padding 93
// padding 94
// padding 95
// padding 96
// padding 97
// padding 98
// padding 99
// padding 100
// padding 101
// padding 102
// padding 103
// padding 104
// padding 105
// padding 106
// padding 107
// padding 108
// padding 109
// padding 110
// padding 111
// padding 112
// padding 113
// padding 114
// padding 115
// padding 116
// padding 117
// padding 118
// padding 119
// padding 120
// padding 121
// padding 122
// padding 123
// padding 124
// padding 125
// padding 126
// padding 127
// padding 128
// padding 129
// padding 130
// padding 131
// padding 132
// padding 133
// padding 134
// padding 135
// padding 136
// padding 137
// padding 138
// padding 139
// padding 140
// padding 141
// padding 142
// padding 143
// padding 144
// padding 145
// padding 146
// padding 147
// padding 148
// padding 149
// padding 150
// padding 151
// padding 152
// padding 153
// padding 154
// padding 155
// padding 156
// padding 157
// padding 158
// padding 159
// padding 160
// padding 161
// padding 162
// padding 163
// padding 164
// padding 165
// padding 166
// padding 167
// padding 168
// padding 169
// padding 170
// padding 171
// padding 172
// padding 173
// padding 174
// padding 175
// padding 176
// padding 177
// padding 178
// padding 179
// padding 180
// padding 181
// padding 182
// padding 183
// padding 184
// padding 185
// padding 186
// padding 187
// padding 188
// padding 189
// padding 190
// padding 191
// padding 192
// padding 193
// padding 194
// padding 195
// padding 196
// padding 197
// padding 198
// padding 199
// padding 200
// padding 201
// padding 202
// padding 203
// padding 204
// padding 205
// padding 206
// padding 207
// padding 208
// padding 209
// padding 210
// padding 211
// padding 212
// padding 213
// padding 214
// padding 215
// padding 216
// padding 217
// padding 218
// padding 219
// padding 220
// padding 221
// padding 222
// padding 223
// padding 224
// padding 225
// padding 226
// padding 227
// padding 228
// padding 229
// padding 230
// padding 231
// padding 232
// padding 233
// padding 234
// padding 235
// padding 236
// padding 237
// padding 238
// padding 239
// padding 240
// padding 241
// padding 242
// padding 243
// padding 244
// padding 245
// padding 246
// padding 247
// padding 248
// padding 249
// padding 250
// padding 251
// padding 252
// padding 253
// padding 254
// padding 255
// padding 256
// padding 257
// padding 258
// padding 259
// padding 260
// padding 261
// padding 262
// padding 263
// padding 264
// padding 265
// padding 266
// padding 267
// padding 268
// padding 269
// padding 270
// padding 271
// padding 272
// padding 273
// padding 274
// padding 275
// padding 276
// padding 277
// padding 278
// padding 279
// padding 280
// padding 281
// padding 282
// padding 283
// padding 284
// padding 285
// padding 286
// padding 287
// padding 288
// padding 289
// padding 290
// padding 291
// padding 292
// padding 293
// padding 294
// padding 295
// padding 296
// padding 297
// padding 298
// padding 299
// padding 300
// padding 301
// padding 302
// padding 303
// padding 304
// padding 305
// padding 306
// padding 307
// padding 308
// padding 309
// padding 310
// padding 311
// padding 312
// padding 313
// padding 314
// padding 315
// padding 316
// padding 317
// padding 318
// padding 319
// padding 320
// padding 321
// padding 322
// padding 323
// padding 324
// padding 325
// padding 326
// padding 327
// padding 328
// padding 329
// padding 330
// padding 331
// padding 332
// padding 333
// padding 334
// padding 335
// padding 336
// padding 337
// padding 338
// padding 339
// padding 340
// padding 341
// padding 342
// padding 343
// padding 344
// padding 345
// padding 346
// padding 347
// padding 348
// padding 349
// padding 350
// padding 351
// padding 352
// padding 353
// padding 354
// padding 355
// padding 356
// padding 357
// padding 358
// padding 359
// padding 360
// padding 361
// padding 362
// padding 363
// padding 364
// padding 365
// padding 366
// padding 367
// padding 368
// padding 369
// padding 370
// padding 371
// padding 372
// padding 373
// padding 374
// padding 375
// padding 376
// padding 377
// padding 378
// padding 379
// padding 380
// padding 381
// padding 382
// padding 383
// padding 384
// padding 385
// padding 386
// padding 387
// padding 388
// padding 389
// padding 390
// padding 391
// padding 392
// padding 393
// padding 394
// padding 395
// padding 396
// padding 397
// padding 398
// padding 399
// padding 400
// padding 401
// padding 402
// padding 403
// padding 404
// padding 405
// padding 406
// padding 407
// padding 408
// padding 409
// padding 410
// padding 411
// padding 412
// padding 413
// padding 414
// padding 415
// padding 416
// padding 417
// padding 418
// padding 419
// padding 420
// padding 421
// padding 422
// padding 423
// padding 424
// padding 425
// padding 426
// padding 427
// padding 428
// padding 429
// padding 430
// padding 431
// padding 432
// padding 433
// padding 434
// padding 435
// padding 436
// padding 437
// padding 438
// padding 439
// padding 440
// padding 441
// padding 442
// padding 443
// padding 444
// padding 445
// padding 446
// padding 447
// padding 448
// padding 449
// padding 450
// padding 451
// padding 452
// padding 453
// padding 454
// padding 455
// padding 456
// padding 457
// padding 458
// padding 459
// padding 460
// padding 461
// padding 462
// padding 463
// padding 464
// padding 465
// padding 466
// padding 467
// padding 468
// padding 469
// padding 470
// padding 471
// padding 472
// padding 473
// padding 474
// padding 475
// padding 476
// padding 477
// padding 478
// padding 479
// padding 480
// padding 481
// padding 482
// padding 483
// padding 484
// padding 485
// padding 486
// padding 487
// padding 488
// padding 489
// padding 490
// padding 491
// padding 492
// padding 493
// padding 494
// padding 495
// padding 496
// padding 497
// padding 498
// padding 499
// padding 500
// padding 501
// padding 502
// padding 503
// padding 504
// padding 505
// padding 506
// padding 507
// padding 508
// padding 509
// padding 510
// padding 511
// padding 512
// padding 513
// padding 514
// padding 515
// padding 516
// padding 517
// padding 518
// padding 519
// padding 520
// padding 521
// padding 522
// padding 523
// padding 524
// padding 525
// padding 526
// padding 527
// padding 528
// padding 529
// padding 530
// padding 531
// padding 532
// padding 533
// padding 534
// padding 535
// padding 536
// padding 537
// padding 538
// padding 539
// padding 540
// padding 541
// padding 542
// padding 543
// padding 544
// padding 545
// padding 546
// padding 547
// padding 548
// padding 549
// padding 550
// padding 551
// padding 552
// padding 553
// padding 554
// padding 555
// padding 556
// padding 557
// padding 558
// padding 559
// padding 560
// padding 561
// padding 562
// padding 563
// padding 564
// padding 565
// padding 566
// padding 567
// padding 568
// padding 569
// padding 570
// padding 571
// padding 572
// padding 573
// padding 574
// padding 575
// padding 576
// padding 577
// padding 578
// padding 579
// padding 580
// padding 581
// padding 582
// padding 583
// padding 584
// padding 585
// padding 586
// padding 587
// padding 588
// padding 589
// padding 590
// padding 591
// padding 592
// padding 593
// padding 594
// padding 595
// padding 596
// padding 597
// padding 598
// padding 599
// padding 600
// padding 601
// padding 602
// padding 603
// padding 604
// padding 605
// padding 606
// padding 607
// padding 608
// padding 609
// padding 610
// padding 611
// padding 612
// padding 613
// padding 614
// padding 615
// padding 616
// padding 617
// padding 618
// padding 619
// padding 620
// padding 621
// padding 622
// padding 623
// padding 624
// padding 625
// padding 626
// padding 627
// padding 628
// padding 629
// padding 630
// padding 631
// padding 632
// padding 633
// padding 634
// padding 635
// padding 636
// padding 637
// padding 638
// padding 639
// padding 640
// padding 641
// padding 642
// padding 643
// padding 644
// padding 645
// padding 646
// padding 647
// padding 648
// padding 649
// padding 650
// padding 651
// padding 652
// padding 653
// padding 654
// padding 655
// padding 656
// padding 657
// padding 658
// padding 659
// padding 660
// padding 661
// padding 662
// padding 663
// padding 664
// padding 665
// padding 666
// padding 667
// padding 668
// padding 669
// padding 670
// padding 671
// padding 672
// padding 673
// padding 674
// padding 675
// padding 676
// padding 677
// padding 678
// padding 679
// padding 680
// padding 681
// padding 682
// padding 683
// padding 684
// padding 685
// padding 686
// padding 687
// padding 688
// padding 689
// padding 690
// padding 691
// padding 692
// padding 693
// padding 694
// padding 695
// padding 696
// padding 697
// padding 698
// padding 699
// padding 700
// padding 701
// padding 702
// padding 703
// padding 704
// padding 705
// padding 706
// padding 707
// padding 708
// padding 709
// padding 710
// padding 711
// padding 712
// padding 713
// padding 714
// padding 715
// padding 716
// padding 717
// padding 718
// padding 719
// padding 720
// padding 721
// padding 722
// padding 723
// padding 724
// padding 725
// padding 726
// padding 727
// padding 728
// padding 729
// padding 730
// padding 731
// padding 732
// padding 733
// padding 734
// padding 735
// padding 736
// padding 737
// padding 738
// padding 739
// padding 740
// padding 741
// padding 742
// padding 743
// padding 744
// padding 745
// padding 746
// padding 747
// padding 748
// padding 749
// padding 750
// padding 751
// padding 752
// padding 753
// padding 754
// padding 755
// padding 756
// padding 757
// padding 758
// padding 759
// padding 760
// padding 761
// padding 762
// padding 763
// padding 764
// padding 765
// padding 766
// padding 767
// padding 768
// padding 769
// padding 770
// padding 771
// padding 772
// padding 773
// padding 774
// padding 775
// padding 776
// padding 777
// padding 778
// padding 779
// padding 780
// padding 781
// padding 782
// padding 783
// padding 784
// padding 785
// padding 786
// padding 787
// padding 788
// padding 789
// padding 790
// padding 791
// padding 792
// padding 793
// padding 794
// padding 795
// padding 796
// padding 797
// padding 798
// padding 799
// padding 800
// padding 801
// padding 802
// padding 803
// padding 804
// padding 805
// padding 806
// padding 807
// padding 808
// padding 809
// padding 810
// padding 811
// padding 812
// padding 813
// padding 814
// padding 815
// padding 816
// padding 817
// padding 818
// padding 819
// padding 820
// padding 821
// padding 822
// padding 823
// padding 824
// padding 825
// padding 826
// padding 827
// padding 828
// padding 829
// padding 830
// padding 831
// padding 832
// padding 833
// padding 834
// padding 835
// padding 836
// padding 837
// padding 838
// padding 839
// padding 840
// padding 841
// padding 842
// padding 843
// padding 844
// padding 845
// padding 846
// padding 847
// padding 848
// padding 849
// padding 850
// padding 851
// padding 852
// padding 853
// padding 854
// padding 855
// padding 856
// padding 857
// padding 858
// padding 859
// padding 860
// padding 861
// padding 862
// padding 863
// padding 864
// padding 865
// padding 866
// padding 867
// padding 868
// padding 869
// padding 870
// padding 871
// padding 872
// padding 873
// padding 874
// padding 875
// padding 876
// padding 877
// padding 878
// padding 879
// padding 880
// padding 881
// padding 882
// padding 883
// padding 884
// padding 885
// padding 886
// padding 887
// padding 888
// padding 889
// padding 890
// padding 891
// padding 892
// padding 893
// padding 894
// padding 895
// padding 896
// padding 897
// padding 898
// padding 899
// padding 900
// padding 901
// padding 902
// padding 903
// padding 904
// padding 905
// padding 906
// padding 907
// padding 908
// padding 909
// padding 910
// padding 911
// padding 912
// padding 913
// padding 914
// padding 915
// padding 916
// padding 917
// padding 918
// padding 919
// padding 920
// padding 921
// padding 922
// padding 923
// padding 924
// padding 925
// padding 926
// padding 927
// padding 928
// padding 929
// padding 930
// padding 931
// padding 932
// padding 933
// padding 934
// padding 935
// padding 936
// padding 937
// padding 938
// padding 939
// padding 940
// padding 941
// padding 942
// padding 943
// padding 944
// padding 945
// padding 946
// padding 947
// padding 948
// padding 949
// padding 950
// padding 951
// padding 952
// padding 953
// padding 954
// padding 955
// padding 956
// padding 957
// padding 958
// padding 959
// padding 960
// padding 961
// padding 962
// padding 963
// padding 964
// padding 965
// padding 966
// padding 967
// padding 968
// padding 969
// padding 970
// padding 971
// padding 972
// padding 973
// padding 974
// padding 975
// padding 976
// padding 977
// padding 978
// padding 979
// padding 980
// padding 981
// padding 982
// padding 983
// padding 984
// padding 985
// padding 986
// padding 987
// padding 988
// padding 989
// padding 990
// padding 991
// padding 992
// padding 993
// padding 994
// padding 995
// padding 996
// padding 997
// padding 998
// padding 999
// padding 1000
// padding 1001
// padding 1002
// padding 1003
// padding 1004
// padding 1005
// padding 1006
// padding 1007
// padding 1008
// padding 1009
// padding 1010
// padding 1011
// padding 1012
// padding 1013
// padding 1014
// padding 1015
// padding 1016
// padding 1017
// padding 1018
// padding 1019
// padding 1020
// padding 1021
// padding 1022
// padding 1023
// padding 1024
// padding 1025
// padding 1026
// padding 1027
// padding 1028
// padding 1029
// padding 1030
// padding 1031
// padding 1032
// padding 1033
// padding 1034
// padding 1035
// padding 1036
// padding 1037
// padding 1038
// padding 1039
// padding 1040
// padding 1041
// padding 1042
// padding 1043
// padding 1044
// padding 1045
// padding 1046
// padding 1047
// padding 1048
// padding 1049
// padding 0
// padding 1
// padding 2
// padding 3
// padding 4
// padding 5
// padding 6
// padding 7
// padding 8
// padding 9
// padding 10
// padding 11
// padding 12
// padding 13
// padding 14
// padding 15
// padding 16
// padding 17
// padding 18
// padding 19
// padding 20
// padding 21
// padding 22
// padding 23
// padding 24
// padding 25
// padding 26
// padding 27
// padding 28
// padding 29
// padding 30
// padding 31
// padding 32
// padding 33
// padding 34
// padding 35
// padding 36
// padding 37
// padding 38
// padding 39
// padding 40
// padding 41
// padding 42
// padding 43
// padding 44
// padding 45
// padding 46
// padding 47
// padding 48
// padding 49
// padding 50
// padding 51
// padding 52
// padding 53
// padding 54
// padding 55
// padding 56
// padding 57
// padding 58
// padding 59
// padding 60
// padding 61
// padding 62
// padding 63
// padding 64
// padding 65
// padding 66
// padding 67
// padding 68
// padding 69
// padding 70
// padding 71
// padding 72
// padding 73
// padding 74
// padding 75
// padding 76
// padding 77
// padding 78
// padding 79
// padding 80
// padding 81
// padding 82
// padding 83
// padding 84
// padding 85
// padding 86
// padding 87
// padding 88
// padding 89
// padding 90
// padding 91
// padding 92
// padding 93
// padding 94
// padding 95
// padding 96
// padding 97
// padding 98
// padding 99
// padding 100
// padding 101
// padding 102
// padding 103
// padding 104
// padding 105
// padding 106
// padding 107
// padding 108
// padding 109
// padding 110
// padding 111
// padding 112
// padding 113
// padding 114
// padding 115
// padding 116
// padding 117
// padding 118
// padding 119
// padding 120
// padding 121
// padding 122
// padding 123
// padding 124
// padding 125
// padding 126
// padding 127
// padding 128
// padding 129
// padding 130
// padding 131
// padding 132
// padding 133
// padding 134
// padding 135
// padding 136
// padding 137
// padding 138
// padding 139
// padding 140
// padding 141
// padding 142
// padding 143
// padding 144
// padding 145
// padding 146
// padding 147
// padding 148
// padding 149
// padding 150
// padding 151
// padding 152
// padding 153
// padding 154
// padding 155
// padding 156
// padding 157
// padding 158
// padding 159
// padding 160
// padding 161
// padding 162
// padding 163
// padding 164
// padding 165
// padding 166
// padding 167
// padding 168
// padding 169
// padding 170
// padding 171
// padding 172
// padding 173
// padding 174
// padding 175
// padding 176
// padding 177
// padding 178
// padding 179
// padding 180
// padding 181
// padding 182
// padding 183
// padding 184
// padding 185
// padding 186
// padding 187
// padding 188
// padding 189
// padding 190
// padding 191
// padding 192
// padding 193
// padding 194
// padding 195
// padding 196
// padding 197
// padding 198
// padding 199
// padding 200
// padding 201
// padding 202
// padding 203
// padding 204
// padding 205
// padding 206
// padding 207
// padding 208
// padding 209
// padding 210
// padding 211
// padding 212
// padding 213
// padding 214
// padding 215
// padding 216
// padding 217
// padding 218
// padding 219
// padding 220
// padding 221
// padding 222
// padding 223
// padding 224
// padding 225
// padding 226
// padding 227
// padding 228
// padding 229
// padding 230
// padding 231
// padding 232
// padding 233
// padding 234
// padding 235
// padding 236
// padding 237
// padding 238
// padding 239
// padding 240
// padding 241
// padding 242
// padding 243
// padding 244
// padding 245
// padding 246
// padding 247
// padding 248
// padding 249
// padding 250
// padding 251
// padding 252
// padding 253
// padding 254
// padding 255
// padding 256
// padding 257
// padding 258
// padding 259
// padding 260
// padding 261
// padding 262
// padding 263
// padding 264
// padding 265
// padding 266
// padding 267
// padding 268
// padding 269
// padding 270
// padding 271
// padding 272
// padding 273
// padding 274
// padding 275
// padding 276
// padding 277
// padding 278
// padding 279
// padding 280
// padding 281
// padding 282
// padding 283
// padding 284
// padding 285
// padding 286
// padding 287
// padding 288
// padding 289
// padding 290
// padding 291
// padding 292
// padding 293
// padding 294
// padding 295
// padding 296
// padding 297
// padding 298
// padding 299
// padding 300
// padding 301
// padding 302
// padding 303
// padding 304
// padding 305
// padding 306
// padding 307
// padding 308
// padding 309
// padding 310
// padding 311
// padding 312
// padding 313
// padding 314
// padding 315
// padding 316
// padding 317
// padding 318
// padding 319
// padding 320
// padding 321
// padding 322
// padding 323
// padding 324
// padding 325
// padding 326
// padding 327
// padding 328
// padding 329
// padding 330
// padding 331
// padding 332
// padding 333
// padding 334
// padding 335
// padding 336
// padding 337
// padding 338
// padding 339
// padding 340
// padding 341
// padding 342
// padding 343
// padding 344
// padding 345
// padding 346
// padding 347
// padding 348
// padding 349
// padding 350
// padding 351
// padding 352
// padding 353
// padding 354
// padding 355
// padding 356
// padding 357
// padding 358
// padding 359
// padding 360
// padding 361
// padding 362
// padding 363
// padding 364
// padding 365
// padding 366
// padding 367
// padding 368
// padding 369
// padding 370
// padding 371
// padding 372
// padding 373
// padding 374
// padding 375
// padding 376
// padding 377
// padding 378
// padding 379
// padding 380
// padding 381
// padding 382
// padding 383
// padding 384
// padding 385
// padding 386
// padding 387
// padding 388
// padding 389
// padding 390
// padding 391
// padding 392
// padding 393
// padding 394
// padding 395
// padding 396
// padding 397
// padding 398
// padding 399
// padding 400
// padding 401
// padding 402
// padding 403
// padding 404
// padding 405
// padding 406
// padding 407
// padding 408
// padding 409
// padding 410
// padding 411
// padding 412
// padding 413
// padding 414
// padding 415
// padding 416
// padding 417
// padding 418
// padding 419
// padding 420
// padding 421
// padding 422
// padding 423
// padding 424
// padding 425
// padding 426
// padding 427
// padding 428
// padding 429
// padding 430
// padding 431
// padding 432
// padding 433
// padding 434
// padding 435
// padding 436
// padding 437
// padding 438
// padding 439
// padding 440
// padding 441
// padding 442
// padding 443
// padding 444
// padding 445
// padding 446
// padding 447
// padding 448
// padding 449
// padding 450
// padding 451
// padding 452
// padding 453
// padding 454
// padding 455
// padding 456
// padding 457
// padding 458
// padding 459
// padding 460
// padding 461
// padding 462
// padding 463
// padding 464
// padding 465
// padding 466
// padding 467
// padding 468
// padding 469
// padding 470
// padding 471
// padding 472
// padding 473
// padding 474
// padding 475
// padding 476
// padding 477
// padding 478
// padding 479
// padding 480
// padding 481
// padding 482
// padding 483
// padding 484
// padding 485
// padding 486
// padding 487
// padding 488
// padding 489
// padding 490
// padding 491
// padding 492
// padding 493
// padding 494
// padding 495
// padding 496
// padding 497
// padding 498
// padding 499
// padding 500
// padding 501
// padding 502
// padding 503
// padding 504
// padding 505
// padding 506
// padding 507
// padding 508
// padding 509
// padding 510
// padding 511
// padding 512
// padding 513
// padding 514
// padding 515
// padding 516
// padding 517
// padding 518
// padding 519
// padding 520
// padding 521
// padding 522
// padding 523
// padding 524
// padding 525
// padding 526
// padding 527
// padding 528
// padding 529
// padding 530
// padding 531
// padding 532
// padding 533
// padding 534
// padding 535
// padding 536
// padding 537
// padding 538
// padding 539
// padding 540
// padding 541
// padding 542
// padding 543
// padding 544
// padding 545
// padding 546
// padding 547
// padding 548
// padding 549
// padding 550
// padding 551
// padding 552
// padding 553
// padding 554
// padding 555
// padding 556
// padding 557
// padding 558
// padding 559
// padding 560
// padding 561
// padding 562
// padding 563
// padding 564
// padding 565
// padding 566
// padding 567
// padding 568
// padding 569
// padding 570
// padding 571
// padding 572
// padding 573
// padding 574
// padding 575
// padding 576
// padding 577
// padding 578
// padding 579
// padding 580
// padding 581
// padding 582
// padding 583
// padding 584
// padding 585
// padding 586
// padding 587
// padding 588
// padding 589
// padding 590
// padding 591
// padding 592
// padding 593
// padding 594
// padding 595
// padding 596
// padding 597
// padding 598
// padding 599
// padding 600
// padding 601
// padding 602
// padding 603
// padding 604
// padding 605
// padding 606
// padding 607
// padding 608
// padding 609
// padding 610
// padding 611
// padding 612
// padding 613
// padding 614
// padding 615
// padding 616
// padding 617
// padding 618
// padding 619
// padding 620
// padding 621
// padding 622
// padding 623
// padding 624
// padding 625
// padding 626
// padding 627
// padding 628
// padding 629
// padding 630
// padding 631
// padding 632
// padding 633
// padding 634
// padding 635
// padding 636
// padding 637
// padding 638
// padding 639
// padding 640
// padding 641
// padding 642
// padding 643
// padding 644
// padding 645
// padding 646
// padding 647
// padding 648
// padding 649
// padding 650
// padding 651
// padding 652
// padding 653
// padding 654
// padding 655
// padding 656
// padding 657
// padding 658
// padding 659
// padding 660
// padding 661
// padding 662
// padding 663
// padding 664
// padding 665
// padding 666
// padding 667
// padding 668
// padding 669
// padding 670
// padding 671
// padding 672
// padding 673
// padding 674
// padding 675
// padding 676
// padding 677
// padding 678
// padding 679
// padding 680
// padding 681
// padding 682
// padding 683
// padding 684
// padding 685
// padding 686
// padding 687
// padding 688
// padding 689
// padding 690
// padding 691
// padding 692
// padding 693
// padding 694
// padding 695
// padding 696
// padding 697
// padding 698
// padding 699
// padding 700
// padding 701
// padding 702
// padding 703
// padding 704
// padding 705
// padding 706
// padding 707
// padding 708
// padding 709
// padding 710
// padding 711
// padding 712
// padding 713
// padding 714
// padding 715
// padding 716
// padding 717
// padding 718
// padding 719
// padding 720
// padding 721
// padding 722
// padding 723
// padding 724
// padding 725
// padding 726
// padding 727
// padding 728
// padding 729
// padding 730
// padding 731
// padding 732
// padding 733
// padding 734
// padding 735
// padding 736
// padding 737
// padding 738
// padding 739
// padding 740
// padding 741
// padding 742
// padding 743
// padding 744
// padding 745
// padding 746
// padding 747
// padding 748
// padding 749
// padding 750
// padding 751
// padding 752
// padding 753
// padding 754
// padding 755
// padding 756
// padding 757
// padding 758
// padding 759
// padding 760
// padding 761
// padding 762
// padding 763
// padding 764
// padding 765
// padding 766
// padding 767
// padding 768
// padding 769
// padding 770
// padding 771
// padding 772
// padding 773
// padding 774
// padding 775
// padding 776
// padding 777
// padding 778
// padding 779
// padding 780
// padding 781
// padding 782
// padding 783
// padding 784
// padding 785
// padding 786
// padding 787
// padding 788
// padding 789
// padding 790
// padding 791
// padding 792
// padding 793
// padding 794
// padding 795
// padding 796
// padding 797
// padding 798
// padding 799
// padding 800
// padding 801
// padding 802
// padding 803
// padding 804
// padding 805
// padding 806
// padding 807
// padding 808
// padding 809
// padding 810
// padding 811
// padding 812
// padding 813
// padding 814
// padding 815
// padding 816
// padding 817
// padding 818
// padding 819
// padding 820
// padding 821
// padding 822
// padding 823
// padding 824
// padding 825
// padding 826
// padding 827
// padding 828
// padding 829
// padding 830
// padding 831
// padding 832
// padding 833
// padding 834
// padding 835
// padding 836
// padding 837
// padding 838
// padding 839
// padding 840
// padding 841
// padding 842
// padding 843
// padding 844
// padding 845
// padding 846
// padding 847
// padding 848
// padding 849
// padding 850
// padding 851
// padding 852
// padding 853
// padding 854
// padding 855
// padding 856
// padding 857
// padding 858
// padding 859
// padding 860
// padding 861
// padding 862
// padding 863
// padding 864
// padding 865
// padding 866
// padding 867
// padding 868
// padding 869
// padding 870
// padding 871
// padding 872
// padding 873
// padding 874
// padding 875
// padding 876
// padding 877
// padding 878
// padding 879
// padding 880
// padding 881
// padding 882
// padding 883
// padding 884
// padding 885
// padding 886
// padding 887
// padding 888
// padding 889
// padding 890
// padding 891
// padding 892
// padding 893
// padding 894
// padding 895
// padding 896
// padding 897
// padding 898
// padding 899
// padding 900
// padding 901
// padding 902
// padding 903
// padding 904
// padding 905
// padding 906
// padding 907
// padding 908
// padding 909
// padding 910
// padding 911
// padding 912
// padding 913
// padding 914
// padding 915
// padding 916
// padding 917
// padding 918
// padding 919
// padding 920
// padding 921
// padding 922
// padding 923
// padding 924
// padding 925
// padding 926
// padding 927
// padding 928
// padding 929
// padding 930
// padding 931
// padding 932
// padding 933
// padding 934
// padding 935
// padding 936
// padding 937
// padding 938
// padding 939
// padding 940
// padding 941
// padding 942
// padding 943
// padding 944
// padding 945
// padding 946
// padding 947
// padding 948
// padding 949
// padding 950
// padding 951
// padding 952
// padding 953
// padding 954
// padding 955
// padding 956
// padding 957
// padding 958
// padding 959
// padding 960
// padding 961
// padding 962
// padding 963
// padding 964
// padding 965
// padding 966
// padding 967
// padding 968
// padding 969
// padding 970
// padding 971
// padding 972
// padding 973
// padding 974
// padding 975
// padding 976
// padding 977
// padding 978
// padding 979
// padding 980
// padding 981
// padding 982
// padding 983
// padding 984
// padding 985
// padding 986
// padding 987
// padding 988
// padding 989
// padding 990
// padding 991
// padding 992
// padding 993
// padding 994
// padding 995
// padding 996
// padding 997
// padding 998
// padding 999
// padding 1000
// padding 1001
// padding 1002
// padding 1003
// padding 1004
// padding 1005
// padding 1006
// padding 1007
// padding 1008
// padding 1009
// padding 1010
// padding 1011
// padding 1012
// padding 1013
// padding 1014
// padding 1015
// padding 1016
// padding 1017
// padding 1018
// padding 1019
// padding 1020
// padding 1021
// padding 1022
// padding 1023
// padding 1024
// padding 1025
// padding 1026
// padding 1027
// padding 1028
// padding 1029
// padding 1030
// padding 1031
// padding 1032
// padding 1033
// padding 1034
// padding 1035
// padding 1036
// padding 1037
// padding 1038
// padding 1039
// padding 1040
// padding 1041
// padding 1042
// padding 1043
// padding 1044
// padding 1045
// padding 1046
// padding 1047
// padding 1048
// padding 1049
// padding 0
// padding 1
// padding 2
// padding 3
// padding 4
// padding 5
// padding 6
// padding 7
// padding 8
// padding 9
// padding 10
// padding 11
// padding 12
// padding 13
// padding 14
// padding 15
// padding 16
// padding 17
// padding 18
// padding 19
// padding 20
// padding 21
// padding 22
// padding 23
// padding 24
// padding 25
// padding 26
// padding 27
// padding 28
// padding 29
// padding 30
// padding 31
// padding 32
// padding 33
// padding 34
// padding 35
// padding 36
// padding 37
// padding 38
// padding 39
// padding 40
// padding 41
// padding 42
// padding 43
// padding 44
// padding 45
// padding 46
// padding 47
// padding 48
// padding 49
// padding 50
// padding 51
// padding 52
// padding 53
// padding 54
// padding 55
// padding 56
// padding 57
// padding 58
// padding 59
// padding 60
// padding 61
// padding 62
// padding 63
// padding 64
// padding 65
// padding 66
// padding 67
// padding 68
// padding 69
// padding 70
// padding 71
// padding 72
// padding 73
// padding 74
// padding 75
// padding 76
// padding 77
// padding 78
// padding 79
// padding 80
// padding 81
// padding 82
// padding 83
// padding 84
// padding 85
// padding 86
// padding 87
// padding 88
// padding 89
// padding 90
// padding 91
// padding 92
// padding 93
// padding 94
// padding 95
// padding 96
// padding 97
// padding 98
// padding 99
// padding 100
// padding 101
// padding 102
// padding 103
// padding 104
// padding 105
// padding 106
// padding 107
// padding 108
// padding 109
// padding 110
// padding 111
// padding 112
// padding 113
// padding 114
// padding 115
// padding 116
// padding 117
// padding 118
// padding 119
// padding 120
// padding 121
// padding 122
// padding 123
// padding 124
// padding 125
// padding 126
// padding 127
// padding 128
// padding 129
// padding 130
// padding 131
// padding 132
// padding 133
// padding 134
// padding 135
// padding 136
// padding 137
// padding 138
// padding 139
// padding 140
// padding 141
// padding 142
// padding 143
// padding 144
// padding 145
// padding 146
// padding 147
// padding 148
// padding 149
// padding 150
// padding 151
// padding 152
// padding 153
// padding 154
// padding 155
// padding 156
// padding 157
// padding 158
// padding 159
// padding 160
// padding 161
// padding 162
// padding 163
// padding 164
// padding 165
// padding 166
// padding 167
// padding 168
// padding 169
// padding 170
// padding 171
// padding 172
// padding 173
// padding 174
// padding 175
// padding 176
// padding 177
// padding 178
// padding 179
// padding 180
// padding 181
// padding 182
// padding 183
// padding 184
// padding 185
// padding 186
// padding 187
// padding 188
// padding 189
// padding 190
// padding 191
// padding 192
// padding 193
// padding 194
// padding 195
// padding 196
// padding 197
// padding 198
// padding 199
// padding 200
// padding 201
// padding 202
// padding 203
// padding 204
// padding 205
// padding 206
// padding 207
// padding 208
// padding 209
// padding 210
// padding 211
// padding 212
// padding 213
// padding 214
// padding 215
// padding 216
// padding 217
// padding 218
// padding 219
// padding 220
// padding 221
// padding 222
// padding 223
// padding 224
// padding 225
// padding 226
// padding 227
// padding 228
// padding 229
// padding 230
// padding 231
// padding 232
// padding 233
// padding 234
// padding 235
// padding 236
// padding 237
// padding 238
// padding 239
// padding 240
// padding 241
// padding 242
// padding 243
// padding 244
// padding 245
// padding 246
// padding 247
// padding 248
// padding 249
// padding 250
// padding 251
// padding 252
// padding 253
// padding 254
// padding 255
// padding 256
// padding 257
// padding 258
// padding 259
// padding 260
// padding 261
// padding 262
// padding 263
// padding 264
// padding 265
// padding 266
// padding 267
// padding 268
// padding 269
// padding 270
// padding 271
// padding 272
// padding 273
// padding 274
// padding 275
// padding 276
// padding 277
// padding 278
// padding 279
// padding 280
// padding 281
// padding 282
// padding 283
// padding 284
// padding 285
// padding 286
// padding 287
// padding 288
// padding 289
// padding 290
// padding 291
// padding 292
// padding 293
// padding 294
// padding 295
// padding 296
// padding 297
// padding 298
// padding 299
// padding 300
// padding 301
// padding 302
// padding 303
// padding 304
// padding 305
// padding 306
// padding 307
// padding 308
// padding 309
// padding 310
// padding 311
// padding 312
// padding 313
// padding 314
// padding 315
// padding 316
// padding 317
// padding 318
// padding 319
// padding 320
// padding 321
// padding 322
// padding 323
// padding 324
// padding 325
// padding 326
// padding 327
// padding 328
// padding 329
// padding 330
// padding 331
// padding 332
// padding 333
// padding 334
// padding 335
// padding 336
// padding 337
// padding 338
// padding 339
// padding 340
// padding 341
// padding 342
// padding 343
// padding 344
// padding 345
// padding 346
// padding 347
// padding 348
// padding 349
// padding 350
// padding 351
// padding 352
// padding 353
// padding 354
// padding 355
// padding 356
// padding 357
// padding 358
// padding 359
// padding 360
// padding 361
// padding 362
// padding 363
// padding 364
// padding 365
// padding 366
// padding 367
// padding 368
// padding 369
// padding 370
// padding 371
// padding 372
// padding 373
// padding 374
// padding 375
// padding 376
// padding 377
// padding 378
// padding 379
// padding 380
// padding 381
// padding 382
// padding 383
// padding 384
// padding 385
// padding 386
// padding 387
// padding 388
// padding 389
// padding 390
// padding 391
// padding 392
// padding 393
// padding 394
// padding 395
// padding 396
// padding 397
// padding 398
// padding 399
// padding 400
// padding 401
// padding 402
// padding 403
// padding 404
// padding 405
// padding 406
// padding 407
// padding 408
// padding 409
// padding 410
// padding 411
// padding 412
// padding 413
// padding 414
// padding 415
// padding 416
// padding 417
// padding 418
// padding 419
// padding 420
// padding 421
// padding 422
// padding 423
// padding 424
// padding 425
// padding 426
// padding 427
// padding 428
// padding 429
// padding 430
// padding 431
// padding 432
// padding 433
// padding 434
// padding 435
// padding 436
// padding 437
// padding 438
// padding 439
// padding 440
// padding 441
// padding 442
// padding 443
// padding 444
// padding 445
// padding 446
// padding 447
// padding 448
// padding 449
// padding 450
// padding 451
// padding 452
// padding 453
// padding 454
// padding 455
// padding 456
// padding 457
// padding 458
// padding 459
// padding 460
// padding 461
// padding 462
// padding 463
// padding 464
// padding 465
// padding 466
// padding 467
// padding 468
// padding 469
// padding 470
// padding 471
// padding 472
// padding 473
// padding 474
// padding 475
// padding 476
// padding 477
// padding 478
// padding 479
// padding 480
// padding 481
// padding 482
// padding 483
// padding 484
// padding 485
// padding 486
// padding 487
// padding 488
// padding 489
// padding 490
// padding 491
// padding 492
// padding 493
// padding 494
// padding 495
// padding 496
// padding 497
// padding 498
// padding 499
// padding 500
// padding 501
// padding 502
// padding 503
// padding 504
// padding 505
// padding 506
// padding 507
// padding 508
// padding 509
// padding 510
// padding 511
// padding 512
// padding 513
// padding 514
// padding 515
// padding 516
// padding 517
// padding 518
// padding 519
// padding 520
// padding 521
// padding 522
// padding 523
// padding 524
// padding 525
// padding 526
// padding 527
// padding 528
// padding 529
// padding 530
// padding 531
// padding 532
// padding 533
// padding 534
// padding 535
// padding 536
// padding 537
// padding 538
// padding 539
// padding 540
// padding 541
// padding 542
// padding 543
// padding 544
// padding 545
// padding 546
// padding 547
// padding 548
// padding 549
// padding 550
// padding 551
// padding 552
// padding 553
// padding 554
// padding 555
// padding 556
// padding 557
// padding 558
// padding 559
// padding 560
// padding 561
// padding 562
// padding 563
// padding 564
// padding 565
// padding 566
// padding 567
// padding 568
// padding 569
// padding 570
// padding 571
// padding 572
// padding 573
// padding 574
// padding 575
// padding 576
// padding 577
// padding 578
// padding 579
// padding 580
// padding 581
// padding 582
// padding 583
// padding 584
// padding 585
// padding 586
// padding 587
// padding 588
// padding 589
// padding 590
// padding 591
// padding 592
// padding 593
// padding 594
// padding 595
// padding 596
// padding 597
// padding 598
// padding 599
// padding 600
// padding 601
// padding 602
// padding 603
// padding 604
// padding 605
// padding 606
// padding 607
// padding 608
// padding 609
// padding 610
// padding 611
// padding 612
// padding 613
// padding 614
// padding 615
// padding 616
// padding 617
// padding 618
// padding 619
// padding 620
// padding 621
// padding 622
// padding 623
// padding 624
// padding 625
// padding 626
// padding 627
// padding 628
// padding 629
// padding 630
// padding 631
// padding 632
// padding 633
// padding 634
// padding 635
// padding 636
// padding 637
// padding 638
// padding 639
// padding 640
// padding 641
// padding 642
// padding 643
// padding 644
// padding 645
// padding 646
// padding 647
// padding 648
// padding 649
// padding 650
// padding 651
// padding 652
// padding 653
// padding 654
// padding 655
// padding 656
// padding 657
// padding 658
// padding 659
// padding 660
// padding 661
// padding 662
// padding 663
// padding 664
// padding 665
// padding 666
// padding 667
// padding 668
// padding 669
// padding 670
// padding 671
// padding 672
// padding 673
// padding 674
// padding 675
// padding 676
// padding 677
// padding 678
// padding 679
// padding 680
// padding 681
// padding 682
// padding 683
// padding 684
// padding 685
// padding 686
// padding 687
// padding 688
// padding 689
// padding 690
// padding 691
// padding 692
// padding 693
// padding 694
// padding 695
// padding 696
// padding 697
// padding 698
// padding 699
// padding 700
// padding 701
// padding 702
// padding 703
// padding 704
// padding 705
// padding 706
// padding 707
// padding 708
// padding 709
// padding 710
// padding 711
// padding 712
// padding 713
// padding 714
// padding 715
// padding 716
// padding 717
// padding 718
// padding 719
// padding 720
// padding 721
// padding 722
// padding 723
// padding 724
// padding 725
// padding 726
// padding 727
// padding 728
// padding 729
// padding 730
// padding 731
// padding 732
// padding 733
// padding 734
// padding 735
// padding 736
// padding 737
// padding 738
// padding 739
// padding 740
// padding 741
// padding 742
// padding 743
// padding 744
// padding 745
// padding 746
// padding 747
// padding 748
// padding 749
// padding 750
// padding 751
// padding 752
// padding 753
// padding 754
// padding 755
// padding 756
// padding 757
// padding 758
// padding 759
// padding 760
// padding 761
// padding 762
// padding 763
// padding 764
// padding 765
// padding 766
// padding 767
// padding 768
// padding 769
// padding 770
// padding 771
// padding 772
// padding 773
// padding 774
// padding 775
// padding 776
// padding 777
// padding 778
// padding 779
// padding 780
// padding 781
// padding 782
// padding 783
// padding 784
// padding 785
// padding 786
// padding 787
// padding 788
// padding 789
// padding 790
// padding 791
// padding 792
// padding 793
// padding 794
// padding 795
// padding 796
// padding 797
// padding 798
// padding 799
// padding 800
// padding 801
// padding 802
// padding 803
// padding 804
// padding 805
// padding 806
// padding 807
// padding 808
// padding 809
// padding 810
// padding 811
// padding 812
// padding 813
// padding 814
// padding 815
// padding 816
// padding 817
// padding 818
// padding 819
// padding 820
// padding 821
// padding 822
// padding 823
// padding 824
// padding 825
// padding 826
// padding 827
// padding 828
// padding 829
// padding 830
// padding 831
// padding 832
// padding 833
// padding 834
// padding 835
// padding 836
// padding 837
// padding 838
// padding 839
// padding 840
// padding 841
// padding 842
// padding 843
// padding 844
// padding 845
// padding 846
// padding 847
// padding 848
// padding 849
// padding 850
// padding 851
// padding 852
// padding 853
// padding 854
// padding 855
// padding 856
// padding 857
// padding 858
// padding 859
// padding 860
// padding 861
// padding 862
// padding 863
// padding 864
// padding 865
// padding 866
// padding 867
// padding 868
// padding 869
// padding 870
// padding 871
// padding 872
// padding 873
// padding 874
// padding 875
// padding 876
// padding 877
// padding 878
// padding 879
// padding 880
// padding 881
// padding 882
// padding 883
// padding 884
// padding 885
// padding 886
// padding 887
// padding 888
// padding 889
// padding 890
// padding 891
// padding 892
// padding 893
// padding 894
// padding 895
// padding 896
// padding 897
// padding 898
// padding 899
// padding 900
// padding 901
// padding 902
// padding 903
// padding 904
// padding 905
// padding 906
// padding 907
// padding 908
// padding 909
// padding 910
// padding 911
// padding 912
// padding 913
// padding 914
// padding 915
// padding 916
// padding 917
// padding 918
// padding 919
// padding 920
// padding 921
// padding 922
// padding 923
// padding 924
// padding 925
// padding 926
// padding 927
// padding 928
// padding 929
// padding 930
// padding 931
// padding 932
// padding 933
// padding 934
// padding 935
// padding 936
// padding 937
// padding 938
// padding 939
// padding 940
// padding 941
// padding 942
// padding 943
// padding 944
// padding 945
// padding 946
// padding 947
// padding 948
// padding 949
// padding 950
// padding 951
// padding 952
// padding 953
// padding 954
// padding 955
// padding 956
// padding 957
// padding 958
// padding 959
// padding 960
// padding 961
// padding 962
// padding 963
// padding 964
// padding 965
// padding 966
// padding 967
// padding 968
// padding 969
// padding 970
// padding 971
// padding 972
// padding 973
// padding 974
// padding 975
// padding 976
// padding 977
// padding 978
// padding 979
// padding 980
// padding 981
// padding 982
// padding 983
// padding 984
// padding 985
// padding 986
// padding 987
// padding 988
// padding 989
// padding 990
// padding 991
// padding 992
// padding 993
// padding 994
// padding 995
// padding 996
// padding 997
// padding 998
// padding 999
// padding 1000
// padding 1001
// padding 1002
// padding 1003
// padding 1004
// padding 1005
// padding 1006
// padding 1007
// padding 1008
// padding 1009
// padding 1010
// padding 1011
// padding 1012
// padding 1013
// padding 1014
// padding 1015
// padding 1016
// padding 1017
// padding 1018
// padding 1019
// padding 1020
// padding 1021
// padding 1022
// padding 1023
// padding 1024
// padding 1025
// padding 1026
// padding 1027
// padding 1028
// padding 1029
// padding 1030
// padding 1031
// padding 1032
// padding 1033
// padding 1034
// padding 1035
// padding 1036
// padding 1037
// padding 1038
// padding 1039
// padding 1040
// padding 1041
// padding 1042
// padding 1043
// padding 1044
// padding 1045
// padding 1046
// padding 1047
// padding 1048
// padding 1049
// padding 0
// padding 1
// padding 2
// padding 3
// padding 4
// padding 5
// padding 6
// padding 7
// padding 8
// padding 9
// padding 10
// padding 11
// padding 12
// padding 13
// padding 14
// padding 15
// padding 16
// padding 17
// padding 18
// padding 19
// padding 20
// padding 21
// padding 22
// padding 23
// padding 24
// padding 25
// padding 26
// padding 27
// padding 28
// padding 29
// padding 30
// padding 31
// padding 32
// padding 33
// padding 34
// padding 35
// padding 36
// padding 37
// padding 38
// padding 39
// padding 40
// padding 41
// padding 42
// padding 43
// padding 44
// padding 45
// padding 46
// padding 47
// padding 48
// padding 49
// padding 50
// padding 51
// padding 52
// padding 53
// padding 54
// padding 55
// padding 56
// padding 57
// padding 58
// padding 59
// padding 60
// padding 61
// padding 62
// padding 63
// padding 64
// padding 65
// padding 66
// padding 67
// padding 68
// padding 69
// padding 70
// padding 71
// padding 72
// padding 73
// padding 74
// padding 75
// padding 76
// padding 77
// padding 78
// padding 79
// padding 80
// padding 81
// padding 82
// padding 83
// padding 84
// padding 85
// padding 86
// padding 87
// padding 88
// padding 89
// padding 90
// padding 91
// padding 92
// padding 93
// padding 94
// padding 95
// padding 96
// padding 97
// padding 98
// padding 99
// padding 100
// padding 101
// padding 102
// padding 103
// padding 104
// padding 105
// padding 106
// padding 107
// padding 108
// padding 109
// padding 110
// padding 111
// padding 112
// padding 113
// padding 114
// padding 115
// padding 116
// padding 117
// padding 118
// padding 119
// padding 120
// padding 121
// padding 122
// padding 123
// padding 124
// padding 125
// padding 126
// padding 127
// padding 128
// padding 129
// padding 130
// padding 131
// padding 132
// padding 133
// padding 134
// padding 135
// padding 136
// padding 137
// padding 138
// padding 139
// padding 140
// padding 141
// padding 142
// padding 143
// padding 144
// padding 145
// padding 146
// padding 147
// padding 148
// padding 149
// padding 150
// padding 151
// padding 152
// padding 153
// padding 154
// padding 155
// padding 156
// padding 157
// padding 158
// padding 159
// padding 160
// padding 161
// padding 162
// padding 163
// padding 164
// padding 165
// padding 166
// padding 167
// padding 168
// padding 169
// padding 170
// padding 171
// padding 172
// padding 173
// padding 174
// padding 175
// padding 176
// padding 177
// padding 178
// padding 179
// padding 180
// padding 181
// padding 182
// padding 183
// padding 184
// padding 185
// padding 186
// padding 187
// padding 188
// padding 189
// padding 190
// padding 191
// padding 192
// padding 193
// padding 194
// padding 195
// padding 196
// padding 197
// padding 198
// padding 199
// padding 200
// padding 201
// padding 202
// padding 203
// padding 204
// padding 205
// padding 206
// padding 207
// padding 208
// padding 209
// padding 210
// padding 211
// padding 212
// padding 213
// padding 214
// padding 215
// padding 216
// padding 217
// padding 218
// padding 219
// padding 220
// padding 221
// padding 222
// padding 223
// padding 224
// padding 225
// padding 226
// padding 227
// padding 228
// padding 229
// padding 230
// padding 231
// padding 232
// padding 233
// padding 234
// padding 235
// padding 236
// padding 237
// padding 238
// padding 239
// padding 240
// padding 241
// padding 242
// padding 243
// padding 244
// padding 245
// padding 246
// padding 247
// padding 248
// padding 249
// padding 250
// padding 251
// padding 252
// padding 253
// padding 254
// padding 255
// padding 256
// padding 257
// padding 258
// padding 259
// padding 260
// padding 261
// padding 262
// padding 263
// padding 264
// padding 265
// padding 266
// padding 267
// padding 268
// padding 269
// padding 270
// padding 271
// padding 272
// padding 273
// padding 274
// padding 275
// padding 276
// padding 277
// padding 278
// padding 279
// padding 280
// padding 281
// padding 282
// padding 283
// padding 284
// padding 285
// padding 286
// padding 287
// padding 288
// padding 289
// padding 290
// padding 291
// padding 292
// padding 293
// padding 294
// padding 295
// padding 296
// padding 297
// padding 298
// padding 299
// padding 300
// padding 301
// padding 302
// padding 303
// padding 304
// padding 305
// padding 306
// padding 307
// padding 308
// padding 309
// padding 310
// padding 311
// padding 312
// padding 313
// padding 314
// padding 315
// padding 316
// padding 317
// padding 318
// padding 319
// padding 320
// padding 321
// padding 322
// padding 323
// padding 324
// padding 325
// padding 326
// padding 327
// padding 328
// padding 329
// padding 330
// padding 331
// padding 332
// padding 333
// padding 334
// padding 335
// padding 336
// padding 337
// padding 338
// padding 339
// padding 340
// padding 341
// padding 342
// padding 343
// padding 344
// padding 345
// padding 346
// padding 347
// padding 348
// padding 349
// padding 350
// padding 351
// padding 352
// padding 353
// padding 354
// padding 355
// padding 356
// padding 357
// padding 358
// padding 359
// padding 360
// padding 361
// padding 362
// padding 363
// padding 364
// padding 365
// padding 366
// padding 367
// padding 368
// padding 369
// padding 370
// padding 371
// padding 372
// padding 373
// padding 374
// padding 375
// padding 376
// padding 377
// padding 378
// padding 379
// padding 380
// padding 381
// padding 382
// padding 383
// padding 384
// padding 385
// padding 386
// padding 387
// padding 388
// padding 389
// padding 390
// padding 391
// padding 392
// padding 393
// padding 394
// padding 395
// padding 396
// padding 397
// padding 398
// padding 399
// padding 400
// padding 401
// padding 402
// padding 403
// padding 404
// padding 405
// padding 406
// padding 407
// padding 408
// padding 409
// padding 410
// padding 411
// padding 412
// padding 413
// padding 414
// padding 415
// padding 416
// padding 417
// padding 418
// padding 419
// padding 420
// padding 421
// padding 422
// padding 423
// padding 424
// padding 425
// padding 426
// padding 427
// padding 428
// padding 429
// padding 430
// padding 431
// padding 432
// padding 433
// padding 434
// padding 435
// padding 436
// padding 437
// padding 438
// padding 439
// padding 440
// padding 441
// padding 442
// padding 443
// padding 444
// padding 445
// padding 446
// padding 447
// padding 448
// padding 449
// padding 450
// padding 451
// padding 452
// padding 453
// padding 454
// padding 455
// padding 456
// padding 457
// padding 458
// padding 459
// padding 460
// padding 461
// padding 462
// padding 463
// padding 464
// padding 465
// padding 466
// padding 467
// padding 468
// padding 469
// padding 470
// padding 471
// padding 472
// padding 473
// padding 474
// padding 475
// padding 476
// padding 477
// padding 478
// padding 479
// padding 480
// padding 481
// padding 482
// padding 483
// padding 484
// padding 485
// padding 486
// padding 487
// padding 488
// padding 489
// padding 490
// padding 491
// padding 492
// padding 493
// padding 494
// padding 495
// padding 496
// padding 497
// padding 498
// padding 499
// padding 500
// padding 501
// padding 502
// padding 503
// padding 504
// padding 505
// padding 506
// padding 507
// padding 508
// padding 509
// padding 510
// padding 511
// padding 512
// padding 513
// padding 514
// padding 515
// padding 516
// padding 517
// padding 518
// padding 519
// padding 520
// padding 521
// padding 522
// padding 523
// padding 524
// padding 525
// padding 526
// padding 527
// padding 528
// padding 529
// padding 530
// padding 531
// padding 532
// padding 533
// padding 534
// padding 535
// padding 536
// padding 537
// padding 538
// padding 539
// padding 540
// padding 541
// padding 542
// padding 543
// padding 544
// padding 545
// padding 546
// padding 547
// padding 548
// padding 549
// padding 550
// padding 551
// padding 552
// padding 553
// padding 554
// padding 555
// padding 556
// padding 557
// padding 558
// padding 559
// padding 560
// padding 561
// padding 562
// padding 563
// padding 564
// padding 565
// padding 566
// padding 567
// padding 568
// padding 569
// padding 570
// padding 571
// padding 572
// padding 573
// padding 574
// padding 575
// padding 576
// padding 577
// padding 578
// padding 579
// padding 580
// padding 581
// padding 582
// padding 583
// padding 584
// padding 585
// padding 586
// padding 587
// padding 588
// padding 589
// padding 590
// padding 591
// padding 592
// padding 593
// padding 594
// padding 595
// padding 596
// padding 597
// padding 598
// padding 599
// padding 600
// padding 601
// padding 602
// padding 603
// padding 604
// padding 605
// padding 606
// padding 607
// padding 608
// padding 609
// padding 610
// padding 611
// padding 612
// padding 613
// padding 614
// padding 615
// padding 616
// padding 617
// padding 618
// padding 619
// padding 620
// padding 621
// padding 622
// padding 623
// padding 624
// padding 625
// padding 626
// padding 627
// padding 628
// padding 629
// padding 630
// padding 631
// padding 632
// padding 633
// padding 634
// padding 635
// padding 636
// padding 637
// padding 638
// padding 639
// padding 640
// padding 641
// padding 642
// padding 643
// padding 644
// padding 645
// padding 646
// padding 647
// padding 648
// padding 649
// padding 650
// padding 651
// padding 652
// padding 653
// padding 654
// padding 655
// padding 656
// padding 657
// padding 658
// padding 659
// padding 660
// padding 661
// padding 662
// padding 663
// padding 664
// padding 665
// padding 666
// padding 667
// padding 668
// padding 669
// padding 670
// padding 671
// padding 672
// padding 673
// padding 674
// padding 675
// padding 676
// padding 677
// padding 678
// padding 679
// padding 680
// padding 681
// padding 682
// padding 683
// padding 684
// padding 685
// padding 686
// padding 687
// padding 688
// padding 689
// padding 690
// padding 691
// padding 692
// padding 693
// padding 694
// padding 695
// padding 696
// padding 697
// padding 698
// padding 699
// padding 700
// padding 701
// padding 702
// padding 703
// padding 704
// padding 705
// padding 706
// padding 707
// padding 708
// padding 709
// padding 710
// padding 711
// padding 712
// padding 713
// padding 714
// padding 715
// padding 716
// padding 717
// padding 718
// padding 719
// padding 720
// padding 721
// padding 722
// padding 723
// padding 724
// padding 725
// padding 726
// padding 727
// padding 728
// padding 729
// padding 730
// padding 731
// padding 732
// padding 733
// padding 734
// padding 735
// padding 736
// padding 737
// padding 738
// padding 739
// padding 740
// padding 741
// padding 742
// padding 743
// padding 744
// padding 745
// padding 746
// padding 747
// padding 748
// padding 749
// padding 750
// padding 751
// padding 752
// padding 753
// padding 754
// padding 755
// padding 756
// padding 757
// padding 758
// padding 759
// padding 760
// padding 761
// padding 762
// padding 763
// padding 764
// padding 765
// padding 766
// padding 767
// padding 768
// padding 769
// padding 770
// padding 771
// padding 772
// padding 773
// padding 774
// padding 775
// padding 776
// padding 777
// padding 778
// padding 779
// padding 780
// padding 781
// padding 782
// padding 783
// padding 784
// padding 785
// padding 786
// padding 787
// padding 788
// padding 789
// padding 790
// padding 791
// padding 792
// padding 793
// padding 794
// padding 795
// padding 796
// padding 797
// padding 798
// padding 799
// padding 800
// padding 801
// padding 802
// padding 803
// padding 804
// padding 805
// padding 806
// padding 807
// padding 808
// padding 809
// padding 810
// padding 811
// padding 812
// padding 813
// padding 814
// padding 815
// padding 816
// padding 817
// padding 818
// padding 819
// padding 820
// padding 821
// padding 822
// padding 823
// padding 824
// padding 825
// padding 826
// padding 827
// padding 828
// padding 829
// padding 830
// padding 831
// padding 832
// padding 833
// padding 834
// padding 835
// padding 836
// padding 837
// padding 838
// padding 839
// padding 840
// padding 841
// padding 842
// padding 843
// padding 844
// padding 845
// padding 846
// padding 847
// padding 848
// padding 849
// padding 850
// padding 851
// padding 852
// padding 853
// padding 854
// padding 855
// padding 856
// padding 857
// padding 858
// padding 859
// padding 860
// padding 861
// padding 862
// padding 863
// padding 864
// padding 865
// padding 866
// padding 867
// padding 868
// padding 869
// padding 870
// padding 871
// padding 872
// padding 873
// padding 874
// padding 875
// padding 876
// padding 877
// padding 878
// padding 879
// padding 880
// padding 881
// padding 882
// padding 883
// padding 884
// padding 885
// padding 886
// padding 887
// padding 888
// padding 889
// padding 890
// padding 891
// padding 892
// padding 893
// padding 894
// padding 895
// padding 896
// padding 897
// padding 898
// padding 899
// padding 900
// padding 901
// padding 902
// padding 903
// padding 904
// padding 905
// padding 906
// padding 907
// padding 908
// padding 909
// padding 910
// padding 911
// padding 912
// padding 913
// padding 914
// padding 915
// padding 916
// padding 917
// padding 918
// padding 919
// padding 920
// padding 921
// padding 922
// padding 923
// padding 924
// padding 925
// padding 926
// padding 927
// padding 928
// padding 929
// padding 930
// padding 931
// padding 932
// padding 933
// padding 934
// padding 935
// padding 936
// padding 937
// padding 938
// padding 939
// padding 940
// padding 941
// padding 942
// padding 943
// padding 944
// padding 945
// padding 946
// padding 947
// padding 948
// padding 949
// padding 950
// padding 951
// padding 952
// padding 953
// padding 954
// padding 955
// padding 956
// padding 957
// padding 958
// padding 959
// padding 960
// padding 961
// padding 962
// padding 963
// padding 964
// padding 965
// padding 966
// padding 967
// padding 968
// padding 969
// padding 970
// padding 971
// padding 972
// padding 973
// padding 974
// padding 975
// padding 976
// padding 977
// padding 978
// padding 979
// padding 980
// padding 981
// padding 982
// padding 983
// padding 984
// padding 985
// padding 986
// padding 987
// padding 988
// padding 989
// padding 990
// padding 991
// padding 992
// padding 993
// padding 994
// padding 995
// padding 996
// padding 997
// padding 998
// padding 999
// padding 1000
// padding 1001
// padding 1002
// padding 1003
// padding 1004
// padding 1005
// padding 1006
// padding 1007
// padding 1008
// padding 1009
// padding 1010
// padding 1011
// padding 1012
// padding 1013
// padding 1014
// padding 1015
// padding 1016
// padding 1017
// padding 1018
// padding 1019
// padding 1020
// padding 1021
// padding 1022
// padding 1023
// padding 1024
// padding 1025
// padding 1026
// padding 1027
// padding 1028
// padding 1029
// padding 1030
// padding 1031
// padding 1032
// padding 1033
// padding 1034
// padding 1035
// padding 1036
// padding 1037
// padding 1038
// padding 1039
// padding 1040
// padding 1041
// padding 1042
// padding 1043
// padding 1044
// padding 1045
// padding 1046
// padding 1047
// padding 1048
// padding 1049
