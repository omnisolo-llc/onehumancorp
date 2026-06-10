use std::sync::Arc;
use crate::db::DB;

pub struct SeoPrerenderWorker {
    db: Arc<DB>,
}

impl SeoPrerenderWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn handle(&self, job: crate::queue::Job) -> Result<Result<(), String>, String> {
        let payload: serde_json::Value = match serde_json::from_str(&job.payload) {
            Ok(p) => p,
            Err(e) => {
                tracing::error!("Failed to parse payload: {}", e);
                return Err("Failed to parse payload".into());
            }
        };

        let product_id = payload.get("product_id").and_then(|v| v.as_str()).unwrap_or("");
        if product_id.is_empty() {
            tracing::error!("product_id is missing");
            return Err("product_id is missing".into());
        }

        let mut tx = match self.db.pool.begin().await {
            Ok(tx) => tx,
            Err(e) => {
                tracing::error!("Failed to begin transaction: {}", e);
                return Err("Failed to begin db transaction".into());
            }
        };

        if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &job.tenant_id).await {
            tracing::error!("Failed to set org context: {}", e);
            return Err("Failed to set org context".into());
        }

        use sqlx::Row;

        let row = match sqlx::query("SELECT title, description FROM products WHERE id = $1 AND tenant_id = $2")
            .bind(product_id)
            .bind(&job.tenant_id)
            .fetch_one(&mut *tx)
            .await {
            Ok(r) => r,
            Err(e) => {
                tracing::error!("Failed to fetch product {}: {}", product_id, e);
                let _ = tx.rollback().await;
                return Err("Product not found".into());
            }
        };

        let title: String = row.try_get("title").unwrap_or_else(|_| format!("Product {}", product_id));
        let description: String = row.try_get("description").unwrap_or_else(|_| "".to_string());

        let json_ld = serde_json::json!({
            "@context": "https://schema.org/",
            "@type": "Product",
            "name": title,
            "description": description,
            "sku": product_id,
        });

        let html_content = format!(
            "<!DOCTYPE html><html><head><title>{}</title><meta name=\"description\" content=\"{}\"><script type=\"application/ld+json\">{}</script></head><body><h1>{}</h1></body></html>",
            title, description, json_ld.to_string(), title
        );

        let route_key = format!("/product/{}", product_id);
        let id = uuid::Uuid::new_v4().to_string();

        match sqlx::query(
            "INSERT INTO storefront_edge_cache (id, tenant_id, route_key, html_content)
             VALUES ($1, $2, $3, $4)
             ON CONFLICT (tenant_id, route_key)
             DO UPDATE SET html_content = $4, updated_at = CURRENT_TIMESTAMP"
        )
        .bind(&id)
        .bind(&job.tenant_id)
        .bind(&route_key)
        .bind(&html_content)
        .execute(&mut *tx)
        .await {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("Failed to upsert cache: {}", e);
                let _ = tx.rollback().await;
                return Err("Failed to upsert cache".into());
            }
        };

        match tx.commit().await {
            Ok(_) => Ok(Ok(())),
            Err(e) => {
                tracing::error!("Failed to commit transaction: {}", e);
                Err("Failed to commit db transaction".into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;

    #[tokio::test]
    async fn test_seo_prerender_worker() {
        let database_url = std::env::var("OHC_DATABASE_URL").unwrap_or_else(|_| "postgres://localhost/dummy".to_string());
        if !database_url.contains("test") {
            return; // Skip if not a test database
        }

        let pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

        let tenant_id = uuid::Uuid::new_v4().to_string();
        let product_id = uuid::Uuid::new_v4().to_string();

        // Create tenant
        sqlx::query("INSERT INTO tenants (id, name) VALUES ($1, 'Test Tenant')")
            .bind(&tenant_id)
            .execute(&pool)
            .await
            .unwrap();

        // Create product
        sqlx::query("INSERT INTO products (id, tenant_id, title, description, type) VALUES ($1, $2, 'My SEO Product', 'A very SEO friendly product', 'physical')")
            .bind(&product_id)
            .bind(&tenant_id)
            .execute(&pool)
            .await
            .unwrap();

        let db = Arc::new(crate::db::DB { pool: pool.clone(), store: crate::db::DbStore::Postgres });
        let worker = SeoPrerenderWorker::new(db.clone());

        let job_id = uuid::Uuid::new_v4().to_string();
        let payload = serde_json::json!({
            "product_id": product_id
        }).to_string();

        let job = crate::queue::Job {
            id: job_id.clone(),
            tenant_id: tenant_id.clone(),
            parent_task_id: "".to_string(),
            job_type: "seo_prerender".to_string(),
            payload,
            status: "PENDING".to_string(),
            retry_count: 0,
            max_retries: 3,
            next_retry_at: chrono::Utc::now(),
            locked_until: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let result = worker.handle(job).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_ok());

        // Verify HTML
        let route_key = format!("/product/{}", product_id);
        let row = sqlx::query("SELECT html_content FROM storefront_edge_cache WHERE tenant_id = $1 AND route_key = $2")
            .bind(&tenant_id)
            .bind(&route_key)
            .fetch_one(&pool)
            .await
            .unwrap();

        let html: String = sqlx::Row::get(&row, "html_content");
        assert!(html.contains("<title>My SEO Product</title>"));
        assert!(html.contains("<meta name=\"description\" content=\"A very SEO friendly product\">"));
        assert!(html.contains("\"@type\":\"Product\""));
        assert!(html.contains(&format!("\"sku\":\"{}\"", product_id)));
    }
}
