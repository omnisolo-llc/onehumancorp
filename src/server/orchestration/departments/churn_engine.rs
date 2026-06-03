use crate::orchestration::departments::orchestrator::AgentOrchestrator;
use crate::orchestration::departments::types::{Customer360, DepartmentType, TimelineEvent, ApprovalRequest, ApprovalStatus, ActionRisk};
use chrono::{Utc, Duration, DateTime};
use sqlx::Row;
use serde_json::json;

pub struct ChurnPredictionEngine;

impl ChurnPredictionEngine {
    pub async fn run_nightly_job(orchestrator: &AgentOrchestrator, _ignored_tenant: &str) -> Result<(), String> {
        let pool = match &orchestrator.db.store {
            crate::db::DbStore::Postgres => &orchestrator.db.pool,
            crate::db::DbStore::Sqlite(pool) => pool,
        };

        // Query distinct tenants from the database instead of hardcoding 'system'
        let tenants: Vec<String> = sqlx::query("SELECT DISTINCT tenant_id FROM customer360")
            .fetch_all(pool)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|r| r.get("tenant_id"))
            .collect();

        for tenant_id in tenants {
            let customers: Vec<Customer360> = sqlx::query(
                "SELECT * FROM customer360 WHERE tenant_id = ?"
            )
            .bind(&tenant_id)
            .fetch_all(pool)
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|r| {
                let prefs_str: Option<String> = r.try_get("preferences").ok();
                Customer360 {
                    id: r.get("id"),
                    tenant_id: r.get("tenant_id"),
                    customer_id: r.get("customer_id"),
                    email: r.try_get("email").ok(),
                    phone: r.try_get("phone").ok(),
                    mood: r.try_get("mood").ok(),
                    preferences: prefs_str.and_then(|s| serde_json::from_str(&s).ok()),
                    created_at: r.try_get("created_at").ok(),
                    updated_at: r.try_get("updated_at").ok(),
                    status: r.try_get("status").ok(),
                    expected_purchase_cadence_days: r.try_get("expected_purchase_cadence_days").ok(),
                }
            })
            .collect();

            for mut c in customers {
                let interactions: Vec<TimelineEvent> = sqlx::query(
                    "SELECT * FROM interaction_timeline WHERE tenant_id = ? AND customer_id = ? ORDER BY occurred_at DESC"
                )
                .bind(&tenant_id)
                .bind(&c.customer_id)
                .fetch_all(pool)
                .await
                .map_err(|e| e.to_string())?
                .into_iter()
                .map(|r| {
                    let meta_str: Option<String> = r.try_get("metadata").ok();
                    TimelineEvent {
                        id: r.get("id"),
                        tenant_id: r.get("tenant_id"),
                        customer_id: r.get("customer_id"),
                        event_type: r.get("event_type"),
                        source: r.get("source"),
                        content: r.get("content"),
                        metadata: meta_str.and_then(|s| serde_json::from_str(&s).ok()),
                        created_at: r.try_get("occurred_at").ok(),
                    }
                })
                .collect();

                // Calculate expected cadence based on interaction history
                if interactions.len() > 1 {
                    let mut total_days = 0.0;
                    let mut diffs_count = 0;
                    for i in 0..(interactions.len() - 1) {
                        if let (Some(t1), Some(t2)) = (interactions[i].created_at, interactions[i + 1].created_at) {
                            let diff = t1.signed_duration_since(t2).num_days();
                            if diff > 0 {
                                total_days += diff as f64;
                                diffs_count += 1;
                            }
                        }
                    }

                    if diffs_count > 0 {
                        c.expected_purchase_cadence_days = Some(total_days / diffs_count as f64);
                    }
                }

                // Check if customer is at risk
                if let Some(cadence) = c.expected_purchase_cadence_days {
                    if let Some(last_interaction) = interactions.first() {
                        if let Some(last_time) = last_interaction.created_at {
                            let days_since = Utc::now().signed_duration_since(last_time).num_days() as f64;

                            // Deviation > 1.5x expected cadence -> At-Risk
                            if days_since > cadence * 1.5 {
                                let old_status = c.status.clone();
                                c.status = Some("At-Risk".to_string());

                                orchestrator.upsert_customer360(&c).await?;

                                // Trigger winback draft if status just changed
                                if old_status.as_deref() != Some("At-Risk") {
                                    let req = ApprovalRequest {
                                        id: uuid::Uuid::new_v4().to_string(),
                                        tenant_id: tenant_id.to_string(),
                                        department: DepartmentType::CustomerSuccess,
                                        description: format!("Winback Opportunity for {}", c.customer_id),
                                        status: ApprovalStatus::PendingApproval,
                                        action_risk: ActionRisk::DraftForReview,
                                        payload: Some(json!({
                                            "action": "send_sms",
                                            "customer_id": c.customer_id,
                                            "message": format!("Hi there! We just got some new items we think you'd love. Here's 10% off if you drop by this week!")
                                        })),
                                    };
                                    orchestrator.create_approval_request(req).await?;
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
