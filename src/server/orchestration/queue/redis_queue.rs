use super::queue::{Job, TaskQueue};
use async_trait::async_trait;

pub struct RedisTaskQueue {
    client: redis::Client,
    queue_name: String,
}

impl RedisTaskQueue {
    pub fn new(redis_url: &str, queue_name: &str) -> Result<Self, String> {
        let client = redis::Client::open(redis_url).map_err(|e| e.to_string())?;
        Ok(Self {
            client,
            queue_name: queue_name.to_string(),
        })
    }
}

#[async_trait]
impl TaskQueue for RedisTaskQueue {
    async fn enqueue_batch(&self, jobs: Vec<Job>) -> Result<(), String> {
        if jobs.is_empty() { return Ok(()); }
        let mut conn = self.client.get_multiplexed_tokio_connection().await.map_err(|e| e.to_string())?;
        let mut pipe = redis::pipe();
        for job in jobs {
            let payload_json = serde_json::to_string(&job).map_err(|e| e.to_string())?;
            pipe.cmd("RPUSH").arg(&self.queue_name).arg(payload_json);
        }
        let _: () = pipe.query_async(&mut conn).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn enqueue(&self, job: Job) -> Result<(), String> {
        let mut conn = self.client.get_multiplexed_tokio_connection().await.map_err(|e| e.to_string())?;
        let payload_json = serde_json::to_string(&job).map_err(|e| e.to_string())?;

        let _: () = redis::cmd("RPUSH")
            .arg(&self.queue_name)
            .arg(payload_json)
            .query_async(&mut conn)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn dequeue(&self, roles: Vec<String>) -> Result<Option<Job>, String> {
        let mut conn = self.client.get_multiplexed_tokio_connection().await.map_err(|e| e.to_string())?;

        let result: Option<(String, String)> = redis::cmd("BLPOP")
            .arg(&self.queue_name)
            .arg(1)
            .query_async(&mut conn)
            .await
            .map_err(|e| e.to_string())?;

        if let Some((_, payload_json)) = result {
            if let Ok(job) = serde_json::from_str::<Job>(&payload_json) {
                if roles.contains(&job.agent_role) {
                    return Ok(Some(job));
                } else {
                    let _ = self.enqueue(job).await;
                }
            }
        }
        Ok(None)
    }

    async fn complete(&self, _job_id: &str) -> Result<(), String> {
        Ok(())
    }

    async fn fail(&self, _job_id: &str, _reason: &str) -> Result<(), String> {
        Ok(())
    }
}
