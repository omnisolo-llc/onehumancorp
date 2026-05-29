use sqlx::{PgPool, Row, Executor};
use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct CapitalAdvance {
    pub id: String,
    pub tenant_id: String,
    pub amount: f64,
    pub fee: f64,
    pub repayment_percentage: f64,
    pub status: String,
}

pub struct CapitalEngine {
    pool: PgPool,
}

impl CapitalEngine {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_advance(&self, tenant_id: &str, amount: f64, fee: f64, repayment_percentage: f64) -> Result<CapitalAdvance, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let advance = CapitalAdvance {
            id: id.clone(),
            tenant_id: tenant_id.to_string(),
            amount,
            fee,
            repayment_percentage,
            status: "ACTIVE".to_string(),
        };

        let mut tx = self.pool.begin().await?;
        tx.execute(format!("SET app.current_tenant = '{}'", tenant_id).as_str()).await?;

        sqlx::query(
            "INSERT INTO capital_advances (id, tenant_id, amount, fee, repayment_percentage, status)
             VALUES ($1, $2, $3, $4, $5, $6)"
        )
        .bind(&advance.id)
        .bind(&advance.tenant_id)
        .bind(advance.amount)
        .bind(advance.fee)
        .bind(advance.repayment_percentage)
        .bind(&advance.status)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(advance)
    }

    pub async fn process_repayment(&self, tenant_id: &str, transaction_amount: f64) -> Result<f64, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        tx.execute(format!("SET app.current_tenant = '{}'", tenant_id).as_str()).await?;

        let active_advances = sqlx::query(
            "SELECT id, amount, fee, repayment_percentage FROM capital_advances WHERE tenant_id = $1 AND status = 'ACTIVE'"
        )
        .bind(tenant_id)
        .fetch_all(&mut *tx)
        .await?;

        let mut total_repaid = 0.0;

        for row in active_advances {
            let advance_id: String = row.get("id");
            let advance_amount: f64 = row.get("amount");
            let fee: f64 = row.get("fee");
            let repayment_percentage: f64 = row.get("repayment_percentage");

            let target_repayment = advance_amount + fee;

            let past_repayments: f64 = sqlx::query(
                "SELECT COALESCE(SUM(amount), 0.0) FROM repayment_events WHERE advance_id = $1"
            )
            .bind(&advance_id)
            .fetch_one(&mut *tx)
            .await?
            .get(0);

            if past_repayments < target_repayment {
                let mut deduction = transaction_amount * repayment_percentage;
                if past_repayments + deduction > target_repayment {
                    deduction = target_repayment - past_repayments;
                }

                if deduction > 0.0 {
                    let repayment_id = Uuid::new_v4().to_string();
                    sqlx::query(
                        "INSERT INTO repayment_events (id, tenant_id, advance_id, amount) VALUES ($1, $2, $3, $4)"
                    )
                    .bind(repayment_id)
                    .bind(tenant_id)
                    .bind(&advance_id)
                    .bind(deduction)
                    .execute(&mut *tx)
                    .await?;

                    total_repaid += deduction;

                    if past_repayments + deduction >= target_repayment {
                        sqlx::query("UPDATE capital_advances SET status = 'REPAID' WHERE id = $1")
                            .bind(&advance_id)
                            .execute(&mut *tx)
                            .await?;
                    }
                }
            }
        }

        tx.commit().await?;

        Ok(total_repaid)
    }

    pub async fn analyze_capital_needs(&self, tenant_id: &str) -> Result<Option<CapitalAdvance>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        tx.execute(format!("SET app.current_tenant = '{}'", tenant_id).as_str()).await?;

        // Simplified predictive logic: if total recent bookings > $1000 and low capital advances, offer advance
        let total_bookings: f64 = sqlx::query(
            "SELECT COALESCE(SUM(total_amount), 0.0) FROM bookings WHERE tenant_id = $1 AND created_at > CURRENT_TIMESTAMP - INTERVAL '7 days'"
        )
        .bind(tenant_id)
        .fetch_one(&mut *tx)
        .await?
        .get(0);

        let active_count: i64 = sqlx::query(
            "SELECT COUNT(*) FROM capital_advances WHERE tenant_id = $1 AND status = 'ACTIVE'"
        )
        .bind(tenant_id)
        .fetch_one(&mut *tx)
        .await?
        .get(0);

        tx.commit().await?;

        if total_bookings > 1000.0 && active_count == 0 {
            // Offer a 30% advance on recent bookings
            let advance_amount = total_bookings * 0.3;
            // 10% fee
            let fee = advance_amount * 0.1;

            let id = Uuid::new_v4().to_string();
            let advance = CapitalAdvance {
                id,
                tenant_id: tenant_id.to_string(),
                amount: advance_amount,
                fee,
                repayment_percentage: 0.1, // 10% repayment deduction
                status: "OFFERED".to_string(),
            };
            return Ok(Some(advance));
        }

        Ok(None)
    }
}
