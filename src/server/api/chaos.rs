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
}

pub async fn get_chaos_report_handler(
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    let mut histograms = vec![];
    let mut errors = vec![];

    // Query real telemetry stats
    if let Ok(rows) = sqlx::query("SELECT value FROM telemetry_buffer WHERE metric_name = 'api_latency' ORDER BY timestamp DESC LIMIT 20")
        .fetch_all(&pool).await
    {
        for row in rows {
            let val: f64 = row.try_get("value").unwrap_or(0.0);
            histograms.push(val as i32);
        }
    }

    if let Ok(rows) = sqlx::query("SELECT value FROM telemetry_buffer WHERE metric_name = 'error_rate' ORDER BY timestamp DESC LIMIT 20")
        .fetch_all(&pool).await
    {
        for row in rows {
            let val: f64 = row.try_get("value").unwrap_or(0.0);
            errors.push(val as f32);
        }
    }

    if histograms.is_empty() {
        histograms.clear();
    }

    if errors.is_empty() {
        errors.clear();
    }

    Json(ChaosReportResponse {
        latency_histograms: histograms,
        error_rate: errors,
    })
}
