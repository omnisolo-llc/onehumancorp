use sqlx::{PgPool, Row};
use uuid::Uuid;
use tracing::info;

#[derive(Debug, Clone)]
pub struct LoyaltyEngine {
    pool: PgPool,
}

impl LoyaltyEngine {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn award_points(&self, tenant_id: &str, customer_id: &str, phone_number: &str, order_id: &str, order_amount: i32) -> Result<i32, String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        // 1. Check if loyalty is enabled for tenant
        let settings_query = r#"
            SELECT is_enabled, points_per_currency_unit
            FROM loyalty_settings
            WHERE tenant_id = $1
        "#;

        let row = sqlx::query(settings_query)
            .bind(tenant_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        if row.is_none() {
            return Ok(0); // Not enabled or no settings
        }

        let row = row.unwrap();
        let is_enabled: bool = row.get("is_enabled");
        if !is_enabled {
            return Ok(0);
        }

        let points_per_unit: i32 = row.get("points_per_currency_unit");

        // Calculate points (assuming order_amount is in smallest currency unit, e.g., cents)
        // For simplicity, let's say 1 currency unit = 100 cents
        let points_to_award = (order_amount / 100) * points_per_unit;

        if points_to_award <= 0 {
            return Ok(0);
        }

        // 2. Upsert ledger
        let ledger_id = Uuid::new_v4().to_string();
        let upsert_query = r#"
            INSERT INTO loyalty_ledgers (id, tenant_id, customer_id, phone_number, points_balance, lifetime_points_earned)
            VALUES ($1, $2, $3, $4, $5, $5)
            ON CONFLICT (tenant_id, customer_id) DO UPDATE SET
                points_balance = loyalty_ledgers.points_balance + EXCLUDED.points_balance,
                lifetime_points_earned = loyalty_ledgers.lifetime_points_earned + EXCLUDED.lifetime_points_earned,
                updated_at = CURRENT_TIMESTAMP
            RETURNING points_balance, lifetime_points_earned
        "#;

        let _ledger_row = sqlx::query(upsert_query)
            .bind(&ledger_id)
            .bind(tenant_id)
            .bind(customer_id)
            .bind(phone_number)
            .bind(points_to_award)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        // 3. Record transaction
        let tx_id = Uuid::new_v4().to_string();
        let record_tx_query = r#"
            INSERT INTO loyalty_transactions (id, tenant_id, customer_id, order_id, points_change, transaction_type, description)
            VALUES ($1, $2, $3, $4, $5, 'earn', 'Points earned from order')
        "#;

        sqlx::query(record_tx_query)
            .bind(&tx_id)
            .bind(tenant_id)
            .bind(customer_id)
            .bind(order_id)
            .bind(points_to_award)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;

        info!("Awarded {} points to customer {} for order {}", points_to_award, customer_id, order_id);

        // TODO: Integrate Customer Success Agent logic here asynchronously
        // to send SMS if milestones are reached or if high LTV.

        Ok(points_to_award)
    }

    pub async fn calculate_discount(&self, tenant_id: &str, customer_id: &str) -> Result<i32, String> {
         let query = r#"
            SELECT l.points_balance, s.currency_value_per_point, s.minimum_redemption_points
            FROM loyalty_ledgers l
            JOIN loyalty_settings s ON l.tenant_id = s.tenant_id
            WHERE l.tenant_id = $1 AND l.customer_id = $2 AND s.is_enabled = true
        "#;

        let row = sqlx::query(query)
            .bind(tenant_id)
            .bind(customer_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(r) = row {
            let balance: i32 = r.get("points_balance");
            let min_points: i32 = r.get("minimum_redemption_points");
            let value_per_point: String = r.get("currency_value_per_point");

            if balance >= min_points {
                                // Convert BigDecimal to f64 for simple math, then to integer cents
                let v = value_per_point.to_string().parse::<f64>().unwrap_or(0.01);
                let discount_cents = (balance as f64 * v * 100.0) as i32;
                return Ok(discount_cents);
            }
        }
        Ok(0)
    }

    pub async fn redeem_points(&self, tenant_id: &str, customer_id: &str, points_to_redeem: i32, order_id: &str) -> Result<(), String> {
        let mut tx = self.pool.begin().await.map_err(|e| e.to_string())?;

        // 1. Deduct from ledger
        let deduct_query = r#"
            UPDATE loyalty_ledgers
            SET points_balance = points_balance - $1, updated_at = CURRENT_TIMESTAMP
            WHERE tenant_id = $2 AND customer_id = $3 AND points_balance >= $1
            RETURNING id
        "#;

        let row = sqlx::query(deduct_query)
            .bind(points_to_redeem)
            .bind(tenant_id)
            .bind(customer_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        if row.is_none() {
            return Err("Insufficient points or ledger not found".to_string());
        }

        // 2. Record transaction
        let tx_id = Uuid::new_v4().to_string();
        let record_tx_query = r#"
            INSERT INTO loyalty_transactions (id, tenant_id, customer_id, order_id, points_change, transaction_type, description)
            VALUES ($1, $2, $3, $4, $5, 'redeem', 'Points redeemed on order')
        "#;

        sqlx::query(record_tx_query)
            .bind(&tx_id)
            .bind(tenant_id)
            .bind(customer_id)
            .bind(order_id)
            .bind(-points_to_redeem)
            .execute(&mut *tx)
            .await
            .map_err(|e| e.to_string())?;

        tx.commit().await.map_err(|e| e.to_string())?;
        info!("Redeemed {} points for customer {} on order {}", points_to_redeem, customer_id, order_id);
        Ok(())
    }
}
