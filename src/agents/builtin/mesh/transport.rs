use redis::AsyncCommands;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::broadcast;
use dashmap::DashMap;

pub use crate::proto::hub::TeammateMeshEvent as Message;

#[async_trait]
pub trait MeshTransport: Send + Sync {
    async fn publish(&self, topic: &str, message: Message) -> Result<(), String>;
    async fn subscribe(&self, topic: &str, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String>;

    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String>;
    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String>;

    async fn register_presence(&self, agent_id: &str, status: &str, ttl_seconds: u64) -> Result<(), String>;
    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String>;
}

pub struct MemoryTransport {
    subs: DashMap<String, broadcast::Sender<Message>>,
    presence: DashMap<String, (String, std::time::Instant)>, // agent_id -> (status, expires_at)
    locks: DashMap<String, (String, std::time::Instant)>, // resource -> (owner, expires_at)
}

impl MemoryTransport {
    pub fn new() -> Self {
        MemoryTransport {
            subs: DashMap::new(),
            presence: DashMap::new(),
            locks: DashMap::new(),
        }
    }
}

#[async_trait]
impl MeshTransport for MemoryTransport {
    async fn publish(&self, topic: &str, message: Message) -> Result<(), String> {
        if let Some(tx) = self.subs.get(topic) {
            let _ = tx.send(message);
        }
        Ok(())
    }

    async fn subscribe(&self, topic: &str, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let tx = self.subs.entry(topic.to_string()).or_insert_with(|| {
            let (tx, _) = broadcast::channel(100);
            tx
        }).clone();

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

    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String> {
        let now = std::time::Instant::now();

        // Remove expired locks
        let expired_keys: Vec<String> = self.locks.iter()
            .filter(|entry| entry.value().1 <= now)
            .map(|entry| entry.key().clone())
            .collect();

        for key in expired_keys {
            self.locks.remove(&key);
        }

        let expires_at = now + std::time::Duration::from_secs(ttl_seconds);
        use dashmap::mapref::entry::Entry;
        match self.locks.entry(resource.to_string()) {
            Entry::Vacant(e) => {
                e.insert((owner.to_string(), expires_at));
                Ok(true)
            }
            Entry::Occupied(_) => {
                Ok(false)
            }
        }
    }

    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
        self.locks.remove_if(resource, |_, (lock_owner, _)| lock_owner == owner);
        Ok(())
    }

    async fn register_presence(&self, agent_id: &str, status: &str, ttl_seconds: u64) -> Result<(), String> {
        let expires_at = std::time::Instant::now() + std::time::Duration::from_secs(ttl_seconds);
        self.presence.insert(agent_id.to_string(), (status.to_string(), expires_at));
        Ok(())
    }

    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
        let now = std::time::Instant::now();

        // Remove expired
        let expired_keys: Vec<String> = self.presence.iter()
            .filter(|entry| entry.value().1 <= now)
            .map(|entry| entry.key().clone())
            .collect();

        for key in expired_keys {
            self.presence.remove(&key);
        }

        let agents = self.presence.iter()
            .map(|entry| (entry.key().clone(), entry.value().0.clone()))
            .collect();

        Ok(agents)
    }
}


#[derive(Clone)]
pub struct IpcTransport {
    pool: sqlx::SqlitePool,
    subs: DashMap<String, broadcast::Sender<Message>>,
}

