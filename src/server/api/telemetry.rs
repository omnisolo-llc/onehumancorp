use axum::{Json, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct MetricBatchItem {
    pub metric_name: String,
    pub metric_type: String,
    pub value: f32,
    pub labels: Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

pub async fn sync_telemetry_handler(
    Json(batch): Json<Vec<MetricBatchItem>>,
) -> impl IntoResponse {
    tracing::debug!("Received telemetry batch with {} items", batch.len());

    for item in batch {
        // In a real cloud environment, we would ingest this into Prometheus
        // For now, we simulate ingestion by logging
        tracing::trace!("Ingesting metric: {} = {} at {}", item.metric_name, item.value, item.timestamp);
    }

    StatusCode::OK
}
