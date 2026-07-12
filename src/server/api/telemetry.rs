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
                let is_telemetry_enabled = ::server_config::is_telemetry_enabled();
                if !is_telemetry_enabled {
                    continue;
                }
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
                if true {
                    let input_tokens = if t_type == "input" { count } else { 0 };
                    let output_tokens = if t_type == "output" { count } else { 0 };

                    let cost_usd = ::server_pricing::calculator::calculate_cost(model, input_tokens, output_tokens, 0);

                    let model_string = model.to_string();
                    let tenant_id = item
                        .labels
                        .get("tenant_id")
                        .or_else(|| item.labels.get("organization_id"))
                        .and_then(Value::as_str)
                        .unwrap_or("unknown_tenant")
                        .to_string();
                    let count_clone = count;
                    let model_clone = model_string.clone();
                    tokio::spawn(async move {
                        let pool = crate::db::get_pool();
                        let _ = ::server_telemetry::record_llm_call_cost(&pool, &tenant_id, &model_string, cost_usd).await;
                        let cost_cents = (cost_usd * 100.0).round() as i64;
                        let labels_cents = serde_json::json!({
                            "tenant_id": tenant_id.clone(),
                            "model": model_string.clone()
                        });
                        let _ = ::server_telemetry::buffer_metric_i64(&pool, "ohc_llm_cost_total_cents", "counter", cost_cents, labels_cents).await;

                        if let Ok(redis_url) = std::env::var("REDIS_URL") {
                            if let Ok(client) = redis::Client::open(redis_url) {
                                let limiter = ::server_pricing::rate_limit::RedisRateLimiter::new(client);
                                let _ = limiter.record_token_usage(&tenant_id, &model_clone, count_clone).await;
                            }
                        }
                    });
                }
            }
            "agent_api_call" => {
                let is_telemetry_enabled = ::server_config::is_telemetry_enabled();
                if !is_telemetry_enabled {
                    continue;
                }
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

                if true {
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
                        let _ = ::server_telemetry::record_outbound_api_cost(&pool, &tenant_id, &api_string, cost_usd).await;
                        let cost_cents = (cost_usd * 100.0).round() as i64;
                        let labels_cents = serde_json::json!({
                            "tenant_id": tenant_id.clone(),
                            "api": api_string.clone()
                        });
                        let _ = ::server_telemetry::buffer_metric_i64(&pool, "ohc_llm_cost_total_cents", "counter", cost_cents, labels_cents).await;
                    });
                }
            }
            "agent_api_error" => {
                let is_telemetry_enabled = ::server_config::is_telemetry_enabled();
                if !is_telemetry_enabled {
                    continue;
                }
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
                let is_telemetry_enabled = ::server_config::is_telemetry_enabled();
                if !is_telemetry_enabled {
                    continue;
                }
                let i_type = item
                    .labels
                    .get("type")
                    .and_then(Value::as_str)
                    .unwrap_or("");
                ::server_telemetry::record_human_interaction(i_type);
            }
            "meeting_event" => {
                let is_telemetry_enabled = ::server_config::is_telemetry_enabled();
                if !is_telemetry_enabled {
                    continue;
                }
                let e_type = item
                    .labels
                    .get("event_type")
                    .or_else(|| item.labels.get("type"))
                    .and_then(Value::as_str)
                    .unwrap_or("");
                ::server_telemetry::record_meeting_event(e_type);
            }
            "swarm_task_completed" => {
                let is_telemetry_enabled = ::server_config::is_telemetry_enabled();
                if !is_telemetry_enabled {
                    continue;
                }
                let mission_id = item
                    .labels
                    .get("mission_id")
                    .and_then(Value::as_str)
                    .unwrap_or("");
                ::server_telemetry::record_swarm_task_completed(mission_id);
            }
            _ => {
                let is_telemetry_enabled = ::server_config::is_telemetry_enabled();
                if !is_telemetry_enabled {
                    continue;
                }
                let pool = crate::db::get_pool();
                let redacted_labels = ::server_telemetry::redact_interface_pii(item.labels.clone());
                let _ = ::server_telemetry::buffer_metric(&pool, &item.metric_name, &item.metric_type, item.value, redacted_labels).await;

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
