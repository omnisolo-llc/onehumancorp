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
    println!("Received telemetry batch with {} items", batch.len());

    for item in batch {
        // In a real cloud environment, we would ingest this into Prometheus
        // For now, we simulate ingestion by logging
        println!("Ingesting metric: {} = {} at {}", item.metric_name, item.value, item.timestamp);

        // Emitting basic Opentelemetry metric events locally
        let meter = opentelemetry::global::meter("ohc_telemetry_sync");
        let mut attrs = Vec::new();
        if let Some(obj) = item.labels.as_object() {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    attrs.push(opentelemetry::KeyValue::new(k.clone(), s.to_string()));
                } else {
                    attrs.push(opentelemetry::KeyValue::new(k.clone(), v.to_string()));
                }
            }
        }

        if item.metric_type == "counter" {
            // we use build() to create it. We can't cache it easily here without once_cell in bazel build,
            // so this simulates local test observability. In a real cloud prod environment, metrics are cached per app context.
            let counter = meter.f64_counter(item.metric_name.clone()).build();
            counter.add(item.value as f64, &attrs);
        } else if item.metric_type == "gauge" {
            let gauge = meter.f64_up_down_counter(item.metric_name.clone()).build();
            gauge.add(item.value as f64, &attrs); // use up down counter to simulate gauge observability locally
        }
    }

    StatusCode::OK
}
