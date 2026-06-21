use tracing::{info, error};
use std::sync::Arc;
use tokio::time::{interval, Duration};

use crate::db::DB;
use crate::domain::repository::agent_feed_repo::{AgentFeedRepository, AgentFeedItem};
use chrono::Utc;
use uuid::Uuid;
use sqlx::Row;

pub struct CfoWorker {
    pub db: Arc<DB>,
}

impl CfoWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub fn start(self: Arc<Self>) {
        info!("Starting CFO Worker...");
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(3600)); // Run every hour
            loop {
                interval.tick().await;
                if let Err(e) = self.process_cashflow().await {
                    error!("Error processing cashflow: {}", e);
                }
            }
        });
    }

    pub async fn process_cashflow(&self) -> Result<(), sqlx::Error> {
        let repo = AgentFeedRepository::new(self.db.pool.clone());
        let pool = &self.db.pool;

        // Query expected income from pending invoices within 7 days
        let invoices = sqlx::query(
            r#"
            SELECT tenant_id, SUM(total_amount) as income
            FROM invoices
            WHERE status = 'pending' AND due_date < $1
            GROUP BY tenant_id
            "#
        )
        .bind(Utc::now() + chrono::Duration::days(7))
        .fetch_all(pool)
        .await?;

        // Query expected expenses from recurring expenses or ledger within 7 days
        // Assuming `ledger_entries` exist with `entry_type = 'expense'`
        let expenses = sqlx::query(
            r#"
            SELECT tenant_id, SUM(amount) as outgoing
            FROM ledger_entries
            WHERE entry_type = 'expense' AND created_at > $1
            GROUP BY tenant_id
            "#
        )
        .bind(Utc::now() - chrono::Duration::days(7)) // Simulating expected upcoming based on last week
        .fetch_all(pool)
        .await?;

        use std::collections::HashMap;
        let mut projections: HashMap<String, (f64, f64)> = HashMap::new();

        for row in invoices {
            let tenant_id: String = row.get("tenant_id");
            let income: f64 = row.try_get("income").unwrap_or(0.0);
            projections.insert(tenant_id, (income, 0.0));
        }

        for row in expenses {
            let tenant_id: String = row.get("tenant_id");
            let outgoing: f64 = row.try_get("outgoing").unwrap_or(0.0);
            let entry = projections.entry(tenant_id).or_insert((0.0, 0.0));
            entry.1 = outgoing;
        }

        // We bypass LLM here and format a direct message as requested by architecture design limitations
        for (tenant_id, (income, outgoing)) in projections {
            if outgoing > income {
                let diff = outgoing - income;

                // Check if we already alerted them recently
                let already_alerted = sqlx::query(
                    r#"
                    SELECT id FROM agent_feed_items
                    WHERE tenant_id = $1 AND event_source = 'cfo_agent_worker'
                    AND created_at > $2
                    "#
                )
                .bind(&tenant_id)
                .bind(Utc::now() - chrono::Duration::days(1))
                .fetch_optional(pool)
                .await?;

                if already_alerted.is_some() {
                    continue; // Skip spamming
                }

                let message_text = format!(
                    "You have a projected cash deficit of ${:.2} this week. \
                    You have ${:.2} incoming from pending invoices and ${:.2} outgoing expenses. \
                    Would you like to send reminders for overdue invoices?",
                    diff, income, outgoing
                );

                let item = AgentFeedItem {
                    id: Uuid::new_v4().to_string(),
                    tenant_id: tenant_id.clone(),
                    event_source: "cfo_agent_worker".to_string(),
                    context_payload: Some(sqlx::types::Json(serde_json::json!({
                        "message": message_text,
                        "deficit": diff,
                        "incoming": income,
                        "outgoing": outgoing
                    }))),
                    proposed_action: Some(sqlx::types::Json(serde_json::json!([
                        {
                            "action_type": "Send Invoice Reminder",
                            "target": "All Overdue"
                        }
                    ]))),
                    lifecycle_state: "PENDING_APPROVAL".to_string(),
                    created_at: Some(Utc::now()),
                    updated_at: Some(Utc::now()),
                };

                match repo.create(item).await {
                    Ok(_) => info!("Successfully created CFO alert in agent feed for tenant {}", tenant_id),
                    Err(e) => error!("Failed to create CFO alert for tenant {}: {}", tenant_id, e),
                }
            }
        }

        Ok(())
    }
}
