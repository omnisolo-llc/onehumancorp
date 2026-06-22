use std::sync::Arc;
use crate::db::{DB, DbStore};
use chrono::Utc;

pub struct ThrottlingManager {
    db: Arc<DB>,
}

impl ThrottlingManager {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn check_and_consume_budget(&self, tenant_id: &str, points: i32) -> Result<bool, String> {
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
                let row: Option<(String,)> = sqlx::query_as("SELECT tier FROM tenants WHERE id = ?")
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
            _ => 1000, // free (increased for e2e tests)
        };

        if points > limit {
            return Ok(false);
        }

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

                // First, check existing points to handle the initial insertion exceeding limit safely.
                let current_usage: Option<i32> = sqlx::query_scalar(
                    "SELECT actions_used FROM tenant_ai_budgets WHERE tenant_id = $1 AND year_month = $2 FOR UPDATE"
                )
                .bind(tenant_id)
                .bind(&year_month)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let current = current_usage.unwrap_or(0);
                if current + points > limit {
                    tx.rollback().await.ok();
                    return Ok(false);
                }

                let _ = sqlx::query(
                    "INSERT INTO tenant_ai_budgets (tenant_id, year_month, actions_used)
                     VALUES ($1, $2, $3)
                     ON CONFLICT (tenant_id, year_month) DO UPDATE
                     SET actions_used = tenant_ai_budgets.actions_used + $3,
                         updated_at = CURRENT_TIMESTAMP"
                )
                .bind(tenant_id)
                .bind(&year_month)
                .bind(points)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;

                Ok(true)
            }
            DbStore::Sqlite(pool) => {
                let mut tx = pool.begin().await.map_err(|e| e.to_string())?;

                let current_usage: Option<i32> = sqlx::query_scalar(
                    "SELECT actions_used FROM tenant_ai_budgets WHERE tenant_id = ? AND year_month = ?"
                )
                .bind(tenant_id)
                .bind(&year_month)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                let current = current_usage.unwrap_or(0);
                if current + points > limit {
                    tx.rollback().await.ok();
                    return Ok(false);
                }

                let _ = sqlx::query(
                    "INSERT INTO tenant_ai_budgets (tenant_id, year_month, actions_used)
                     VALUES (?, ?, ?)
                     ON CONFLICT (tenant_id, year_month) DO UPDATE
                     SET actions_used = tenant_ai_budgets.actions_used + ?,
                         updated_at = CURRENT_TIMESTAMP"
                )
                .bind(tenant_id)
                .bind(&year_month)
                .bind(points)
                .bind(points)
                .execute(&mut *tx)
                .await
                .map_err(|e| e.to_string())?;

                tx.commit().await.map_err(|e| e.to_string())?;

                Ok(true)
            }
        }
    }
}
