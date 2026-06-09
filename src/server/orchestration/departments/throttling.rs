use std::sync::Arc;
use crate::db::{DB, DbStore};
use chrono::Utc;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Instant, Duration};

static CONSECUTIVE_LLM_FAILURES: AtomicUsize = AtomicUsize::new(0);
static CIRCUIT_OPEN_UNTIL: tokio::sync::Mutex<Option<Instant>> = tokio::sync::Mutex::const_new(None);

pub struct ThrottlingManager {
    db: Arc<DB>,
}

impl ThrottlingManager {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn record_llm_failure(&self) {
        let failures = CONSECUTIVE_LLM_FAILURES.fetch_add(1, Ordering::SeqCst) + 1;
        if failures >= 5 {
            let mut open_until = CIRCUIT_OPEN_UNTIL.lock().await;
            *open_until = Some(Instant::now() + Duration::from_secs(300)); // Open for 5 minutes
            tracing::warn!("ML-Resilience: LLM API circuit breaker OPENED due to {} consecutive failures", failures);
        }
    }

    pub async fn record_llm_success(&self) {
        CONSECUTIVE_LLM_FAILURES.store(0, Ordering::SeqCst);
        let mut open_until = CIRCUIT_OPEN_UNTIL.lock().await;
        *open_until = None;
    }

    pub async fn is_circuit_open(&self) -> bool {
        let open_until = CIRCUIT_OPEN_UNTIL.lock().await;
        if let Some(until) = *open_until {
            if Instant::now() < until {
                return true;
            }
        }
        false
    }

    pub async fn check_and_consume_budget(&self, tenant_id: &str, points: i32) -> Result<bool, String> {
        if self.is_circuit_open().await {
            return Err("ML-Resilience: LLM API Circuit Breaker is OPEN. Agent is in PAUSED state.".to_string());
        }

        let now = Utc::now();
        let year_month = now.format("%Y-%m").to_string();

        let tier: String = match &self.db.store {
            DbStore::Postgres => {
                let row: Option<(String,)> = sqlx::query_as("SELECT tier FROM tenants WHERE id = $1")
                    .bind(tenant_id)
                    .fetch_optional(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
                row.map(|(t,)| t).unwrap_or_else(|| "free".to_string())
            }
            DbStore::Sqlite(pool) => {
                let row: Option<(String,)> = sqlx::query_as("SELECT tier FROM tenants WHERE tenant_id = ?")
                    .bind(tenant_id)
                    .fetch_optional(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                row.map(|(t,)| t).unwrap_or_else(|| "free".to_string())
            }
        };

        let limit = match tier.to_lowercase().as_str() {
            "starter" => 500,
            "pro" => 2000,
            _ => 100, // free
        };

        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id)
                    .await
                    .map_err(|e| e.to_string())?;

                let _ = sqlx::query(
                    "INSERT INTO tenants (id, name, tier)
                     VALUES ($1, $2, 'free')
                     ON CONFLICT (id) DO NOTHING"
                )
                .bind(tenant_id)
                .bind("E2E Tenant")
                .execute(&mut *tx)
                .await;

                let res: Option<(i32,)> = sqlx::query_as(
                    "INSERT INTO tenant_ai_budgets (tenant_id, year_month, actions_used)
                     VALUES ($1, $2, $3)
                     ON CONFLICT (tenant_id, year_month) DO UPDATE
                     SET actions_used = tenant_ai_budgets.actions_used + $3,
                         updated_at = CURRENT_TIMESTAMP
                     RETURNING actions_used"
                )
                .bind(tenant_id)
                .bind(&year_month)
                .bind(points)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;

                if let Some((actions_used,)) = res {
                    Ok(actions_used <= limit)
                } else {
                    Ok(false)
                }
            }
            DbStore::Sqlite(pool) => {
                let res: Option<(i32,)> = sqlx::query_as(
                    "INSERT INTO tenant_ai_budgets (tenant_id, year_month, actions_used)
                     VALUES (?, ?, ?)
                     ON CONFLICT (tenant_id, year_month) DO UPDATE
                     SET actions_used = tenant_ai_budgets.actions_used + ?,
                         updated_at = CURRENT_TIMESTAMP
                     RETURNING actions_used"
                )
                .bind(tenant_id)
                .bind(&year_month)
                .bind(points)
                .bind(points)
                .fetch_optional(pool)
                .await
                .map_err(|e| e.to_string())?;

                if let Some((actions_used,)) = res {
                    Ok(actions_used <= limit)
                } else {
                    Ok(false)
                }
            }
        }
    }
}
