use async_trait::async_trait;
pub use crate::ohc::orchestration::Job;

#[async_trait]
pub trait TaskQueue: Send + Sync {
    async fn enqueue(&self, job: Job) -> Result<(), String>;
    async fn dequeue(&self, roles: Vec<String>) -> Result<Option<Job>, String>;
    async fn complete(&self, job_id: &str) -> Result<(), String>;
    async fn fail(&self, job_id: &str, reason: &str) -> Result<(), String>;
}
