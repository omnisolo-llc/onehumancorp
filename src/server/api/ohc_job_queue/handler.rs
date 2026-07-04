use axum::{routing::get, Json, Router, extract::Extension, extract::Query};
use serde_json::json;
use std::sync::OnceLock;

use crate::db;
use ::server_utils::cache::HybridCache;

pub fn router() -> Router<std::sync::Arc<dyn ohc_builtin_agent::mesh::transport::MeshTransport>> {
    Router::new().route("/", get(list_jobs))
}

#[derive(serde::Deserialize)]
pub struct JobQueueQuery {
    pub mobile_optimized: Option<bool>,
}

static OHC_JOB_QUEUE_CACHE: OnceLock<HybridCache<serde_json::Value>> = OnceLock::new();

async fn list_jobs(
    Extension(claims): Extension<::server_common::Claims>,
    Query(query): Query<JobQueueQuery>,
) -> Result<Json<serde_json::Value>, axum::http::StatusCode> {
    let pool = db::get_pool();
    let tenant_id = claims.organization_id.clone().unwrap_or_else(|| ::server_common::auth_utils::get_default_tenant());
    let mobile_optimized = query.mobile_optimized.unwrap_or(false);

    let cache_key = format!("ohc_job_queue:{}:mobile:{}", tenant_id, mobile_optimized);
    let cache = OHC_JOB_QUEUE_CACHE.get_or_init(|| HybridCache::new(crate::get_redis_client()));

    if let Some((cached, is_stale)) = cache.get_with_swr(&cache_key).await {
        if !is_stale {
            return Ok(Json(cached));
        }

        let tenant_id_bg = tenant_id.clone();
        let cache_key_bg = cache_key.clone();
        let pool_bg = pool.clone();

        tokio::spawn(async move {
            let res = fetch_jobs(&pool_bg, &tenant_id_bg, mobile_optimized).await;
            if let Ok(jobs) = res {
                if let Some(c) = OHC_JOB_QUEUE_CACHE.get() {
                    c.set(&cache_key_bg, jobs, std::time::Duration::from_secs(10)).await;
                }
            }
        });
        return Ok(Json(cached));
    }

    match fetch_jobs(&pool, &tenant_id, mobile_optimized).await {
        Ok(jobs) => {
            cache.set(&cache_key, jobs.clone(), std::time::Duration::from_secs(10)).await;
            Ok(Json(jobs))
        },
        Err(_) => Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn fetch_jobs(pool: &sqlx::Pool<sqlx::Postgres>, tenant_id: &str, mobile_optimized: bool) -> Result<serde_json::Value, sqlx::Error> {
    let query_str = if mobile_optimized {
        r#"
        SELECT id, job_type, status, created_at, updated_at
        FROM ohc_job_queue
        WHERE tenant_id = $1
        ORDER BY created_at DESC
        LIMIT 50
        "#
    } else {
        r#"
        SELECT id, job_type, status, retry_count, created_at, updated_at
        FROM ohc_job_queue
        WHERE tenant_id = $1
        ORDER BY created_at DESC
        LIMIT 50
        "#
    };

    let jobs = sqlx::query(query_str)
        .bind(tenant_id)
        .fetch_all(pool)
        .await?;

    let mut response_jobs = Vec::new();
    for job in jobs {
        use sqlx::Row;
        if mobile_optimized {
            response_jobs.push(json!({
                "id": job.try_get::<String, _>("id").unwrap_or_default(),
                "job_type": job.try_get::<String, _>("job_type").unwrap_or_default(),
                "status": job.try_get::<String, _>("status").unwrap_or_default(),
                "created_at": job.try_get::<chrono::DateTime<chrono::Utc>, _>("created_at").unwrap_or_else(|_| chrono::Utc::now()),
                "updated_at": job.try_get::<chrono::DateTime<chrono::Utc>, _>("updated_at").unwrap_or_else(|_| chrono::Utc::now()),
            }));
        } else {
            response_jobs.push(json!({
                "id": job.try_get::<String, _>("id").unwrap_or_default(),
                "job_type": job.try_get::<String, _>("job_type").unwrap_or_default(),
                "status": job.try_get::<String, _>("status").unwrap_or_default(),
                "retry_count": job.try_get::<i32, _>("retry_count").unwrap_or(0),
                "created_at": job.try_get::<chrono::DateTime<chrono::Utc>, _>("created_at").unwrap_or_else(|_| chrono::Utc::now()),
                "updated_at": job.try_get::<chrono::DateTime<chrono::Utc>, _>("updated_at").unwrap_or_else(|_| chrono::Utc::now()),
            }));
        }
    }

    Ok(json!({
        "jobs": response_jobs
    }))
}
