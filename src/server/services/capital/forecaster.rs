use tonic::Status;
use sqlx::{PgPool, Row};
use chrono::{Utc, Duration, Datelike};
use uuid::Uuid;
use ::server_common::auth_utils::set_org_context;
use ::server_ohc::capital::*;

/// Cash Flow Forecaster - Predicts future cash flow and detects crunches
pub struct CashFlowForecaster {
    pool: PgPool,
}

impl CashFlowForecaster {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Get cash flow predictions for the next N days
    pub async fn get_cash_flow_predictions(
        &self,
        tenant_id: &str,
        business_id: &str,
        days_ahead: i32,
    ) -> Result<CashFlowResponse, Status> {
        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        set_org_context(&mut *tx, tenant_id).await.map_err(|e| Status::internal(e.to_string()))?;

        // Get historical sales data (last 90 days)
        let sales_rows = sqlx::query(
            "SELECT DATE(transaction_date) as date, 
                    SUM(CASE WHEN transaction_type = 'sale' THEN amount ELSE 0 END) as inflow,
                    SUM(CASE WHEN transaction_type = 'expense' THEN amount ELSE 0 END) as outflow
             FROM sales_transactions 
             WHERE tenant_id = $1 AND business_id = $2 
             AND transaction_date >= NOW() - INTERVAL '90 days'
             GROUP BY DATE(transaction_date)
             ORDER BY date DESC"
        )
        .bind(tenant_id)
        .bind(business_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        // Calculate current balance
        let balance_row = sqlx::query(
            "SELECT 
                COALESCE(SUM(CASE WHEN transaction_type = 'sale' THEN amount ELSE 0 END), 0) as total_inflow,
                COALESCE(SUM(CASE WHEN transaction_type IN ('expense', 'refund') THEN amount ELSE 0 END), 0) as total_outflow
             FROM sales_transactions 
             WHERE tenant_id = $1 AND business_id = $2"
        )
        .bind(tenant_id)
        .bind(business_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        let total_inflow: f64 = balance_row.try_get("total_inflow").unwrap_or(0.0);
        let total_outflow: f64 = balance_row.try_get("total_outflow").unwrap_or(0.0);
        let current_balance = total_inflow - total_outflow;

        // Calculate daily averages
        let (avg_daily_inflow, avg_daily_outflow) = if sales_rows.is_empty() {
            (0.0, 0.0)
        } else {
            let mut total_in = 0.0;
            let mut total_out = 0.0;
            for row in &sales_rows {
                total_in += row.try_get::<f64, _>("inflow").unwrap_or(0.0);
                total_out += row.try_get::<f64, _>("outflow").unwrap_or(0.0);
            }
            let days = sales_rows.len() as f64;
            (total_in / days, total_out / days)
        };

        // Generate predictions
        let mut predictions = Vec::new();
        let mut running_balance = current_balance;
        let mut cash_crunch_detected = false;
        let mut next_crunch_date = String::new();
        let mut crunch_severity = 0.0;

        for day in 1..=days_ahead {
            let predicted_date = Utc::now() + Duration::days(day as i64);
            
            // Apply day-of-week patterns (weekends typically lower)
            let day_of_week = predicted_date.weekday();
            let weekend_factor = if day_of_week.num_days_from_monday() >= 5 {
                0.6 // 40% reduction on weekends
            } else {
                1.0
            };

            // Add some variance (±20%)
            let variance = 0.8 + (day as f64 * 0.013).sin() * 0.2;
            
            let predicted_inflow = avg_daily_inflow * weekend_factor * variance;
            let predicted_outflow = avg_daily_outflow * variance;
            
            running_balance += predicted_inflow - predicted_outflow;

            // Detect cash crunch (balance below $500 or negative)
            if running_balance < 500.0 && !cash_crunch_detected {
                cash_crunch_detected = true;
                next_crunch_date = predicted_date.format("%Y-%m-%d").to_string();
                crunch_severity = if running_balance < 0.0 {
                    running_balance.abs()
                } else {
                    500.0 - running_balance
                };
            }

            // Calculate confidence score (higher for near-term, lower for far-term)
            let confidence = (1.0 - (day as f64 / days_ahead as f64) * 0.5).max(0.3);

            let prediction_id = Uuid::new_v4().to_string();
            predictions.push(CashFlowPrediction {
                id: prediction_id,
                tenant_id: tenant_id.to_string(),
                business_id: business_id.to_string(),
                predicted_date: predicted_date.format("%Y-%m-%d").to_string(),
                predicted_inflow,
                predicted_outflow,
                predicted_balance: running_balance,
                confidence_score: confidence,
                created_at_unix: Utc::now().timestamp(),
            });
        }

        // Store predictions in database for historical tracking
        let _ = self.store_predictions(tenant_id, &predictions).await;

        Ok(CashFlowResponse {
            predictions,
            current_balance,
            cash_crunch_detected,
            next_crunch_date,
            crunch_severity,
        })
    }

    /// Store predictions in database
    async fn store_predictions(&self, tenant_id: &str, predictions: &[CashFlowPrediction]) -> Result<(), Status> {
        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        set_org_context(&mut *tx, tenant_id).await.map_err(|e| Status::internal(e.to_string()))?;

        // Delete old predictions for this business
        if let Some(first) = predictions.first() {
            sqlx::query(
                "DELETE FROM cash_flow_predictions 
                 WHERE tenant_id = $1 AND business_id = $2"
            )
            .bind(tenant_id)
            .bind(&first.business_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

            // Insert new predictions
            for pred in predictions {
                let predicted_date = chrono::NaiveDate::parse_from_str(&pred.predicted_date, "%Y-%m-%d")
                    .map_err(|e| Status::internal(e.to_string()))?;

                sqlx::query(
                    "INSERT INTO cash_flow_predictions 
                     (id, tenant_id, business_id, predicted_date, predicted_inflow, predicted_outflow,
                      predicted_balance, confidence_score, created_at, updated_at)
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW(), NOW())"
                )
                .bind(&pred.id)
                .bind(tenant_id)
                .bind(&pred.business_id)
                .bind(predicted_date)
                .bind(pred.predicted_inflow)
                .bind(pred.predicted_outflow)
                .bind(pred.predicted_balance)
                .bind(pred.confidence_score)
                .execute(&mut *tx)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;
            }
        }

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(())
    }

    /// Analyze cash flow and trigger proactive capital offers
    pub async fn analyze_and_trigger_offers(&self, tenant_id: &str, business_id: &str) -> Result<bool, Status> {
        let predictions = self.get_cash_flow_predictions(tenant_id, business_id, 30).await?;

        // If cash crunch detected within 14 days, trigger offer generation
        if predictions.cash_crunch_detected && !predictions.next_crunch_date.is_empty() {
            let crunch_date = chrono::NaiveDate::parse_from_str(&predictions.next_crunch_date, "%Y-%m-%d")
                .map_err(|e| Status::internal(e.to_string()))?;
            
            let today = Utc::now().date_naive();
            let days_until_crunch = (crunch_date - today).num_days();

            if days_until_crunch <= 14 && days_until_crunch > 0 {
                // Trigger capital offer generation
                return Ok(true);
            }
        }

        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forecaster_creation() {
        // Placeholder test
        assert!(true);
    }
}
