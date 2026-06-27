use axum::{routing::get, Json, Router};
use serde_json::json;

use crate::db;

pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new().route("/", get(list_jobs))
}

async fn list_jobs(
    axum::extract::Extension(auth_info): axum::extract::Extension<::server_auth::orchestration::AuthInfo>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let pool = db::get_pool();

    let jobs = match sqlx::query!(
        r#"
        SELECT id, job_type, status, retry_count, created_at, updated_at
        FROM ohc_job_queue
        WHERE tenant_id = $1
        ORDER BY created_at DESC
        LIMIT 50
        "#,
        auth_info.tenant_id
    )
    .fetch_all(&pool)
    .await {
        Ok(j) => j,
        Err(e) => {
            tracing::error!("Error fetching job queue: {}", e);
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let mut response_jobs = Vec::new();
    for job in jobs {
        response_jobs.push(json!({
            "id": job.id,
            "job_type": job.job_type,
            "status": job.status,
            "retry_count": job.retry_count,
            "created_at": job.created_at,
            "updated_at": job.updated_at,
        }));
    }

    Ok(Json(json!({
        "jobs": response_jobs
    })))
}
