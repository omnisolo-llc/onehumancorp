use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a discrete unit of work to be executed by a sub-agent within the OHC ecosystem.
/// Jobs are multi-tenant aware and support exponential backoff for retries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    /// Unique identifier for the job (UUID).
    pub id: String,
    /// The organization/tenant ID to ensure data isolation.
    pub tenant_id: String,
    /// The ID of the parent orchestration task that spawned this job.
    pub parent_task_id: String,
    /// The specific role or function this job requires (e.g., "sales_agent", "operations").
    pub job_type: String,
    /// JSON-serialized payload containing the parameters and context for the job.
    pub payload: String,
    /// Current state of the job (e.g., 'PENDING', 'PROCESSING', 'COMPLETED', 'FAILED').
    pub status: String,
    /// Number of times this job has been attempted.
    pub retry_count: i32,
    /// Maximum number of allowed retry attempts before marking the job as 'FAILED'.
    pub max_retries: i32,
    /// Timestamp indicating the earliest time this job should be picked up (used for scheduling and backoff).
    pub next_retry_at: DateTime<Utc>,
    /// Optional timestamp indicating until when the job is locked by a worker.
    pub locked_until: Option<DateTime<Utc>>,
    /// Timestamp when the job was originally created.
    pub created_at: DateTime<Utc>,
    /// Timestamp of the last modification to this job record.
    pub updated_at: DateTime<Utc>,
}

/// Defines the contract for the sub-agent orchestration queue.
/// Implementations handle the storage and distribution of `Job`s to worker nodes,
/// utilizing mechanisms like PostgreSQL `SKIP LOCKED` or Redis depending on the deployment mode.
#[async_trait]
pub trait TaskQueue: Send + Sync {
    /// Adds a single new job to the queue.
    async fn enqueue(&self, job: Job) -> Result<(), String>;

    /// Adds multiple jobs to the queue in a single operation. Defaults to a naive loop but can be optimized.
    async fn enqueue_batch(&self, jobs: Vec<Job>) -> Result<(), String> { for job in jobs { self.enqueue(job).await?; } Ok(()) }

    /// Attempts to claim the next available pending job that matches the provided worker `roles`.
    /// The job will be locked (status updated to 'PROCESSING') to prevent other workers from claiming it.
    async fn dequeue(&self, roles: Vec<String>, estimated_vram: i64, estimated_tokens: i64) -> Result<Option<Job>, String>;

    /// Marks a job as successfully finished.
    async fn complete(&self, job_id: &str) -> Result<(), String>;

    /// Marks a job as failed and increments the retry counter, applying exponential backoff scheduling.
    async fn fail(&self, job_id: &str, reason: &str) -> Result<(), String>;
    async fn cleanup_stale_jobs(&self) -> Result<u64, String>;

}
