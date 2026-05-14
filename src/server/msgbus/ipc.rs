use super::{Bus, DistributedLock, Message};
use async_trait::async_trait;

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
