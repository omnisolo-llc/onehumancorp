use axum::{Json, response::IntoResponse, extract::State};
use serde::Serialize;
use sqlx::Row;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct ChaosReportResponse {
    #[serde(rename = "latencyHistograms")]
    pub latency_histograms: Vec<i32>,
    #[serde(rename = "errorRate")]
    pub error_rate: Vec<f32>,
    #[serde(rename = "latencyP99Cloud")]
    pub latency_p99_cloud: String,
    #[serde(rename = "latencyP99Standalone")]
    pub latency_p99_standalone: String,
    #[serde(rename = "errorRateLlmOutage")]
    pub error_rate_llm_outage: String,
}

pub async fn get_chaos_report_handler(
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    let (histograms_res, errors_res, latency_p99_cloud_res, latency_p99_standalone_res, error_rate_llm_outage_res) = tokio::join!(
        sqlx::query("SELECT value FROM telemetry_buffer WHERE metric_name = 'api_latency' ORDER BY timestamp DESC LIMIT 20").fetch_all(&pool),
        sqlx::query("SELECT value FROM telemetry_buffer WHERE metric_name = 'error_rate' ORDER BY timestamp DESC LIMIT 20").fetch_all(&pool),
        sqlx::query_scalar::<_, f64>("SELECT value FROM telemetry_buffer WHERE metric_name = 'api_latency' AND labels_json LIKE '%\"mode\":\"cloud\"%' ORDER BY value DESC LIMIT 1 OFFSET (SELECT CAST(COUNT(*) * 0.01 AS INTEGER) FROM telemetry_buffer WHERE metric_name = 'api_latency' AND labels_json LIKE '%\"mode\":\"cloud\"%')").fetch_optional(&pool),
        sqlx::query_scalar::<_, f64>("SELECT value FROM telemetry_buffer WHERE metric_name = 'api_latency' AND labels_json LIKE '%\"mode\":\"standalone\"%' ORDER BY value DESC LIMIT 1 OFFSET (SELECT CAST(COUNT(*) * 0.01 AS INTEGER) FROM telemetry_buffer WHERE metric_name = 'api_latency' AND labels_json LIKE '%\"mode\":\"standalone\"%')").fetch_optional(&pool),
        sqlx::query_scalar::<_, f64>("SELECT value FROM telemetry_buffer WHERE metric_name = 'error_rate_llm_outage' ORDER BY timestamp DESC LIMIT 1").fetch_optional(&pool),
    );

    let mut histograms = vec![];
    if let Ok(rows) = histograms_res {
        for row in rows {
            let val: f64 = row.try_get("value").unwrap_or(0.0);
            histograms.push(val as i32);
        }
    }

    let mut errors = vec![];
    if let Ok(rows) = errors_res {
        for row in rows {
            let val: f64 = row.try_get("value").unwrap_or(0.0);
            errors.push(val as f32);
        }
    }

    let latency_p99_cloud = latency_p99_cloud_res.unwrap_or(None).map(|v| format!("{:.0}ms", v)).unwrap_or_else(|| "N/A".to_string());
    let latency_p99_standalone = latency_p99_standalone_res.unwrap_or(None).map(|v| format!("{:.0}ms", v)).unwrap_or_else(|| "N/A".to_string());
    let error_rate_llm_outage = error_rate_llm_outage_res.unwrap_or(None).map(|v| format!("{:.1}% (Handled via Graceful Pause)", v * 100.0)).unwrap_or_else(|| "N/A".to_string());

    Json(ChaosReportResponse {
        latency_histograms: histograms,
        error_rate: errors,
        latency_p99_cloud,
        latency_p99_standalone,
        error_rate_llm_outage,
    })
}