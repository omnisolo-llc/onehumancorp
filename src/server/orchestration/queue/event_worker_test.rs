use crate::orchestration::queue::ohc_job_queue::OHCJob;
use crate::orchestration::queue::worker_pool::JobHandler;
use crate::orchestration::queue::event_worker::EventIngestionWorker;

#[tokio::test]
async fn test_event_ingestion_worker_handles_job() {
    let worker = EventIngestionWorker;
    let job = OHCJob {
        id: "test_job_123".to_string(),
        tenant_id: "org_123".to_string(),
        job_type: "event_ingestion".to_string(),
        payload: r#"{"original_event_type": "test_event", "data": "test"}"#.to_string(),
        status: "PENDING".to_string(),
        retry_count: 0,
        next_retry_at: chrono::Utc::now(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let handle = worker.handle(job);

    // Await the task completion
    let result = handle.await.unwrap();

    // Verify it succeeded
    assert!(result.is_ok());
}
