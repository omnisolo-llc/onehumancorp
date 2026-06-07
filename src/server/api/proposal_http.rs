use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
    http::HeaderMap,
};
use std::sync::Arc;
use tonic::Request;
use ::server_ohc::app::proposal_engine_service_server::ProposalEngineService;
use ::server_ohc::app::{SubmitInquiryRequest, GetProposalRequest};

use crate::services::proposal::NativeProposalService;

pub fn proposal_routes() -> Router<Arc<NativeProposalService>> {
    Router::new()
        .route("/v1/proposals/request", post(submit_inquiry_handler))
        .route("/v1/proposals/:id", get(get_proposal_handler))
}

async fn submit_inquiry_handler(
    State(service): State<Arc<NativeProposalService>>,
    headers: HeaderMap,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let tenant_id = headers.get("x-tenant-id").and_then(|v| v.to_str().ok()).unwrap_or("").to_string();

    let req = Request::new(SubmitInquiryRequest {
        tenant_id: tenant_id.clone(),
        customer_id: payload.get("customerId").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        customer_name: payload.get("customerName").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        customer_email: payload.get("customerEmail").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        customer_phone: payload.get("customerPhone").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        description: payload.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        image_urls: vec![],
    });

    match ProposalEngineService::submit_inquiry(&*service, req).await {
        Ok(res) => {
            let inner = res.into_inner();
            (StatusCode::OK, Json(serde_json::json!({ "inquiryId": inner.inquiry_id, "status": inner.status })))
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.message() }))),
    }
}

async fn get_proposal_handler(
    Path(id): Path<String>,
    State(service): State<Arc<NativeProposalService>>,
    headers: HeaderMap,
) -> impl IntoResponse {
    let tenant_id = headers.get("x-tenant-id").and_then(|v| v.to_str().ok()).unwrap_or("").to_string();

    let req = Request::new(GetProposalRequest {
        tenant_id: tenant_id.clone(),
        proposal_id: id,
    });

    match ProposalEngineService::get_proposal(&*service, req).await {
        Ok(res) => {
            let inner = res.into_inner();
            if let Some(p) = inner.proposal {
                let p_json = serde_json::json!({
                    "id": p.id,
                    "inquiry_id": p.inquiry_id,
                    "status": p.status,
                    "total_amount_cents": p.total_amount_cents,
                    "deposit_percentage": p.deposit_percentage,
                    "deposit_amount_cents": p.deposit_amount_cents,
                    "payment_link_url": p.payment_link_url,
                    "line_items": p.line_items.iter().map(|l| serde_json::json!({
                        "id": l.id,
                        "description": l.description,
                        "quantity": l.quantity,
                        "unit_price_cents": l.unit_price_cents,
                        "total_price_cents": l.total_price_cents
                    })).collect::<Vec<_>>()
                });
                (StatusCode::OK, Json(serde_json::json!({ "proposal": p_json })))
            } else {
                (StatusCode::NOT_FOUND, Json(serde_json::json!({ "error": "Not found" })))
            }
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": e.message() }))),
    }
}
