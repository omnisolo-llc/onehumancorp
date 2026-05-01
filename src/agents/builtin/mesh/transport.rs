use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::broadcast;
use dashmap::DashMap;
use std::time::Instant;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize, prost::Message)]
pub struct Message {
    #[prost(string, tag = "1")]
    pub topic: String,
    #[prost(bytes = "vec", tag = "2")]
    pub payload: Vec<u8>,
}

#[async_trait]
pub trait MeshTransport: Send + Sync {
    async fn publish(&self, topic: &str, message: Message) -> Result<(), String>;
    async fn subscribe(&self, topic: &str, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String>;

    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String>;
    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String>;

    async fn register_presence(&self, agent_id: &str, status: &str, ttl_seconds: u64) -> Result<(), String>;
    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String>;

    async fn ping_agent(&self, agent_id: &str) -> Result<bool, String>;

    // State Handoff
    async fn sync_handoff(&self, mission_id: &str, payload: Vec<u8>, status: &str) -> Result<(), String>;
    async fn pull_handoffs(&self) -> Result<Vec<(String, Vec<u8>, String)>, String>;
}

pub struct MemoryTransport {
    subs: DashMap<String, broadcast::Sender<Message>>,
    presence: DashMap<String, (String, std::time::Instant)>,
    handoffs: DashMap<String, (Vec<u8>, String)>,
    locks: DashMap<String, (String, std::time::Instant)>, // resource -> (owner, expires_at)
}

impl MemoryTransport {
    pub fn new() -> Self {
        MemoryTransport {
            subs: DashMap::new(),
            presence: DashMap::new(),
            handoffs: DashMap::new(),
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

    async fn ping_agent(&self, agent_id: &str) -> Result<bool, String> {
        let now = std::time::Instant::now();
        if let Some(entry) = self.presence.get(agent_id) {
            if entry.value().1 > now {
                return Ok(true);
            }
        }
        Ok(false)
    }

    async fn sync_handoff(&self, mission_id: &str, payload: Vec<u8>, status: &str) -> Result<(), String> {
        self.handoffs.insert(mission_id.to_string(), (payload, status.to_string()));
        Ok(())
    }

    async fn pull_handoffs(&self) -> Result<Vec<(String, Vec<u8>, String)>, String> {
        let mut results = Vec::new();
        for entry in self.handoffs.iter() {
            results.push((entry.key().clone(), entry.value().0.clone(), entry.value().1.clone()));
        }
        Ok(results)
    }
}


use sqlx::{sqlite::SqlitePoolOptions, SqlitePool, Row};
use tokio::sync::OnceCell;

static IPC_POOL: OnceCell<SqlitePool> = OnceCell::const_new();


pub struct IpcTransport {
    pool: SqlitePool,
}

impl IpcTransport {
    pub async fn new() -> Result<Self, String> {
        let pool = IPC_POOL.get_or_init(|| async {
            let db_path = std::env::var("OHC_STANDALONE_DB").unwrap_or_else(|_| "sqlite://standalone.db".to_string());
            Self::init_pool(&db_path).await
        }).await;

        Ok(IpcTransport {
            pool: pool.clone(),
        })
    }

    pub async fn new_for_test(db_url: &str) -> Result<Self, String> {
        let pool = Self::init_pool(db_url).await;
        Ok(IpcTransport { pool })
    }

    async fn init_pool(db_path: &str) -> SqlitePool {
        // Ensure the database file exists
        if db_path.starts_with("sqlite://") && db_path != "sqlite::memory:" {
            let file_path = db_path.replace("sqlite://", "");
            if !std::path::Path::new(&file_path).exists() {
                if let Some(parent) = std::path::Path::new(&file_path).parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                let _ = std::fs::File::create(&file_path);
            }
        }

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(db_path)
            .await
            .unwrap_or_else(|_| {
                SqlitePoolOptions::new()
                    .max_connections(5)
                    .connect_lazy(db_path)
                    .unwrap()
            });

        // Initialize schema
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS mesh_messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                topic TEXT NOT NULL,
                payload BLOB NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )"
        ).execute(&pool).await.expect("Failed to initialize mesh_messages table");

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS mesh_locks (
                resource TEXT PRIMARY KEY,
                owner TEXT NOT NULL,
                expires_at INTEGER NOT NULL
            )"
        ).execute(&pool).await.expect("Failed to initialize mesh_locks table");

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS mesh_presence (
                agent_id TEXT PRIMARY KEY,
                status TEXT NOT NULL,
                expires_at INTEGER NOT NULL
            )"
        ).execute(&pool).await.expect("Failed to initialize mesh_presence table");

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS mesh_handoff (
                mission_id TEXT PRIMARY KEY,
                payload BLOB NOT NULL,
                status TEXT NOT NULL,
                synced_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )"
        ).execute(&pool).await.expect("Failed to initialize mesh_handoff table");

        pool
    }
}
#[async_trait]
impl MeshTransport for IpcTransport {
    async fn publish(&self, topic: &str, message: Message) -> Result<(), String> {
        use prost::Message as ProstMessage;
        let mut buf = Vec::new();
        message.encode(&mut buf).map_err(|e| e.to_string())?;

        // Reliable publish with exponential backoff retries for local IPC
        let mut retries = 3;
        let mut delay = 50;
        loop {
            match sqlx::query("INSERT INTO mesh_messages (topic, payload) VALUES ($1, $2)")
                .bind(topic)
                .bind(&buf)
                .execute(&self.pool)
                .await
            {
                Ok(_) => return Ok(()),
                Err(e) => {
                    if retries == 0 {
                        return Err(e.to_string());
                    }
                    retries -= 1;
                    tokio::time::sleep(std::time::Duration::from_millis(delay)).await;
                    delay *= 2;
                }
            }
        }
    }

