use crate::db::DB;
use super::models::{YieldProfile, PriceAdjustmentEvent, CapacityState, DemandSignal};
use uuid::Uuid;
use chrono::Utc;

pub struct YieldEngine {
    db: DB,
}

impl YieldEngine {
    pub fn new(db: DB) -> Self {
        Self { db }
    }

    pub async fn get_current_price(&self, tenant_id: &str, target_id: &str, base_price_cents: i64) -> Result<i64, String> {
        // Query profile
        let profile_opt = sqlx::query_as!(
            YieldProfile,
            "SELECT * FROM yield_profiles WHERE tenant_id = $1 AND target_id = $2 AND enabled = true",
            tenant_id,
            target_id
        )
        .fetch_optional(&self.db.pool)
        .await
        .map_err(|e| e.to_string())?;

        let profile = match profile_opt {
            Some(p) => p,
            None => return Ok(base_price_cents), // If no profile or disabled, return base price
        };

        // Query latest capacity
        let capacity = sqlx::query_as!(
            CapacityState,
            "SELECT * FROM capacity_states WHERE tenant_id = $1 AND yield_profile_id = $2 ORDER BY updated_at DESC LIMIT 1",
            tenant_id,
            profile.id
        )
        .fetch_optional(&self.db.pool)
        .await
        .map_err(|e| e.to_string())?;

        // Query recent demand signals
        let demand_signals = sqlx::query_as!(
            DemandSignal,
            "SELECT * FROM demand_signals WHERE tenant_id = $1 AND yield_profile_id = $2 AND created_at > NOW() - INTERVAL '1 day'",
            tenant_id,
            profile.id
        )
        .fetch_all(&self.db.pool)
        .await
        .map_err(|e| e.to_string())?;

        // Calculate dynamic price
        let mut final_price = base_price_cents as f64;

        if let Some(cap) = capacity {
            if cap.total > 0 {
                let capacity_ratio = cap.available as f64 / cap.total as f64;
                if capacity_ratio < 0.2 {
                    // Scarcity, increase price by 20%
                    final_price *= 1.20;
                } else if capacity_ratio > 0.8 {
                    // Surplus, decrease price by 10%
                    final_price *= 0.90;
                }
            }
        }

        let mut total_demand_score = 0.0;
        for signal in &demand_signals {
            total_demand_score += signal.score;
        }

        if !demand_signals.is_empty() {
            let avg_demand = total_demand_score / demand_signals.len() as f64;
            // Adjust based on demand (-1.0 to 1.0 score mapping to -10% to +10%)
            final_price *= 1.0 + (avg_demand * 0.1);
        }

        let mut adjusted_price = final_price.round() as i64;

        // Enforce min/max constraints
        if adjusted_price < profile.min_price_cents {
            adjusted_price = profile.min_price_cents;
        }
        if adjusted_price > profile.max_price_cents {
            adjusted_price = profile.max_price_cents;
        }

        Ok(adjusted_price)
    }

    pub async fn update_capacity(&self, tenant_id: &str, target_id: &str, available: i64, total: i64) -> Result<(), String> {
        let profile_opt = sqlx::query_as!(
            YieldProfile,
            "SELECT * FROM yield_profiles WHERE tenant_id = $1 AND target_id = $2",
            tenant_id,
            target_id
        )
        .fetch_optional(&self.db.pool)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(profile) = profile_opt {
            let id = Uuid::new_v4().to_string();
            sqlx::query!(
                "INSERT INTO capacity_states (id, tenant_id, yield_profile_id, available, total) VALUES ($1, $2, $3, $4, $5)",
                id, tenant_id, profile.id, available, total
            )
            .execute(&self.db.pool)
            .await
            .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub async fn add_demand_signal(&self, tenant_id: &str, target_id: &str, signal_type: &str, score: f64) -> Result<(), String> {
        let profile_opt = sqlx::query_as!(
            YieldProfile,
            "SELECT * FROM yield_profiles WHERE tenant_id = $1 AND target_id = $2",
            tenant_id,
            target_id
        )
        .fetch_optional(&self.db.pool)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(profile) = profile_opt {
            let id = Uuid::new_v4().to_string();
            sqlx::query!(
                "INSERT INTO demand_signals (id, tenant_id, yield_profile_id, signal_type, score) VALUES ($1, $2, $3, $4, $5)",
                id, tenant_id, profile.id, signal_type, score
            )
            .execute(&self.db.pool)
            .await
            .map_err(|e| e.to_string())?;
        }
        Ok(())
    }

    pub async fn configure_profile(&self, tenant_id: &str, target_id: &str, target_type: &str, enabled: bool, min_price_cents: i64, max_price_cents: i64) -> Result<(), String> {
        let existing = sqlx::query_as!(
            YieldProfile,
            "SELECT * FROM yield_profiles WHERE tenant_id = $1 AND target_id = $2",
            tenant_id,
            target_id
        )
        .fetch_optional(&self.db.pool)
        .await
        .map_err(|e| e.to_string())?;

        if let Some(profile) = existing {
            sqlx::query!(
                "UPDATE yield_profiles SET enabled = $1, min_price_cents = $2, max_price_cents = $3, updated_at = NOW() WHERE id = $4",
                enabled, min_price_cents, max_price_cents, profile.id
            )
            .execute(&self.db.pool)
            .await
            .map_err(|e| e.to_string())?;
        } else {
            let id = Uuid::new_v4().to_string();
            sqlx::query!(
                "INSERT INTO yield_profiles (id, tenant_id, target_id, target_type, enabled, min_price_cents, max_price_cents) VALUES ($1, $2, $3, $4, $5, $6, $7)",
                id, tenant_id, target_id, target_type, enabled, min_price_cents, max_price_cents
            )
            .execute(&self.db.pool)
            .await
            .map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}
