use sqlx::{PgPool, Row};
use uuid::Uuid;
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CashFlowProjection {
    pub tenant_id: Uuid,
    pub date: DateTime<Utc>,
    pub projected_balance_cents: i64,
    pub safe_to_spend_cents: i64,
    pub reserved_tax_cents: i64,
    pub upcoming_bills_cents: i64,
}

pub struct ForecastingEngine {
    pool: PgPool,
}

impl ForecastingEngine {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Calculates a 90-day cash flow projection for a tenant.
    pub async fn calculate_projection(
        &self,
        tenant_id: Uuid,
        current_balance_cents: i64,
        tax_rate: f64, // E.g., 0.25 for 25%
    ) -> Result<CashFlowProjection, sqlx::Error> {
        let now = Utc::now();
        let target_date = now + Duration::days(90);

        // Fetch pending receivables (unpaid invoices)
        let row_rec = sqlx::query(
            r#"
            SELECT COALESCE(SUM(amount_cents), 0) as total
            FROM invoices
            WHERE tenant_id = $1 AND status = 'PENDING' AND due_date <= $2
            "#
        )
        .bind(tenant_id)
        .bind(target_date)
        .fetch_one(&self.pool)
        .await?;
        let receivables: i64 = row_rec.try_get("total").unwrap_or(0);

        // Fetch pending payables (approved POs, upcoming payroll)
        let row_pay = sqlx::query(
            r#"
            SELECT COALESCE(SUM(amount_cents), 0) as total
            FROM purchase_orders
            WHERE tenant_id = $1 AND status = 'APPROVED' AND expected_delivery_date <= $2
            "#
        )
        .bind(tenant_id)
        .bind(target_date)
        .fetch_one(&self.pool)
        .await?;
        let payables: i64 = row_pay.try_get("total").unwrap_or(0);

        // Project future balance
        let projected_balance_cents = current_balance_cents + receivables - payables;

        // Estimate tax liability on current + projected income (simplified for this example)
        let estimated_taxable_income = current_balance_cents + receivables;
        let reserved_tax_cents = if estimated_taxable_income > 0 {
            (estimated_taxable_income as f64 * tax_rate) as i64
        } else {
            0
        };

        // Safe to spend: current balance minus payables minus tax reserve
        let safe_to_spend_cents = current_balance_cents - payables - reserved_tax_cents;

        // In a real app, this would be persisted to a `cash_flow_projections` table.
        // For now, we just return the calculated struct.

        Ok(CashFlowProjection {
            tenant_id,
            date: target_date,
            projected_balance_cents,
            safe_to_spend_cents,
            reserved_tax_cents,
            upcoming_bills_cents: payables,
        })
    }
}
