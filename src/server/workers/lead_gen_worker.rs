use crate::db::{DbStore, DB};
use chrono::Utc;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use tokio::time::{sleep, Duration};

pub struct LeadGenWorker {
    db: Arc<DB>,
}

impl LeadGenWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        tokio::spawn(async move {
            loop {
                match Self::poll(&db).await {
                    Ok(true) => {
                        // Work was done, poll again immediately
                    }
                    Ok(false) => {
                        // No work found, back off
                        sleep(Duration::from_secs(5)).await;
                    }
                    Err(e) => {
                        tracing::error!("LeadGenWorker error: {}", e);
                        sleep(Duration::from_secs(10)).await;
                    }
                }
            }
        });
    }

    pub async fn poll(db: &DB) -> Result<bool, sqlx::Error> {
        let DbStore::Postgres = &db.store else {
            return Ok(false);
        };

        let pool = &db.pool;
        let mut transaction = pool.begin().await?;

        // Find a job
        let job = sqlx::query(
            r#"
            SELECT id, tenant_id, payload
            FROM ohc_job_queue
            WHERE job_type = 'lead_gen_campaign'
              AND status = 'PENDING'
              AND next_retry_at <= CURRENT_TIMESTAMP
            ORDER BY next_retry_at ASC
            FOR UPDATE SKIP LOCKED
            LIMIT 1
            "#,
        )
        .fetch_optional(&mut *transaction)
        .await?;

        if let Some(row) = job {
            let job_id: String = row.get("id");
            let tenant_id: String = row.get("tenant_id");

            // 1. Update the campaign status
            let _ = sqlx::query("UPDATE lead_gen_campaigns SET status = 'ACTIVE' WHERE tenant_id = $1")
                .bind(&tenant_id)
                .execute(&mut *transaction)
                .await;

            // 2. Insert a simulated test lead and appointment booking into inbox_messages
            let message_id = format!("msg_{}", uuid::Uuid::new_v4());
            let _ = sqlx::query(
                "INSERT INTO inbox_messages (id, tenant_id, source, content, status) VALUES ($1, $2, 'LeadGen', 'New Booking: Sink Repair. $50 deposit paid.', 'UNREAD')"
            )
            .bind(&message_id)
            .bind(&tenant_id)
            .execute(&mut *transaction)
            .await;

            // 3. Mark job as completed
            let _ = sqlx::query("UPDATE ohc_job_queue SET status = 'COMPLETED' WHERE id = $1")
                .bind(&job_id)
                .execute(&mut *transaction)
                .await;

            transaction.commit().await?;
            return Ok(true);
        }

        transaction.rollback().await?;
        Ok(false)
    }
}
