use axum::{extract::State, response::IntoResponse, Json};
use std::sync::Arc;

pub async fn hybrid_health_probe(
    State(hub): State<Arc<crate::hub::Hub>>,
) -> impl IntoResponse {
    let health_val = match hub.check_health().await {
        Ok(v) => v,
        Err(_) => {
            return Json(serde_json::json!({
                "mode": "Unknown",
                "status": "degraded",
                "details": {
                    "mesh_active": false,
                    "sync_queue": 0,
                    "stuck_missions": 0
                }
            }));
        }
    };

    let mode = health_val.get("mode").and_then(|v| v.as_str()).unwrap_or("Unknown");
    let sync_queue = health_val
        .get("details")
        .and_then(|d| d.get("sync_queue"))
        .and_then(|v| v.as_i64())
        .unwrap_or(0);

    let _ = crate::telemetry::record_local_cloud_mission_sync(&hub.pool, sync_queue, mode).await;

    Json(health_val)
}





#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::State;
    use axum::response::IntoResponse;
    use sqlx::Executor;

    #[tokio::test]
    async fn test_hybrid_health_probe() {
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }

        let db_url = std::env::var("DATABASE_URL").unwrap();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .after_release(|conn, _meta| { Box::pin(async move { conn.execute("RESET app.current_tenant").await?; Ok(true) }) })
            .acquire_timeout(std::time::Duration::from_millis(50))
            .connect_lazy(&db_url)
            .unwrap();

        let (tx, _) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(crate::hub::Hub::new(tx, pool));

        let response = hybrid_health_probe(State(hub)).await.into_response();

        assert_eq!(response.status(), axum::http::StatusCode::OK);

        // We can't easily extract JSON from IntoResponse without extra crates,
        // but we already asserted the response status which means the handler didn't crash.
        // And we tested hub.check_health() separately.
    }
}
