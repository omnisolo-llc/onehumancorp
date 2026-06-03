use std::sync::Arc;
use server_lib::db::{DB, DbStore};
use super::models::{Customer360, InteractionTimeline, LoyaltyLedger};
use chrono::Utc;
use uuid::Uuid;

pub struct CrmRepository {
    db: Arc<DB>,
}

impl CrmRepository {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }

    pub async fn create_customer(&self, customer: Customer360) -> Result<Customer360, String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(
                    r#"
                    INSERT INTO ohc_customer360 (
                        id, tenant_id, email, phone, mood, preferences, created_at, updated_at
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                    "#
                )
                .bind(&customer.id).bind(&customer.tenant_id).bind(&customer.email)
                .bind(&customer.phone).bind(&customer.mood).bind(&customer.preferences)
                .bind(&customer.created_at).bind(&customer.updated_at)
                .execute(&self.db.pool).await.map_err(|e| e.to_string())?;

                Ok(customer)
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO ohc_customer360 (
                        id, tenant_id, email, phone, mood, preferences, created_at, updated_at
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                    "#
                )
                .bind(&customer.id).bind(&customer.tenant_id).bind(&customer.email)
                .bind(&customer.phone).bind(&customer.mood).bind(serde_json::to_string(&customer.preferences).unwrap())
                .bind(&customer.created_at).bind(&customer.updated_at)
                .execute(sqlite_pool).await.map_err(|e| e.to_string())?;

                Ok(customer)
            }
        }
    }

    pub async fn get_customer(&self, tenant_id: &str, customer_id: &str) -> Result<Option<Customer360>, String> {
        match &self.db.store {
            DbStore::Postgres => {
                let customer = sqlx::query_as::<_, Customer360>(
                    "SELECT * FROM ohc_customer360 WHERE tenant_id = $1 AND id = $2"
                )
                .bind(tenant_id).bind(customer_id)
                .fetch_optional(&self.db.pool).await.map_err(|e| e.to_string())?;
                Ok(customer)
            }
            DbStore::Sqlite(sqlite_pool) => {
                let row = sqlx::query(
                    "SELECT * FROM ohc_customer360 WHERE tenant_id = $1 AND id = $2"
                )
                .bind(tenant_id).bind(customer_id)
                .fetch_optional(sqlite_pool).await.map_err(|e| e.to_string())?;

                if let Some(r) = row {
                    use sqlx::Row;
                    let prefs_str: String = r.get("preferences");
                    let prefs: serde_json::Value = serde_json::from_str(&prefs_str).unwrap_or(serde_json::json!({}));

                    Ok(Some(Customer360 {
                        id: r.get("id"),
                        tenant_id: r.get("tenant_id"),
                        email: r.get("email"),
                        phone: r.get("phone"),
                        mood: r.get("mood"),
                        preferences: sqlx::types::Json(prefs),
                        created_at: r.get("created_at"),
                        updated_at: r.get("updated_at"),
                    }))
                } else {
                    Ok(None)
                }
            }
        }
    }

    pub async fn update_customer_mood(&self, tenant_id: &str, customer_id: &str, mood: &str) -> Result<(), String> {
         match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(
                    "UPDATE ohc_customer360 SET mood = $1, updated_at = $2 WHERE tenant_id = $3 AND id = $4"
                )
                .bind(mood).bind(Utc::now()).bind(tenant_id).bind(customer_id)
                .execute(&self.db.pool).await.map_err(|e| e.to_string())?;
                Ok(())
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query(
                    "UPDATE ohc_customer360 SET mood = $1, updated_at = $2 WHERE tenant_id = $3 AND id = $4"
                )
                .bind(mood).bind(Utc::now()).bind(tenant_id).bind(customer_id)
                .execute(sqlite_pool).await.map_err(|e| e.to_string())?;
                Ok(())
            }
        }
    }

    pub async fn record_interaction(&self, interaction: InteractionTimeline) -> Result<InteractionTimeline, String> {
        match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(
                    r#"
                    INSERT INTO ohc_interaction_timeline (
                        id, tenant_id, customer_id, source, sentiment, occurred_at
                    ) VALUES ($1, $2, $3, $4, $5, $6)
                    "#
                )
                .bind(&interaction.id).bind(&interaction.tenant_id).bind(&interaction.customer_id)
                .bind(&interaction.source).bind(&interaction.sentiment).bind(&interaction.occurred_at)
                .execute(&self.db.pool).await.map_err(|e| e.to_string())?;

                Ok(interaction)
            }
            DbStore::Sqlite(sqlite_pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO ohc_interaction_timeline (
                        id, tenant_id, customer_id, source, sentiment, occurred_at
                    ) VALUES ($1, $2, $3, $4, $5, $6)
                    "#
                )
                .bind(&interaction.id).bind(&interaction.tenant_id).bind(&interaction.customer_id)
                .bind(&interaction.source).bind(&interaction.sentiment).bind(&interaction.occurred_at)
                .execute(sqlite_pool).await.map_err(|e| e.to_string())?;

                Ok(interaction)
            }
        }
    }

    pub async fn get_interactions(&self, tenant_id: &str, customer_id: &str) -> Result<Vec<InteractionTimeline>, String> {
         match &self.db.store {
            DbStore::Postgres => {
                let interactions = sqlx::query_as::<_, InteractionTimeline>(
                    "SELECT * FROM ohc_interaction_timeline WHERE tenant_id = $1 AND customer_id = $2 ORDER BY occurred_at DESC"
                )
                .bind(tenant_id).bind(customer_id)
                .fetch_all(&self.db.pool).await.map_err(|e| e.to_string())?;
                Ok(interactions)
            }
            DbStore::Sqlite(sqlite_pool) => {
                let interactions = sqlx::query_as::<_, InteractionTimeline>(
                    "SELECT * FROM ohc_interaction_timeline WHERE tenant_id = $1 AND customer_id = $2 ORDER BY occurred_at DESC"
                )
                .bind(tenant_id).bind(customer_id)
                .fetch_all(sqlite_pool).await.map_err(|e| e.to_string())?;
                Ok(interactions)
            }
        }
    }

    pub async fn upsert_loyalty(&self, loyalty: LoyaltyLedger) -> Result<LoyaltyLedger, String> {
         match &self.db.store {
            DbStore::Postgres => {
                sqlx::query(
                    r#"
                    INSERT INTO ohc_loyalty_ledger (
                        id, tenant_id, customer_id, points_balance, tier_name, last_updated
                    ) VALUES ($1, $2, $3, $4, $5, $6)
                    ON CONFLICT (customer_id) DO UPDATE SET
                        points_balance = EXCLUDED.points_balance,
                        tier_name = EXCLUDED.tier_name,
                        last_updated = EXCLUDED.last_updated
                    "#
                )
                .bind(&loyalty.id).bind(&loyalty.tenant_id).bind(&loyalty.customer_id)
                .bind(&loyalty.points_balance).bind(&loyalty.tier_name).bind(&loyalty.last_updated)
                .execute(&self.db.pool).await.map_err(|e| e.to_string())?;

                Ok(loyalty)
            }
            DbStore::Sqlite(sqlite_pool) => {
                // SQLite upsert handling based on the UNIQUE customer_id constraint
                sqlx::query(
                    r#"
                    INSERT INTO ohc_loyalty_ledger (
                        id, tenant_id, customer_id, points_balance, tier_name, last_updated
                    ) VALUES ($1, $2, $3, $4, $5, $6)
                    ON CONFLICT (customer_id) DO UPDATE SET
                        points_balance = excluded.points_balance,
                        tier_name = excluded.tier_name,
                        last_updated = excluded.last_updated
                    "#
                )
                .bind(&loyalty.id).bind(&loyalty.tenant_id).bind(&loyalty.customer_id)
                .bind(&loyalty.points_balance).bind(&loyalty.tier_name).bind(&loyalty.last_updated)
                .execute(sqlite_pool).await.map_err(|e| e.to_string())?;

                Ok(loyalty)
            }
        }
    }

    pub async fn get_loyalty(&self, tenant_id: &str, customer_id: &str) -> Result<Option<LoyaltyLedger>, String> {
         match &self.db.store {
            DbStore::Postgres => {
                let loyalty = sqlx::query_as::<_, LoyaltyLedger>(
                    "SELECT * FROM ohc_loyalty_ledger WHERE tenant_id = $1 AND customer_id = $2"
                )
                .bind(tenant_id).bind(customer_id)
                .fetch_optional(&self.db.pool).await.map_err(|e| e.to_string())?;
                Ok(loyalty)
            }
            DbStore::Sqlite(sqlite_pool) => {
                let loyalty = sqlx::query_as::<_, LoyaltyLedger>(
                    "SELECT * FROM ohc_loyalty_ledger WHERE tenant_id = $1 AND customer_id = $2"
                )
                .bind(tenant_id).bind(customer_id)
                .fetch_optional(sqlite_pool).await.map_err(|e| e.to_string())?;
                Ok(loyalty)
            }
        }
    }
}