impl IpcTransport {
    pub async fn new(db_url: &str) -> Result<Self, String> {
        use sqlx::sqlite::{SqlitePoolOptions, SqliteConnectOptions};
        let options: SqliteConnectOptions = db_url.parse().map_err(|e| format!("Invalid db url: {}", e))?;
        let options = options.create_if_missing(true);
        let pool = SqlitePoolOptions::new().connect_with(options).await.map_err(|e| e.to_string())?;

        // Initialize schema
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS mesh_messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                topic TEXT NOT NULL,
                payload BLOB NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                msg_id TEXT
            )"
        ).execute(&pool).await.map_err(|e| e.to_string())?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS mesh_checkpoints (
                subscriber_id TEXT PRIMARY KEY,
                last_id INTEGER NOT NULL
            )"
        ).execute(&pool).await.map_err(|e| e.to_string())?;

        // Attempt to add the column, ignoring error if it already exists (e.g. duplicate column name)
        match sqlx::query("ALTER TABLE mesh_messages ADD COLUMN msg_id TEXT").execute(&pool).await {
            Ok(_) => {},
            Err(e) => {
                let err_str = e.to_string();
                if !err_str.contains("duplicate column name") {
                    return Err(format!("Failed to migrate mesh_messages: {}", err_str));
                }
            }
        }

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS mesh_locks (
                resource TEXT PRIMARY KEY,
                owner TEXT NOT NULL,
                expires_at DATETIME NOT NULL
            )"
        ).execute(&pool).await.map_err(|e| e.to_string())?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS mesh_presence (
                agent_id TEXT PRIMARY KEY,
                status TEXT NOT NULL,
                expires_at DATETIME NOT NULL
            )"
        ).execute(&pool).await.map_err(|e| e.to_string())?;

        let subs = DashMap::new();

        Ok(IpcTransport { pool, subs })
    }

    pub async fn start_worker(&self) {
        use prost::Message as ProstMessage;
        let pool = self.pool.clone();
        let subs = self.subs.clone();

        let subscriber_id = "builtin_agent_node".to_string();
        let mut last_id: i64 = sqlx::query_scalar("SELECT last_id FROM mesh_checkpoints WHERE subscriber_id = ?")
            .bind(&subscriber_id)
            .fetch_optional(&pool)
            .await
            .unwrap_or(Some(0))
            .unwrap_or(0);

        loop {
            // Poll for new messages
            let rows: Result<Vec<(i64, String, Vec<u8>)>, _> = sqlx::query_as(
                "SELECT id, topic, payload FROM mesh_messages WHERE id > ? ORDER BY id ASC"
            )
            .bind(last_id)
            .fetch_all(&pool)
            .await;

            if let Ok(rows) = rows {
                let has_rows = !rows.is_empty();
                for (id, topic, payload) in rows {
                    last_id = id;
                    if let Some(tx) = subs.get(&topic) {
                        if let Ok(message) = Message::decode(&payload[..]) {
                            let _ = tx.send(message);
                        }
                    }
                }

                if has_rows {
                    let _ = sqlx::query("INSERT INTO mesh_checkpoints (subscriber_id, last_id) VALUES (?, ?) ON CONFLICT(subscriber_id) DO UPDATE SET last_id = excluded.last_id")
                        .bind(&subscriber_id)
                        .bind(last_id)
                        .execute(&pool)
                        .await;
                }
            }

            // Cleanup old messages (keep last 1 hour)
            let _ = sqlx::query("DELETE FROM mesh_messages WHERE created_at < datetime('now', '-1 hour')")
                .execute(&pool)
                .await;

            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
    }
}

