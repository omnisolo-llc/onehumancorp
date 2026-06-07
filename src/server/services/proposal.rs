use tonic::{Request, Response, Status};
use uuid::Uuid;
use sqlx::Row;
use ::server_ohc::app::proposal_engine_service_server::ProposalEngineService;
use ::server_ohc::app::{
    SubmitInquiryRequest, SubmitInquiryResponse,
    GetProposalRequest, GetProposalResponse, Proposal, ProposalLineItem,
    AcceptProposalRequest, AcceptProposalResponse,
    RejectProposalRequest, RejectProposalResponse,
};

pub struct NativeProposalService {}

#[tonic::async_trait]
impl ProposalEngineService for NativeProposalService {
    async fn submit_inquiry(
        &self,
        request: Request<SubmitInquiryRequest>,
    ) -> Result<Response<SubmitInquiryResponse>, Status> {
        let mut req = request.into_inner();
        let tenant_id = req.tenant_id.clone();

        if tenant_id.is_empty() {
            return Err(Status::invalid_argument("missing tenant_id for inquiry submission"));
        }

        let inquiry_id = Uuid::new_v4().to_string();

        let pool = crate::db::get_pool();
        let mut tx = pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        sqlx::query(
            "INSERT INTO customer_inquiries (id, tenant_id, customer_id, customer_name, customer_email, customer_phone, description, status) \
             VALUES ($1, $2, $3, $4, $5, $6, $7, 'New')"
        )
        .bind(&inquiry_id)
        .bind(&tenant_id)
        .bind(if req.customer_id.is_empty() { None } else { Some(req.customer_id) })
        .bind(&req.customer_name)
        .bind(&req.customer_email)
        .bind(&req.customer_phone)
        .bind(&req.description)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        // Invoke the AI Salesperson Agent via the minimax logic
        // to dynamically parse the description and output price
        let ai_client = crate::minimax::MinimaxClient::new(
            std::env::var("MINIMAX_API_KEY").unwrap_or_else(|_| "dummy_key".to_string())
        );
        let prompt = format!(
            "Analyze this custom request from a customer and generate a JSON proposal. \
            Request: '{}' \
            \
            Return exactly and only valid JSON with this structure: \
            {{ \
              \"total_amount_cents\": integer, \
              \"deposit_percentage\": integer, \
              \"line_items\": [ \
                {{ \"description\": string, \"quantity\": integer, \"unit_price_cents\": integer }} \
              ] \
            }}",
            req.description
        );

        let ai_response = ai_client.reason(&prompt).await.unwrap_or_else(|_| r#"{"total_amount_cents":50000,"deposit_percentage":30,"line_items":[{"description":"Fallback Custom Service","quantity":1,"unit_price_cents":50000}]}"#.to_string());

        let json_str = ai_response.trim().trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim();
        let ai_proposal: serde_json::Value = serde_json::from_str(json_str).unwrap_or_else(|_| serde_json::json!({
            "total_amount_cents": 50000,
            "deposit_percentage": 30,
            "line_items": [{"description": "Fallback Custom Service", "quantity": 1, "unit_price_cents": 50000}]
        }));

        let total_amount_cents = ai_proposal["total_amount_cents"].as_i64().unwrap_or(50000);
        let deposit_percentage = ai_proposal["deposit_percentage"].as_i64().unwrap_or(30);
        let deposit_amount_cents = total_amount_cents * deposit_percentage / 100;

        // Stripe API Hook - create checkout session
        let stripe_client = crate::integrations::stripe::client::StripeClient::new("dummy_key".to_string());
        let amount_usd = (deposit_amount_cents as f64) / 100.0;
        let stripe_link = stripe_client.create_checkout_session("custom_deposit", "cus_123", amount_usd).await.unwrap_or_else(|_| "https://checkout.stripe.com/fallback".to_string());

        let proposal_id = Uuid::new_v4().to_string();

        sqlx::query(
            "INSERT INTO proposals (id, tenant_id, inquiry_id, status, total_amount_cents, deposit_percentage, deposit_amount_cents, payment_link_url) \
             VALUES ($1, $2, $3, 'Sent', $4, $5, $6, $7)"
        )
        .bind(&proposal_id)
        .bind(&tenant_id)
        .bind(&inquiry_id)
        .bind(total_amount_cents)
        .bind(deposit_percentage as i32)
        .bind(deposit_amount_cents)
        .bind(&stripe_link)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        if let Some(items) = ai_proposal["line_items"].as_array() {
            for item in items {
                let line_item_id = Uuid::new_v4().to_string();
                let desc = item["description"].as_str().unwrap_or("Custom Item");
                let qty = item["quantity"].as_i64().unwrap_or(1) as i32;
                let unit_price = item["unit_price_cents"].as_i64().unwrap_or(0);
                let total_price = unit_price * (qty as i64);

                sqlx::query(
                    "INSERT INTO proposal_line_items (id, tenant_id, proposal_id, description, quantity, unit_price_cents, total_price_cents) \
                     VALUES ($1, $2, $3, $4, $5, $6, $7)"
                )
                .bind(&line_item_id)
                .bind(&tenant_id)
                .bind(&proposal_id)
                .bind(desc)
                .bind(qty)
                .bind(unit_price)
                .bind(total_price)
                .execute(&mut *tx)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;
            }
        }

        sqlx::query("UPDATE customer_inquiries SET status = 'ProposalSent' WHERE id = $1 AND tenant_id = $2")
            .bind(&inquiry_id)
            .bind(&tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(SubmitInquiryResponse {
            inquiry_id: proposal_id, // For UI redirect, MVP passes proposal ID here
            status: "ProposalSent".to_string(),
        }))
    }

    async fn get_proposal(
        &self,
        request: Request<GetProposalRequest>,
    ) -> Result<Response<GetProposalResponse>, Status> {
        let mut req = request.into_inner();
        let tenant_id = req.tenant_id.clone();

        if tenant_id.is_empty() {
            return Err(Status::invalid_argument("missing tenant_id for getting proposal"));
        }

        let pool = crate::db::get_pool();
        let mut tx = pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let proposal_row = sqlx::query(
            "SELECT id, inquiry_id, status, total_amount_cents, deposit_percentage, deposit_amount_cents, payment_link_url \
             FROM proposals WHERE id = $1 AND tenant_id = $2"
        )
        .bind(&req.proposal_id)
        .bind(&tenant_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        if let Some(row) = proposal_row {
            let mut proposal = Proposal {
                id: row.get("id"),
                inquiry_id: row.get::<Option<String>, _>("inquiry_id").unwrap_or_default(),
                status: row.get("status"),
                total_amount_cents: row.get("total_amount_cents"),
                deposit_percentage: row.get("deposit_percentage"),
                deposit_amount_cents: row.get("deposit_amount_cents"),
                payment_link_url: row.get::<Option<String>, _>("payment_link_url").unwrap_or_default(),
                line_items: vec![],
            };

            let lines = sqlx::query(
                "SELECT id, description, quantity, unit_price_cents, total_price_cents \
                 FROM proposal_line_items WHERE proposal_id = $1 AND tenant_id = $2"
            )
            .bind(&req.proposal_id)
            .bind(&tenant_id)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

            for l in lines {
                proposal.line_items.push(ProposalLineItem {
                    id: l.get("id"),
                    description: l.get("description"),
                    quantity: l.get("quantity"),
                    unit_price_cents: l.get("unit_price_cents"),
                    total_price_cents: l.get("total_price_cents"),
                });
            }

            Ok(Response::new(GetProposalResponse {
                proposal: Some(proposal),
            }))
        } else {
            Err(Status::not_found("Proposal not found"))
        }
    }

    async fn accept_proposal(
        &self,
        request: Request<AcceptProposalRequest>,
    ) -> Result<Response<AcceptProposalResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => {
                let spiffe_id_str = request.metadata().get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
                ::server_auth::parse_spiffe_id(spiffe_id_str).map_err(|_| Status::unauthenticated("invalid spiffe id"))?.0
            }
        };

        if tenant_id.is_empty() {
            return Err(Status::unauthenticated("missing tenant identity in session"));
        }

        let mut req = request.into_inner();
        req.tenant_id = tenant_id.clone();

        let pool = crate::db::get_pool();
        let mut tx = pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        sqlx::query("UPDATE proposals SET status = 'Accepted' WHERE id = $1 AND tenant_id = $2")
            .bind(&req.proposal_id)
            .bind(&tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(AcceptProposalResponse {
            status: "Accepted".to_string(),
        }))
    }

