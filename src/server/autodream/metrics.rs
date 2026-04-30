use sqlx::PgPool;
use crate::telemetry::buffer_metric;

pub async fn record_autodream_batch(
    pool: &PgPool,
    mode: &str,
    processed_count: f32,
    duration_ms: f32,
    error_count: f32,
) {
    let labels = serde_json::json!({"mode": mode});

    let _ = buffer_metric(
        pool,
        "MemoriesProcessedTotal",
        "counter",
        processed_count,
        labels.clone(),
    ).await;

    let _ = buffer_metric(
        pool,
        "BatchProcessingDuration",
        "histogram",
        duration_ms,
        labels.clone(),
    ).await;

    let _ = buffer_metric(
        pool,
        "ConsolidationErrorsTotal",
        "counter",
        error_count,
        labels,
    ).await;
}
