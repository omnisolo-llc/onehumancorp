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


use super::models::*;
pub struct Worker {
    queue: Arc<dyn TaskQueue>,
    roles: Vec<String>,
    handler: Arc<dyn TaskJobHandler>,
}


impl Worker {
    pub fn new(queue: Arc<dyn TaskQueue>, roles: Vec<String>, handler: Arc<dyn TaskJobHandler>) -> Self {
        Worker { queue, roles, handler }
    }

    pub async fn start(&self, mut shutdown_rx: tokio::sync::broadcast::Receiver<()>) {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(100));

        loop {
            tokio::select! {
                _ = interval.tick() => {
                    match self.queue.dequeue(self.roles.clone()).await {
                        Ok(Some(job)) => {
                            tracing::debug!("Worker processing job: {}", job.id);
                            let handle_res = tokio::time::timeout(tokio::time::Duration::from_secs(60), self.handler.handle(job.clone())).await;
                            let handler_res = match handle_res {
                                Ok(Ok(())) => Ok(()),
                                Ok(Err(e)) => Err(e),
                                Err(_) => Err("Timeout executing job".to_string()),
                            };
                            match handler_res {
                                Ok(_) => {
                                    tracing::info!("Worker successfully processed job: {}", job.id);
                                    let _ = self.queue.complete(&job.id, &job.tenant_id).await;
                                }
                                Err(e) => {
                                    tracing::error!("Worker failed to process job: {}, error: {}", job.id, e);
                                    if job.attempts < job.max_attempts {
                                        let mut retry_job = job.clone();
                                        retry_job.attempts += 1;
                                        retry_job.status = "PENDING".to_string();
                                        retry_job.run_after = chrono::Utc::now() + chrono::Duration::seconds(5);
                                        let _ = self.queue.requeue(retry_job).await;
                                    } else {
                                        let _ = self.queue.fail(&job.id, &job.tenant_id, &e).await;
                                    }
                                }
                            }
                        }
                        Ok(None) => {
                            // No job available
                        }
                        Err(e) => {
                            tracing::error!("Worker failed to dequeue job: {}", e);
                        }
                    }
                }
                _ = shutdown_rx.recv() => {
                    tracing::info!("Worker shutting down");
                    break;
                }
            }
        }
    }
}


pub struct WorkerPool {
    queue: Arc<dyn JobQueue>,
    topic: String,
    handler: Arc<dyn JobPayloadHandler>,
    workers: usize,
}


impl WorkerPool {
    pub fn new(queue: Arc<dyn JobQueue>, topic: String, workers: usize, handler: Arc<dyn JobPayloadHandler>) -> Self {
        WorkerPool { queue, topic, handler, workers }
    }

    pub async fn start(&self, shutdown_rx: tokio::sync::broadcast::Sender<()>) {
        for i in 0..self.workers {
            let queue = self.queue.clone();
            let topic = self.topic.clone();
            let handler = self.handler.clone();
            let mut rx = shutdown_rx.subscribe();

            tokio::spawn(async move {
                tracing::info!("Worker {} starting", i);
                loop {
                    tokio::select! {
                        res = queue.pop(&topic) => {
                            match res {
                                Ok(payload) => {
                                    tracing::debug!("Worker {} processing job", i);
                                    if let Err(e) = handler.handle(payload).await {
                                        tracing::error!("Worker {} handler failed: {}", i, e);
                                    }
                                }
                                Err(e) => {
                                    tracing::error!("Worker {} failed to pop: {}", i, e);
                                }
                            }
                        }
                        _ = rx.recv() => {
                            tracing::info!("Worker {} shutting down", i);
                            break;
                        }
                    }
                }
            });
        }
    }
}
