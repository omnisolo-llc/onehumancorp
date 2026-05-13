use std::sync::Arc;
use crate::db::{DB, DbStore};
use chrono::Utc;
use sqlx::Row;

pub struct ThrottlingManager {
    db: Arc<DB>,
}

impl ThrottlingManager {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn check_and_consume_budget(&self, tenant_id: &str, points: i32) -> Result<bool, String> {
        let _now = Utc::now();

        match &self.db.store {
            DbStore::Postgres => {
                let res = sqlx::query("UPDATE tenants SET ai_budget = ai_budget - $1 WHERE tenant_id = $2 RETURNING ai_budget")
                    .bind(points)
                    .bind(tenant_id)
                    .fetch_optional(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;

                if let Some(row) = res {
                    let new_budget: i32 = row.get("ai_budget");
                    Ok(new_budget >= 0)
                } else {
                    Ok(false)
                }
            }
            DbStore::Sqlite(pool) => {
                let res = sqlx::query("UPDATE tenants SET ai_budget = ai_budget - ? WHERE tenant_id = ? RETURNING ai_budget")
                    .bind(points)
                    .bind(tenant_id)
                    .fetch_optional(pool)
                    .await
                    .map_err(|e| e.to_string())?;

                if let Some(row) = res {
                    let new_budget: i32 = row.get("ai_budget");
                    Ok(new_budget >= 0)
                } else {
                    Ok(false)
                }
            }
        }
    }
}
