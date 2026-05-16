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
pub struct MemoryTaskQueue {
    jobs: DashMap<String, Job>,
    role_queues: DashMap<String, Mutex<VecDeque<String>>>,
}


impl MemoryTaskQueue {
    pub fn new() -> Self {
        MemoryTaskQueue {
            jobs: DashMap::new(),
            role_queues: DashMap::new(),
        }
    }
}


#[async_trait]
impl TaskQueue for MemoryTaskQueue {
    async fn enqueue_batch(&self, jobs: Vec<Job>) -> Result<(), String> {
        for job in jobs {
            self.jobs.insert(job.id.clone(), job);
        }
        Ok(())
    }

    async fn enqueue(&self, job: Job) -> Result<(), String> {
        let role = job.agent_role.clone();
        let id = job.id.clone();
        self.jobs.insert(id.clone(), job);

        let queue = self.role_queues.entry(role).or_insert_with(|| Mutex::new(VecDeque::new()));
        let mut q = queue.lock().unwrap();
        q.push_back(id);

        Ok(())
    }

    async fn dequeue(&self, roles: Vec<String>) -> Result<Option<Job>, String> {
        for role in roles {
            if let Some(queue) = self.role_queues.get(&role) {
                let mut q = queue.lock().unwrap();
                // Pop until we find a valid pending job, or queue is empty
                while let Some(job_id) = q.pop_front() {
                    if let Some(mut job_ref) = self.jobs.get_mut(&job_id) {
                        if job_ref.status == "PENDING" {
                            job_ref.status = "IN_PROGRESS".to_string();
                            job_ref.updated_at = Utc::now();
                            return Ok(Some(job_ref.clone()));
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    async fn complete(&self, job_id: &str, tenant_id: &str) -> Result<(), String> {
        if let Some(mut job) = self.jobs.get_mut(job_id) {
            if job.tenant_id != tenant_id {
                return Err("tenant mismatch".to_string());
            }
            job.status = "COMPLETED".to_string();
            job.updated_at = Utc::now();
            Ok(())
        } else {
            Err("job not found".to_string())
        }
    }

    async fn fail(&self, job_id: &str, tenant_id: &str, reason: &str) -> Result<(), String> {
        if let Some(mut job) = self.jobs.get_mut(job_id) {
            if job.tenant_id != tenant_id {
                return Err("tenant mismatch".to_string());
            }
            job.status = "FAILED".to_string();
            job.payload = format!("{} (Reason: {})", job.payload, reason);
            job.updated_at = Utc::now();
            Ok(())
        } else {
            Err("job not found".to_string())
        }
    }

    async fn requeue(&self, job: Job) -> Result<(), String> {
        let role = job.agent_role.clone();
        let id = job.id.clone();
        self.jobs.insert(id.clone(), job);

        let queue = self.role_queues.entry(role).or_insert_with(|| Mutex::new(VecDeque::new()));
        let mut q = queue.lock().unwrap();
        q.push_back(id);
        Ok(())
    }
}


pub struct InMemJobQueue {
    topics: DashMap<String, (mpsc::Sender<Vec<u8>>, Arc<tokio::sync::Mutex<mpsc::Receiver<Vec<u8>>>>)>,
}


impl InMemJobQueue {
    pub fn new() -> Self {
        InMemJobQueue {
            topics: DashMap::new(),
        }
    }

    fn get_or_create_topic(&self, topic: &str) -> (mpsc::Sender<Vec<u8>>, Arc<tokio::sync::Mutex<mpsc::Receiver<Vec<u8>>>>) {
        if let Some(t) = self.topics.get(topic) {
            return t.value().clone();
        }

        let (tx, rx) = mpsc::channel(10000);
        let rx = Arc::new(tokio::sync::Mutex::new(rx));
        let t = (tx, rx);
        self.topics.insert(topic.to_string(), t.clone());
        t
    }
}


#[async_trait]
impl JobQueue for InMemJobQueue {
    async fn push(&self, topic: &str, payload: Vec<u8>) -> Result<(), String> {
        let (tx, _) = self.get_or_create_topic(topic);
        tx.send(payload).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn pop(&self, topic: &str) -> Result<Vec<u8>, String> {
        let (_, rx) = self.get_or_create_topic(topic);
        let mut rx = rx.lock().await;
        rx.recv().await.ok_or_else(|| "channel closed".to_string())
    }
}
