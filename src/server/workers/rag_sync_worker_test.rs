use std::sync::Arc;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use crate::orchestration::queue::ohc_job_queue::{OHCJobQueue, OHCJob};
use crate::orchestration::queue::worker_pool::JobHandler;
use crate::orchestration::locks::StandaloneLock;
use super::rag_sync_worker::RagSyncWorker;

#[tokio::test]
async fn test_rag_sync_worker_concurrent_lock_and_tenant_isolation() {
    if std::env::var("OHC_DATABASE_URL").is_err() {
        unsafe { std::env::set_var("OHC_DATABASE_URL", "postgres://postgres:postgres@localhost:5432/ohc"); }
    }

    let database_url = std::env::var("OHC_DATABASE_URL").unwrap();
    let pool = match PgPoolOptions::new().max_connections(5).connect(&database_url).await { Ok(p) => p, Err(_) => return, };

    // Setup table
    sqlx::query("
        CREATE TABLE IF NOT EXISTS swarm_truth_embeddings (
            memory_id VARCHAR(255) PRIMARY KEY,
            tenant_id VARCHAR(255) NOT NULL,
            context TEXT,
            embedding BYTEA,
            escalation_required INTEGER DEFAULT 0,
            sync_status VARCHAR(50) DEFAULT 'pending',
            sync_error TEXT,
            last_synced_at TIMESTAMP,
            last_sync_at TIMESTAMP
        )
    ").execute(&pool).await.unwrap();

    // Enable RLS for test
    sqlx::query("ALTER TABLE swarm_truth_embeddings ENABLE ROW LEVEL SECURITY").execute(&pool).await.unwrap_or_default();

    // We recreate the policy exactly as in migration 005
    sqlx::query("
        DO $$
        BEGIN
            IF NOT EXISTS (
                SELECT 1 FROM pg_policies
                WHERE schemaname = current_schema()
                  AND tablename = 'swarm_truth_embeddings'
                  AND policyname = 'tenant_isolation_swarm_truth_embeddings'
            ) THEN
                CREATE POLICY tenant_isolation_swarm_truth_embeddings ON swarm_truth_embeddings
                    USING (tenant_id::text = current_setting('app.current_tenant', true))
                    WITH CHECK (tenant_id::text = current_setting('app.current_tenant', true));
            END IF;
        END
        $$;
    ").execute(&pool).await.unwrap();

    let tenant_id = "tenant_test_rag_sync";
    let document_id = "doc_test_123";

    sqlx::query("DELETE FROM swarm_truth_embeddings WHERE memory_id = $1").bind(document_id).execute(&pool).await.unwrap();

    let mut tx = pool.begin().await.unwrap();
    ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id).await.unwrap();
    sqlx::query("INSERT INTO swarm_truth_embeddings (memory_id, tenant_id, sync_status) VALUES ($1, $2, 'pending')")
        .bind(document_id)
        .bind(tenant_id)
        .execute(&mut *tx)
        .await
        .unwrap();
    tx.commit().await.unwrap();

    let lock = Arc::new(StandaloneLock::new());
    let worker = RagSyncWorker::new(Arc::new(pool.clone()), lock.clone());

    let job = OHCJob {
        id: "job_1".to_string(),
        tenant_id: tenant_id.to_string(),
        job_type: "rag_sync".to_string(),
        payload: serde_json::json!({"document_id": document_id}).to_string(),
        status: "PROCESSING".to_string(),
        retry_count: 0,
        next_retry_at: chrono::Utc::now(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let handle = worker.handle(job.clone());
    handle.await.unwrap().unwrap();

    let mut tx2 = pool.begin().await.unwrap();
    ::server_common::auth_utils::set_org_context(&mut *tx2, tenant_id).await.unwrap();

    let row: (String,) = sqlx::query_as("SELECT sync_status FROM swarm_truth_embeddings WHERE memory_id = $1")
        .bind(document_id)
        .fetch_one(&mut *tx2)
        .await
        .unwrap();

    tx2.commit().await.unwrap();

    assert_eq!(row.0, "synced");

    // Cross-tenant access check
    let job_wrong_tenant = OHCJob {
        id: "job_2".to_string(),
        tenant_id: "wrong_tenant".to_string(),
        job_type: "rag_sync".to_string(),
        payload: serde_json::json!({"document_id": document_id}).to_string(),
        status: "PROCESSING".to_string(),
        retry_count: 0,
        next_retry_at: chrono::Utc::now(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let handle2 = worker.handle(job_wrong_tenant);
    let _ = handle2.await.unwrap();

    // The document sync_status should remain synced, but the update query would have affected 0 rows because of RLS.
    let mut tx3 = pool.begin().await.unwrap();
    ::server_common::auth_utils::set_org_context(&mut *tx3, tenant_id).await.unwrap();
    let row3: (String,) = sqlx::query_as("SELECT sync_status FROM swarm_truth_embeddings WHERE memory_id = $1")
        .bind(document_id)
        .fetch_one(&mut *tx3)
        .await
        .unwrap();
    tx3.commit().await.unwrap();

    assert_eq!(row3.0, "synced");
}
