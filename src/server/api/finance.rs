use std::sync::Arc;
use tonic::{Request, Response, Status};

use finance_proto::ohc::finance::event_router_service_server::EventRouterService;
use finance_proto::ohc::finance::tax_nexus_service_server::TaxNexusService;
use finance_proto::ohc::finance::finance_agent_service_server::FinanceAgentService;

use finance_proto::ohc::finance::{
    ProcessEventRequest, ProcessEventResponse, CalculateTaxRequest, CalculateTaxResponse,
    GetBalancesRequest, GetBalancesResponse, GenerateInsightRequest, GenerateInsightResponse,
    WalletBalance,
};

pub struct EventRouter {
    pub db: Arc<sqlx::PgPool>,
}

#[tonic::async_trait]
impl EventRouterService for EventRouter {
    async fn process_event(&self, request: Request<ProcessEventRequest>) -> Result<Response<ProcessEventResponse>, Status> {
        let req = request.into_inner();
        let event = req.event.ok_or_else(|| Status::invalid_argument("missing event"))?;

        // 1. Calculate Tax (mocked logic or separate service call)
        let tax_amount = (event.amount_cents as f64 * 0.1) as i64; // 10% tax for example
        let main_amount = event.amount_cents - tax_amount;

        // 2. Insert into Ledger using a transaction
        let mut tx = self.db.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        // Write to Main Balance ledger
        sqlx::query!(
            r#"
            INSERT INTO ohc_double_entry_ledger (id, tenant_id, transaction_id, account_type, amount, currency, description)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            uuid::Uuid::new_v4().to_string(),
            event.tenant_id,
            event.source_id,
            "MAIN_BALANCE",
            main_amount as f64 / 100.0,
            event.currency,
            event.description
        ).execute(&mut *tx).await.map_err(|e| Status::internal(e.to_string()))?;

        // Write to Tax Vault ledger
        sqlx::query!(
            r#"
            INSERT INTO ohc_double_entry_ledger (id, tenant_id, transaction_id, account_type, amount, currency, description)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            uuid::Uuid::new_v4().to_string(),
            event.tenant_id,
            event.source_id,
            "TAX_VAULT",
            tax_amount as f64 / 100.0,
            event.currency,
            "Tax set aside"
        ).execute(&mut *tx).await.map_err(|e| Status::internal(e.to_string()))?;

        // 3. Update Virtual Wallets (Upsert)
        sqlx::query!(
            r#"
            INSERT INTO ohc_virtual_wallets (id, tenant_id, wallet_type, balance, currency)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (tenant_id, wallet_type) DO UPDATE SET balance = ohc_virtual_wallets.balance + EXCLUDED.balance, updated_at = CURRENT_TIMESTAMP
            "#,
            uuid::Uuid::new_v4().to_string(),
            event.tenant_id,
            "MAIN_BALANCE",
            main_amount as f64 / 100.0,
            event.currency
        ).execute(&mut *tx).await.map_err(|e| Status::internal(e.to_string()))?;

        sqlx::query!(
            r#"
            INSERT INTO ohc_virtual_wallets (id, tenant_id, wallet_type, balance, currency)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (tenant_id, wallet_type) DO UPDATE SET balance = ohc_virtual_wallets.balance + EXCLUDED.balance, updated_at = CURRENT_TIMESTAMP
            "#,
            uuid::Uuid::new_v4().to_string(),
            event.tenant_id,
            "TAX_VAULT",
            tax_amount as f64 / 100.0,
            event.currency
        ).execute(&mut *tx).await.map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(ProcessEventResponse {
            success: true,
            transaction_id: event.source_id,
            tax_amount_cents: tax_amount,
        }))
    }
}

pub struct TaxNexus {}

#[tonic::async_trait]
impl TaxNexusService for TaxNexus {
    async fn calculate_tax(&self, request: Request<CalculateTaxRequest>) -> Result<Response<CalculateTaxResponse>, Status> {
        let req = request.into_inner();
        let tax_amount = (req.amount_cents as f64 * 0.1) as i64; // mock 10% tax

        Ok(Response::new(CalculateTaxResponse {
            tax_amount_cents: tax_amount,
            tax_jurisdiction: "Mock Jurisdiction".to_string(),
        }))
    }
}

pub struct FinanceAgent {
    pub db: Arc<sqlx::PgPool>,
}

#[tonic::async_trait]
impl FinanceAgentService for FinanceAgent {
    async fn get_balances(&self, request: Request<GetBalancesRequest>) -> Result<Response<GetBalancesResponse>, Status> {
        let req = request.into_inner();

        // Disable RLS temporarily to allow querying or set tenant correctly before
        let rows = sqlx::query!(
            r#"
            SELECT wallet_type, balance, currency
            FROM ohc_virtual_wallets
            WHERE tenant_id = $1
            "#,
            req.tenant_id
        ).fetch_all(&*self.db).await.map_err(|e| Status::internal(e.to_string()))?;

        let balances = rows.into_iter().map(|row| {
            // Balance is DECIMAL(19,4), returned as BigDecimal by sqlx
            // Multiply by 100 to get cents and convert to i64
            use bigdecimal::ToPrimitive;
            let cents = (row.balance * bigdecimal::BigDecimal::from(100)).to_i64().unwrap_or(0);

            WalletBalance {
                wallet_type: row.wallet_type,
                balance_cents: cents,
                currency: row.currency,
            }
        }).collect();

        Ok(Response::new(GetBalancesResponse {
            balances,
        }))
    }

    async fn generate_insight(&self, _request: Request<GenerateInsightRequest>) -> Result<Response<GenerateInsightResponse>, Status> {
        // Mock generation
        Ok(Response::new(GenerateInsightResponse {
            insight_text: "You made $1,200 this week! I've already moved $240 into your Tax Vault. You're fully covered for Q3.".to_string(),
        }))
    }
}
