use super::MeshTransport;
use crate::proto::hub::TeammateMeshEvent as Message;
use async_trait::async_trait;
use dashmap::DashMap;
use sqlx::Row;
use std::sync::Arc;
use tokio::sync::broadcast;

pub struct PgTransport {
    pool: sqlx::PgPool,
    subs: DashMap<String, broadcast::Sender<Message>>,
}

impl PgTransport {
    pub async fn new(db_url: &str) -> Result<Self, String> {
        use sqlx::postgres::PgPoolOptions;
        let pool = PgPoolOptions::new()
            .after_release(|conn, _meta| {
                Box::pin(async move {
                    use sqlx::Executor;
                    conn.execute("DISCARD ALL").await?;
                    Ok(true)
                })
            })
            .connect(db_url)
            .await
            .map_err(|e| e.to_string())?;

        // Initialize schema
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS mesh_messages (
                id BIGSERIAL PRIMARY KEY,
                topic TEXT NOT NULL,
                payload BYTEA NOT NULL,
                created_at TIMESTAMPTZ DEFAULT NOW(),
                msg_id TEXT
            )",
        )
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS mesh_checkpoints (
                subscriber_id TEXT PRIMARY KEY,
                last_id BIGINT NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

        // Attempt to add the column, ignoring error if it already exists
        match sqlx::query("ALTER TABLE mesh_messages ADD COLUMN msg_id TEXT")
            .execute(&pool)
            .await
        {
            Ok(_) => {}
            Err(e) => {
                let err_str = e.to_string();
                if !err_str.contains("duplicate column") && !err_str.contains("already exists") {
                    return Err(format!("Failed to migrate mesh_messages: {}", err_str));
                }
            }
        }

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS mesh_locks (
                resource TEXT PRIMARY KEY,
                owner TEXT NOT NULL,
                expires_at TIMESTAMPTZ NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS mesh_presence (
                agent_id TEXT PRIMARY KEY,
                status TEXT NOT NULL,
                expires_at TIMESTAMPTZ NOT NULL
            )",
        )
        .execute(&pool)
        .await
        .map_err(|e| e.to_string())?;

        let subs = DashMap::new();

        Ok(PgTransport { pool, subs })
    }

    pub async fn start_worker(&self) {
        use opentelemetry::{global, KeyValue};
        use prost::Message as ProstMessage;
        let pool = self.pool.clone();
        let subs = self.subs.clone();

        let subscriber_id = "builtin_agent_node".to_string();
        let mut last_id: i64 =
            sqlx::query_scalar("SELECT last_id FROM mesh_checkpoints WHERE subscriber_id = $1")
                .bind(&subscriber_id)
                .fetch_optional(&pool)
                .await
                .unwrap_or(Some(0))
                .unwrap_or(0);

        let meter = global::meter("ohc.postgres");
        let skip_locked_counter = meter.u64_counter("ohc_postgres_skip_locked_total").build();

        loop {
            // Poll for new messages using SKIP LOCKED
            let rows: Result<Vec<(i64, String, Vec<u8>)>, _> = sqlx::query_as(
                "SELECT id, topic, payload FROM mesh_messages WHERE id > $1 ORDER BY id ASC FOR UPDATE SKIP LOCKED"
            )
            .bind(last_id)
            .fetch_all(&pool)
            .await;

            if let Ok(rows) = rows {
                let has_rows = !rows.is_empty();
                for (id, topic, payload) in rows {
                    skip_locked_counter.add(1, &[KeyValue::new("action", "poll_messages")]);
                    last_id = id;
                    if let Some(tx) = subs.get(&topic) {
                        if let Ok(message) = Message::decode(&payload[..]) {
                            let _ = tx.send(message);
                        }
                    }
                }

                if has_rows {
                    let _ = sqlx::query("INSERT INTO mesh_checkpoints (subscriber_id, last_id) VALUES ($1, $2) ON CONFLICT(subscriber_id) DO UPDATE SET last_id = EXCLUDED.last_id")
                        .bind(&subscriber_id)
                        .bind(last_id)
                        .execute(&pool)
                        .await;
                }
            }

            // Cleanup old messages (keep last 1 hour)
            let _ = sqlx::query(
                "DELETE FROM mesh_messages WHERE created_at < NOW() - INTERVAL '1 hour'",
            )
            .execute(&pool)
            .await;

            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
    }
}

#[async_trait]
impl MeshTransport for PgTransport {
    async fn publish(&self, topic: &str, message: Message) -> Result<(), String> {
        use prost::Message as ProstMessage;
        let mut buf = Vec::new();
        message.encode(&mut buf).unwrap();

        let msg_id = if message.msg_id.is_empty() {
            None
        } else {
            Some(message.msg_id.clone())
        };

        sqlx::query("INSERT INTO mesh_messages (topic, payload, msg_id) VALUES ($1, $2, $3)")
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

    async fn subscribe(
        &self,
        topic: &str,
        handler: Box<dyn Fn(Message) + Send + Sync>,
    ) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let tx = self
            .subs
            .entry(topic.to_string())
            .or_insert_with(|| {
                let (tx, _) = broadcast::channel(100);
                tx
            })
            .clone();

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

    async fn acquire_lock(
        &self,
        resource: &str,
        owner: &str,
        ttl_seconds: u64,
    ) -> Result<bool, String> {
        // Cleanup expired locks
        let _ = sqlx::query("DELETE FROM mesh_locks WHERE expires_at <= NOW()")
            .execute(&self.pool)
            .await;

        let result = sqlx::query(
            "INSERT INTO mesh_locks (resource, owner, expires_at) VALUES ($1, $2, NOW() + CAST($3 AS INTERVAL))
             ON CONFLICT(resource) DO UPDATE SET owner = EXCLUDED.owner, expires_at = EXCLUDED.expires_at WHERE mesh_locks.expires_at <= NOW() OR mesh_locks.owner = EXCLUDED.owner"
        )
        .bind(resource)
        .bind(owner)
        .bind(format!("{} seconds", ttl_seconds))
        .execute(&self.pool)
        .await;

        match result {
            Ok(res) => Ok(res.rows_affected() > 0),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
        sqlx::query("DELETE FROM mesh_locks WHERE resource = $1 AND owner = $2")
            .bind(resource)
            .bind(owner)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn register_presence(
        &self,
        agent_id: &str,
        status: &str,
        ttl_seconds: u64,
    ) -> Result<(), String> {
        sqlx::query(
            "INSERT INTO mesh_presence (agent_id, status, expires_at) VALUES ($1, $2, NOW() + CAST($3 AS INTERVAL))
             ON CONFLICT(agent_id) DO UPDATE SET status = EXCLUDED.status, expires_at = EXCLUDED.expires_at"
        )
        .bind(agent_id)
        .bind(status)
        .bind(format!("{} seconds", ttl_seconds))
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
        let _ = sqlx::query("DELETE FROM mesh_presence WHERE expires_at <= NOW()")
            .execute(&self.pool)
            .await;

        let rows: Result<Vec<(String, String)>, _> =
            sqlx::query_as("SELECT agent_id, status FROM mesh_presence")
                .fetch_all(&self.pool)
                .await;

        match rows {
            Ok(r) => Ok(r),
            Err(e) => Err(e.to_string()),
        }
    }
}
