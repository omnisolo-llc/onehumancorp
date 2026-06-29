use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;
use uuid::Uuid;
use serde_json::json;
use crate::domain::repository::agent_feed_repo::{AgentFeedItem};

pub struct ProactiveAnalysisWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
}

impl ProactiveAnalysisWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            poll_interval: Duration::from_secs(60), // Run every minute
        }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        let interval_duration = self.poll_interval;
        tokio::spawn(async move {
            let _pool = db.pool.clone();
            loop {
                let _ = Self::process_jobs(&db).await;
                tokio::time::sleep(interval_duration).await;
            }
        });
    }

    async fn process_jobs(db: &Arc<DB>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let _pool = db.pool.clone();

        let mut tx = db.pool.begin().await?;

        // 1. Find jobs
        let task: Option<(String, String, String)> = sqlx::query_as(
            "SELECT id, tenant_id, payload FROM ohc_job_queue
             WHERE status = 'PENDING' AND job_type = 'proactive_context_analysis'
             LIMIT 1 FOR UPDATE SKIP LOCKED"
        )
        .fetch_optional(&mut *tx)
        .await?;

        let (job_id, tenant_id, payload_str) = match task {
            Some(t) => t,
            None => {
                tx.rollback().await?;
                return Ok(());
            }
        };

        // Mark as processing
        sqlx::query("UPDATE ohc_job_queue SET status = 'PROCESSING', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(&job_id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        tracing::info!("ProactiveAnalysisWorker processing job {} for tenant {}", job_id, tenant_id); // pii-safe

        let _payload: serde_json::Value = serde_json::from_str(&payload_str).unwrap_or(json!({}));

        // 2. Perform analysis (Simulation of LLM call/context query)
        // In a real scenario, this would query upcoming bookings, unread messages, stock, etc.
        // For now, we'll create a synthetic actionable insight.
        let proposed_action = json!({
            "action_type": "review_insight",
            "description": "You have 3 pending estimates from yesterday that need follow-up. Would you like me to send a reminder?"
        });

        let context_payload = json!({
            "trigger": "stale_estimates",
            "insight_type": "operations"
        });

        // 3. Insert into Agent Feed
        let item = AgentFeedItem {
            id: Uuid::new_v4().to_string(),
            tenant_id: tenant_id.clone(),
            event_source: "proactive_analysis".to_string(),
            context_payload: Some(sqlx::types::Json(context_payload)),
            proposed_action: Some(sqlx::types::Json(proposed_action)),
            lifecycle_state: "PENDING_APPROVAL".to_string(),
            created_at: Some(chrono::Utc::now()),
            updated_at: Some(chrono::Utc::now()),
        };

        // We use an admin override or the repo directly
        // The repository itself doesn't set the app.current_tenant context,
        // so we must do it manually for RLS if required, or execute a direct query bypassing RLS as admin worker.
        // However, repo.create does a direct insert. If RLS is enabled, we need to set context.

        let mut conn = db.pool.acquire().await?;
        ::server_common::auth_utils::set_org_context(&mut *conn, &tenant_id).await.map_err(|e| e.to_string())?;

        sqlx::query(
            r#"
            INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#
        )
        .bind(&item.id)
        .bind(&item.tenant_id)
        .bind(&item.event_source)
        .bind(&item.context_payload)
        .bind(&item.proposed_action)
        .bind(&item.lifecycle_state)
        .bind(&item.created_at)
        .bind(&item.updated_at)
        .execute(&mut *conn)
        .await?;

        // 4. Mark Job as Completed
        sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(&job_id)
            .execute(&db.pool)
            .await?;

        Ok(())
    }
}
