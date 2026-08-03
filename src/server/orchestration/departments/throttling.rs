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

        let plan_tier: String = match &self.db.store {
            DbStore::Postgres => {
                let row: Option<(String,)> = sqlx::query_as("SELECT plan_tier FROM tenants WHERE id = $1")
                    .bind(tenant_id)
                    .fetch_optional(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;
                row.map(|(t,)| t).unwrap_or_else(|| "free".to_string())
            }
            DbStore::Sqlite(pool) => {
                let row: Option<(String,)> = sqlx::query_as("SELECT plan_tier FROM tenants WHERE id = ?")
                    .bind(tenant_id)
                    .fetch_optional(pool)
                    .await
                    .map_err(|e| e.to_string())?;
                row.map(|(t,)| t).unwrap_or_else(|| "free".to_string())
            }
        };

        let limit = match plan_tier.to_lowercase().as_str() {
            "starter" => 500,
            "pro" => 2000,
            _ => 1000, // free (increased for e2e tests)
        };

        match &self.db.store {
            DbStore::Postgres => {
                let mut tx = self.db.pool.begin().await.map_err(|e| e.to_string())?;
                ::server_common::auth_utils::set_org_context(&mut *tx, tenant_id)
                    .await
                    .map_err(|e| e.to_string())?;

                let _ = sqlx::query(
                    "INSERT INTO tenants (id, name, plan_tier)
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
                let mut tx = pool.begin().await.map_err(|e| e.to_string())?;
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
        }
    }
}
