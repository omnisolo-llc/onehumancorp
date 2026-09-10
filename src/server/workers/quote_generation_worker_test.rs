use std::sync::Arc;
use sqlx::PgPool;
use uuid::Uuid;
use crate::orchestration::queue::omnisolo_job_queue::OmniSoloJob;
use crate::workers::quote_generation_worker::{QuoteGenerationWorker, QuoteGenerationPayload};

#[tokio::test]
async fn test_quote_generation_worker_creates_feed_item() {
    let pool = crate::db::tests::get_test_db_pool().await;
    let tenant_id = "test-quote-generation-tenant";

    // Create quote entity
    let quote_id = Uuid::new_v4();
    sqlx::query("INSERT INTO quotes (id, tenant_id, status) VALUES ($1, $2, 'NEW')")
        .bind(quote_id)
        .bind(tenant_id)
        .execute(&pool)
        .await
        .unwrap();

    let payload = QuoteGenerationPayload {
        is_proposal: false,
        inquiry: "Test inquiry for plumbing".to_string(),
        customer_id: Uuid::new_v4().to_string(),
        entity_id: quote_id.to_string(),
    };

    let job = OmniSoloJob {
        id: "test-job-123".to_string(),
        tenant_id: tenant_id.to_string(),
        job_type: "quote_generation".to_string(),
        payload: serde_json::to_string(&payload).unwrap(),
        status: "pending".to_string(),
        run_after: chrono::Utc::now(),
        retry_count: 0,
        last_error: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let worker = QuoteGenerationWorker { pool: Arc::new(pool.clone()) };
    worker.do_handle(job).await.unwrap();

    // Verify quote is DRAFT
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM quotes WHERE id = $1 AND status = 'DRAFT'")
        .bind(quote_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 1);

    // Verify agent feed item was created
    let feed_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM agent_feed_items WHERE tenant_id = $1 AND lifecycle_state = 'PENDING_APPROVAL'")
        .bind(tenant_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(feed_count, 1);
}
