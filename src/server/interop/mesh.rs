use async_trait::async_trait;
use tokio::sync::mpsc;
use tokio::sync::broadcast;
use std::collections::HashMap;
use tokio::sync::RwLock;
use redis::AsyncCommands;
use futures_util::StreamExt;

#[async_trait]
pub trait TeammateMeshApi: Send + Sync {
    async fn publish(&self, channel: &str, data: Vec<u8>) -> Result<(), String>;
    async fn subscribe(&self, channel: &str) -> Result<mpsc::Receiver<Vec<u8>>, String>;
    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String>;
    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String>;
}

pub struct StandaloneTeammateMesh {
    channels: RwLock<HashMap<String, broadcast::Sender<Vec<u8>>>>,
    locks: RwLock<HashMap<String, String>>, // simple lock mock
}

impl StandaloneTeammateMesh {
    pub fn new() -> Self {
        Self {
            channels: RwLock::new(HashMap::new()),
            locks: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl TeammateMeshApi for StandaloneTeammateMesh {
    async fn publish(&self, channel: &str, data: Vec<u8>) -> Result<(), String> {
        let mut channels = self.channels.write().await;
        let tx = channels.entry(channel.to_string()).or_insert_with(|| {
            let (tx, _) = broadcast::channel(1024);
            tx
        });
        let _ = tx.send(data);
        Ok(())
    }

    async fn subscribe(&self, channel: &str) -> Result<mpsc::Receiver<Vec<u8>>, String> {
        let mut channels = self.channels.write().await;
        let tx = channels.entry(channel.to_string()).or_insert_with(|| {
            let (tx, _) = broadcast::channel(1024);
            tx
        });
        let mut rx = tx.subscribe();
        let (mpsc_tx, mpsc_rx) = mpsc::channel(1024);

        tokio::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                if mpsc_tx.send(msg).await.is_err() {
                    break;
                }
            }
        });

        Ok(mpsc_rx)
    }

    async fn acquire_lock(&self, resource: &str, owner: &str, _ttl_seconds: u64) -> Result<bool, String> {
        let mut locks = self.locks.write().await;
        if locks.contains_key(resource) && locks.get(resource).unwrap() != owner {
            return Ok(false);
        }
        locks.insert(resource.to_string(), owner.to_string());
        Ok(true)
    }

    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
        let mut locks = self.locks.write().await;
        if let Some(current_owner) = locks.get(resource) {
            if current_owner == owner {
                locks.remove(resource);
            }
        }
        Ok(())
    }
}

pub struct CloudTeammateMesh {
    client: redis::Client,
    publisher: tokio::sync::Mutex<redis::aio::MultiplexedConnection>,
}

impl CloudTeammateMesh {
    pub async fn new(redis_url: &str) -> Result<Self, String> {
        let client = redis::Client::open(redis_url).map_err(|e| e.to_string())?;
        let pub_conn = client.get_multiplexed_async_connection().await.map_err(|e| e.to_string())?;

        Ok(Self {
            client,
            publisher: tokio::sync::Mutex::new(pub_conn),
        })
    }
}

#[async_trait]
impl TeammateMeshApi for CloudTeammateMesh {
    async fn publish(&self, channel: &str, data: Vec<u8>) -> Result<(), String> {
        let mut conn = self.publisher.lock().await;
        let _: () = conn.publish(channel, data).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn subscribe(&self, channel: &str) -> Result<mpsc::Receiver<Vec<u8>>, String> {
        let mut pubsub_conn = self.client.get_async_pubsub().await.map_err(|e| e.to_string())?;
        pubsub_conn.subscribe(channel).await.map_err(|e| e.to_string())?;

        let (tx, rx) = mpsc::channel(1024);
        let mut stream = pubsub_conn.into_on_message();

        tokio::spawn(async move {
            while let Some(msg) = stream.next().await {
                if let Ok(payload) = msg.get_payload::<Vec<u8>>() {
                    if tx.send(payload).await.is_err() {
                        break;
                    }
                }
            }
        });

        Ok(rx)
    }

    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String> {
        let mut conn = self.publisher.lock().await;
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
        let mut conn = self.publisher.lock().await;
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
