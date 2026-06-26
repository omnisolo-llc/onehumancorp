use axum::{routing::get, Json, Router, extract::Extension};
use serde_json::json;

use crate::db;

pub fn router() -> Router<std::sync::Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    Router::new().route("/", get(list_jobs))
}

async fn list_jobs(
    Extension(claims): Extension<::server_common::Claims>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let pool = db::get_pool();
    let tenant_id = claims.organization_id.clone().unwrap_or_else(|| ::server_common::auth_utils::get_default_tenant());

    let jobs = match sqlx::query(
        r#"
        SELECT id, job_type, status, retry_count, created_at, updated_at
        FROM ohc_job_queue
        WHERE tenant_id = $1
        ORDER BY created_at DESC
        LIMIT 50
        "#
    )
    .bind(tenant_id)
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
        use sqlx::Row;
        response_jobs.push(json!({
            "id": job.try_get::<String, _>("id").unwrap_or_default(),
            "job_type": job.try_get::<String, _>("job_type").unwrap_or_default(),
            "status": job.try_get::<String, _>("status").unwrap_or_default(),
            "retry_count": job.try_get::<i32, _>("retry_count").unwrap_or(0),
            "created_at": job.try_get::<chrono::DateTime<chrono::Utc>, _>("created_at").unwrap_or_else(|_| chrono::Utc::now()),
            "updated_at": job.try_get::<chrono::DateTime<chrono::Utc>, _>("updated_at").unwrap_or_else(|_| chrono::Utc::now()),
        }));
    }

    Ok(Json(json!({
        "jobs": response_jobs
    })))
}
