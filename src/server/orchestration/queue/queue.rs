use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub parent_task_id: String,
    pub agent_role: String,
    pub payload: String,
    pub status: String,
    pub attempts: i32,
    pub max_attempts: i32,
    pub run_after: DateTime<Utc>,
    pub locked_until: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[async_trait]
pub trait TaskQueue: Send + Sync {
    async fn enqueue(&self, job: Job) -> Result<(), String>;
    async fn dequeue(&self, roles: Vec<String>) -> Result<Option<Job>, String>;
    async fn complete(&self, job_id: &str) -> Result<(), String>;
    async fn fail(&self, job_id: &str, reason: &str) -> Result<(), String>;
}
