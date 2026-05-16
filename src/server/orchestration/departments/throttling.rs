use crate::db::{DB, DbStore};
use chrono::Utc;
use sqlx::Row;
use std::sync::Arc;

pub struct ThrottlingManager {
    db: Arc<DB>,
}

impl ThrottlingManager {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn check_and_consume_budget(
        &self,
        tenant_id: &str,
        points: i32,
    ) -> Result<bool, String> {
        let _now = Utc::now();

        match &self.db.store {
            DbStore::Postgres => {
                let res = sqlx::query("UPDATE tenants SET ai_budget = ai_budget - $1 WHERE tenant_id = $2 AND ai_budget >= $1 RETURNING ai_budget")
                    .bind(points)
                    .bind(tenant_id)
                    .fetch_optional(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())?;

                Ok(res.is_some())
            }
            DbStore::Sqlite(pool) => {
                let res = sqlx::query("UPDATE tenants SET ai_budget = ai_budget - ? WHERE tenant_id = ? AND ai_budget >= ? RETURNING ai_budget")
                    .bind(points)
                    .bind(tenant_id)
                    .bind(points)
                    .fetch_optional(pool)
                    .await
                    .map_err(|e| e.to_string())?;

                Ok(res.is_some())
            }
        }
    }
}
