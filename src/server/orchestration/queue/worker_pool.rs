use std::sync::Arc;
use tokio::task::JoinHandle;
use tokio::sync::broadcast;
use std::time::Duration;
use super::ohc_job_queue::{OHCJobQueue, OHCJob};

pub trait JobHandler: Send + Sync {
    fn handle(&self, job: OHCJob) -> tokio::task::JoinHandle<Result<(), String>>;
}

pub struct WorkerPool {
    workers: Vec<JoinHandle<()>>,
    shutdown_tx: broadcast::Sender<()>,
}

impl WorkerPool {
    pub fn new(queue: Arc<OHCJobQueue>, num_workers: usize, job_types: Vec<String>, handler: Arc<dyn JobHandler>) -> Self {
        Self::new_with_timeout(queue, num_workers, job_types, handler, 60000)
    }

    pub fn new_with_timeout(queue: Arc<OHCJobQueue>, num_workers: usize, job_types: Vec<String>, handler: Arc<dyn JobHandler>, timeout_ms: u64) -> Self {
        let (shutdown_tx, _) = broadcast::channel(1);
        let mut workers = Vec::with_capacity(num_workers);

        for i in 0..num_workers {
            let mut shutdown_rx = shutdown_tx.subscribe();
            let queue_clone = queue.clone();
            let types_clone = job_types.clone();
            let handler_clone = handler.clone();

            let handle = tokio::spawn(async move {
                tracing::info!("Worker {} started listening for {:?}", i, types_clone);

                loop {
                    tokio::select! {
                        _ = shutdown_rx.recv() => {
                            tracing::info!("Worker {} shutting down", i);
                            break;
                        }

                        // Wait a bit before polling again to avoid busy-waiting loop without jobs
                        _ = tokio::time::sleep(Duration::from_millis(500)) => {
                            let type_strs: Vec<&str> = types_clone.iter().map(AsRef::as_ref).collect();
                            match queue_clone.dequeue(type_strs).await {
                                Ok(Some(job)) => {
                                    tracing::debug!("Worker {} processing job {}", i, job.id);
                                    let job_id = job.id.clone();

                                    // Process
                                    let mut join_handle = handler_clone.handle(job);
                                    let result = tokio::time::timeout(Duration::from_millis(timeout_ms), &mut join_handle).await;

                                    match result {
                                        Ok(Ok(Ok(()))) => {
                                            if let Err(e) = queue_clone.complete(&job_id).await {
                                                ::server_telemetry::record_error_signal("[bug] Failed to complete job");
                                                tracing::trace!("Failed to complete job {}: {}", job_id, e);
                                            }
                                        }
                                        Ok(Ok(Err(e))) => {
                                            ::server_telemetry::record_error_signal("[bug] Job handler returned error");
                                            tracing::trace!("Job handler returned error for {}: {}", job_id, e);
                                            if let Err(fail_err) = queue_clone.fail(&job_id, 3, &e).await {
                                                ::server_telemetry::record_error_signal("[bug] Failed to register fail for job");
                                                tracing::trace!("Failed to register fail for job {}: {}", job_id, fail_err);
                                            }
                                        }
                                        Ok(Err(e)) => {
                                            ::server_telemetry::record_error_signal("[bug] Job panicked/join error");
                                            tracing::trace!("Job {} panicked/join error: {}", job_id, e);
                                            if let Err(fail_err) = queue_clone.fail(&job_id, 3, &e.to_string()).await {
                                                ::server_telemetry::record_error_signal("[bug] Failed to register fail for job");
                                                tracing::trace!("Failed to register fail for job {}: {}", job_id, fail_err);
                                            }
                                        }
                                        Err(_) => {
                                            ::server_telemetry::record_error_signal("[bug] Job timed out");
                                            tracing::trace!("Job {} timed out after {} ms", job_id, timeout_ms);
                                            join_handle.abort();
                                            if let Err(fail_err) = queue_clone.fail(&job_id, 3, "Agent execution exceeded 60-second ML-Resilience timeout rule").await {
                                                ::server_telemetry::record_error_signal("[bug] Failed to register fail for timed out job");
                                                tracing::trace!("Failed to register fail for timed out job {}: {}", job_id, fail_err);
                                            }
                                        }
                                    }
                                }
                                Ok(None) => {
                                    // No jobs, loop back and sleep
                                }
                                Err(e) => {
                                    ::server_telemetry::record_error_signal("[bug] Worker failed to dequeue");
                                    tracing::trace!("Worker {} failed to dequeue: {}", i, e);
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
