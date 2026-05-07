use serde_json::{Value, Map};
use sqlx::{PgPool, query};
use chrono::Utc;
use std::sync::OnceLock;
use opentelemetry::global;
use opentelemetry::metrics::UpDownCounter;

static SUB_AGENT_QUEUE_LENGTH_GAUGE: OnceLock<UpDownCounter<i64>> = OnceLock::new();

pub fn get_deployment_mode() -> &'static str {
    static DEPLOYMENT_MODE: OnceLock<String> = OnceLock::new();
    DEPLOYMENT_MODE.get_or_init(|| {
        if std::env::var("OHC_MULTITENANT").unwrap_or_else(|_| "false".to_string()) == "true" {
            "Cloud".to_string()
        } else {
            "Standalone".to_string()
        }
    })
}
pub fn get_queue_length_gauge() -> &'static UpDownCounter<i64> {
    SUB_AGENT_QUEUE_LENGTH_GAUGE.get_or_init(|| {
        let meter = global::meter("ohc.sub_agent");
        meter.i64_up_down_counter("ohc.sub_agent.queue_length")
            .with_description("The current number of jobs in the sub-agent task queue")
            .build()
    })
}

pub async fn record_autodream_sync(pool: &PgPool, count: f32) -> Result<(), Box<dyn std::error::Error>> {
    buffer_metric(pool, "autodream_records_synced_total", "counter", count, serde_json::json!({})).await
}

pub async fn record_autodream_sync_error(pool: &PgPool, count: f32, error_type: &str) -> Result<(), Box<dyn std::error::Error>> {
    buffer_metric(pool, "autodream_sync_errors_total", "counter", count, serde_json::json!({ "error": error_type })).await
}

pub async fn record_sync_escalation(pool: &PgPool, count: f32, mode: &str) -> Result<(), Box<dyn std::error::Error>> {
    buffer_metric(pool, "sync_escalation_total", "counter", count, serde_json::json!({ "mode": mode })).await
}

pub async fn record_sync_daemon_batch_size(pool: &PgPool, count: f32, mode: &str) -> Result<(), Box<dyn std::error::Error>> {
    buffer_metric(pool, "sync_daemon_batch_size", "gauge", count, serde_json::json!({ "mode": mode })).await
}

pub async fn record_sync_latency(pool: &PgPool, latency_ms: f32, mode: &str) -> Result<(), Box<dyn std::error::Error>> {
    buffer_metric(pool, "sync_latency_ms", "histogram", latency_ms, serde_json::json!({ "mode": mode })).await
}

pub async fn record_sync_payload_size(pool: &PgPool, size_bytes: f32, mode: &str) -> Result<(), Box<dyn std::error::Error>> {
    buffer_metric(pool, "sync_payload_size_bytes", "histogram", size_bytes, serde_json::json!({ "mode": mode })).await
}

pub async fn record_sync_daemon_error_total(pool: &PgPool, count: f32, mode: &str, error_type: &str) -> Result<(), Box<dyn std::error::Error>> {
    buffer_metric(pool, "sync_daemon_error_total", "counter", count, serde_json::json!({ "mode": mode, "error": error_type })).await
}


pub async fn record_sqlite_lock_contention(pool: &PgPool, operation: &str) -> Result<(), Box<dyn std::error::Error>> {
    buffer_metric(pool, "ohc_sqlite_lock_contention_total", "counter", 1.0, serde_json::json!({ "operation": operation })).await
}

pub async fn record_sqlite_retry_exhausted(pool: &PgPool, operation: &str) -> Result<(), Box<dyn std::error::Error>> {
    buffer_metric(pool, "ohc_sqlite_retry_exhausted_total", "counter", 1.0, serde_json::json!({ "operation": operation })).await
}

pub async fn record_queue_length(pool: &PgPool, delta: i32) -> Result<(), Box<dyn std::error::Error>> {
    let deployment_mode = get_deployment_mode();

    get_queue_length_gauge().add(delta as i64, &[opentelemetry::KeyValue::new("deployment_mode", deployment_mode)]);
    let payload = serde_json::json!({ "delta": delta, "deployment_mode": deployment_mode });

    buffer_metric(pool, "ohc_sub_agent_queue_length", "gauge", delta as f32, payload).await
}

pub async fn record_token_usage_forecast(pool: &PgPool, org_id: &str, forecast: f32) -> Result<(), Box<dyn std::error::Error>> {
    buffer_metric(pool, "ohc_token_usage_forecast", "gauge", forecast, serde_json::json!({ "organization_id": org_id })).await
}

pub async fn record_agent_cost(pool: &PgPool, agent_id: &str, organization_id: &str, role: &str, model: &str, entity: &str, cost: f64) -> Result<(), Box<dyn std::error::Error>> {
    buffer_metric(
        pool,
        "ohc_agent_cost",
        "counter",
        cost as f32,
        serde_json::json!({
            "agent_id": agent_id,
            "organization_id": organization_id,
            "role": role,
            "model": model,
            "entity": entity,
        }),
    )
    .await
}

pub async fn record_api_call_cost(pool: &PgPool, organization_id: &str, entity: &str, cost: f64) -> Result<(), Box<dyn std::error::Error>> {
    buffer_metric(
        pool,
        "ohc_api_call_cost",
        "counter",
        cost as f32,
        serde_json::json!({
            "organization_id": organization_id,
            "entity": entity,
        }),
    )
    .await
}

