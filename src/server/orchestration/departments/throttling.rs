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

    pub async fn check_and_consume_budget(&self, tenant_id: &str, points: i32) -> Result<(bool, bool), String> {
        let _now = Utc::now();
        let soft_limit_threshold = 20;

        match &self.db.store {
            DbStore::Postgres => {
                let res = sqlx::query("UPDATE tenants SET ai_budget = ai_budget - $1 WHERE tenant_id = $2 AND ai_budget >= $1 RETURNING ai_budget")
                    .bind(points)
                    .bind(tenant_id)
                    .fetch_optional(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;

                if let Some(row) = res {
                    use sqlx::Row;
                    let remaining: i32 = row.get("ai_budget");
                    // Return true for success, and true if hit soft limit (just crossed it, or below it)
                    let hit_soft_limit = remaining <= soft_limit_threshold && (remaining + points) > soft_limit_threshold;
                    Ok((true, hit_soft_limit))
                } else {
                    Ok((false, false))
                }
            }
            DbStore::Sqlite(pool) => {
                let res = sqlx::query("UPDATE tenants SET ai_budget = ai_budget - ? WHERE tenant_id = ? AND ai_budget >= ? RETURNING ai_budget")
                    .bind(points)
                    .bind(tenant_id)
                    .bind(points)
                    .fetch_optional(pool)
                    .await
                    .map_err(|e| e.to_string())?;

                if let Some(row) = res {
                    use sqlx::Row;
                    let remaining: i32 = row.get("ai_budget");
                    let hit_soft_limit = remaining <= soft_limit_threshold && (remaining + points) > soft_limit_threshold;
                    Ok((true, hit_soft_limit))
                } else {
                    Ok((false, false))
                }
            }
        }
    }
}
