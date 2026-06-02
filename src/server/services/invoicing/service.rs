use tonic::{Request, Response, Status};
use std::sync::Arc;
use uuid::Uuid;
use sqlx::PgPool;

use ::server_ohc::invoicing::*;
use ::server_ohc::invoicing::invoice_service_server::InvoiceService;
use crate::integrations::stripe::client::StripeClient;
// use crate::services::agent::service::AgentServiceClient; // Need to verify correct path

pub struct MyInvoicingService {
    pool: PgPool,
    stripe_client: Arc<StripeClient>,
    // agent_client for NLP
}

impl MyInvoicingService {
    pub fn new(pool: PgPool, stripe_client: Arc<StripeClient>) -> Self {
        Self { pool, stripe_client }
    }
}

#[tonic::async_trait]
impl InvoiceService for MyInvoicingService {
    async fn create_invoice(
        &self,
        request: Request<CreateInvoiceRequest>,
    ) -> Result<Response<CreateInvoiceResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => return Err(Status::unauthenticated("Missing authentication info")),
        };

        let req = request.into_inner();
        let customer_id = req.customer_id;
        let prompt = req.natural_language_prompt;

        let items = vec![
            InvoiceItem {
                id: Uuid::new_v4().to_string(),
                description: format!("Parsed from: {}", prompt),
                amount_usd: 50.0,
            }
        ];

        let total_amount: f64 = items.iter().map(|i| i.amount_usd).sum();
        let invoice_id = Uuid::new_v4().to_string();

        let payment_link = match self.stripe_client.create_checkout_session("mock_price", &customer_id, total_amount).await {
            Ok(url) => url,
            Err(_) => return Err(Status::internal("Failed to generate payment link")),
        };

        let mut tx = self.pool.begin().await.map_err(|_| Status::internal("DB transaction failed"))?;

        sqlx::query!(
            "INSERT INTO invoices (id, tenant_id, customer_id, status, total_amount_usd, payment_link) VALUES ($1, $2, $3, $4, $5, $6)",
            invoice_id,
            tenant_id,
            customer_id,
            "draft",
            total_amount,
            payment_link
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(format!("DB error: {}", e)))?;

        for item in &items {
            sqlx::query!(
                "INSERT INTO invoice_items (id, invoice_id, description, amount_usd) VALUES ($1, $2, $3, $4)",
                item.id,
                invoice_id,
                item.description,
                item.amount_usd
            )
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(format!("DB error: {}", e)))?;
        }

        tx.commit().await.map_err(|_| Status::internal("DB commit failed"))?;

        let invoice = Invoice {
            id: invoice_id,
            tenant_id,
            customer_id,
            total_amount_usd: total_amount,
            status: "draft".to_string(),
            items,
            payment_link,
        };

        Ok(Response::new(CreateInvoiceResponse { invoice: Some(invoice) }))
    }

    async fn get_invoice(
        &self,
        request: Request<GetInvoiceRequest>,
    ) -> Result<Response<GetInvoiceResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => return Err(Status::unauthenticated("Missing authentication info")),
        };

        let req = request.into_inner();

        let inv_record = sqlx::query!(
            "SELECT id, customer_id, status, total_amount_usd, payment_link FROM invoices WHERE id = $1 AND tenant_id = $2",
            req.invoice_id,
            tenant_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Status::internal(e.to_string()))?
        .ok_or_else(|| Status::not_found("Invoice not found"))?;

        let item_records = sqlx::query!(
            "SELECT id, description, amount_usd FROM invoice_items WHERE invoice_id = $1",
            req.invoice_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let items: Vec<InvoiceItem> = item_records.into_iter().map(|rec| InvoiceItem {
            id: rec.id,
            description: rec.description,
            amount_usd: rec.amount_usd as f64,
        }).collect();

        let invoice = Invoice {
            id: inv_record.id,
            tenant_id,
            customer_id: inv_record.customer_id,
            total_amount_usd: inv_record.total_amount_usd as f64,
            status: inv_record.status,
            items,
            payment_link: inv_record.payment_link.unwrap_or_default(),
        };

        Ok(Response::new(GetInvoiceResponse { invoice: Some(invoice) }))
    }

    async fn list_invoices(
        &self,
        request: Request<ListInvoicesRequest>,
    ) -> Result<Response<ListInvoicesResponse>, Status> {
        let auth_info = request.extensions().get::<::server_auth::orchestration::AuthInfo>().cloned();
        let tenant_id = match auth_info {
            Some(info) => info.org_id,
            None => return Err(Status::unauthenticated("Missing authentication info")),
        };

        let req = request.into_inner();
        let customer_id = req.customer_id;

        let inv_records = if customer_id.is_empty() {
            sqlx::query!(
                "SELECT id, customer_id, status, total_amount_usd, payment_link FROM invoices WHERE tenant_id = $1",
                tenant_id
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
        } else {
            sqlx::query!(
                "SELECT id, customer_id, status, total_amount_usd, payment_link FROM invoices WHERE tenant_id = $1 AND customer_id = $2",
                tenant_id, customer_id
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Status::internal(e.to_string()))?
        };

        let mut invoices = Vec::new();
        for rec in inv_records {
            let item_records = sqlx::query!(
                "SELECT id, description, amount_usd FROM invoice_items WHERE invoice_id = $1",
                rec.id
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

            let items: Vec<InvoiceItem> = item_records.into_iter().map(|item_rec| InvoiceItem {
                id: item_rec.id,
                description: item_rec.description,
                amount_usd: item_rec.amount_usd as f64,
            }).collect();

            invoices.push(Invoice {
                id: rec.id,
                tenant_id: tenant_id.clone(),
                customer_id: rec.customer_id,
                total_amount_usd: rec.total_amount_usd as f64,
                status: rec.status,
                items,
                payment_link: rec.payment_link.unwrap_or_default(),
            });
        }

        Ok(Response::new(ListInvoicesResponse { invoices }))
    }
}
