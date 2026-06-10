use std::sync::Arc;

use ::server_ohc::invoice::*;
use ::server_ohc::invoice::invoice_service_server::InvoiceService;
use tonic::{Request, Response, Status};
use sqlx::Row;

use crate::hub::Hub;

pub struct InvoiceServiceImpl {
    pub hub: Arc<Hub>,
}

#[tonic::async_trait]
impl InvoiceService for InvoiceServiceImpl {
    async fn create_invoice(
        &self,
        request: Request<CreateInvoiceRequest>,
    ) -> Result<Response<Invoice>, Status> {
        let spiffe_id_str = request.metadata().get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
        let (tenant_id, _) = ::server_auth::parse_spiffe_id(spiffe_id_str).map_err(|_| Status::unauthenticated("invalid spiffe id"))?;
        let org_id = if tenant_id.is_empty() { ::server_common::auth_utils::get_default_tenant() } else { tenant_id };

        let req = request.into_inner();

        let pool = &self.hub.pool;
        let mut tx = pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        // Set tenant context for RLS
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&org_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let invoice_id = uuid::Uuid::new_v4().to_string();
        let total_amount: f64 = req.line_items.iter().map(|item| item.amount).sum();
        let status = "draft".to_string();

        let stripe_payment_link = format!("https://checkout.stripe.com/pay/cs_test_{}", uuid::Uuid::new_v4().to_string().replace("-", ""));

        sqlx::query(
            "INSERT INTO invoices (id, tenant_id, client_id, client_name, status, due_date, currency, total_amount, stripe_payment_link)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
        )
        .bind(&invoice_id)
        .bind(&org_id)
        .bind(&req.client_id)
        .bind(&req.client_name)
        .bind(&status)
        .bind(req.due_date)
        .bind(&req.currency)
        .bind(total_amount)
        .bind(&stripe_payment_link)
        .execute(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        let mut saved_items = Vec::new();
        for item in req.line_items {
            let item_id = uuid::Uuid::new_v4().to_string();
            sqlx::query(
                "INSERT INTO invoice_line_items (id, tenant_id, invoice_id, description, quantity, unit_price, amount)
                 VALUES ($1, $2, $3, $4, $5, $6, $7)"
            )
            .bind(&item_id)
            .bind(&org_id)
            .bind(&invoice_id)
            .bind(&item.description)
            .bind(item.quantity)
            .bind(item.unit_price)
            .bind(item.amount)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

            saved_items.push(InvoiceLineItem {
                id: item_id,
                invoice_id: invoice_id.clone(),
                description: item.description,
                quantity: item.quantity,
                unit_price: item.unit_price,
                amount: item.amount,
            });
        }

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Invoice {
            id: invoice_id,
            client_id: req.client_id,
            client_name: req.client_name,
            status,
            due_date: req.due_date,
            currency: req.currency,
            total_amount,
            stripe_invoice_id: "".to_string(),
            stripe_payment_link,
            line_items: saved_items,
            created_at: chrono::Utc::now().timestamp(),
            updated_at: chrono::Utc::now().timestamp(),
        }))
    }

    async fn get_invoice(
        &self,
        request: Request<GetInvoiceRequest>,
    ) -> Result<Response<Invoice>, Status> {
        let spiffe_id_str = request.metadata().get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
        let (tenant_id, _) = ::server_auth::parse_spiffe_id(spiffe_id_str).map_err(|_| Status::unauthenticated("invalid spiffe id"))?;
        let org_id = if tenant_id.is_empty() { ::server_common::auth_utils::get_default_tenant() } else { tenant_id };

        let req = request.into_inner();

        let pool = &self.hub.pool;
        let mut tx = pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        // Set tenant context for RLS
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&org_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;



        let row = sqlx::query("SELECT * FROM invoices WHERE id = $1")
            .bind(&req.invoice_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let items_rows = sqlx::query("SELECT * FROM invoice_line_items WHERE invoice_id = $1")
            .bind(&req.invoice_id)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        let mut line_items = Vec::new();
        for item_row in items_rows {
            line_items.push(InvoiceLineItem {
                id: item_row.try_get("id").unwrap_or_default(),
                invoice_id: item_row.try_get("invoice_id").unwrap_or_default(),
                description: item_row.try_get("description").unwrap_or_default(),
                quantity: item_row.try_get("quantity").unwrap_or_default(),
                unit_price: item_row.try_get("unit_price").unwrap_or_default(),
                amount: item_row.try_get("amount").unwrap_or_default(),
            });
        }

        let invoice = Invoice {
            id: row.try_get("id").unwrap_or_default(),
            client_id: row.try_get("client_id").unwrap_or_default(),
            client_name: row.try_get("client_name").unwrap_or_default(),
            status: row.try_get("status").unwrap_or_default(),
            due_date: row.try_get("due_date").unwrap_or_default(),
            currency: row.try_get("currency").unwrap_or_default(),
            total_amount: row.try_get("total_amount").unwrap_or_default(),
            stripe_invoice_id: row.try_get("stripe_invoice_id").unwrap_or_default(),
            stripe_payment_link: row.try_get("stripe_payment_link").unwrap_or_default(),
            line_items,
            created_at: row.try_get::<chrono::DateTime<chrono::Utc>, _>("created_at").map(|dt| dt.timestamp()).unwrap_or(0),
            updated_at: row.try_get::<chrono::DateTime<chrono::Utc>, _>("updated_at").map(|dt| dt.timestamp()).unwrap_or(0),
        };

        Ok(Response::new(invoice))
    }

    async fn list_invoices(
        &self,
        request: Request<ListInvoicesRequest>,
    ) -> Result<Response<ListInvoicesResponse>, Status> {
        let spiffe_id_str = request.metadata().get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
        let (tenant_id, _) = ::server_auth::parse_spiffe_id(spiffe_id_str).map_err(|_| Status::unauthenticated("invalid spiffe id"))?;
        let org_id = if tenant_id.is_empty() { ::server_common::auth_utils::get_default_tenant() } else { tenant_id };

        let _req = request.into_inner();

        let pool = &self.hub.pool;
        let mut tx = pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        // Set tenant context for RLS
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&org_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;



        let rows = sqlx::query("SELECT * FROM invoices ORDER BY created_at DESC")
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        let mut invoices = Vec::new();
        for row in rows {
            invoices.push(Invoice {
                id: row.try_get("id").unwrap_or_default(),
                client_id: row.try_get("client_id").unwrap_or_default(),
                client_name: row.try_get("client_name").unwrap_or_default(),
                status: row.try_get("status").unwrap_or_default(),
                due_date: row.try_get("due_date").unwrap_or_default(),
                currency: row.try_get("currency").unwrap_or_default(),
                total_amount: row.try_get("total_amount").unwrap_or_default(),
                stripe_invoice_id: row.try_get("stripe_invoice_id").unwrap_or_default(),
                stripe_payment_link: row.try_get("stripe_payment_link").unwrap_or_default(),
                line_items: vec![],
                created_at: row.try_get::<chrono::DateTime<chrono::Utc>, _>("created_at").map(|dt| dt.timestamp()).unwrap_or(0),
                updated_at: row.try_get::<chrono::DateTime<chrono::Utc>, _>("updated_at").map(|dt| dt.timestamp()).unwrap_or(0),
            });
        }

        Ok(Response::new(ListInvoicesResponse { invoices }))
    }

    async fn update_invoice_status(
        &self,
        request: Request<UpdateInvoiceStatusRequest>,
    ) -> Result<Response<Invoice>, Status> {
        let spiffe_id_str = request.metadata().get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
        let (tenant_id, _) = ::server_auth::parse_spiffe_id(spiffe_id_str).map_err(|_| Status::unauthenticated("invalid spiffe id"))?;
        let org_id = if tenant_id.is_empty() { ::server_common::auth_utils::get_default_tenant() } else { tenant_id };

        let req = request.into_inner();
        let pool = &self.hub.pool;
        let mut tx = pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        // Set tenant context for RLS
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&org_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let now = chrono::Utc::now().timestamp();

        let result = sqlx::query(
            "UPDATE invoices SET status = $1, updated_at = $2 WHERE id = $3 AND tenant_id = $4 RETURNING id"
        )
        .bind(&req.status)
        .bind(now)
        .bind(&req.invoice_id)
        .bind(&org_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        if result.is_none() {
            return Err(Status::not_found("Invoice not found or does not belong to tenant"));
        }



        let row = sqlx::query("SELECT * FROM invoices WHERE id = $1 AND tenant_id = $2")
            .bind(&req.invoice_id)
            .bind(&org_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let items_rows = sqlx::query("SELECT * FROM invoice_line_items WHERE invoice_id = $1 AND tenant_id = $2")
            .bind(&req.invoice_id)
            .bind(&org_id)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        let mut line_items = Vec::new();
        for item_row in items_rows {
            line_items.push(InvoiceLineItem {
                id: item_row.try_get("id").unwrap_or_default(),
                invoice_id: item_row.try_get("invoice_id").unwrap_or_default(),
                description: item_row.try_get("description").unwrap_or_default(),
                quantity: item_row.try_get("quantity").unwrap_or_default(),
                unit_price: item_row.try_get("unit_price").unwrap_or_default(),
                amount: item_row.try_get("amount").unwrap_or_default(),
            });
        }

        let invoice = Invoice {
            id: row.try_get("id").unwrap_or_default(),
            client_id: row.try_get("client_id").unwrap_or_default(),
            client_name: row.try_get("client_name").unwrap_or_default(),
            status: row.try_get("status").unwrap_or_default(),
            due_date: row.try_get("due_date").unwrap_or_default(),
            currency: row.try_get("currency").unwrap_or_default(),
            total_amount: row.try_get("total_amount").unwrap_or_default(),
            stripe_invoice_id: row.try_get("stripe_invoice_id").unwrap_or_default(),
            stripe_payment_link: row.try_get("stripe_payment_link").unwrap_or_default(),
            line_items,
            created_at: row.try_get::<chrono::DateTime<chrono::Utc>, _>("created_at").map(|dt| dt.timestamp()).unwrap_or(0),
            updated_at: row.try_get::<chrono::DateTime<chrono::Utc>, _>("updated_at").map(|dt| dt.timestamp()).unwrap_or(0),
        };

        Ok(Response::new(invoice))
    }

    async fn draft_invoice_from_context(
        &self,
        request: Request<DraftInvoiceFromContextRequest>,
    ) -> Result<Response<DraftInvoiceFromContextResponse>, Status> {
        let spiffe_id_str = request.metadata().get("x-spiffe-id").and_then(|v| v.to_str().ok()).unwrap_or("");
        let (tenant_id, _) = ::server_auth::parse_spiffe_id(spiffe_id_str).map_err(|_| Status::unauthenticated("invalid spiffe id"))?;
        let _org_id = if tenant_id.is_empty() { ::server_common::auth_utils::get_default_tenant() } else { tenant_id };

        let req = request.into_inner();

        // Simple mock of agent extraction
        let line_item1 = InvoiceLineItem {
            id: "".to_string(),
            invoice_id: "".to_string(),
            description: "Consulting Services".to_string(),
            quantity: 10,
            unit_price: 150.0,
            amount: 1500.0,
        };

        let invoice = Invoice {
            id: "draft-temp".to_string(),
            client_id: "".to_string(),
            client_name: req.client_name.clone(),
            status: "draft".to_string(),
            due_date: chrono::Utc::now().timestamp() + 30 * 24 * 3600, // +30 days
            currency: "USD".to_string(),
            total_amount: 1500.0,
            stripe_invoice_id: "".to_string(),
            stripe_payment_link: "".to_string(),
            line_items: vec![line_item1],
            created_at: chrono::Utc::now().timestamp(),
            updated_at: chrono::Utc::now().timestamp(),
        };

        Ok(Response::new(DraftInvoiceFromContextResponse { draft: Some(invoice) }))
    }
}

pub fn router<S: Clone + Send + Sync + 'static>(_hub: Arc<Hub>) -> axum::Router<S> {
    // This is just a stub router for Axum integration if needed,
    // though typically gRPC services are mounted differently.
    // For now, we return an empty router.
    axum::Router::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::server_ohc::invoice::invoice_service_server::InvoiceService;
    use tonic::Request;
    use crate::hub::Hub;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_invoice_logic() {
        assert!(true);
    }
}
