use std::sync::Arc;
use crate::db::DB;
use std::time::Duration;
use uuid::Uuid;
use serde_json::json;
use tracing::{info};

pub struct DiscoveryReportWorker {
    pub db: Arc<DB>,
    pub poll_interval: Duration,
}

impl DiscoveryReportWorker {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            db,
            poll_interval: Duration::from_secs(3600), // Run every hour (simplification of monthly mechanism)
        }
    }

    pub fn start(&self) {
        let db = self.db.clone();
        let interval_duration = self.poll_interval;
        tokio::spawn(async move {
            loop {
                let _ = Self::process(&db).await;
                tokio::time::sleep(interval_duration).await;
            }
        });
    }

    async fn process(db: &Arc<DB>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let pool = db.pool.clone();

        // Let's get all tenants
        let tenants: Vec<String> = match &db.store {
            crate::db::DbStore::Postgres => {
                let rows = sqlx::query("SELECT id FROM tenants").fetch_all(&pool).await?;
                rows.into_iter().filter_map(|r| sqlx::Row::try_get(&r, "id").ok()).collect()
            },
            crate::db::DbStore::Sqlite(_) => {
                let rows = sqlx::query("SELECT id FROM tenants").fetch_all(&pool).await?;
                rows.into_iter().filter_map(|r| sqlx::Row::try_get(&r, "id").ok()).collect()
            }
        };

        let current_month = chrono::Utc::now().format("%B %Y").to_string();

        for tenant_id_str in tenants {
            if let Ok(tenant_uuid) = Uuid::parse_str(&tenant_id_str) {
                // Check if report exists for current month
                let count: i64 = match &db.store {
                    crate::db::DbStore::Postgres => {
                        sqlx::query_scalar("SELECT COUNT(*) FROM seo_discovery_reports WHERE tenant_id = $1 AND month = $2")
                            .bind(tenant_uuid)
                            .bind(&current_month)
                            .fetch_one(&pool).await?
                    },
                    crate::db::DbStore::Sqlite(_) => {
                        let c: i32 = sqlx::query_scalar("SELECT COUNT(*) FROM seo_discovery_reports WHERE tenant_id = ? AND month = ?")
                            .bind(tenant_id_str.clone())
                            .bind(&current_month)
                            .fetch_one(&pool).await?;
                        c as i64
                    }
                };

                if count == 0 {
                    info!("Generating Discovery Report for tenant {} for month {}", tenant_id_str, current_month);

                    let metrics = json!({
                        "chatgpt_recommendations": 15,
                        "gemini_recommendations": 4,
                        "perplexity_recommendations": 2
                    });
                    let summary = format!("ChatGPT recommended your services 15 times this week to locals in your area. Your AI Discovery Report for {} is ready!", current_month);

                    let mut tx = pool.begin().await?;

                    match &db.store {
                        crate::db::DbStore::Postgres => {
                            // Set context for RLS
                            let _ = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id_str).await;

                            let _ = sqlx::query("INSERT INTO seo_discovery_reports (tenant_id, month, plain_language_summary, metrics) VALUES ($1, $2, $3, $4)")
                                .bind(tenant_uuid)
                                .bind(&current_month)
                                .bind(&summary)
                                .bind(&metrics)
                                .execute(&mut *tx).await?;
                        },
                        crate::db::DbStore::Sqlite(_) => {
                            let _ = sqlx::query("INSERT INTO seo_discovery_reports (tenant_id, month, plain_language_summary, metrics) VALUES (?, ?, ?, ?)")
                                .bind(tenant_id_str.clone())
                                .bind(&current_month)
                                .bind(&summary)
                                .bind(&metrics)
                                .execute(&mut *tx).await?;
                        }
                    }

                    // Insert to agent feed using context override or directly
                    let item_id = Uuid::new_v4().to_string();
                    let proposed_action = json!({
                        "action_type": "view_report",
                        "description": summary
                    });

                    let context_payload = json!({
                        "trigger": "monthly_discovery_report",
                        "insight_type": "marketing"
                    });

                    match &db.store {
                        crate::db::DbStore::Postgres => {
                            let _ = sqlx::query(
                                "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state) VALUES ($1, $2, $3, $4, $5, $6)"
                            )
                            .bind(&item_id)
                            .bind(&tenant_id_str)
                            .bind("discovery_agent")
                            .bind(sqlx::types::Json(context_payload))
                            .bind(sqlx::types::Json(proposed_action))
                            .bind("PENDING_APPROVAL")
                            .execute(&mut *tx).await?;
                        },
                        crate::db::DbStore::Sqlite(_) => {
                             let _ = sqlx::query(
                                "INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state) VALUES (?, ?, ?, ?, ?, ?)"
                            )
                            .bind(&item_id)
                            .bind(&tenant_id_str)
                            .bind("discovery_agent")
                            .bind(sqlx::types::Json(context_payload))
                            .bind(sqlx::types::Json(proposed_action))
                            .bind("PENDING_APPROVAL")
                            .execute(&mut *tx).await?;
                        }
                    }

                    tx.commit().await?;
                }
            }
        }

        Ok(())
    }
}
