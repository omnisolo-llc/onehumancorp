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
use ::server_common::auth_utils::set_org_context;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]

pub struct Job {
    pub id: String,
    pub tenant_id: String,
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
    async fn enqueue_batch(&self, jobs: Vec<Job>) -> Result<(), String> { for job in jobs { self.enqueue(job).await?; } Ok(()) }
    async fn dequeue(&self, roles: Vec<String>) -> Result<Option<Job>, String>;
        async fn complete(&self, job_id: &str, tenant_id: &str) -> Result<(), String>;
    async fn fail(&self, job_id: &str, tenant_id: &str, reason: &str) -> Result<(), String>;
    async fn requeue(&self, job: Job) -> Result<(), String>;
}


#[async_trait]
pub trait TaskJobHandler: Send + Sync {
    async fn handle(&self, job: Job) -> Result<(), String>;
}


#[async_trait]
pub trait JobQueue: Send + Sync {
    async fn push(&self, topic: &str, payload: Vec<u8>) -> Result<(), String>;
    async fn pop(&self, topic: &str) -> Result<Vec<u8>, String>;
}


#[async_trait]
pub trait JobPayloadHandler: Send + Sync {
    async fn handle(&self, payload: Vec<u8>) -> Result<(), String>;
}


#[derive(Debug, Clone)]
pub struct SubAgentJob {
    pub id: String,
    pub tenant_id: String,
    pub parent_task_id: String,
    pub payload: serde_json::Value,
    pub status: String,
    pub worker_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}


#[derive(Debug, Clone)]
pub struct SharedTaskModel {
    pub id: String,
    pub tenant_id: String,
    pub parent_id: Option<String>,
    pub epic_id: Option<String>,
    pub title: String,
    pub status: String,
    pub assigned_agent: Option<String>,
    pub payload: serde_json::Value,
    pub dependencies: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