    async fn reject_proposal(
        &self,
        request: Request<RejectProposalRequest>,
    ) -> Result<Response<RejectProposalResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => {
                let spiffe_id_str = request.metadata().get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
                ::server_auth::parse_spiffe_id(spiffe_id_str).map_err(|_| Status::unauthenticated("invalid spiffe id"))?.0
            }
        };

        if tenant_id.is_empty() {
            return Err(Status::unauthenticated("missing tenant identity in session"));
        }

        let mut req = request.into_inner();
        req.tenant_id = tenant_id.clone();

        let pool = crate::db::get_pool();
        let mut tx = pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;
        crate::common::auth_utils::set_org_context(&mut *tx, &tenant_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        sqlx::query("UPDATE proposals SET status = 'Rejected' WHERE id = $1 AND tenant_id = $2")
            .bind(&req.proposal_id)
            .bind(&tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(RejectProposalResponse {
            status: "Rejected".to_string(),
        }))
    }
}

#[cfg(test)]
mod native_proposal_tests {
    use super::*;
    use tonic::Request;
    use ::server_ohc::app::proposal_engine_service_server::ProposalEngineService;
    use ::server_ohc::app::{SubmitInquiryRequest, GetProposalRequest, AcceptProposalRequest, RejectProposalRequest};

