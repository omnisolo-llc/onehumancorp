use super::queue::{Job, TaskQueue};
use async_trait::async_trait;

pub struct RedisTaskQueue {
    client: redis::Client,
    queue_name: String,
    connection: tokio::sync::OnceCell<redis::aio::MultiplexedConnection>,
}

impl RedisTaskQueue {
    pub fn new(redis_url: &str, queue_name: &str) -> Result<Self, String> {
        let client = redis::Client::open(redis_url).map_err(|e| e.to_string())?;
        Ok(Self {
            client,
            queue_name: queue_name.to_string(),
            connection: tokio::sync::OnceCell::new(),
        })
    }

    async fn get_connection(&self) -> Result<redis::aio::MultiplexedConnection, String> {
        let conn = self.connection.get_or_try_init(|| async {
            self.client.get_multiplexed_tokio_connection().await
        }).await.map_err(|e| e.to_string())?;
        Ok(conn.clone())
    }
}

#[async_trait]
impl TaskQueue for RedisTaskQueue {
    async fn enqueue_batch(&self, jobs: Vec<Job>) -> Result<(), String> {
        if jobs.is_empty() { return Ok(()); }
        let mut conn = self.get_connection().await?;
        let mut pipe = redis::pipe();
        for job in jobs {
            let payload_json = serde_json::to_string(&job).map_err(|e| e.to_string())?;
            pipe.cmd("ZADD").arg(&self.queue_name).arg(job.next_retry_at.timestamp_millis()).arg(payload_json);
        }
        let _: () = pipe.query_async(&mut conn).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn enqueue(&self, job: Job) -> Result<(), String> {
        let mut conn = self.get_connection().await?;
        let payload_json = serde_json::to_string(&job).map_err(|e| e.to_string())?;

        let _: () = redis::cmd("ZADD")
            .arg(&self.queue_name)
            .arg(job.next_retry_at.timestamp_millis())
            .arg(payload_json)
            .query_async(&mut conn)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn dequeue(&self, roles: Vec<String>, _estimated_vram: i64, _estimated_tokens: i64) -> Result<Option<Job>, String> {
        let mut conn = self.get_connection().await?;
        let now = chrono::Utc::now().timestamp_millis();

        // Pop min from zset
        let result: Vec<(String, f64)> = redis::cmd("ZPOPMIN")
            .arg(&self.queue_name)
            .arg(1)
            .query_async(&mut conn)
            .await
            .map_err(|e| e.to_string())?;

        if !result.is_empty() {
            let (payload_json, score) = &result[0];
            if *score > now as f64 {
                // Not ready, put it back
                let _: () = redis::cmd("ZADD")
                    .arg(&self.queue_name)
                    .arg(*score)
                    .arg(payload_json)
                    .query_async(&mut conn)
                    .await
                    .map_err(|e| e.to_string())?;
                return Ok(None);
            }

            if let Ok(job) = serde_json::from_str::<Job>(payload_json) {
                if roles.contains(&job.job_type) {
                    // Would do quota check here before returning
                    return Ok(Some(job));
                } else {
                    // Not right role, put it back
                    let _ = self.enqueue(job).await;
                }
            } else {
                // Corrupted data - log error and gracefully discard the bad payload.
                tracing::warn!("Redis mailbox corruption detected: Dropping malformed JSON payload.");
                // Since ZPOPMIN already removed it, doing nothing drops the bad message.
            }
        }
        Ok(None)
    }

    async fn complete(&self, _job_id: &str) -> Result<(), String> {
        Ok(())
    }

    async fn fail(&self, _job_id: &str, _reason: &str) -> Result<(), String> {
        // Normally we'd fetch the job state from a tracking hash, update attempt count, and enqueue
        // Since we lack state in this simplified trait interface we assume it's handled via requeue by the caller or we would implement it here
        Ok(())
    }
}
