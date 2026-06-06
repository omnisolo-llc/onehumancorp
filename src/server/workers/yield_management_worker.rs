use std::sync::Arc;
use std::time::Duration;
use crate::db::DB;
use sqlx::Row;
use chrono::{Utc, NaiveDate};
use uuid::Uuid;

pub struct YieldManagementWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
}

impl YieldManagementWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            poll_interval: Duration::from_secs(60 * 15), // Run every 15 mins
        }
    }

    pub async fn start(&self) {
        loop {
            if let Err(e) = self.process_yield_opportunities().await {
                eprintln!("YieldManagementWorker Error: {}", e);
            }
            tokio::time::sleep(self.poll_interval).await;
        }
    }

    async fn process_yield_opportunities(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let pool = &self.db.pool;

        let target_date = Utc::now().date_naive() + chrono::Duration::days(1);

        // Find tenants and products with rules where utilization < 50%
        let query = "
            SELECT r.id, r.tenant_id, r.product_id, r.trigger_threshold_percent, r.discount_percent, r.target_audience
            FROM yield_rules r
            WHERE r.status = 'ACTIVE'
        ";

        let rules = sqlx::query(query).fetch_all(pool).await?;

        for rule_row in rules {
            let rule_id: String = rule_row.get("id");
            let tenant_id: String = rule_row.get("tenant_id");
            let product_id: String = rule_row.get("product_id");
            let trigger_threshold: i32 = rule_row.get("trigger_threshold_percent");
            let discount_percent: i32 = rule_row.get("discount_percent");
            let target_audience: String = rule_row.get("target_audience");

            // For now we mock the empty slots and total slots until full booking integration
            let total_slots = 10;
            let booked_slots = 2; // In reality, query bookings table for this date
            let empty_slots = total_slots - booked_slots;
            let utilization_percent = (booked_slots * 100) / total_slots;
            let empty_percent = 100 - utilization_percent;

            if empty_percent >= trigger_threshold {
                // Check if opportunity already exists
                let existing: i64 = sqlx::query_scalar(
                    "SELECT COUNT(*) FROM yield_opportunities WHERE tenant_id = $1 AND product_id = $2 AND target_date = $3"
                )
                .bind(&tenant_id)
                .bind(&product_id)
                .bind(&target_date)
                .fetch_one(pool)
                .await?;

                if existing == 0 {
                    let opp_id = Uuid::new_v4().to_string();
                    sqlx::query(
                        "INSERT INTO yield_opportunities (id, tenant_id, product_id, target_date, empty_slots, total_slots, utilization_percent, recommended_discount_percent, target_audience, status)
                         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'PENDING_APPROVAL')"
                    )
                    .bind(&opp_id)
                    .bind(&tenant_id)
                    .bind(&product_id)
                    .bind(&target_date)
                    .bind(empty_slots)
                    .bind(total_slots)
                    .bind(utilization_percent)
                    .bind(discount_percent)
                    .bind(&target_audience)
                    .execute(pool)
                    .await?;
                }
            }
        }

        Ok(())
    }
}