    async fn subscribe(&self, topic: &str, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let topic_str = topic.to_string();
        let pool = self.pool.clone();

        let worker = tokio::spawn(async move {
            let mut last_id: i64 = 0;
            // Get current max id to only receive new messages
            if let Ok(row) = sqlx::query("SELECT MAX(id) as max_id FROM mesh_messages").fetch_one(&pool).await {
                last_id = row.try_get("max_id").unwrap_or(0);
            }

            loop {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;

                let result = sqlx::query("SELECT id, payload FROM mesh_messages WHERE topic = $1 AND id > $2 ORDER BY id ASC")
                    .bind(&topic_str)
                    .bind(last_id)
                    .fetch_all(&pool)
                    .await;

                if let Ok(rows) = result {
                    for row in rows {
                        let id: i64 = row.get("id");
                        last_id = std::cmp::max(last_id, id);

                        let payload: Vec<u8> = row.get("payload");
                        use prost::Message as ProstMessage;
                        if let Ok(msg) = Message::decode(&payload[..]) {
                            handler(msg);
                        }
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
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        let expires_at = now + ttl_seconds;

        // Clean up expired locks first
        let _ = sqlx::query("DELETE FROM mesh_locks WHERE expires_at <= $1")
            .bind(now as i64)
            .execute(&self.pool)
            .await;

        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        let row: Option<(i64,)> = sqlx::query_as("SELECT 1 FROM mesh_locks WHERE resource = $1")
            .bind(resource)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        if row.is_some() {
            let _ = tx.rollback().await;
            return Ok(false);
        }

        let result = sqlx::query("INSERT INTO mesh_locks (resource, owner, expires_at) VALUES ($1, $2, $3)")
            .bind(resource)
            .bind(owner)
            .bind(expires_at as i64)
            .execute(&mut *tx)
            .await;

        match result {
            Ok(_) => {
                let _ = tx.commit().await;
                Ok(true)
            }
            Err(_) => {
                let _ = tx.rollback().await;
                Ok(false)
            }
        }
    }

    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
        let _ = sqlx::query("DELETE FROM mesh_locks WHERE resource = $1 AND owner = $2")
            .bind(resource)
            .bind(owner)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn register_presence(&self, agent_id: &str, status: &str, ttl_seconds: u64) -> Result<(), String> {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        let expires_at = now + ttl_seconds;

        let _ = sqlx::query(
            "INSERT INTO mesh_presence (agent_id, status, expires_at) VALUES ($1, $2, $3)
             ON CONFLICT(agent_id) DO UPDATE SET status = excluded.status, expires_at = excluded.expires_at"
        )
        .bind(agent_id)
        .bind(status)
        .bind(expires_at as i64)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

        // Clean up expired presence
        let _ = sqlx::query("DELETE FROM mesh_presence WHERE expires_at <= $1")
            .bind(now as i64)
            .execute(&self.pool)
            .await;

        let rows = sqlx::query("SELECT agent_id, status FROM mesh_presence")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut agents = Vec::new();
        for row in rows {
            agents.push((row.get("agent_id"), row.get("status")));
        }

        Ok(agents)
    }

    async fn ping_agent(&self, agent_id: &str) -> Result<bool, String> {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        let row: Option<(i64,)> = sqlx::query_as("SELECT 1 FROM mesh_presence WHERE agent_id = $1 AND expires_at > $2")
            .bind(agent_id)
            .bind(now as i64)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(row.is_some())
    }

    async fn sync_handoff(&self, mission_id: &str, payload: Vec<u8>, status: &str) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO mesh_handoff (mission_id, payload, status) VALUES ($1, $2, $3)
             ON CONFLICT(mission_id) DO UPDATE SET payload = excluded.payload, status = excluded.status, synced_at = CURRENT_TIMESTAMP"
        )
        .bind(mission_id)
        .bind(payload)
        .bind(status)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn pull_handoffs(&self) -> Result<Vec<(String, Vec<u8>, String)>, String> {
        let rows = sqlx::query("SELECT mission_id, payload, status FROM mesh_handoff")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut handoffs = Vec::new();
        use sqlx::Row;
        for row in rows {
            handoffs.push((row.get("mission_id"), row.get("payload"), row.get("status")));
        }

        Ok(handoffs)
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
        use redis::AsyncCommands;
        use base64::{Engine as _, engine::general_purpose::STANDARD};

        let mut conn = self.publish_conn.lock().await;

        let mut buf = Vec::new();
        message.encode(&mut buf).unwrap();
        let payload_b64 = STANDARD.encode(&buf);

        let _: () = conn.publish(topic, payload_b64).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn subscribe(&self, topic: &str, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        use prost::Message as ProstMessage;
        use futures_util::StreamExt;
        use base64::{Engine as _, engine::general_purpose::STANDARD};

        // We use into_pubsub to get a pubsub connection
        // The deprecation warning indicates this uses a different underlying connection, which is what we want for subscribe anyway
        #[allow(deprecated)]
        let mut pubsub = self.client.get_async_connection().await.map_err(|e| e.to_string())?.into_pubsub();

        pubsub.subscribe(topic).await.map_err(|e| e.to_string())?;
        let mut stream = pubsub.into_on_message();

        let worker = tokio::spawn(async move {
            while let Some(msg) = stream.next().await {
                if let Ok(payload_b64) = msg.get_payload::<String>() {
                    if let Ok(buf) = STANDARD.decode(&payload_b64) {
                        if let Ok(message) = Message::decode(&buf[..]) {
                            handler(message);
                        }
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
        use redis::AsyncCommands;
        let mut conn = self.publish_conn.lock().await;

        let key = format!("lock:{}", resource);
        let result: bool = redis::cmd("SET")
            .arg(&key)
            .arg(owner)
            .arg("NX")
            .arg("EX")
            .arg(ttl_seconds)
            .query_async(&mut *conn)
            .await
            .unwrap_or(false);

        Ok(result)
    }

    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
        use redis::AsyncCommands;
        let mut conn = self.publish_conn.lock().await;

        let key = format!("lock:{}", resource);

        // Use a Lua script to ensure we only delete the lock if we own it
        let script = redis::Script::new(
            "if redis.call('get', KEYS[1]) == ARGV[1] then return redis.call('del', KEYS[1]) else return 0 end"
        );

        let _: () = script
            .key(&key)
            .arg(owner)
            .invoke_async(&mut *conn)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn register_presence(&self, agent_id: &str, status: &str, ttl_seconds: u64) -> Result<(), String> {
        let mut conn = self.publish_conn.lock().await;
        use redis::AsyncCommands;

        let key = "mesh:presence";

        let mut pipe = redis::pipe();
        pipe.atomic()
            .cmd("HSET").arg(key).arg(agent_id).arg(status)
            .cmd("EXPIRE").arg(key).arg(ttl_seconds);

        let _: () = pipe.query_async(&mut *conn).await.map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
        let mut conn = self.publish_conn.lock().await;
        use redis::AsyncCommands;

        let key = "mesh:presence";
        let hash: std::collections::HashMap<String, String> = conn.hgetall(key).await.unwrap_or_default();

        let agents = hash.into_iter().collect();
        Ok(agents)
    }

    async fn ping_agent(&self, agent_id: &str) -> Result<bool, String> {
        let mut conn = self.publish_conn.lock().await;
        use redis::AsyncCommands;
        let key = "mesh:presence";
        let status: Option<String> = conn.hget(key, agent_id).await.map_err(|e| e.to_string())?;
        Ok(status.is_some())
    }

    async fn sync_handoff(&self, mission_id: &str, payload: Vec<u8>, status: &str) -> Result<(), String> {
        let mut conn = self.publish_conn.lock().await;
        use redis::AsyncCommands;
        use base64::{Engine as _, engine::general_purpose::STANDARD};

        let key = "mesh:handoff";
        let data = serde_json::json!({
            "payload": STANDARD.encode(&payload),
            "status": status
        }).to_string();

        let _: () = conn.hset(key, mission_id, data).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn pull_handoffs(&self) -> Result<Vec<(String, Vec<u8>, String)>, String> {
        let mut conn = self.publish_conn.lock().await;
        use redis::AsyncCommands;
        use base64::{Engine as _, engine::general_purpose::STANDARD};

        let key = "mesh:handoff";
        let hash: std::collections::HashMap<String, String> = conn.hgetall(key).await.unwrap_or_default();

        let mut handoffs = Vec::new();
        for (mission_id, data_str) in hash {
            if let Ok(data) = serde_json::from_str::<serde_json::Value>(&data_str) {
                if let (Some(payload_b64), Some(status)) = (data["payload"].as_str(), data["status"].as_str()) {
                    if let Ok(payload) = STANDARD.decode(payload_b64) {
                        handoffs.push((mission_id, payload, status.to_string()));
                    }
                }
            }
        }
        Ok(handoffs)
    }
}

pub async fn create_transport(redis_url: Option<&str>, is_cloud: bool, standalone_url_override: Option<&str>) -> Result<Arc<dyn MeshTransport>, String> {
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

    // Standalone fallback: we use IpcTransport
    let ipc_res = match standalone_url_override {
        Some(url) => IpcTransport::new_for_test(url).await,
        None => IpcTransport::new().await,
    };
    match ipc_res {
        Ok(t) => {
            println!("Initialized IpcTransport (Standalone)");
            return Ok(Arc::new(t));
        },
        Err(e) => {
            println!("Failed to initialize IpcTransport (Standalone): {}. Falling back to MemoryTransport.", e);
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
    async fn test_memory_transport() {
        let transport = MemoryTransport::new();
        let received = Arc::new(AtomicBool::new(false));
        let received_clone = received.clone();

        let handler = Box::new(move |msg: Message| {
            if msg.topic == "test_topic" && msg.payload == b"hello" {
                received_clone.store(true, Ordering::SeqCst);
            }
        });

        let cancel = transport.subscribe("test_topic", handler).await.unwrap();

        let msg = Message {
            topic: "test_topic".to_string(),
            payload: b"hello".to_vec(),
        };

        transport.publish("test_topic", msg).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        assert!(received.load(Ordering::SeqCst));
        cancel();
    }


    #[tokio::test]
    async fn test_ipc_transport_pubsub() {
        let temp_dir = std::env::var("TEST_TMPDIR").unwrap_or_else(|_| "/tmp".to_string());
        let db_path = format!("{}/test_ipc_pubsub_{}.db", temp_dir, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros());
        let _db_url = format!("sqlite://{}", db_path);
        let transport = IpcTransport::new_for_test(&_db_url).await.unwrap();





        let received = Arc::new(AtomicBool::new(false));
        let received_clone = received.clone();

        let handler = Box::new(move |msg: Message| {
            if msg.topic == "ipc_topic" && msg.payload == b"hello ipc" {
                received_clone.store(true, Ordering::SeqCst);
            }
        });

        let cancel = transport.subscribe("ipc_topic", handler).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;

        let msg = Message {
            topic: "ipc_topic".to_string(),
            payload: b"hello ipc".to_vec(),
        };

        transport.publish("ipc_topic", msg).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

        assert!(received.load(Ordering::SeqCst));
        cancel();
    }

    #[tokio::test]
    async fn test_ipc_transport_locking() {
        let temp_dir = std::env::var("TEST_TMPDIR").unwrap_or_else(|_| "/tmp".to_string());
        let db_path = format!("{}/test_ipc_lock_{}.db", temp_dir, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros());
        let _db_url = format!("sqlite://{}", db_path);
        let transport = IpcTransport::new_for_test(&_db_url).await.unwrap();





        // Test lock acquisition
        let acquired = transport.acquire_lock("ipc_resource", "agent_1", 10).await.unwrap();
        assert!(acquired);

        // Test mutual exclusion
        let acquired_again = transport.acquire_lock("ipc_resource", "agent_2", 10).await.unwrap();
        assert!(!acquired_again);

        // Test lock release
        transport.release_lock("ipc_resource", "agent_1").await.unwrap();

        // Test lock acquisition after release
        let acquired_after_release = transport.acquire_lock("ipc_resource", "agent_2", 10).await.unwrap();
        assert!(acquired_after_release);
    }

    #[tokio::test]
    async fn test_create_transport_standalone() {
        let temp_dir = std::env::var("TEST_TMPDIR").unwrap_or_else(|_| "/tmp".to_string());
        let db_path = format!("{}/test_create_standalone_{}.db", temp_dir, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros());
        let _db_url = format!("sqlite://{}", db_path);

        let _transport = create_transport(None, false, Some(&_db_url)).await.unwrap();
        // Since MemoryTransport isn't easily castable back without Any, we just ensure it didn't err
        assert!(true);
    }

    #[tokio::test]
    async fn test_create_transport_redis_fails() {
        let temp_dir = std::env::var("TEST_TMPDIR").unwrap_or_else(|_| "/tmp".to_string());
        let db_path = format!("{}/test_create_redis_fails_{}.db", temp_dir, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros());
        let _db_url = format!("sqlite://{}", db_path);


        // Provide invalid url
        let transport = create_transport(Some("redis://localhost:9999"), false, Some(&_db_url)).await;
        // In standalone, it should fallback to Memory (now Ipc), so it's Ok
        assert!(transport.is_ok());

        // In cloud, it should err
        let transport = create_transport(Some("redis://localhost:9999"), true, Some(&_db_url)).await;
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
            println!("Skipping redis transport test due to missing redis connection");
            return;
        }
        let transport = transport.unwrap();

        // Setup channel for verification
        let (tx, mut rx) = tokio::sync::mpsc::channel(10);
        let tx_arc = Arc::new(tokio::sync::Mutex::new(tx));
        let handler = Box::new(move |msg: Message| {
            let tx_clone = tx_arc.clone();
            tokio::spawn(async move {
                let mut tx = tx_clone.lock().await;
                let _ = tx.send(msg).await;
            });
        });

        // Wait for connection to settle
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        let cancel = transport.subscribe("test_topic_redis", handler).await.unwrap();

        // Wait for subscription to propagate
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let msg = Message {
            topic: "test_topic_redis".to_string(),
            payload: b"hello redis".to_vec(),
        };

        transport.publish("test_topic_redis", msg.clone()).await.unwrap();

        // Use timeout to prevent hanging test
        let result = tokio::time::timeout(std::time::Duration::from_secs(2), rx.recv()).await;

        assert!(result.is_ok());
        if let Ok(Some(received_msg)) = result {
             assert_eq!(received_msg.topic, "test_topic_redis");
             assert_eq!(received_msg.payload, b"hello redis");
        } else {
             panic!("Did not receive message");
        }

        cancel();
    }
}