#[async_trait]
impl MeshTransport for IpcTransport {
    async fn publish(&self, topic: &str, message: Message) -> Result<(), String> {
        use prost::Message as ProstMessage;
        let mut buf = Vec::new();
        message.encode(&mut buf).unwrap();

        let msg_id = if message.msg_id.is_empty() {
            None
        } else {
            Some(message.msg_id.clone())
        };

        sqlx::query("INSERT INTO mesh_messages (topic, payload, msg_id) VALUES (?, ?, ?)")
            .bind(topic)
            .bind(buf)
            .bind(msg_id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        // Deliver to local subscribers without polling delay
        if let Some(tx) = self.subs.get(topic) {
            let _ = tx.send(message);
        }

        Ok(())
    }

    async fn subscribe(&self, topic: &str, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let tx = self.subs.entry(topic.to_string()).or_insert_with(|| {
            let (tx, _) = broadcast::channel(100);
            tx
        }).clone();

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

    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String> {
        // Cleanup expired locks
        let _ = sqlx::query("DELETE FROM mesh_locks WHERE expires_at <= datetime('now')")
            .execute(&self.pool)
            .await;

        let result = sqlx::query(
            "INSERT INTO mesh_locks (resource, owner, expires_at) VALUES (?, ?, datetime('now', ?))
             ON CONFLICT(resource) DO UPDATE SET owner = excluded.owner, expires_at = excluded.expires_at WHERE expires_at <= datetime('now')"
        )
        .bind(resource)
        .bind(owner)
        .bind(format!("+{} seconds", ttl_seconds))
        .execute(&self.pool)
        .await;

        match result {
            Ok(res) => Ok(res.rows_affected() > 0),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
        sqlx::query("DELETE FROM mesh_locks WHERE resource = ? AND owner = ?")
            .bind(resource)
            .bind(owner)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn register_presence(&self, agent_id: &str, status: &str, ttl_seconds: u64) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO mesh_presence (agent_id, status, expires_at) VALUES (?, ?, datetime('now', ?))
             ON CONFLICT(agent_id) DO UPDATE SET status = excluded.status, expires_at = excluded.expires_at"
        )
        .bind(agent_id)
        .bind(status)
        .bind(format!("+{} seconds", ttl_seconds))
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
        let _ = sqlx::query("DELETE FROM mesh_presence WHERE expires_at <= datetime('now')")
            .execute(&self.pool)
            .await;

        let rows: Result<Vec<(String, String)>, _> = sqlx::query_as(
            "SELECT agent_id, status FROM mesh_presence"
        )
        .fetch_all(&self.pool)
        .await;

        match rows {
            Ok(r) => Ok(r),
            Err(e) => Err(e.to_string()),
        }
    }
}

pub struct RedisTransport {

    client: redis::Client,
    publish_conn: tokio::sync::Mutex<redis::aio::MultiplexedConnection>,
}

impl RedisTransport {
    pub async fn new(redis_url: &str) -> Result<Self, String> {
        let client = redis::Client::open(redis_url).map_err(|e| e.to_string())?;
        let publish_conn = client.get_multiplexed_tokio_connection().await.map_err(|e| e.to_string())?;

        Ok(RedisTransport {
            client,
            publish_conn: tokio::sync::Mutex::new(publish_conn),
        })
    }
}

#[async_trait]
impl MeshTransport for RedisTransport {
    async fn publish(&self, topic: &str, message: Message) -> Result<(), String> {
        use prost::Message as ProstMessage;

        let mut conn = self.publish_conn.lock().await;

        let mut buf = Vec::new();
        message.encode(&mut buf).unwrap();

        let _: () = conn.publish(topic, buf).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn subscribe(&self, topic: &str, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        use prost::Message as ProstMessage;
        use futures_util::StreamExt;

        let mut pubsub = self.client.get_async_pubsub().await.map_err(|e| e.to_string())?;

        pubsub.subscribe(topic).await.map_err(|e| e.to_string())?;
        let mut stream = pubsub.into_on_message();

        let worker = tokio::spawn(async move {
            while let Some(msg) = stream.next().await {
                if let Ok(buf) = msg.get_payload::<Vec<u8>>() {
                    if let Ok(message) = Message::decode(&buf[..]) {
                        handler(message);
                    }
                }
            }
        });

        let cancel = Box::new(move || {
            worker.abort();
        });

        Ok(cancel)
    }

    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String> {
        let mut conn = self.publish_conn.lock().await;
        let key = format!("lock:{}", resource);
        let res: Option<String> = redis::cmd("SET")
            .arg(&key)
            .arg(owner)
            .arg("NX")
            .arg("EX")
            .arg(ttl_seconds)
            .query_async(&mut *conn)
            .await
            .map_err(|e| e.to_string())?;

        Ok(res.is_some())
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

    async fn register_presence(&self, agent_id: &str, status: &str, ttl_seconds: u64) -> Result<(), String> {
        let mut conn = self.publish_conn.lock().await;
        let key = format!("presence:{}", agent_id);
        let _: () = redis::cmd("SET")
            .arg(&key)
            .arg(status)
            .arg("EX")
            .arg(ttl_seconds)
            .query_async(&mut *conn)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
        let mut conn = self.publish_conn.lock().await;
        let keys: Vec<String> = redis::cmd("KEYS")
            .arg("presence:*")
            .query_async(&mut *conn)
            .await
            .map_err(|e| e.to_string())?;

        let mut active = Vec::new();
        for key in keys {
            let status: Option<String> = redis::cmd("GET").arg(&key).query_async(&mut *conn).await.map_err(|e| e.to_string())?;
            if let Some(s) = status {
                let agent_id = key.strip_prefix("presence:").unwrap_or(&key).to_string();
                active.push((agent_id, s));
            }
        }
        Ok(active)
    }
}

pub struct NatsTransport {
    client: async_nats::Client,
    memory_fallback: Arc<MemoryTransport>,
}

impl NatsTransport {
    pub async fn new(url: &str) -> Result<Self, String> {
        let client = async_nats::connect(url).await.map_err(|e| e.to_string())?;
        Ok(Self {
            client,
            memory_fallback: Arc::new(MemoryTransport::new()),
        })
    }
}

#[async_trait]
impl MeshTransport for NatsTransport {
    async fn publish(&self, topic: &str, message: Message) -> Result<(), String> {
        use prost::Message as ProstMessage;
        let mut buf = Vec::new();
        message.encode(&mut buf).map_err(|e| e.to_string())?;
        self.client.publish(topic.to_string(), buf.into()).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn subscribe(&self, topic: &str, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        use prost::Message as ProstMessage;
        use futures::StreamExt;

        let mut subscriber = self.client.subscribe(topic.to_string()).await.map_err(|e| e.to_string())?;

        let worker = tokio::spawn(async move {
            while let Some(msg) = subscriber.next().await {
                if let Ok(decoded) = Message::decode(&msg.payload[..]) {
                    handler(decoded);
                }
            }
        });

        Ok(Box::new(move || {
            worker.abort();
        }))
    }

    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String> {
        self.memory_fallback.acquire_lock(resource, owner, ttl_seconds).await
    }

    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
        self.memory_fallback.release_lock(resource, owner).await
    }

    async fn register_presence(&self, agent_id: &str, status: &str, ttl_seconds: u64) -> Result<(), String> {
        self.memory_fallback.register_presence(agent_id, status, ttl_seconds).await
    }

    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
        self.memory_fallback.get_active_agents().await
    }
}


pub async fn create_transport(redis_url: Option<&str>, is_cloud: bool) -> Result<Arc<dyn MeshTransport>, String> {
    if let Ok(nats_url) = std::env::var("NATS_URL") {
        match NatsTransport::new(&nats_url).await {
            Ok(t) => {
                println!("Initialized NatsTransport");
                return Ok(Arc::new(t));
            },
            Err(e) => {
                println!("Failed to initialize NatsTransport: {}. Falling back to default transport.", e);
            }
        }
    }

    if is_cloud {
        if let Some(url) = redis_url {
            match RedisTransport::new(url).await {
                Ok(t) => {
                    println!("Initialized RedisTransport");
                    return Ok(Arc::new(t));
                },
                Err(e) => {
                    return Err(format!("Failed to initialize RedisTransport in cloud mode: {}", e));
                }
            }
        } else {
            return Err("Redis URL is required in cloud mode".to_string());
        }
    }

    // Standalone fallback
    if let Ok(db_url) = std::env::var("DATABASE_URL") {
        if db_url.starts_with("sqlite") {
            match IpcTransport::new(&db_url).await {
                Ok(t) => {
                    let t_clone = t.clone();
                    tokio::spawn(async move { t_clone.start_worker().await; });
                    println!("Initialized IpcTransport (Standalone)");
                    return Ok(Arc::new(t));
                },
                Err(e) => {
                    println!("Failed to initialize IpcTransport (Standalone): {}. Falling back to MemoryTransport.", e);
                }
            }
        }
    }

    if let Some(url) = redis_url {
        match RedisTransport::new(url).await {
            Ok(t) => {
                println!("Initialized RedisTransport (Standalone)");
                return Ok(Arc::new(t));
            },
            Err(e) => {
                println!("Failed to initialize RedisTransport (Standalone): {}. Falling back to MemoryTransport.", e);
            }
        }
    }

    println!("Initialized MemoryTransport");
    Ok(Arc::new(MemoryTransport::new()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[tokio::test]
    async fn test_ipc_transport() {
        let tmp_dir = std::env::var("TEST_TMPDIR").unwrap_or_else(|_| "/tmp".to_string());
        let db_path = format!("{}/test_ipc_{}.sqlite", tmp_dir, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());
        let db_url = format!("sqlite://{}", db_path);

        let transport = IpcTransport::new(&db_url).await.unwrap();

        let t_clone = transport.clone();
        tokio::spawn(async move { t_clone.start_worker().await; });

        let received = Arc::new(AtomicBool::new(false));
        let received_clone = received.clone();

        let handler = Box::new(move |msg: Message| {
            if msg.action == "ipc_test_topic" && msg.payload == b"ipc_hello" {
                received_clone.store(true, Ordering::SeqCst);
            }
        });

        let cancel = transport.subscribe("ipc_test_topic", handler).await.unwrap();
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        let msg = Message {
            agent_id: "test".to_string(),
            action: "ipc_test_topic".to_string(),
            status: "ok".to_string(),
            payload: b"ipc_hello".to_vec(),
            msg_id: uuid::Uuid::new_v4().to_string(),
        };

        transport.publish("ipc_test_topic", msg).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        assert!(received.load(Ordering::SeqCst));
        cancel();
    }

    #[tokio::test]
    async fn test_ipc_transport_checkpoints() {
        let tmp_dir = std::env::var("TEST_TMPDIR").unwrap_or_else(|_| "/tmp".to_string());
        let db_path = format!("{}/test_ipc_checkpoints_{}.sqlite", tmp_dir, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());
        let db_url = format!("sqlite://{}", db_path);

        let transport = IpcTransport::new(&db_url).await.unwrap();

        let msg = Message {
            agent_id: "test".to_string(),
            action: "ipc_checkpoint_topic".to_string(),
            status: "ok".to_string(),
            payload: b"ipc_checkpoint".to_vec(),
            msg_id: uuid::Uuid::new_v4().to_string(),
        };

        transport.publish("ipc_checkpoint_topic", msg).await.unwrap();

        let t_clone = transport.clone();
        tokio::spawn(async move { t_clone.start_worker().await; });

        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;

        let subscriber_id = "builtin_agent_node".to_string();
        let last_id: i64 = sqlx::query_scalar("SELECT last_id FROM mesh_checkpoints WHERE subscriber_id = ?")
            .bind(&subscriber_id)
            .fetch_one(&transport.pool)
            .await.unwrap();

        assert!(last_id > 0);
    }

    #[tokio::test]
    async fn test_ipc_transport_locking() {
        let tmp_dir = std::env::var("TEST_TMPDIR").unwrap_or_else(|_| "/tmp".to_string());
        let db_path = format!("{}/test_ipc_locks_{}.sqlite", tmp_dir, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());
        let db_url = format!("sqlite://{}", db_path);

        let transport = IpcTransport::new(&db_url).await.unwrap();

        let t_clone = transport.clone();
        tokio::spawn(async move { t_clone.start_worker().await; });

        let acquired = transport.acquire_lock("ipc_resource", "agent_1", 1).await.unwrap();
        assert!(acquired);

        let acquired_again = transport.acquire_lock("ipc_resource", "agent_2", 1).await.unwrap();
        assert!(!acquired_again);

        transport.release_lock("ipc_resource", "agent_1").await.unwrap();

        let acquired_after_release = transport.acquire_lock("ipc_resource", "agent_2", 1).await.unwrap();
        assert!(acquired_after_release);
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
        let acquired = transport.acquire_lock("my_resource", "agent_1", 10).await.unwrap();
        assert!(acquired);

        // Test mutual exclusion
        let acquired_again = transport.acquire_lock("my_resource", "agent_2", 10).await.unwrap();
        assert!(!acquired_again);

        // Test lock release
        transport.release_lock("my_resource", "agent_1").await.unwrap();

        // Test lock acquisition after release
        let acquired_after_release = transport.acquire_lock("my_resource", "agent_2", 10).await.unwrap();
        assert!(acquired_after_release);
    }

    #[tokio::test]
    async fn test_memory_transport_lock_expiration() {
        let transport = MemoryTransport::new();

        // Acquire lock with short TTL (1 second)
        let acquired = transport.acquire_lock("expiring_resource", "agent_1", 1).await.unwrap();
        assert!(acquired);

        // Sleep for 2 seconds to let lock expire
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Second agent should be able to acquire lock now
        let acquired_after_expiration = transport.acquire_lock("expiring_resource", "agent_2", 10).await.unwrap();
        assert!(acquired_after_expiration);
    }

    #[tokio::test]
    async fn test_memory_transport_presence() {
        let transport = MemoryTransport::new();

        // Register presence
        transport.register_presence("agent_1", "online", 10).await.unwrap();
        transport.register_presence("agent_2", "busy", 1).await.unwrap();

        // Get active agents
        let mut active_agents = transport.get_active_agents().await.unwrap();
        active_agents.sort();

        assert_eq!(active_agents.len(), 2);
        assert_eq!(active_agents[0], ("agent_1".to_string(), "online".to_string()));
        assert_eq!(active_agents[1], ("agent_2".to_string(), "busy".to_string()));

        // Wait for agent_2 presence to expire
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Get active agents again
        let active_agents_after_expiration = transport.get_active_agents().await.unwrap();
        assert_eq!(active_agents_after_expiration.len(), 1);
        assert_eq!(active_agents_after_expiration[0], ("agent_1".to_string(), "online".to_string()));
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

        let cancel = transport.subscribe("test_topic_redis", handler).await.unwrap();

        // Wait for subscription to propagate
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let msg = Message {
            agent_id: "test".to_string(),
            action: "test_topic_redis".to_string(),
            status: "ok".to_string(),
            payload: b"hello redis".to_vec(),
            msg_id: uuid::Uuid::new_v4().to_string(),
        };

        transport.publish("test_topic_redis", msg.clone()).await.unwrap();

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
