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

                ::server_telemetry::record_token_usage(agent_id, role, model, t_type, count);

                // Track cost dynamically
                let is_telemetry_enabled = ::server_config::get().telemetry_enabled;
                if is_telemetry_enabled {
                    let cost_per_1k = match model {
                        "claude-3-opus" => 15.0,
                        "claude-3-sonnet" => 3.0,
                        "claude-3-haiku" => 0.25,
                        "gpt-4" => 30.0,
                        "gpt-4o" => 5.0,
                        "gpt-3.5-turbo" => 0.5,
                        _ => 1.0,
                    };
                    let cost_usd = (count as f64 / 1000.0) * cost_per_1k;

                    let model_string = model.to_string();
                    let tenant_id = item
                        .labels
                        .get("tenant_id")
                        .or_else(|| item.labels.get("organization_id"))
                        .and_then(Value::as_str)
                        .unwrap_or("unknown_tenant")
                        .to_string();
                    tokio::spawn(async move {
                        let pool = crate::db::get_pool();
                        let cost_cents = (cost_usd as f64 * 100.0).round() as i64;
                        let _ = ::server_telemetry::record_llm_call_cost(&pool, &tenant_id, &model_string, cost_cents).await;

                        let labels_cents = serde_json::json!({
                            "tenant_id": tenant_id.clone(),
                            "model": model_string.clone()
                        });
                        let _ = ::server_telemetry::buffer_metric_i64(&pool, "ohc_mission_cost_cents", "counter", cost_cents, labels_cents).await;
                    });
                }
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
                ::server_telemetry::record_agent_api_call(agent_id, role, api);

                let is_telemetry_enabled = ::server_config::get().telemetry_enabled;
                if is_telemetry_enabled {
                    let cost_usd = 0.001; // Example fixed cost for external API call
                    let api_string = api.to_string();
                    let tenant_id = item
                        .labels
                        .get("tenant_id")
                        .or_else(|| item.labels.get("organization_id"))
                        .and_then(Value::as_str)
                        .unwrap_or("unknown_tenant")
                        .to_string();
                    tokio::spawn(async move {
                        let pool = crate::db::get_pool();
                        let cost_cents = (cost_usd as f64 * 100.0).round() as i64;
                        let _ = ::server_telemetry::record_outbound_api_cost(&pool, &tenant_id, &api_string, cost_cents).await;

                        let labels_cents = serde_json::json!({
                            "tenant_id": tenant_id.clone(),
                            "api": api_string.clone()
                        });
                        let _ = ::server_telemetry::buffer_metric_i64(&pool, "ohc_mission_cost_cents", "counter", cost_cents, labels_cents).await;
                    });
                }
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
                ::server_telemetry::record_agent_api_error(agent_id, role, api);
            }
            "human_interaction" => {
                let i_type = item
                    .labels
                    .get("type")
                    .and_then(Value::as_str)
                    .unwrap_or("");
                ::server_telemetry::record_human_interaction(i_type);
            }
            "meeting_event" => {
                let e_type = item
                    .labels
                    .get("event_type")
                    .or_else(|| item.labels.get("type"))
                    .and_then(Value::as_str)
                    .unwrap_or("");
                ::server_telemetry::record_meeting_event(e_type);
            }
            "swarm_task_completed" => {
                let mission_id = item
                    .labels
                    .get("mission_id")
                    .and_then(Value::as_str)
                    .unwrap_or("");
                ::server_telemetry::record_swarm_task_completed(mission_id);
            }
            _ => {
                let is_telemetry_enabled = ::server_config::get().telemetry_enabled;
                if is_telemetry_enabled {
                    let pool = crate::db::get_pool();
                    let _ = ::server_telemetry::buffer_metric(&pool, &item.metric_name, &item.metric_type, item.value, item.labels.clone()).await;
                }
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
