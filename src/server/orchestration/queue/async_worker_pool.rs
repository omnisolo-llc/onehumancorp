use std::sync::Arc;
use tokio::task::JoinHandle;
use tokio::sync::broadcast;
use std::time::Duration;
use super::ohc_async_jobs::{OHCAsyncJobQueue, OHCAsyncJob};

pub trait AsyncJobHandler: Send + Sync {
    fn handle(&self, job: OHCAsyncJob) -> tokio::task::JoinHandle<Result<(), String>>;
}

pub struct AsyncWorkerPool {
    workers: Vec<JoinHandle<()>>,
    shutdown_tx: broadcast::Sender<()>,
}

impl AsyncWorkerPool {
    pub fn new(queue: Arc<OHCAsyncJobQueue>, num_workers: usize, event_types: Vec<String>, handler: Arc<dyn AsyncJobHandler>) -> Self {
        Self::new_with_timeout(queue, num_workers, event_types, handler, 60000)
    }

    pub fn new_with_timeout(queue: Arc<OHCAsyncJobQueue>, num_workers: usize, event_types: Vec<String>, handler: Arc<dyn AsyncJobHandler>, timeout_ms: u64) -> Self {
        let (shutdown_tx, _) = broadcast::channel(1);
        let mut workers = Vec::with_capacity(num_workers);

        for i in 0..num_workers {
            let mut shutdown_rx = shutdown_tx.subscribe();
            let queue_clone = queue.clone();
            let types_clone = event_types.clone();
            let handler_clone = handler.clone();

            let handle = tokio::spawn(async move {
                tracing::info!("Async Worker {} started listening for {:?}", i, types_clone);
                let mut current_sleep_ms = 10;

                loop {
                    tokio::select! {
                        _ = shutdown_rx.recv() => {
                            tracing::info!("Async Worker {} shutting down", i);
                            break;
                        }

                        _ = tokio::time::sleep(Duration::from_millis(current_sleep_ms)) => {
                            let type_strs: Vec<&str> = types_clone.iter().map(AsRef::as_ref).collect();
                            match queue_clone.dequeue(type_strs).await {
                                Ok(Some(job)) => {
                                    current_sleep_ms = 10;
                                    tracing::debug!("Async Worker {} processing job {}", i, job.id);
                                    let job_id = job.id.clone();

                                    let mut join_handle = handler_clone.handle(job);
                                    let result = tokio::time::timeout(Duration::from_millis(timeout_ms), &mut join_handle).await;

                                    match result {
                                        Ok(Ok(Ok(()))) => {
                                            if let Err(e) = queue_clone.complete(&job_id).await {
                                                tracing::trace!("Failed to complete job {}: {}", job_id, e);
                                            }
                                        }
                                        Ok(Ok(Err(e))) => {
                                            tracing::trace!("Job handler returned error for {}: {}", job_id, e);
                                            if let Err(fail_err) = queue_clone.fail(&job_id, 3, &e).await {
                                                tracing::trace!("Failed to register fail for job {}: {}", job_id, fail_err);
                                            }
                                        }
                                        Ok(Err(e)) => {
                                            tracing::trace!("Job {} panicked/join error: {}", job_id, e);
                                            if let Err(fail_err) = queue_clone.fail(&job_id, 3, &e.to_string()).await {
                                                tracing::trace!("Failed to register fail for job {}: {}", job_id, fail_err);
                                            }
                                        }
                                        Err(_) => {
                                            tracing::trace!("Job {} timed out after {} ms", job_id, timeout_ms);
                                            join_handle.abort();
                                            if let Err(fail_err) = queue_clone.fail(&job_id, 3, "Execution exceeded timeout rule").await {
                                                tracing::trace!("Failed to register fail for timed out job {}: {}", job_id, fail_err);
                                            }
                                        }
                                    }
                                }
                                Ok(None) => {
                                    current_sleep_ms = std::cmp::min(current_sleep_ms * 2, 500);
                                }
                                Err(_) => {
                                    current_sleep_ms = std::cmp::min(current_sleep_ms * 2, 500);
                                }
                            }
                        }
                    }
                }
            });

            workers.push(handle);
        }

        Self {
            workers,
            shutdown_tx,
        }
    }

    pub async fn shutdown(self) {
        let _ = self.shutdown_tx.send(());
        for worker in self.workers {
            let _ = worker.await;
        }
    }
}
