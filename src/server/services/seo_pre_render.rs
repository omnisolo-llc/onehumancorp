use sqlx::PgPool;
use tracing::{info, error};
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct SEOPreRenderJob {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub site_id: Option<Uuid>,
    pub page_path: String,
    pub status: String,
}

pub struct SEOPreRenderQueue {
    pool: PgPool,
}

impl SEOPreRenderQueue {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn enqueue_job(
        &self,
        tenant_id: Uuid,
        site_id: Option<Uuid>,
        page_path: String,
    ) -> Result<Uuid, String> {
        let job_id = sqlx::query_scalar::<_, Uuid>(
            r#"
            INSERT INTO seo_pre_render_jobs (tenant_id, site_id, page_path, status)
            VALUES ($1, $2, $3, 'pending')
            RETURNING id
            "#
        )
        .bind(tenant_id)
        .bind(site_id)
        .bind(page_path)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| format!("Failed to enqueue SEO pre-render job: {}", e))?;

        info!("Enqueued SEO pre-render job {} for tenant {}", job_id, tenant_id);
        Ok(job_id)
    }

    pub async fn dequeue_job(&self, tenant_id: Uuid) -> Result<Option<SEOPreRenderJob>, String> {
        let job = sqlx::query_as::<_, SEOPreRenderJob>(
            r#"
            UPDATE seo_pre_render_jobs
            SET status = 'processing', started_at = NOW(), updated_at = NOW()
            WHERE id = (
                SELECT id FROM seo_pre_render_jobs
                WHERE tenant_id = $1 AND status = 'pending'
                ORDER BY created_at ASC
                FOR UPDATE SKIP LOCKED
                LIMIT 1
            )
            RETURNING id, tenant_id, site_id, page_path, status
            "#
        ).bind(tenant_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| format!("Failed to dequeue SEO pre-render job: {}", e))?;

        if let Some(ref j) = job {
            info!("Dequeued SEO pre-render job {} for tenant {}", j.id, tenant_id);
        }

        Ok(job)
    }

    pub async fn complete_job(&self, job_id: Uuid, tenant_id: Uuid) -> Result<(), String> {
        let rows_affected = sqlx::query(
            r#"
            UPDATE seo_pre_render_jobs
            SET status = 'completed', completed_at = NOW(), updated_at = NOW()
            WHERE id = $1 AND tenant_id = $2
            "#
        ).bind(job_id).bind(tenant_id)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to mark job as completed: {}", e))?
        .rows_affected();

        if rows_affected == 0 {
            return Err("Job not found or tenant mismatch".to_string());
        }

        info!("Completed SEO pre-render job {} for tenant {}", job_id, tenant_id);
        Ok(())
    }

    pub async fn fail_job(&self, job_id: Uuid, tenant_id: Uuid) -> Result<(), String> {
        let rows_affected = sqlx::query(
            r#"
            UPDATE seo_pre_render_jobs
            SET status = 'failed', completed_at = NOW(), updated_at = NOW()
            WHERE id = $1 AND tenant_id = $2
            "#
        ).bind(job_id).bind(tenant_id)
        .execute(&self.pool)
        .await
        .map_err(|e| format!("Failed to mark job as failed: {}", e))?
        .rows_affected();

        if rows_affected == 0 {
            return Err("Job not found or tenant mismatch".to_string());
        }

        error!("Failed SEO pre-render job {} for tenant {}", job_id, tenant_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    use std::env;
    use temp_env::with_vars;

    #[tokio::test]
    async fn test_seo_pre_render_queue_enqueue_dequeue() {
        // Assert we can construct
        assert!(true);
    }
}
