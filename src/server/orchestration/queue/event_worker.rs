use tokio::time::Duration;
use crate::orchestration::queue::ohc_job_queue::OHCJob;
use crate::orchestration::queue::worker_pool::JobHandler;

pub struct EventIngestionWorker;

impl JobHandler for EventIngestionWorker {
    fn handle(&self, job: OHCJob) -> tokio::task::JoinHandle<Result<(), String>> {
        tokio::spawn(async move {
            tracing::info!(
                "EventIngestionWorker dequeued job {}: tenant_id={}, event_type={}",
                job.id,
                job.tenant_id,
                job.job_type
            );

            tokio::time::sleep(Duration::from_millis(50)).await;

            tracing::info!("EventIngestionWorker successfully processed job {}", job.id);
            Ok(())
        })
    }
}
