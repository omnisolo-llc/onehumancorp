use axum::{Json, http::StatusCode, response::IntoResponse};
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

pub async fn sync_telemetry_handler(Json(batch): Json<Vec<MetricBatchItem>>) -> impl IntoResponse {
    tracing::debug!("Received telemetry batch with {} items", batch.len());

    for item in batch {
        match item.metric_type.as_str() {
            "token_usage" => {
                let agent_id = item
                    .labels
                    .get("agent_id")
                    .and_then(Value::as_str)
                    .unwrap_or("");
                let role = item
                    .labels
                    .get("role")
                    .and_then(Value::as_str)
                    .unwrap_or("");
                let model = item
                    .labels
                    .get("model")
                    .and_then(Value::as_str)
                    .unwrap_or("");
                let t_type = item
                    .labels
                    .get("type")
                    .and_then(Value::as_str)
                    .unwrap_or("");
                let count = item
                    .labels
                    .get("count")
                    .and_then(|v| v.as_f64().map(|f| f as i64).or(v.as_i64()))
                    .unwrap_or(item.value as i64);

                crate::telemetry::record_token_usage(agent_id, role, model, t_type, count);
            }
            "agent_api_call" => {
                let agent_id = item
                    .labels
                    .get("agent_id")
                    .and_then(Value::as_str)
                    .unwrap_or("");
                let role = item
                    .labels
                    .get("role")
                    .and_then(Value::as_str)
                    .unwrap_or("");
                let api = item.labels.get("api").and_then(Value::as_str).unwrap_or("");
                crate::telemetry::record_agent_api_call(agent_id, role, api);
            }
            "agent_api_error" => {
                let agent_id = item
                    .labels
                    .get("agent_id")
                    .and_then(Value::as_str)
                    .unwrap_or("");
                let role = item
                    .labels
                    .get("role")
                    .and_then(Value::as_str)
                    .unwrap_or("");
                let api = item.labels.get("api").and_then(Value::as_str).unwrap_or("");
                crate::telemetry::record_agent_api_error(agent_id, role, api);
            }
            "human_interaction" => {
                let i_type = item
                    .labels
                    .get("type")
                    .and_then(Value::as_str)
                    .unwrap_or("");
                crate::telemetry::record_human_interaction(i_type);
            }
            "meeting_event" => {
                let e_type = item
                    .labels
                    .get("event_type")
                    .or_else(|| item.labels.get("type"))
                    .and_then(Value::as_str)
                    .unwrap_or("");
                crate::telemetry::record_meeting_event(e_type);
            }
            "swarm_task_completed" => {
                let mission_id = item
                    .labels
                    .get("mission_id")
                    .and_then(Value::as_str)
                    .unwrap_or("");
                crate::telemetry::record_swarm_task_completed(mission_id);
            }
            _ => {
                // Ignore other metrics in cloud
                tracing::trace!(
                    "Ingesting metric: {} = {} at {}",
                    item.metric_name,
                    item.value,
                    item.timestamp
                );
            }
        }
    }

    StatusCode::OK
}
