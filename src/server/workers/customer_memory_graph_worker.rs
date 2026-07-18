use sqlx::PgPool;
use std::sync::Arc;
use chrono::Utc;

pub struct CustomerMemoryGraphWorker {
    pool: Arc<PgPool>,
}

impl CustomerMemoryGraphWorker {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }

    pub async fn process_job(&self, tenant_id: &str, job_id: uuid::Uuid, event_id: uuid::Uuid) -> Result<(), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;
        ::server_common::auth_utils::set_system_context(&mut *tx).await.map_err(|e| e.to_string())?;

        // 1. Fetch event
        let event = sqlx::query!(
            "SELECT customer_id, channel, raw_content FROM interaction_events WHERE id = $1 AND tenant_id = $2",
            event_id,
            tenant_id
        )
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let event = match event {
            Some(e) => e,
            None => return Err("Event not found".to_string()),
        };

        // 2. Extract facts via AI pipeline (we just do a mock for tests/basic extraction)
        let extracted_facts = format!("Extracted from {}", event.raw_content);
        let snippet_id = uuid::Uuid::new_v4();

        sqlx::query!(
            "INSERT INTO context_snippets (id, tenant_id, customer_id, category, extracted_value) VALUES ($1, $2, $3, 'fact', $4)",
            snippet_id,
            tenant_id,
            event.customer_id,
            extracted_facts
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        // 3. Rebuild summary
        let snippets = sqlx::query!(
            "SELECT extracted_value FROM context_snippets WHERE customer_id = $1 AND tenant_id = $2",
            event.customer_id,
            tenant_id
        )
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let total_interactions_row = sqlx::query!(
            "SELECT COUNT(*) as count FROM interaction_events WHERE customer_id = $1 AND tenant_id = $2",
            event.customer_id,
            tenant_id
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        let total_interactions = total_interactions_row.count.unwrap_or(0);

        let new_summary = snippets.into_iter().map(|s| s.extracted_value).collect::<Vec<_>>().join("\n");
        let segments = vec!["active".to_string()];

        let profile_summary = crate::services::customer_memory_graph::service::CustomerProfileSummary {
            total_interactions,
            last_interaction: Some(Utc::now()),
            segments,
            preferences: vec![],
            summary: new_summary,
        };

        sqlx::query!(
            "UPDATE customers SET profile_summary = $1 WHERE id = $2 AND tenant_id = $3",
            serde_json::to_value(profile_summary).unwrap(),
            event.customer_id,
            tenant_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        // 4. Mark job as complete
        sqlx::query!(
            "UPDATE interaction_event_jobs SET status = 'completed', updated_at = CURRENT_TIMESTAMP WHERE job_id = $1",
            job_id
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;
        Ok(())
    }
}

pub async fn run_worker(pool: Arc<PgPool>) {
    loop {
        let mut tx = match pool.begin().await {
            Ok(tx) => tx,
            Err(e) => {
                tracing::error!("Failed to begin tx in memory graph worker: {}", e);
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                continue;
            }
        };

        if let Err(e) = ::server_common::auth_utils::set_system_context(&mut *tx).await {
            tracing::error!("Failed to set system context in memory graph worker: {}", e);
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            continue;
        }

        let job_opt = sqlx::query!(
            "UPDATE interaction_event_jobs SET status = 'processing', updated_at = CURRENT_TIMESTAMP
             WHERE job_id = (
                 SELECT job_id FROM interaction_event_jobs
                 WHERE status = 'pending'
                 ORDER BY created_at ASC
                 LIMIT 1
                 FOR UPDATE SKIP LOCKED
             ) RETURNING job_id, tenant_id, interaction_event_id"
        )
        .fetch_optional(&mut *tx)
        .await;

        match job_opt {
            Ok(Some(job)) => {
                if let Err(e) = tx.commit().await {
                    tracing::error!("Failed to commit job acquisition: {}", e);
                    continue;
                }

                let worker = CustomerMemoryGraphWorker::new(pool.clone());
                if let Err(e) = worker.process_job(&job.tenant_id, job.job_id, job.interaction_event_id).await {
                    tracing::error!("Failed to process memory graph job: {}", e);
                    // Reset status on error
                    let mut tx2 = match pool.begin().await {
                        Ok(t) => t,
                        Err(_) => continue,
                    };
                    let _ = ::server_common::auth_utils::set_system_context(&mut *tx2).await;
                    let _ = sqlx::query!(
                        "UPDATE interaction_event_jobs SET status = 'pending', retry_count = retry_count + 1 WHERE job_id = $1",
                        job.job_id
                    )
                    .execute(&mut *tx2)
                    .await;
                    let _ = tx2.commit().await;
                }
            }
            Ok(None) => {
                let _ = tx.rollback().await;
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
            Err(e) => {
                let _ = tx.rollback().await;
                tracing::error!("Failed to query memory graph jobs: {}", e);
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        }
    }
}
