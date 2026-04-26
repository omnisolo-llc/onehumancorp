use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Debug, Clone)]
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

pub struct MemoryTaskQueue {
    jobs: RwLock<HashMap<String, Job>>,
}

impl MemoryTaskQueue {
    pub fn new() -> Self {
        MemoryTaskQueue {
            jobs: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl TaskQueue for MemoryTaskQueue {
    async fn enqueue(&self, job: Job) -> Result<(), String> {
        let mut jobs = self.jobs.write().unwrap();
        jobs.insert(job.id.clone(), job);
        Ok(())
    }

    async fn dequeue(&self, roles: Vec<String>) -> Result<Option<Job>, String> {
        let mut jobs = self.jobs.write().unwrap();
        for job in jobs.values_mut() {
            if job.status == "PENDING" && roles.contains(&job.agent_role) {
                job.status = "IN_PROGRESS".to_string();
                job.updated_at = Utc::now();
                return Ok(Some(job.clone()));
            }
        }
        Ok(None)
    }

    async fn complete(&self, job_id: &str) -> Result<(), String> {
        let mut jobs = self.jobs.write().unwrap();
        if let Some(job) = jobs.get_mut(job_id) {
            job.status = "COMPLETED".to_string();
            job.updated_at = Utc::now();
            Ok(())
        } else {
            Err("job not found".to_string())
        }
    }

    async fn fail(&self, job_id: &str, reason: &str) -> Result<(), String> {
        let mut jobs = self.jobs.write().unwrap();
        if let Some(job) = jobs.get_mut(job_id) {
            job.status = "FAILED".to_string();
            job.payload = format!("{} (Reason: {})", job.payload, reason);
            job.updated_at = Utc::now();
            Ok(())
        } else {
            Err("job not found".to_string())
        }
    }
}
