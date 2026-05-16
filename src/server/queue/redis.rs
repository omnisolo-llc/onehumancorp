#![allow(dead_code)]


use async_trait::async_trait;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use std::collections::VecDeque;
use std::sync::Mutex;

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use sqlx::Row;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
use super::models::*;
pub struct RedisTaskQueue {
    client: redis::Client,
    queue_name: String,
    connection: tokio::sync::OnceCell<redis::aio::MultiplexedConnection>,
}


impl RedisTaskQueue {
    pub fn new(redis_url: &str, queue_name: &str) -> Result<Self, String> {
        let client = redis::Client::open(redis_url).map_err(|e| e.to_string())?;
        Ok(RedisTaskQueue {
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
            let queue_job = ::interop_proto::ohc::interop::QueueJob {
                id: job.id,
                tenant_id: job.tenant_id,
                parent_task_id: job.parent_task_id,
                agent_role: job.agent_role,
                payload: job.payload,
                status: job.status,
                attempts: job.attempts,
                max_attempts: job.max_attempts,
                run_after_ms: job.run_after.timestamp_millis(),
                locked_until_ms: job.locked_until.map(|dt| dt.timestamp_millis()).unwrap_or(0),
                created_at_ms: job.created_at.timestamp_millis(),
                updated_at_ms: job.updated_at.timestamp_millis(),
            };
            let buf = prost::Message::encode_to_vec(&queue_job);
            pipe.cmd("RPUSH").arg(&self.queue_name).arg(buf);
        }
        let _: () = pipe.query_async(&mut conn).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn enqueue(&self, job: Job) -> Result<(), String> {
        let mut conn = self.get_connection().await?;
        let queue_job = ::interop_proto::ohc::interop::QueueJob {
            id: job.id,
            tenant_id: job.tenant_id,
            parent_task_id: job.parent_task_id,
            agent_role: job.agent_role,
            payload: job.payload,
            status: job.status,
            attempts: job.attempts,
            max_attempts: job.max_attempts,
            run_after_ms: job.run_after.timestamp_millis(),
            locked_until_ms: job.locked_until.map(|dt| dt.timestamp_millis()).unwrap_or(0),
            created_at_ms: job.created_at.timestamp_millis(),
            updated_at_ms: job.updated_at.timestamp_millis(),
        };
        let buf = prost::Message::encode_to_vec(&queue_job);
        // We use an RPUSH to the redis list
        let _: () = redis::cmd("RPUSH")
            .arg(&self.queue_name)
            .arg(buf)
            .query_async(&mut conn)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn dequeue(&self, roles: Vec<String>) -> Result<Option<Job>, String> {
        let mut conn = self.get_connection().await?;

        // Use BLPOP with 1 second timeout to avoid busy loop
        let result: Option<(String, Vec<u8>)> = redis::cmd("BLPOP")
            .arg(&self.queue_name)
            .arg(1)
            .query_async(&mut conn)
            .await
            .map_err(|e| e.to_string())?;

        if let Some((_, payload_bytes)) = result {
            if let Ok(queue_job) = <::interop_proto::ohc::interop::QueueJob as prost::Message>::decode(&payload_bytes[..]) {
                let job = Job {
                    id: queue_job.id.clone(),
                    tenant_id: queue_job.tenant_id,
                    parent_task_id: queue_job.parent_task_id,
                    agent_role: queue_job.agent_role.clone(),
                    payload: queue_job.payload,
                    status: queue_job.status,
                    attempts: queue_job.attempts,
                    max_attempts: queue_job.max_attempts,
                    run_after: chrono::DateTime::from_timestamp_millis(queue_job.run_after_ms).unwrap_or_else(chrono::Utc::now),
                    locked_until: if queue_job.locked_until_ms > 0 { Some(chrono::DateTime::from_timestamp_millis(queue_job.locked_until_ms).unwrap_or_else(chrono::Utc::now)) } else { None },
                    created_at: chrono::DateTime::from_timestamp_millis(queue_job.created_at_ms).unwrap_or_else(chrono::Utc::now),
                    updated_at: chrono::DateTime::from_timestamp_millis(queue_job.updated_at_ms).unwrap_or_else(chrono::Utc::now),
                };
                if roles.contains(&job.agent_role) {
                    let _: () = redis::cmd("HSET").arg(format!("{}_processing", self.queue_name)).arg(&job.id).arg(&payload_bytes).query_async(&mut conn).await.map_err(|e| e.to_string())?;
                    return Ok(Some(job));
                } else {
                    // Not intended for this worker role, push it back.
                    let _ = self.enqueue(job).await;
                }
            }
        }
        Ok(None)
    }

    async fn complete(&self, job_id: &str, tenant_id: &str) -> Result<(), String> {
        let mut conn = self.get_connection().await?;
        let processing_key = format!("{}_processing", self.queue_name);
        let result: Option<Vec<u8>> = redis::cmd("HGET").arg(&processing_key).arg(job_id).query_async(&mut conn).await.map_err(|e| e.to_string())?;
        if let Some(payload_bytes) = result {
            if let Ok(queue_job) = <::interop_proto::ohc::interop::QueueJob as prost::Message>::decode(&payload_bytes[..]) {
                if queue_job.tenant_id != tenant_id {
                    return Err("tenant mismatch".to_string());
                }
                let _: () = redis::cmd("HDEL").arg(&processing_key).arg(job_id).query_async(&mut conn).await.map_err(|e| e.to_string())?;
                return Ok(());
            }
        }
        Err("job not found".to_string())
    }

    async fn fail(&self, job_id: &str, tenant_id: &str, _reason: &str) -> Result<(), String> {
        let mut conn = self.get_connection().await?;
        let processing_key = format!("{}_processing", self.queue_name);
        let result: Option<Vec<u8>> = redis::cmd("HGET").arg(&processing_key).arg(job_id).query_async(&mut conn).await.map_err(|e| e.to_string())?;
        if let Some(payload_bytes) = result {
            if let Ok(queue_job) = <::interop_proto::ohc::interop::QueueJob as prost::Message>::decode(&payload_bytes[..]) {
                if queue_job.tenant_id != tenant_id {
                    return Err("tenant mismatch".to_string());
                }
                let _: () = redis::cmd("HDEL").arg(&processing_key).arg(job_id).query_async(&mut conn).await.map_err(|e| e.to_string())?;
                return Ok(());
            }
        }
        Err("job not found".to_string())
    }

    async fn requeue(&self, job: Job) -> Result<(), String> {
        let queue_job = ::interop_proto::ohc::interop::QueueJob {
            id: job.id,
            tenant_id: job.tenant_id,
            parent_task_id: job.parent_task_id,
            agent_role: job.agent_role,
            payload: job.payload,
            status: job.status,
            attempts: job.attempts,
            max_attempts: job.max_attempts,
            run_after_ms: job.run_after.timestamp_millis(),
            locked_until_ms: job.locked_until.map(|dt| dt.timestamp_millis()).unwrap_or(0),
            created_at_ms: job.created_at.timestamp_millis(),
            updated_at_ms: job.updated_at.timestamp_millis(),
        };
        let payload_bytes = prost::Message::encode_to_vec(&queue_job);
        let mut conn = self.get_connection().await?;
        let _: () = redis::cmd("RPUSH")
            .arg(&self.queue_name)
            .arg(&payload_bytes)
            .query_async(&mut conn)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
