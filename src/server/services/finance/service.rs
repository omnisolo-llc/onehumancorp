use tonic::{Request, Response, Status};
use ::server_ohc::finance::*;
use ::server_ohc::finance::finance_service_server::FinanceService;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct MyFinanceService {
    pool: Arc<PgPool>,
}

impl MyFinanceService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[tonic::async_trait]
impl FinanceService for MyFinanceService {
    async fn get_finance_summary(
        &self,
        request: Request<GetFinanceSummaryRequest>,
    ) -> Result<Response<GetFinanceSummaryResponse>, Status> {
        let req = request.into_inner();
        let _org_id = req.organization_id;

        let summary = FinanceSummary {
            total_money_in: 5400.0,
            total_money_out: 1200.0,
            estimated_tax_safe: 800.0,
            net_profit: 4200.0,
        };

        Ok(Response::new(GetFinanceSummaryResponse { summary: Some(summary) }))
    }

    async fn process_receipt(
        &self,
        request: Request<ProcessReceiptRequest>,
    ) -> Result<Response<ProcessReceiptResponse>, Status> {
        let req = request.into_inner();
        let org_id = req.organization_id;

        let tx_id = Uuid::new_v4().to_string();
        let receipt_id = Uuid::new_v4().to_string();

        let transaction = Transaction {
            id: tx_id.clone(),
            tenant_id: org_id.clone(),
            bank_account_id: "default_account".to_string(),
            amount: 45.20,
            currency: "USD".to_string(),
            r#type: "EXPENSE".to_string(),
            status: "COMPLETED".to_string(),
            description: "Supplies from Home Depot".to_string(),
            vendor: "Home Depot".to_string(),
            categorized_as: "Supplies".to_string(),
            date_unix: chrono::Utc::now().timestamp(),
            created_at_unix: chrono::Utc::now().timestamp(),
            updated_at_unix: chrono::Utc::now().timestamp(),
        };

        let receipt = Receipt {
            id: receipt_id,
            tenant_id: org_id,
            transaction_id: tx_id,
            file_url: req.file_url,
            extracted_text: "Home Depot $45.20 Supplies".to_string(),
            uploaded_at_unix: chrono::Utc::now().timestamp(),
        };

        Ok(Response::new(ProcessReceiptResponse {
            success: true,
            transaction: Some(transaction),
            receipt: Some(receipt),
        }))
    }
}
