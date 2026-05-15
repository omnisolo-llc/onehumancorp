use tonic::{Request, Response, Status};
use tokio_stream::{Stream, StreamExt};
use std::pin::Pin;
use std::sync::Arc;
use crate::MyHubService;
use ::server_ohc::orchestration::*;
use ::server_ohc::orchestration::hub_service_server::HubService;
use crate::hub::Hub;
use chrono::Utc;

impl MyHubService {
    pub async fn impl_get_my_plan(
        &self,
        request: tonic::Request<::server_ohc::orchestration::EmptyRequest>,
    ) -> Result<tonic::Response<::server_ohc::orchestration::MyPlanResponse>, tonic::Status> {
        let tenant_id = request.metadata().get("x-tenant-id")
            .map(|v| v.to_str().unwrap_or("default"))
            .unwrap_or("default");

        let tier = self.hub.tracker().get_tenant_tier(tenant_id).await.unwrap_or(::server_pricing::rate_limit::PlanTier::Free);
        let ai_used = self.hub.tracker().get_tenant_actions_used(tenant_id).await.unwrap_or(0);
        let storage_used_bytes = self.hub.tracker().get_tenant_storage_used(tenant_id).await.unwrap_or(0);

        let plan_name = match tier {
            ::server_pricing::rate_limit::PlanTier::Free => "Free",
            ::server_pricing::rate_limit::PlanTier::Starter => "Starter",
            ::server_pricing::rate_limit::PlanTier::Pro => "Pro",
            ::server_pricing::rate_limit::PlanTier::Business => "Business",
        }.to_string();

        let ai_limit = tier.monthly_action_limit().map(|v| v as i32);
        let storage_limit = tier.storage_limit_mb().map(|v| (v as i64) * 1024 * 1024);

        let next_bill_estimated = match tier {
            ::server_pricing::rate_limit::PlanTier::Free => 0,
            ::server_pricing::rate_limit::PlanTier::Starter => 9,
            ::server_pricing::rate_limit::PlanTier::Pro => 29,
            ::server_pricing::rate_limit::PlanTier::Business => 79,
        };

        Ok(tonic::Response::new(::server_ohc::orchestration::MyPlanResponse {
            current_plan: plan_name,
            ai_actions_used: ai_used as i32,
            ai_actions_limit: ai_limit,
            storage_used_bytes: storage_used_bytes,
            storage_limit_bytes: storage_limit,
            next_bill_estimated: next_bill_estimated,
        }))
    }
    pub async fn impl_get_cost_dashboard(
        &self,
        request: tonic::Request<::server_ohc::orchestration::EmptyRequest>,
    ) -> Result<tonic::Response<::server_ohc::orchestration::CostDashboardResponse>, tonic::Status> {
        let tenant_id = request.metadata().get("x-tenant-id")
            .map(|v| v.to_str().unwrap_or("default"))
            .unwrap_or("default");

        let auditor = self.hub.get_cost_auditor();
        let llm_cost_f64 = auditor.get_total_cost();
        let total_revenue_f64 = auditor.get_total_revenue();

        let storage_bytes = self.hub.tracker().get_tenant_storage_used(tenant_id).await.unwrap_or(0);
        let storage_gb = storage_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
        let storage_cost_f64 = storage_gb * 0.10; // $0.10 per GB

        let payment_fees_f64 = total_revenue_f64 * 0.029;

        let total_costs_f64 = llm_cost_f64 + storage_cost_f64 + payment_fees_f64;

        Ok(tonic::Response::new(::server_ohc::orchestration::CostDashboardResponse {
            total_revenue: (total_revenue_f64 * 100.0) as i64,
            total_costs: (total_costs_f64 * 100.0) as i64,
            llm_cost: (llm_cost_f64 * 100.0) as i64,
            storage_cost: (storage_cost_f64 * 100.0) as i64,
            payment_fees: (payment_fees_f64 * 100.0) as i64,
            period_start: "2024-05-01".to_string(), // In a real app this would be computed
            period_end: "2024-05-31".to_string(),
        }))
    }
    pub async fn impl_select_plan(
        &self,
        request: tonic::Request<::server_ohc::orchestration::SelectPlanRequest>,
    ) -> Result<tonic::Response<::server_ohc::orchestration::SelectPlanResponse>, tonic::Status> {
        let tenant_id = request.metadata().get("x-tenant-id")
            .map(|v| v.to_str().unwrap_or("default"))
            .unwrap_or("default").to_string();
        let req = request.into_inner();

        let stripe_key = std::env::var("STRIPE_API_KEY").unwrap_or_else(|_| "sk_test_mock".to_string());
        let client = crate::integrations::stripe::client::StripeClient::new(stripe_key);
        let mercadopago_client = std::env::var("MERCADOPAGO_ACCESS_TOKEN").ok().map(|token| crate::integrations::mercadopago::client::MercadoPagoClient::new(token));

        let amount = match req.plan_id.as_str() {
            "Starter" => 9.0,
            "Pro" => 29.0,
            "Business" => 79.0,
            _ => 0.0
        };

        let optimal_pm = crate::integrations::stripe::routing::PaymentRouter::optimize_payment_method(amount);
        let savings = crate::integrations::stripe::routing::PaymentRouter::calculate_fee_savings(amount);
        if savings > 0.0 {
            tracing::info!("Optimized payment method to {:?} to save ${:.2} on transaction fees", optimal_pm, savings);
        }

        let is_latam = req.plan_id.ends_with("_latam");
        let url = if let Some(mp_client) = mercadopago_client.filter(|_| is_latam) {
            mp_client.create_checkout_preference(&req.plan_id, &tenant_id).await
        } else {
            client.create_checkout_session(&req.plan_id, &tenant_id, amount).await
        }
            .map_err(|e| tonic::Status::internal(e))?;

        Ok(tonic::Response::new(::server_ohc::orchestration::SelectPlanResponse {
            success: true,
            checkout_url: url,
        }))
    }
    pub async fn impl_cancel_subscription(
        &self,
        request: tonic::Request<::server_ohc::orchestration::CancelSubscriptionRequest>,
    ) -> Result<tonic::Response<::server_ohc::orchestration::CancelSubscriptionResponse>, tonic::Status> {
        let req = request.into_inner();
        let stripe_key = std::env::var("STRIPE_API_KEY").unwrap_or_else(|_| "sk_test_mock".to_string());
        let client = crate::integrations::stripe::client::StripeClient::new(stripe_key);
        let _mercadopago_client = std::env::var("MERCADOPAGO_ACCESS_TOKEN").ok().map(|token| crate::integrations::mercadopago::client::MercadoPagoClient::new(token));

        client.cancel_subscription(&req.plan_id).await
            .map_err(|e| tonic::Status::internal(e))?;

        Ok(tonic::Response::new(::server_ohc::orchestration::CancelSubscriptionResponse {
            success: true,
        }))
    }
    pub async fn impl_download_invoice(
        &self,
        _request: tonic::Request<::server_ohc::orchestration::DownloadInvoiceRequest>,
    ) -> Result<tonic::Response<::server_ohc::orchestration::DownloadInvoiceResponse>, tonic::Status> {
        Ok(tonic::Response::new(::server_ohc::orchestration::DownloadInvoiceResponse {
            pdf_url: "https://invoice.stripe.com/...".to_string(),
        }))
    }
}
