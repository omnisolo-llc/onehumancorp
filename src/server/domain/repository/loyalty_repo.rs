use std::sync::Arc;
use crate::db::{DB, DbStore};
use super::models::{LoyaltyLedger, InteractionTimeline, Customer};
use chrono::Utc;

pub struct LoyaltyRepository {
    db: Arc<DB>,
}

impl LoyaltyRepository {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn get_ledger(&self, tenant_id: &str, customer_id: &str) -> Result<Option<LoyaltyLedger>, String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_as::<_, LoyaltyLedger>(
                    "SELECT id, tenant_id, customer_id, points_balance, tier_name, last_updated FROM loyalty_ledger WHERE tenant_id = $1 AND customer_id = $2"
                )
                .bind(tenant_id)
                .bind(customer_id)
                .fetch_optional(&self.db.pool)
                .await
                .map_err(|e| e.to_string())
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as::<_, LoyaltyLedger>(
                    "SELECT id, tenant_id, customer_id, points_balance, tier_name, last_updated FROM loyalty_ledger WHERE tenant_id = ? AND customer_id = ?"
                )
                .bind(tenant_id)
                .bind(customer_id)
                .fetch_optional(sqlite_pool)
                .await
                .map_err(|e| e.to_string())
            }
        }
    }

    pub async fn accrue_points(&self, tenant_id: &str, customer_id: &str, points: i32, new_tier: &str) -> Result<LoyaltyLedger, String> {
        let now = Utc::now();
        let ledger = self.get_ledger(tenant_id, customer_id).await?;

        if let Some(existing) = ledger {
            let updated_points = existing.points_balance.unwrap_or(0) + points;
            match &self.db.store {
                DbStore::Postgres => {
                    sqlx::query_as::<_, LoyaltyLedger>(
                        r#"
                        UPDATE loyalty_ledger
                        SET points_balance = $1, tier_name = $2, last_updated = $3
                        WHERE id = $4
                        RETURNING id, tenant_id, customer_id, points_balance, tier_name, last_updated
                        "#
                    )
                    .bind(updated_points)
                    .bind(new_tier)
                    .bind(now)
                    .bind(&existing.id)
                    .fetch_one(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())
                }
                DbStore::Sqlite(sqlite_pool) => {
                    sqlx::query_as::<_, LoyaltyLedger>(
                        r#"
                        UPDATE loyalty_ledger
                        SET points_balance = ?, tier_name = ?, last_updated = ?
                        WHERE id = ?
                        RETURNING id, tenant_id, customer_id, points_balance, tier_name, last_updated
                        "#
                    )
                    .bind(updated_points)
                    .bind(new_tier)
                    .bind(now)
                    .bind(&existing.id)
                    .fetch_one(sqlite_pool)
                    .await
                    .map_err(|e| e.to_string())
                }
            }
        } else {
            let id = uuid::Uuid::new_v4().to_string();
            match &self.db.store {
                DbStore::Postgres => {
                    sqlx::query_as::<_, LoyaltyLedger>(
                        r#"
                        INSERT INTO loyalty_ledger (id, tenant_id, customer_id, points_balance, tier_name, last_updated)
                        VALUES ($1, $2, $3, $4, $5, $6)
                        RETURNING id, tenant_id, customer_id, points_balance, tier_name, last_updated
                        "#
                    )
                    .bind(&id)
                    .bind(tenant_id)
                    .bind(customer_id)
                    .bind(points)
                    .bind(new_tier)
                    .bind(now)
                    .fetch_one(&self.db.pool)
                    .await
                    .map_err(|e| e.to_string())
                }
                DbStore::Sqlite(sqlite_pool) => {
                    sqlx::query_as::<_, LoyaltyLedger>(
                        r#"
                        INSERT INTO loyalty_ledger (id, tenant_id, customer_id, points_balance, tier_name, last_updated)
                        VALUES (?, ?, ?, ?, ?, ?)
                        RETURNING id, tenant_id, customer_id, points_balance, tier_name, last_updated
                        "#
                    )
                    .bind(&id)
                    .bind(tenant_id)
                    .bind(customer_id)
                    .bind(points)
                    .bind(new_tier)
                    .bind(now)
                    .fetch_one(sqlite_pool)
                    .await
                    .map_err(|e| e.to_string())
                }
            }
        }
    }

    pub async fn record_interaction(&self, tenant_id: &str, customer_id: &str, source: &str, sentiment: Option<&str>) -> Result<InteractionTimeline, String> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = Utc::now();
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query_as::<_, InteractionTimeline>(
                    r#"
                    INSERT INTO interaction_timeline (id, tenant_id, customer_id, source, sentiment, occurred_at)
                    VALUES ($1, $2, $3, $4, $5, $6)
                    RETURNING id, tenant_id, customer_id, source, sentiment, occurred_at
                    "#
                )
                .bind(&id)
                .bind(tenant_id)
                .bind(customer_id)
                .bind(source)
                .bind(sentiment)
                .bind(now)
                .fetch_one(&self.db.pool)
                .await
                .map_err(|e| e.to_string())
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query_as::<_, InteractionTimeline>(
                    r#"
                    INSERT INTO interaction_timeline (id, tenant_id, customer_id, source, sentiment, occurred_at)
                    VALUES (?, ?, ?, ?, ?, ?)
                    RETURNING id, tenant_id, customer_id, source, sentiment, occurred_at
                    "#
                )
                .bind(&id)
                .bind(tenant_id)
                .bind(customer_id)
                .bind(source)
                .bind(sentiment)
                .bind(now)
                .fetch_one(sqlite_pool)
                .await
                .map_err(|e| e.to_string())
            }
        }
    }
}