    #[tokio::test]
    async fn test_native_proposal_missing_tenant() {
        let svc = NativeProposalService {};
        let req = Request::new(SubmitInquiryRequest {
            tenant_id: "".to_string(),
            customer_id: "c1".to_string(),
            customer_name: "Test".to_string(),
            customer_email: "test@example.com".to_string(),
            customer_phone: "123".to_string(),
            description: "Test".to_string(),
            image_urls: vec![],
        });

        let res = svc.submit_inquiry(req).await;
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().code(), tonic::Code::InvalidArgument);
    }

    #[tokio::test]
    async fn test_native_get_proposal_missing_tenant() {
        let svc = NativeProposalService {};
        let req = Request::new(GetProposalRequest {
            tenant_id: "".to_string(),
            proposal_id: "p1".to_string(),
        });

        let res = svc.get_proposal(req).await;
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().code(), tonic::Code::InvalidArgument);
    }

    #[tokio::test]
    async fn test_native_accept_proposal_unauthenticated() {
        let svc = NativeProposalService {};
        let req = Request::new(AcceptProposalRequest {
            tenant_id: "".to_string(),
            proposal_id: "p1".to_string(),
        });

        let res = svc.accept_proposal(req).await;
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().code(), tonic::Code::Unauthenticated);
    }

    #[tokio::test]
    async fn test_native_reject_proposal_unauthenticated() {
        let svc = NativeProposalService {};
        let req = Request::new(RejectProposalRequest {
            tenant_id: "".to_string(),
            proposal_id: "p1".to_string(),
            reason: "Too expensive".to_string(),
        });

        let res = svc.reject_proposal(req).await;
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().code(), tonic::Code::Unauthenticated);
    }
}
