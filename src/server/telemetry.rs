use serde_json::{Value, Map};
use sqlx::{PgPool, query};
use chrono::Utc;

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
    k.contains("credential") ||
    k.contains("session_id") ||
    k.contains("session_data") ||
    k.contains("payload")
}

fn is_email(s: &str) -> bool {
    s.contains('@') && s.contains('.')
}
