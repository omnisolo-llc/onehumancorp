use serde_json::{Value, Map};
use sqlx::{PgPool, query};
use chrono::Utc;

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
    let payload = serde_json::json!({ "delta": delta });
    buffer_metric(pool, "ohc_sub_agent_queue_length", "gauge", delta as f32, payload).await
}

pub async fn record_llm_token_usage(pool: &PgPool, tokens: f32, model: &str, tenant_id: &str, agent_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    buffer_metric(pool, "ohc_llm_token_usage_total", "counter", tokens, serde_json::json!({ "model": model, "tenant_id": tenant_id, "agent_id": agent_id })).await
}

pub async fn record_storage_read_bytes(pool: &PgPool, bytes: f32, tenant_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    buffer_metric(pool, "ohc_storage_read_bytes_total", "counter", bytes, serde_json::json!({ "tenant_id": tenant_id })).await
}

pub async fn record_storage_write_bytes(pool: &PgPool, bytes: f32, tenant_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    buffer_metric(pool, "ohc_storage_write_bytes_total", "counter", bytes, serde_json::json!({ "tenant_id": tenant_id })).await
}

pub async fn record_email_send(pool: &PgPool, tenant_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    buffer_metric(pool, "ohc_email_sends_total", "counter", 1.0, serde_json::json!({ "tenant_id": tenant_id })).await
}

pub async fn record_outbound_api_call(pool: &PgPool, api_name: &str, tenant_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    buffer_metric(pool, "ohc_outbound_api_calls_total", "counter", 1.0, serde_json::json!({ "api_name": api_name, "tenant_id": tenant_id })).await
}

pub async fn buffer_metric(
    pool: &PgPool,
    metric_name: &str,
    metric_type: &str,
    value: f32,
    labels: Value,
) -> Result<(), Box<dyn std::error::Error>> {
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
    k.contains("credential")
}

fn is_email(s: &str) -> bool {
    s.contains('@') && s.contains('.')
}
