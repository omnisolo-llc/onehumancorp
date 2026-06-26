use async_trait::async_trait;
use serde_json::Value;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{error, info, warn};

pub struct AiJobQueueWorker {
    db_pool: Arc<PgPool>,
}

impl AiJobQueueWorker {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }

    pub async fn run(&self) {
        loop {
            match self.process_next_job().await {
                Ok(true) => {
                    // Job processed successfully, immediately poll again
                }
                Ok(false) => {
                    // No jobs available, sleep briefly
                    sleep(Duration::from_secs(2)).await;
                }
                Err(e) => {
                    // Error processing job
                    error!("Error in AiJobQueueWorker: {:?}", e);
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }
    }

    async fn process_next_job(&self) -> Result<bool, sqlx::Error> {
        let mut tx = self.db_pool.begin().await?;

        // Use SKIP LOCKED to dequeue the next available job
        let job = sqlx::query(
            r#"
            UPDATE ai_jobs
            SET
                status = 'processing',
                updated_at = NOW(),
                attempts = attempts + 1,
                last_error = NULL
            WHERE id = (
                SELECT id
                FROM ai_jobs
                WHERE status = 'pending'
                  AND (run_after IS NULL OR run_after <= NOW())
                ORDER BY priority DESC, created_at ASC
                FOR UPDATE SKIP LOCKED
                LIMIT 1
            )
            RETURNING id, tenant_id, job_type, payload
            "#,
        )
        .fetch_optional(&mut *tx)
        .await?;

        if let Some(row) = job {
            let job_id: String = row.try_get("id")?;
            let tenant_id: String = row.try_get("tenant_id")?;
            let job_type: String = row.try_get("job_type")?;
            let payload: Value = row.try_get("payload")?;

            info!("Processing AI job {} of type {} for tenant {}", job_id, job_type, tenant_id);

            // Execute the actual job logic based on job_type
            let result = self.execute_job(&job_type, payload).await;

            match result {
                Ok(_) => {
                    sqlx::query(
                        "UPDATE ai_jobs SET status = 'completed', updated_at = NOW() WHERE id = $1",
                    )
                    .bind(&job_id)
                    .execute(&mut *tx)
                    .await?;
                    info!("Successfully completed AI job {}", job_id);
                }
                Err(e) => {
                    warn!("Failed to process AI job {}: {:?}", job_id, e);

                    // Note: In a full implementation, we would check attempts <= max_attempts
                    // and either set status to 'pending' (for retry) or 'failed' (DLQ).
                    sqlx::query(
                        r#"
                        UPDATE ai_jobs
                        SET
                            status = CASE
                                WHEN attempts >= max_attempts THEN 'failed'
                                ELSE 'pending'
                            END,
                            last_error = $2,
                            updated_at = NOW(),
                            run_after = NOW() + (POW(2, attempts) * interval '1 minute')
                        WHERE id = $1
                        "#
                    )
                    .bind(&job_id)
                    .bind(e.to_string())
                    .execute(&mut *tx)
                    .await?;
                }
            }

            tx.commit().await?;
            Ok(true)
        } else {
            tx.rollback().await?;
            Ok(false)
        }
    }

    async fn execute_job(&self, job_type: &str, _payload: Value) -> Result<(), String> {
        // Here we would route the job to the appropriate AI department
        // For now, we just simulate work
        match job_type {
            "draft_reply" => {
                info!("Drafting reply...");
                Ok(())
            }
            "analyze_sentiment" => {
                info!("Analyzing sentiment...");
                Ok(())
            }
            _ => Err(format!("Unknown job type: {}", job_type)),
        }
    }
}
