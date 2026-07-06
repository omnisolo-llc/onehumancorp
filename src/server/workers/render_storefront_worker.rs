use crate::orchestration::queue::{OHCJobQueue, OHCJob};
use crate::orchestration::queue::worker_pool::JobHandler;
use sqlx::PgPool;
use std::sync::Arc;
use serde_json::json;

pub struct RenderStorefrontWorker {
    pub pool: Arc<PgPool>,
}

impl RenderStorefrontWorker {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub fn start(self: Arc<Self>) {
        let pool_arc = self.pool.clone();
        tokio::spawn(async move {
            let queue = Arc::new(OHCJobQueue::new(pool_arc));
            let worker_pool = crate::orchestration::queue::WorkerPool::new(
                queue,
                1,
                vec!["RenderStorefrontToEdge".to_string()],
                self.clone() as Arc<dyn JobHandler>
            );
            worker_pool.await_termination().await;
        });
    }
}

#[async_trait::async_trait]
impl JobHandler for RenderStorefrontWorker {
    async fn handle_job(&self, job: OHCJob) -> Result<(), String> {
        let tenant_id = job.tenant_id.clone();
        tracing::info!("Running RenderStorefrontToEdge for tenant: {}", tenant_id);

        // Notify via agent feed
        let feed_id = uuid::Uuid::new_v4().to_string();
        let query = r#"
            INSERT INTO tenant_feed_items (
                id, tenant_id, title, description, action_type, action_payload, status
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7
            )
        "#;

        let title = "SEO Storefront Updated";
        let description = "I noticed you added new items. I've pre-rendered a new SEO page and pushed it live to capture local traffic. View Performance?";
        let payload = json!({"action": "view_performance"});

        sqlx::query(query)
            .bind(&feed_id)
            .bind(&tenant_id)
            .bind(title)
            .bind(description)
            .bind("view_performance")
            .bind(&payload)
            .bind("pending")
            .execute(&*self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    use std::env;

    #[tokio::test]
    async fn test_render_storefront_worker() {
        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://ohc:ohc@localhost:5432/ohc".to_string());
        if let Ok(pool) = PgPool::connect(&database_url).await {
            let worker = RenderStorefrontWorker::new(Arc::new(pool.clone()));
            let job = OHCJob {
                id: uuid::Uuid::new_v4().to_string(),
                tenant_id: "test-tenant-seo".to_string(),
                job_type: "RenderStorefrontToEdge".to_string(),
                payload: Some(json!({"product_id": "123"})),
                status: "PENDING".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                attempts: 0,
                locked_at: None,
                locked_by: None,
                last_error: None,
            };

            let res = worker.handle_job(job).await;
            assert!(res.is_ok());

            let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM tenant_feed_items WHERE tenant_id = $1 AND title = 'SEO Storefront Updated'")
                .bind("test-tenant-seo")
                .fetch_one(&pool)
                .await
                .unwrap();

            assert!(count.0 > 0);
        }
    }
}
