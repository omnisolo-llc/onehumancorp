use axum::{extract::State, routing::get, Json, Router};
use std::sync::Arc;
use serde_json::json;

use crate::db;

pub fn router() -> Router<Arc<crate::hub::Hub>> {
    Router::new().route("/", get(list_jobs))
}

async fn list_jobs(
    State(hub): State<Arc<crate::hub::Hub>>,
    crate::auth::extractors::TenantAuth(auth): crate::auth::extractors::TenantAuth,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let pool = hub.pool.clone();

    let jobs = match sqlx::query(
        r#"
        SELECT id, job_type, status, retry_count, created_at, updated_at
        FROM ohc_job_queue
        WHERE tenant_id = $1
        ORDER BY created_at DESC
        LIMIT 50
        "#
    )
    .bind(&auth.tenant_id)
    .fetch_all(&pool)
    .await {
        Ok(j) => j,
        Err(e) => {
            tracing::error!("Error fetching job queue: {}", e);
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let mut response_jobs = Vec::new();
    use sqlx::Row;
    for job in jobs {
        response_jobs.push(json!({
            "id": job.try_get::<String, _>("id").unwrap_or_default(),
            "job_type": job.try_get::<String, _>("job_type").unwrap_or_default(),
            "status": job.try_get::<String, _>("status").unwrap_or_default(),
            "retry_count": job.try_get::<i32, _>("retry_count").unwrap_or(0),
            "created_at": job.try_get::<chrono::DateTime<chrono::Utc>, _>("created_at").ok(),
            "updated_at": job.try_get::<chrono::DateTime<chrono::Utc>, _>("updated_at").ok(),
        }));
    }

    Ok(Json(json!({
        "jobs": response_jobs
    })))
}