pub async fn record_swarm_job_latency_by_entity(pool: &PgPool, mode: &str, entity: &str, latency: f64) -> Result<(), Box<dyn std::error::Error>> {
    buffer_metric(
        pool,
        "ohc_swarm_job_latency_by_entity_seconds",
        "histogram",
        latency as f32,
        serde_json::json!({
            "mode": mode,
            "entity": entity,
        }),
    )
    .await
}


pub async fn record_token_budget_alert(pool: &PgPool, org_id: &str, alert_type: &str) -> Result<(), Box<dyn std::error::Error>> {
    buffer_metric(pool, "ohc_token_budget_alert_total", "counter", 1.0, serde_json::json!({ "organization_id": org_id, "alert_type": alert_type })).await
}



pub async fn record_capability_violation(pool: &PgPool, agent_id: &str, capability: &str) -> Result<(), Box<dyn std::error::Error>> {
    buffer_metric(pool, "capability_violation_total", "counter", 1.0, serde_json::json!({ "agent_id": agent_id, "capability": capability })).await
}



pub async fn record_rag_escalation(pool: &PgPool, org_id: &str, error: &str) -> Result<(), Box<dyn std::error::Error>> {
    buffer_metric(pool, "ohc_rag_escalation_total", "counter", 1.0, serde_json::json!({ "organization_id": org_id, "error": error })).await
}


pub async fn buffer_metric(
    pool: &PgPool,
    metric_name: &str,
    metric_type: &str,
    value: f32,
    labels: Value,
) -> Result<(), Box<dyn std::error::Error>> {
    // In standalone mode, do not sync telemetry to cloud unless explicitly enabled
    let is_telemetry_enabled = crate::config::get().telemetry_enabled;

    if !is_telemetry_enabled {
        return Ok(());
    }

    let redacted_labels = redact_interface_pii(labels);
    let labels_json = serde_json::to_string(&redacted_labels)?;

    query(
        "INSERT INTO telemetry_buffer (metric_name, metric_type, value, labels_json, timestamp, sync_status)
         VALUES ($1, $2, $3, $4, $5, 'pending')"
    )
    .bind(metric_name)
    .bind(metric_type)
    .bind(value)
    .bind(labels_json)
    .bind(Utc::now())
    .execute(pool)
    .await?;

    Ok(())
}

pub fn redact_interface_pii(val: Value) -> Value {
    match val {
        Value::Object(map) => {
            let mut new_map = Map::new();
            for (k, v) in map {
                if is_sensitive_key(&k) {
                    new_map.insert(k, Value::String("[REDACTED]".to_string()));
                } else {
                    new_map.insert(k, redact_interface_pii(v));
                }
            }
            Value::Object(new_map)
        }
        Value::Array(arr) => {
            let new_arr = arr.into_iter().map(redact_interface_pii).collect();
            Value::Array(new_arr)
        }
        Value::String(s) => {
            if is_email(&s) {
                Value::String("[EMAIL_REDACTED]".to_string())
            } else {
                Value::String(s)
            }
        }
        _ => val,
    }
}

fn is_sensitive_key(key: &str) -> bool {
    let k = key.to_lowercase();
    k.contains("password") ||
    k.contains("secret") ||
    k.contains("key") ||
    k.contains("token") ||
    k.contains("auth") ||
    k.contains("cookie") ||
    k.contains("credential") ||
    k.contains("email") ||
    k.contains("phone") ||
    k.contains("ssn") ||
    k.contains("address") ||
    k.contains("name") ||
    k.contains("pii") ||
    k.contains("tenant_id") ||
    k.contains("organization_id") ||
    k.contains("session_id") ||
    k.contains("payload") ||
    k.contains("credit_card") ||
    k.contains("dob") ||
    k.contains("billing")
}

fn is_email(s: &str) -> bool {
    s.contains('@') && s.contains('.')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_queue_length_gauge() {
        let gauge = get_queue_length_gauge();
        gauge.add(1, &[]);
        // Calling it again should return the same instance
        let gauge2 = get_queue_length_gauge();
        gauge2.add(1, &[]);
    }

    #[test]
    fn test_redact_interface_pii() {
        let original_json = serde_json::json!({
            "safe_field": "safe_value",
            "nested": {
                "password": "my_super_secret_password",
                "email": "user@example.com",
                "another_safe": "value"
            },
            "array": [
                { "ssn": "123-45-6789" },
                { "phone": "555-1234" }
            ],
            "raw_email": "test@test.com",
            "API_KEY": "sk-123456"
        });

        let redacted_json = redact_interface_pii(original_json);

        assert_eq!(redacted_json["safe_field"], "safe_value");
        assert_eq!(redacted_json["nested"]["password"], "[REDACTED]");
        assert_eq!(redacted_json["nested"]["email"], "[REDACTED]");
        assert_eq!(redacted_json["nested"]["another_safe"], "value");
        assert_eq!(redacted_json["array"][0]["ssn"], "[REDACTED]");
        assert_eq!(redacted_json["array"][1]["phone"], "[REDACTED]");
        // Since `raw_email`'s value contains an @ and ., it is considered an email by `is_email` check, NOT by the key!
        // But wait! `raw_email` key also contains "email"! Let's test a key that does NOT contain sensitive words but HAS email string
        assert_eq!(redacted_json["raw_email"], "[REDACTED]"); // "email" in key matched first!
        assert_eq!(redacted_json["API_KEY"], "[REDACTED]");
    }
}
