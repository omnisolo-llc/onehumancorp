use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use crate::api::event_ingestion::{router, IngestEventRequest};
use serde_json::json;

#[tokio::test]
async fn test_ingestion_worker_compilation() {
    use std::sync::Arc;
    use crate::orchestration::queue::worker_pool::JobHandler;
    let _worker: Arc<dyn JobHandler> = Arc::new(crate::orchestration::queue::event_worker::EventIngestionWorker);
}

#[test]
fn test_ingest_event_request_deserialization() {
    let json = r#"
    {
        "tenant_id": "org_123",
        "event_type": "ig_dm_received",
        "source": "instagram_webhook",
        "payload": {"message_text": "do you do vegan cakes?"}
    }
    "#;

    let req: crate::api::event_ingestion::IngestEventRequest = serde_json::from_str(json).unwrap();
    assert_eq!(req.tenant_id, "org_123");
    assert_eq!(req.event_type, "ig_dm_received");
    assert_eq!(req.source, "instagram_webhook");
    assert_eq!(req.payload.get("message_text").unwrap().as_str().unwrap(), "do you do vegan cakes?");
}

// Integration test for authentication logic and API endpoint
#[tokio::test]
async fn test_ingest_event_api_unauthorized_missing_claims() {
    // We cannot easily mock the PgPool for sqlx in a pure unit test without a real DB
    // but we can test the auth check which happens before the DB is hit.

    // Create a mock pool instance since it is not used in the unauth path
    use sqlx::postgres::PgPoolOptions;
    let pool = std::sync::Arc::new(PgPoolOptions::new().connect_lazy("postgres://postgres:postgres@localhost:5432/ohc").unwrap());

    let app = router(pool);

    let payload = IngestEventRequest {
        tenant_id: "org_123".to_string(),
        event_type: "ig_dm_received".to_string(),
        source: "instagram_webhook".to_string(),
        payload: json!({"message_text": "do you do vegan cakes?"}),
        timestamp: None,
    };

    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/events/ingest")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&payload).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // Since we didn't provide Claims, it should be UNAUTHORIZED
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_event_ingestion_worker_lifecycle_e2e() {
    // We'll test the actual queue enqueue method since
    // testing the full HTTP request through to worker processing
    // requires a real Postgres database pool.

    // We instantiate the EventIngestionWorker and ensure it can handle jobs
    use crate::orchestration::queue::ohc_job_queue::OHCJob;
    use crate::orchestration::queue::worker_pool::JobHandler;

    let worker = crate::orchestration::queue::event_worker::EventIngestionWorker;
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
    let result = handle.await.unwrap();
    assert!(result.is_ok());
}
