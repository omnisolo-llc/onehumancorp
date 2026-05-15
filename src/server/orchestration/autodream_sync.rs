use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use chrono::Utc;
use serde::{Serialize, Deserialize};
use sqlx::{Pool, Postgres, Sqlite};
use uuid::Uuid;
use futures_util::StreamExt;
use std::collections::HashMap;

// --- Teammate Mesh (Redis Pub/Sub & Local Mutex Fallback) ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshMessage {
    pub id: String,
    pub sender_id: String,
    pub receiver_id: Option<String>,
    pub channel: String,
    pub payload: String,
    pub created_at: chrono::DateTime<Utc>,
}

pub struct TeammateMesh {
    redis_client: Option<redis::Client>,
    local_channels: Arc<Mutex<HashMap<String, mpsc::Sender<MeshMessage>>>>,
}

impl TeammateMesh {
    pub fn new(redis_url: Option<&str>) -> Self {
        let redis_client = redis_url.map(|url| redis::Client::open(url).expect("Failed to connect to Redis"));
        Self {
            redis_client,
            local_channels: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn subscribe(&self, channel: &str) -> mpsc::Receiver<MeshMessage> {
        let (tx, rx) = mpsc::channel(100);

        if let Some(client) = &self.redis_client {
            let mut pubsub = client.get_async_connection().await.unwrap().into_pubsub();
            pubsub.subscribe(channel).await.unwrap();
            let mut stream = pubsub.into_on_message();

            let tx_clone = tx.clone();
            tokio::spawn(async move {
                while let Some(msg) = stream.next().await {
                    if let Ok(payload) = msg.get_payload::<String>() {
                        if let Ok(mesh_msg) = serde_json::from_str::<MeshMessage>(&payload) {
                            let _ = tx_clone.send(mesh_msg).await;
                        }
                    }
                }
            });
        }

        let mut channels = self.local_channels.lock().await;
        channels.insert(channel.to_string(), tx);
        rx
    }

    pub async fn publish(&self, channel: &str, message: MeshMessage) -> Result<(), String> {
        let payload = serde_json::to_string(&message).map_err(|e| e.to_string())?;

        if let Some(client) = &self.redis_client {
            let mut conn = client.get_async_connection().await.map_err(|e| e.to_string())?;
            redis::cmd("PUBLISH")
                .arg(channel)
                .arg(&payload)
                .query_async::<_, ()>(&mut conn)
                .await
                .map_err(|e| e.to_string())?;
        }

        let channels = self.local_channels.lock().await;
        if let Some(tx) = channels.get(channel) {
            let _ = tx.send(message).await;
        }

        Ok(())
    }
}

// --- AutoDream (pgvector Memory Consolidation) ---

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryEmbedding {
    pub id: String,
    pub organization_id: String,
    pub content: String,
    pub embedding: Vec<f32>,
    pub importance: f32,
    pub created_at: chrono::DateTime<Utc>,
}

pub struct AutoDream {
    pg_pool: Pool<Postgres>,
    sqlite_pool: Pool<Sqlite>,
}

impl AutoDream {
    pub fn new(pg_pool: Pool<Postgres>, sqlite_pool: Pool<Sqlite>) -> Self {
        Self { pg_pool, sqlite_pool }
    }

    pub async fn consolidate_memory(&self, org_id: &str, memories: Vec<MemoryEmbedding>) -> Result<(), String> {
        let mut tx = self.pg_pool.begin().await.map_err(|e| e.to_string())?;

        for mem in memories {
            let embedding_str = format!("{:?}", mem.embedding);
            sqlx::query(
                r#"
                INSERT INTO consolidated_memory (id, organization_id, content, embedding, importance, created_at)
                VALUES ($1, $2, $3, $4::vector, $5, $6)
                "#
            )
            .bind(&mem.id)
            .bind(&mem.organization_id)
            .bind(&mem.content)
            .bind(&embedding_str)
            .bind(mem.importance)
            .bind(mem.created_at)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;
        }

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn recall_memories(&self, org_id: &str, query_embedding: Vec<f32>, limit: i64) -> Result<Vec<MemoryEmbedding>, String> {
        let embedding_str = format!("{:?}", query_embedding);

        let rows = sqlx::query(
            r#"
            SELECT id, organization_id, content, embedding::text, importance, created_at
            FROM consolidated_memory
            WHERE organization_id = $1
            ORDER BY embedding <-> $2::vector
            LIMIT $3
            "#
        )
        .bind(org_id)
        .bind(&embedding_str)
        .bind(limit)
        .fetch_all(&self.pg_pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut memories = Vec::new();
        for row in rows {
            use sqlx::Row;
            let emb_str: String = row.get("embedding");
            let embedding: Vec<f32> = serde_json::from_str(&emb_str).unwrap_or_default();

            memories.push(MemoryEmbedding {
                id: row.get("id"),
                organization_id: row.get("organization_id"),
                content: row.get("content"),
                embedding,
                importance: row.get("importance"),
                created_at: row.get("created_at"),
            });
        }

        Ok(memories)
    }
}
