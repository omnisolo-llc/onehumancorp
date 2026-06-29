use std::sync::Arc;
use std::time::Duration;
use crate::db::DB;
use crate::minimax::MinimaxClient;

pub struct MigrationWorker {
    db: Arc<DB>,
}

impl MigrationWorker {
    pub fn new(db: Arc<DB>) -> Self {
        MigrationWorker { db }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        tokio::spawn(async move {
            tracing::info!("MigrationWorker started");
            loop {
                if let Err(e) = Self::process_next_job(&db).await {
                    ::server_telemetry::record_error_signal("[bug] MigrationWorker error");
                    tracing::error!("MigrationWorker error: {}", e);
                }
                tokio::time::sleep(Duration::from_secs(10)).await;
            }
        });
    }

    async fn process_next_job(db: &Arc<DB>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut tx = db.pool.begin().await?;

        // Find a pending migration job, locking the row.
        let job: Option<(String, String, String)> = sqlx::query_as(
            "SELECT id, tenant_id, url FROM migration_jobs
             WHERE status = 'PENDING'
             ORDER BY created_at ASC
             LIMIT 1 FOR UPDATE SKIP LOCKED"
        )
        .fetch_optional(&mut *tx)
        .await?;

        let (job_id, tenant_id, url) = match job {
            Some(j) => j,
            None => return Ok(()),
        };

        tracing::info!("Processing migration job {} for tenant {} (url: {})", job_id, tenant_id, url); // pii-safe

        // Update status to IN_PROGRESS
        sqlx::query("UPDATE migration_jobs SET status = 'IN_PROGRESS', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(&job_id)
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;

        // Execute migration
        let result = Self::perform_migration(db, &tenant_id, &url).await;

        let mut tx = db.pool.begin().await?;
        crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id).await?;

        match result {
            Ok(products_json) => {
                sqlx::query("UPDATE migration_jobs SET status = 'COMPLETED', extracted_products = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2")
                    .bind(&products_json)
                    .bind(&job_id)
                    .execute(&mut *tx)
                    .await?;
            }
            Err(e) => {
                ::server_telemetry::record_error_signal("[bug] Migration job failed");
                tracing::error!("Migration job {} failed: {}", job_id, e);
                sqlx::query("UPDATE migration_jobs SET status = 'FAILED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
                    .bind(&job_id)
                    .execute(&mut *tx)
                    .await?;
            }
        }
        tx.commit().await?;

        Ok(())
    }

    async fn perform_migration(db: &Arc<DB>, tenant_id: &str, url: &str) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
        // Fetch content
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(60))
            .build()?;
        let res = client.get(url).send().await?;
        if !res.status().is_success() {
            return Err(format!("Failed to fetch URL: {}", res.status()).into());
        }
        let html_content = res.text().await?;

        // Use AI to extract products
        let api_key = std::env::var("OHC_MINIMAX_API_KEY").unwrap_or_else(|_| "fake-key".to_string());
        let minimax = MinimaxClient::new(api_key);

        let prompt = format!(
            "You are an AI web crawler and data extractor. Extract product catalog data from the following raw HTML snippet from an ecommerce site.
            Return ONLY a valid JSON array of objects, where each object has fields: 'title' (string), 'price_cents' (integer, eg $25.00 -> 2500), 'description' (string), and 'type' (string, default 'physical').
            Do not include markdown blocks or any other text.
            HTML: \n{}",
            &html_content.chars().take(20000).collect::<String>() // truncate to avoid massive prompts
        );

        let mut attempts = 0;
        let mut response = String::new();
        while attempts < 3 {
            let res = tokio::time::timeout(std::time::Duration::from_secs(60), minimax.reason(&prompt)).await;
            match res {
                Ok(Ok(content)) => {
                    response = content;
                    break;
                },
                _ => {
                    attempts += 1;
                    if attempts == 3 {
                        return Err("AI migration extraction failed after 3 attempts".into());
                    }
                    tokio::time::sleep(std::time::Duration::from_secs(2u64.pow(attempts))).await;
                }
            }
        }

        response = response.trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim().to_string();

        let products: Vec<serde_json::Value> = serde_json::from_str(&response).unwrap_or_default();

        let mut tx = db.pool.begin().await?;
        crate::common::auth_utils::set_org_context(&mut *tx, tenant_id).await?;

        let mut extracted_array = Vec::new();

        for product in products {
            let title = product.get("title").and_then(|v| v.as_str()).unwrap_or("Unknown Product");
            let desc = product.get("description").and_then(|v| v.as_str()).unwrap_or("");
            let price_cents = product.get("price_cents").and_then(|v| v.as_i64()).unwrap_or(0);
            let p_type = product.get("type").and_then(|v| v.as_str()).unwrap_or("physical");

            let id = uuid::Uuid::new_v4().to_string();

            sqlx::query(
                "INSERT INTO products (id, tenant_id, name, title, description, price_cents, type, currency, fulfillment_strategy, inventory_count, _sync_status)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, 'USD', 'standard', 10, 'pending')"
            )
            .bind(&id)
            .bind(tenant_id)
            .bind(title)
            .bind(title)
            .bind(desc)
            .bind(price_cents)
            .bind(p_type)
            .execute(&mut *tx)
            .await?;

            extracted_array.push(serde_json::json!({
                "id": id,
                "title": title,
                "price_cents": price_cents
            }));
        }

        tx.commit().await?;

        Ok(serde_json::Value::Array(extracted_array))
    }
}
