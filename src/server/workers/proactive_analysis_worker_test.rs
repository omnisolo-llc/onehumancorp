use std::sync::Arc;

#[tokio::test]
async fn test_proactive_analysis_worker_poll() {
    let _ = tracing_subscriber::fmt::try_init();

    if std::env::var("OHC_DATABASE_URL").is_err() && std::env::var("OHC_STANDALONE_MODE").unwrap_or_default() != "1" {
        return; // skip if not configured for tests
    }

    let db = crate::db::DB::new().await.unwrap();
    let pool = db.pool.clone();

    // Prepare tenant for testing
    let tenant_id = "test_tenant_proactive_analysis";
    let _ = sqlx::query("INSERT INTO tenants (id, name, tier) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING")
        .bind(tenant_id)
        .bind("Test Tenant")
        .bind("free")
        .execute(&pool)
        .await;

    // We just ensure poll does not panic and handles an empty run.
    let db_arc = Arc::new(db);
    let worker = crate::workers::proactive_analysis_worker::ProactiveAnalysisWorker::new(db_arc.clone());

    let result = crate::workers::proactive_analysis_worker::ProactiveAnalysisWorker::poll(&worker.db).await;

    // It should succeed (the AI call will fail gracefully, but the poll loop completes)
    assert!(result.is_ok());

    // Clean up
    let _ = sqlx::query("DELETE FROM tenants WHERE id = $1")
        .bind(tenant_id)
        .execute(&pool)
        .await;
}
