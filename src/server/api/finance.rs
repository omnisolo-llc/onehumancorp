use axum::{
    routing::{get, post},
    Router, Json, Extension,
};
use std::sync::Arc;
use sqlx::PgPool;
use serde::{Deserialize, Serialize};
use crate::services::finance::service::MyFinanceService;
use ::server_ohc::finance::{GetFinanceSummaryRequest, ProcessReceiptRequest};
use tonic::Request;
use ::server_ohc::finance::finance_service_server::FinanceService;

#[derive(Deserialize)]
pub struct ProcessReceiptPayload {
    pub file_url: String,
    pub extracted_text: Option<String>,
}

pub fn routes(pool: Arc<PgPool>) -> Router {
    let service = Arc::new(MyFinanceService::new(pool.clone()));

    Router::new()
        .route("/summary", get(get_summary))
        .route("/receipt", post(process_receipt))
        .layer(Extension(service))
}

async fn get_summary(
    Extension(service): Extension<Arc<MyFinanceService>>,
) -> Json<serde_json::Value> {
    let req = GetFinanceSummaryRequest {
        organization_id: "test_org".to_string(), // In reality, get from auth context
    };

    let res = service.get_finance_summary(Request::new(req)).await;
    match res {
        Ok(response) => {
            let summary = response.into_inner().summary.unwrap();
            Json(serde_json::json!({
                "total_money_in": summary.total_money_in,
                "total_money_out": summary.total_money_out,
                "estimated_tax_safe": summary.estimated_tax_safe,
                "net_profit": summary.net_profit,
            }))
        },
        Err(e) => Json(serde_json::json!({"error": e.message()})),
    }
}

async fn process_receipt(
    Extension(service): Extension<Arc<MyFinanceService>>,
    Json(payload): Json<ProcessReceiptPayload>,
) -> Json<serde_json::Value> {
    let req = ProcessReceiptRequest {
        organization_id: "test_org".to_string(), // Get from auth context
        file_url: payload.file_url,
        extracted_text: payload.extracted_text.unwrap_or_default(),
    };

    let res = service.process_receipt(Request::new(req)).await;
    match res {
        Ok(response) => {
            let resp = response.into_inner();
            Json(serde_json::json!({
                "success": resp.success,
                // Serialize transaction/receipt as needed...
            }))
        },
        Err(e) => Json(serde_json::json!({"error": e.message()})),
    }
}
