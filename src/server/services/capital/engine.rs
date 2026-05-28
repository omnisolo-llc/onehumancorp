use tonic::Status;
use sqlx::{PgPool, Row};
use chrono::{Utc, Duration};
use uuid::Uuid;
use ::server_common::auth_utils::set_org_context;
use ::server_ohc::capital::*;

/// Capital Engine - Generates and manages capital offers
pub struct CapitalEngine {
    pool: PgPool,
}

impl CapitalEngine {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Calculate available capital based on business performance
    pub async fn calculate_available_capital(&self, tenant_id: &str, business_id: &str) -> Result<f64, Status> {
        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        set_org_context(&mut *tx, tenant_id).await.map_err(|e| Status::internal(e.to_string()))?;

        // Get last 90 days of sales
        let sales_row = sqlx::query(
            "SELECT COALESCE(SUM(amount), 0) as total_sales, COUNT(*) as transaction_count
             FROM sales_transactions 
             WHERE tenant_id = $1 AND business_id = $2 
             AND transaction_type = 'sale'
             AND transaction_date >= NOW() - INTERVAL '90 days'"
        )
        .bind(tenant_id)
        .bind(business_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        let total_sales: f64 = sales_row.try_get("total_sales").unwrap_or(0.0);
        let transaction_count: i64 = sales_row.try_get("transaction_count").unwrap_or(0);

        // Calculate available capital as 10-20% of 90-day sales
        // More transactions = higher confidence = higher percentage
        let percentage = if transaction_count > 50 {
            0.20
        } else if transaction_count > 20 {
            0.15
        } else if transaction_count > 10 {
            0.10
        } else {
            0.05
        };

        let available = total_sales * percentage;
        
        // Cap at $50,000 for safety
        Ok(available.min(50000.0))
    }

    /// Check if business is eligible for capital
    async fn check_eligibility(&self, tenant_id: &str, business_id: &str) -> Result<(bool, String), Status> {
        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        set_org_context(&mut *tx, tenant_id).await.map_err(|e| Status::internal(e.to_string()))?;

        // Check for sales history (at least 30 days)
        let sales_row = sqlx::query(
            "SELECT COUNT(*) as count, MIN(transaction_date) as first_sale
             FROM sales_transactions 
             WHERE tenant_id = $1 AND business_id = $2 AND transaction_type = 'sale'"
        )
        .bind(tenant_id)
        .bind(business_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let count: i64 = sales_row.try_get("count").unwrap_or(0);
        
        if count < 5 {
            tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;
            return Ok((false, "Need at least 5 sales transactions to qualify for capital".to_string()));
        }

        // Check for active advances with high outstanding balance
        let advance_row = sqlx::query(
            "SELECT COALESCE(SUM(total_owed - amount_repaid), 0) as outstanding
             FROM capital_advances 
             WHERE tenant_id = $1 AND business_id = $2 AND status = 'active'"
        )
        .bind(tenant_id)
        .bind(business_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        let outstanding: f64 = advance_row.try_get("outstanding").unwrap_or(0.0);
        
        if outstanding > 10000.0 {
            return Ok((false, "Outstanding balance too high. Pay down existing advances first.".to_string()));
        }

        Ok((true, String::new()))
    }

    /// Get capital offers for a business
    pub async fn get_capital_offers(&self, tenant_id: &str, business_id: &str) -> Result<CapitalOffersResponse, Status> {
        let (eligible, reason) = self.check_eligibility(tenant_id, business_id).await?;

        if !eligible {
            return Ok(CapitalOffersResponse {
                offers: vec![],
                eligible_for_capital: false,
                ineligibility_reason: reason,
            });
        }

        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        set_org_context(&mut *tx, tenant_id).await.map_err(|e| Status::internal(e.to_string()))?;

        // Check for existing pending offers
        let existing_offers = sqlx::query(
            "SELECT id, tenant_id, business_id, offer_amount, flat_fee_amount, flat_fee_percentage,
                    repayment_percentage, estimated_repayment_days, total_repayment_amount,
                    status, reason, 
                    EXTRACT(EPOCH FROM expires_at)::bigint as expires_at_unix,
                    EXTRACT(EPOCH FROM accepted_at)::bigint as accepted_at_unix,
                    EXTRACT(EPOCH FROM created_at)::bigint as created_at_unix
             FROM capital_offers 
             WHERE tenant_id = $1 AND business_id = $2 
             AND status = 'pending' 
             AND expires_at > NOW()
             ORDER BY created_at DESC"
        )
        .bind(tenant_id)
        .bind(business_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        if !existing_offers.is_empty() {
            let mut offers = Vec::new();
            for row in existing_offers {
                offers.push(CapitalOffer {
                    id: row.get("id"),
                    tenant_id: row.get("tenant_id"),
                    business_id: row.get("business_id"),
                    offer_amount: row.try_get("offer_amount").unwrap_or(0.0),
                    flat_fee_amount: row.try_get("flat_fee_amount").unwrap_or(0.0),
                    flat_fee_percentage: row.try_get("flat_fee_percentage").unwrap_or(0.0),
                    repayment_percentage: row.try_get("repayment_percentage").unwrap_or(0.0),
                    estimated_repayment_days: row.try_get("estimated_repayment_days").unwrap_or(0),
                    total_repayment_amount: row.try_get("total_repayment_amount").unwrap_or(0.0),
                    status: row.get("status"),
                    reason: row.try_get("reason").unwrap_or_default(),
                    expires_at_unix: row.try_get("expires_at_unix").unwrap_or(0),
                    accepted_at_unix: row.try_get("accepted_at_unix").unwrap_or(0),
                    created_at_unix: row.try_get("created_at_unix").unwrap_or(0),
                });
            }
            tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;
            return Ok(CapitalOffersResponse {
                offers,
                eligible_for_capital: true,
                ineligibility_reason: String::new(),
            });
        }

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        // Generate new offers
        let available_capital = self.calculate_available_capital(tenant_id, business_id).await?;

        if available_capital < 100.0 {
            return Ok(CapitalOffersResponse {
                offers: vec![],
                eligible_for_capital: false,
                ineligibility_reason: "Insufficient sales history to generate capital offers".to_string(),
            });
        }

        // Generate 3 offer tiers
        let offers = self.generate_offer_tiers(tenant_id, business_id, available_capital).await?;

        Ok(CapitalOffersResponse {
            offers,
            eligible_for_capital: true,
            ineligibility_reason: String::new(),
        })
    }

    /// Generate tiered capital offers
    async fn generate_offer_tiers(&self, tenant_id: &str, business_id: &str, max_amount: f64) -> Result<Vec<CapitalOffer>, Status> {
        let mut offers = Vec::new();
        let now = Utc::now();
        let expires_at = now + Duration::hours(72); // 72 hour expiry

        // Tier 1: Small (25% of max) - 8% fee, 10% repayment
        let tier1_amount = (max_amount * 0.25).round();
        if tier1_amount >= 100.0 {
            offers.push(self.create_offer(
                tenant_id,
                business_id,
                tier1_amount,
                0.08,
                0.10,
                60,
                expires_at,
                "Quick access to working capital",
            ).await?);
        }

        // Tier 2: Medium (50% of max) - 10% fee, 12% repayment
        let tier2_amount = (max_amount * 0.50).round();
        if tier2_amount >= 100.0 {
            offers.push(self.create_offer(
                tenant_id,
                business_id,
                tier2_amount,
                0.10,
                0.12,
                90,
                expires_at,
                "Balanced capital for growth",
            ).await?);
        }

        // Tier 3: Large (100% of max) - 12% fee, 15% repayment
        let tier3_amount = max_amount.round();
        if tier3_amount >= 100.0 {
            offers.push(self.create_offer(
                tenant_id,
                business_id,
                tier3_amount,
                0.12,
                0.15,
                120,
                expires_at,
                "Maximum capital for major investments",
            ).await?);
        }

        Ok(offers)
    }

    /// Create a single capital offer
    async fn create_offer(
        &self,
        tenant_id: &str,
        business_id: &str,
        amount: f64,
        flat_fee_pct: f64,
        repayment_pct: f64,
        estimated_days: i32,
        expires_at: chrono::DateTime<Utc>,
        reason: &str,
    ) -> Result<CapitalOffer, Status> {
        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        set_org_context(&mut *tx, tenant_id).await.map_err(|e| Status::internal(e.to_string()))?;

        let offer_id = Uuid::new_v4().to_string();
        let flat_fee_amount = amount * flat_fee_pct;
        let total_repayment = amount + flat_fee_amount;
        let now = Utc::now();

        sqlx::query(
            "INSERT INTO capital_offers 
             (id, tenant_id, business_id, offer_amount, flat_fee_amount, flat_fee_percentage,
              repayment_percentage, estimated_repayment_days, total_repayment_amount,
              status, reason, expires_at, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)"
        )
        .bind(&offer_id)
        .bind(tenant_id)
        .bind(business_id)
        .bind(amount)
        .bind(flat_fee_amount)
        .bind(flat_fee_pct)
        .bind(repayment_pct)
        .bind(estimated_days)
        .bind(total_repayment)
        .bind("pending")
        .bind(reason)
        .bind(expires_at)
        .bind(now)
        .bind(now)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(CapitalOffer {
            id: offer_id,
            tenant_id: tenant_id.to_string(),
            business_id: business_id.to_string(),
            offer_amount: amount,
            flat_fee_amount,
            flat_fee_percentage: flat_fee_pct,
            repayment_percentage: repayment_pct,
            estimated_repayment_days: estimated_days,
            total_repayment_amount: total_repayment,
            status: "pending".to_string(),
            reason: reason.to_string(),
            expires_at_unix: expires_at.timestamp(),
            accepted_at_unix: 0,
            created_at_unix: now.timestamp(),
        })
    }

    /// Accept a capital offer and create an advance
    pub async fn accept_capital_offer(&self, tenant_id: &str, offer_id: &str) -> Result<AcceptOfferResponse, Status> {
        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        set_org_context(&mut *tx, tenant_id).await.map_err(|e| Status::internal(e.to_string()))?;

        // Get the offer
        let offer_row = sqlx::query(
            "SELECT id, tenant_id, business_id, offer_amount, flat_fee_amount, flat_fee_percentage,
                    repayment_percentage, total_repayment_amount, status, expires_at
             FROM capital_offers 
             WHERE id = $1 AND tenant_id = $2"
        )
        .bind(offer_id)
        .bind(tenant_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let offer_row = match offer_row {
            Some(row) => row,
            None => {
                tx.rollback().await.map_err(|e| Status::internal(e.to_string()))?;
                return Ok(AcceptOfferResponse {
                    advance: None,
                    success: false,
                    message: "Offer not found".to_string(),
                });
            }
        };

        let status: String = offer_row.get("status");
        if status != "pending" {
            tx.rollback().await.map_err(|e| Status::internal(e.to_string()))?;
            return Ok(AcceptOfferResponse {
                advance: None,
                success: false,
                message: "Offer is no longer available".to_string(),
            });
        }

        let expires_at: chrono::DateTime<Utc> = offer_row.get("expires_at");
        if expires_at < Utc::now() {
            tx.rollback().await.map_err(|e| Status::internal(e.to_string()))?;
            return Ok(AcceptOfferResponse {
                advance: None,
                success: false,
                message: "Offer has expired".to_string(),
            });
        }

        // Update offer status
        let now = Utc::now();
        sqlx::query(
            "UPDATE capital_offers 
             SET status = 'accepted', accepted_at = $1, updated_at = $2
             WHERE id = $3"
        )
        .bind(now)
        .bind(now)
        .bind(offer_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        // Create advance
        let advance_id = Uuid::new_v4().to_string();
        let business_id: String = offer_row.get("business_id");
        let principal: f64 = offer_row.try_get("offer_amount").unwrap_or(0.0);
        let flat_fee: f64 = offer_row.try_get("flat_fee_amount").unwrap_or(0.0);
        let total_owed: f64 = offer_row.try_get("total_repayment_amount").unwrap_or(0.0);
        let repayment_pct: f64 = offer_row.try_get("repayment_percentage").unwrap_or(0.0);

        sqlx::query(
            "INSERT INTO capital_advances 
             (id, tenant_id, business_id, offer_id, principal_amount, flat_fee_amount,
              total_owed, amount_repaid, repayment_percentage, status, disbursed_at, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)"
        )
        .bind(&advance_id)
        .bind(tenant_id)
        .bind(&business_id)
        .bind(offer_id)
        .bind(principal)
        .bind(flat_fee)
        .bind(total_owed)
        .bind(0.0)
        .bind(repayment_pct)
        .bind("active")
        .bind(now)
        .bind(now)
        .bind(now)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        let advance = CapitalAdvance {
            id: advance_id,
            tenant_id: tenant_id.to_string(),
            business_id,
            offer_id: offer_id.to_string(),
            principal_amount: principal,
            flat_fee_amount: flat_fee,
            total_owed,
            amount_repaid: 0.0,
            repayment_percentage: repayment_pct,
            status: "active".to_string(),
            disbursed_at_unix: now.timestamp(),
            repaid_at_unix: 0,
            created_at_unix: now.timestamp(),
        };

        Ok(AcceptOfferResponse {
            advance: Some(advance),
            success: true,
            message: format!("Capital advance of ${:.2} approved and disbursed", principal),
        })
    }

    /// Process automatic repayment from a sale
    pub async fn process_automatic_repayment(&self, tenant_id: &str, business_id: &str, sale_amount: f64) -> Result<(), Status> {
        let mut tx = self.pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        set_org_context(&mut *tx, tenant_id).await.map_err(|e| Status::internal(e.to_string()))?;

        // Get active advances
        let advances = sqlx::query(
            "SELECT id, repayment_percentage, total_owed, amount_repaid
             FROM capital_advances 
             WHERE tenant_id = $1 AND business_id = $2 AND status = 'active'
             ORDER BY created_at ASC"
        )
        .bind(tenant_id)
        .bind(business_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        for advance in advances {
            let advance_id: String = advance.get("id");
            let repayment_pct: f64 = advance.try_get("repayment_percentage").unwrap_or(0.0);
            let total_owed: f64 = advance.try_get("total_owed").unwrap_or(0.0);
            let amount_repaid: f64 = advance.try_get("amount_repaid").unwrap_or(0.0);
            
            let remaining = total_owed - amount_repaid;
            if remaining <= 0.0 {
                continue;
            }

            let repayment_amount = (sale_amount * repayment_pct).min(remaining);
            let new_amount_repaid = amount_repaid + repayment_amount;

            // Update advance
            let now = Utc::now();
            let new_status = if new_amount_repaid >= total_owed {
                "repaid"
            } else {
                "active"
            };

            sqlx::query(
                "UPDATE capital_advances 
                 SET amount_repaid = $1, status = $2, repaid_at = $3, updated_at = $4
                 WHERE id = $5"
            )
            .bind(new_amount_repaid)
            .bind(new_status)
            .bind(if new_status == "repaid" { Some(now) } else { None })
            .bind(now)
            .bind(&advance_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

            // Record repayment
            let repayment_id = Uuid::new_v4().to_string();
            sqlx::query(
                "INSERT INTO capital_repayments 
                 (id, tenant_id, advance_id, repayment_amount, transaction_amount, repayment_date, created_at)
                 VALUES ($1, $2, $3, $4, $5, $6, $7)"
            )
            .bind(&repayment_id)
            .bind(tenant_id)
            .bind(&advance_id)
            .bind(repayment_amount)
            .bind(sale_amount)
            .bind(now)
            .bind(now)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;
        }

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(())
    }
}
