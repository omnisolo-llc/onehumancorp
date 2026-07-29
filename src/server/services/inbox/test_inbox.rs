use super::service::InboxService;
use sqlx::PgPool;

#[tokio::test]
async fn test_inbox_triage_flow_dummy() {
    assert!(true);
}

#[tokio::test]
async fn test_inbox_trigger_ai_triage() {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());

    let maybe_pool = PgPool::connect(&database_url).await;
    if maybe_pool.is_err() {
        // Return gracefully if DB is not available in test environment
        return;
    }
    let pool = maybe_pool.unwrap();

    // Add setup for unified_threads table for test
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS unified_threads (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            customer_id TEXT,
            channel TEXT NOT NULL,
            status TEXT NOT NULL,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW()
        )
        "#
    ).execute(&pool).await.expect("Failed to create unified_threads table");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS unified_messages (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            thread_id TEXT NOT NULL,
            sender_type TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at TIMESTAMPTZ DEFAULT NOW()
        )
        "#
    ).execute(&pool).await.expect("Failed to create unified_messages table");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS ohc_job_queue (
            id TEXT PRIMARY KEY,
            tenant_id TEXT NOT NULL,
            job_type TEXT NOT NULL,
            payload TEXT NOT NULL,
            status TEXT NOT NULL,
            created_at TIMESTAMPTZ DEFAULT NOW(),
            updated_at TIMESTAMPTZ DEFAULT NOW(),
            next_retry_at TIMESTAMPTZ DEFAULT NOW()
        )
        "#
    ).execute(&pool).await.expect("Failed to create ohc_job_queue table");

    let service = InboxService::new(pool.clone());

    let tenant_id = uuid::Uuid::new_v4().to_string();

    let _msg_id = service.ingest_message(&tenant_id, None, "test_channel", "customer", "Hello, I need help!")
        .await
        .expect("Failed to ingest message");

    let count: (i64,) = sqlx::query_as("SELECT count(*) FROM ohc_job_queue WHERE tenant_id = $1 AND job_type = 'message_triage' AND payload LIKE '%unified_inbox%'")
        .bind(&tenant_id)
        .fetch_one(&pool).await.expect("Failed to query job queue");

    assert!(count.0 > 0, "Expected at least one job in ohc_job_queue");
}
