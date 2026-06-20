use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct LeadRecoveryConfig {
    pub abandoned_after: chrono::Duration,
    pub batch_limit: i64,
}

impl Default for LeadRecoveryConfig {
    fn default() -> Self {
        Self {
            abandoned_after: chrono::Duration::hours(2),
            batch_limit: 100,
        }
    }
}

#[derive(Debug, Default)]
pub struct LeadRecoverySummary {
    pub scanned: i64,
    pub dispatched: i64,
}

pub async fn run_lead_recovery_scan_once(
    pool: Arc<PgPool>,
    config: LeadRecoveryConfig,
) -> Result<LeadRecoverySummary, String> {
    let mut summary = LeadRecoverySummary::default();
    let abandoned_before = Utc::now() - config.abandoned_after;

    let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

    // 1. Scan stalled service_leads
    let stalled_leads = sqlx::query(
        "SELECT id, tenant_id, customer_id, status, updated_at, description
         FROM service_leads
         WHERE status IN ('new', 'estimating') AND updated_at <= $1
         LIMIT $2 FOR UPDATE SKIP LOCKED"
    )
    .bind(abandoned_before)
    .bind(config.batch_limit)
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    for row in stalled_leads {
        summary.scanned += 1;
        let id: String = row.try_get("id").unwrap_or_default();
        let tenant_id: String = row.try_get("tenant_id").unwrap_or_default();
        let customer_id_uuid: Option<uuid::Uuid> = row.try_get("customer_id").unwrap_or_default();
        let customer_id = customer_id_uuid.map(|u| u.to_string());
        let description: Option<String> = row.try_get("description").unwrap_or_default();

        let payload = serde_json::json!({
            "feature_type": "lead_recovery",
            "source_type": "service_lead",
            "source_id": id,
            "description": description.unwrap_or_else(|| "Missed Service Lead".to_string()),
            "draft_reply": "Hi there! I noticed you reached out for an estimate. Are you still looking for help with this? Let me know and I can get a quick quote over to you today.",
        });

        let action_id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO agent_action_requests (id, tenant_id, source, agent_type, action_type, payload, status)
             VALUES ($1, $2, 'lead_recovery_worker', 'Customer Relationship', 'lead_recovery', $3, 'Pending')"
        )
        .bind(&action_id)
        .bind(&tenant_id)
        .bind(&payload)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query("UPDATE service_leads SET status = 'abandoned', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(&id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        // Add to recovery_attempts to track it
        let attempt_id = Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO recovery_attempts (id, tenant_id, customer_id, source_event_id, assistant_message_id, status) VALUES ($1, $2, $3, $4, $5, 'DRAFTED')")
            .bind(&attempt_id)
            .bind(&tenant_id)
            .bind(&customer_id)
            .bind(&id)
            .bind(&action_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        summary.dispatched += 1;
    }

    // 2. Scan stalled estimates
    let stalled_estimates = sqlx::query(
        "SELECT id, tenant_id, customer_id, status, updated_at, description
         FROM estimates
         WHERE status IN ('draft', 'sent') AND updated_at <= $1
         LIMIT $2 FOR UPDATE SKIP LOCKED"
    )
    .bind(abandoned_before)
    .bind(config.batch_limit)
    .fetch_all(&mut *tx)
    .await
    .map_err(|e| e.to_string())?;

    for row in stalled_estimates {
        summary.scanned += 1;
        let id: String = row.try_get("id").unwrap_or_default();
        let tenant_id: String = row.try_get("tenant_id").unwrap_or_default();
        let customer_id_uuid: Option<uuid::Uuid> = row.try_get("customer_id").unwrap_or_default();
        let customer_id = customer_id_uuid.map(|u| u.to_string());
        let description: Option<String> = row.try_get("description").unwrap_or_default();

        let payload = serde_json::json!({
            "feature_type": "lead_recovery",
            "source_type": "estimate",
            "source_id": id,
            "description": description.unwrap_or_else(|| "Stalled Estimate".to_string()),
            "draft_reply": "Hello! Just checking in to see if you had any questions about the estimate I sent over. I'm happy to walk through it if that helps!",
        });

        let action_id = Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO agent_action_requests (id, tenant_id, source, agent_type, action_type, payload, status)
             VALUES ($1, $2, 'lead_recovery_worker', 'Sales', 'lead_recovery', $3, 'Pending')"
        )
        .bind(&action_id)
        .bind(&tenant_id)
        .bind(&payload)
        .execute(&mut *tx)
        .await
        .map_err(|e| e.to_string())?;

        sqlx::query("UPDATE estimates SET status = 'abandoned', updated_at = CURRENT_TIMESTAMP WHERE id = $1")
            .bind(&id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        // Add to recovery_attempts to track it
        let attempt_id = Uuid::new_v4().to_string();
        sqlx::query("INSERT INTO recovery_attempts (id, tenant_id, customer_id, source_event_id, assistant_message_id, status) VALUES ($1, $2, $3, $4, $5, 'DRAFTED')")
            .bind(&attempt_id)
            .bind(&tenant_id)
            .bind(&customer_id)
            .bind(&id)
            .bind(&action_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        summary.dispatched += 1;
    }

    tx.commit().await.map_err(|e| e.to_string())?;

    Ok(summary)
}

pub fn spawn_lead_recovery_background_workers(pool: Arc<PgPool>) {
    let scan_pool = pool.clone();
    tokio::spawn(async move {
        let interval_seconds = std::env::var("OHC_LEAD_RECOVERY_SCAN_INTERVAL_SECONDS")
            .ok()
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(300)
            .max(30);
        loop {
            match run_lead_recovery_scan_once(scan_pool.clone(), LeadRecoveryConfig::default()).await {
                Ok(summary) => {
                    if summary.scanned > 0 || summary.dispatched > 0 {
                        tracing::info!(
                            scanned = summary.scanned,
                            dispatched = summary.dispatched,
                            "lead recovery scan completed"
                        );
                    }
                }
                Err(err) => {
                    ::server_telemetry::record_error_signal("Lead recovery scan failed");
                    tracing::warn!("Lead recovery scan failed: {}", err);
                }
            }
            tokio::time::sleep(std::time::Duration::from_secs(interval_seconds)).await;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lead_recovery_config() {
        let config = LeadRecoveryConfig::default();
        assert_eq!(config.abandoned_after, chrono::Duration::hours(2));
        assert_eq!(config.batch_limit, 100);
    }
}
