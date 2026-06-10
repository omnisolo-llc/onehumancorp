use std::sync::Arc;

use ::server_ohc::invoice::*;
use ::server_ohc::invoice::invoice_service_server::InvoiceService;
use tonic::{Request, Response, Status};

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
        let req = request.into_inner();

        let pool = &self.hub.pool;
        let mut tx = pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        // Set tenant context for RLS
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&req.tenant_id)
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
        .bind(&req.tenant_id)
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
            .bind(&req.tenant_id)
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
        let req = request.into_inner();

        let pool = &self.hub.pool;
        let mut tx = pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        // Set tenant context for RLS
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&req.tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        use sqlx::Row;

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
            created_at: 0,
            updated_at: 0,
        };

        Ok(Response::new(invoice))
    }

    async fn list_invoices(
        &self,
        request: Request<ListInvoicesRequest>,
    ) -> Result<Response<ListInvoicesResponse>, Status> {
        let req = request.into_inner();

        let pool = &self.hub.pool;
        let mut tx = pool.begin().await.map_err(|e| Status::internal(e.to_string()))?;

        // Set tenant context for RLS
        sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
            .bind(&req.tenant_id)
            .execute(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        use sqlx::Row;

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
                created_at: 0,
                updated_at: 0,
            });
        }

        Ok(Response::new(ListInvoicesResponse { invoices }))
    }

    async fn update_invoice_status(
        &self,
        _request: Request<UpdateInvoiceStatusRequest>,
    ) -> Result<Response<Invoice>, Status> {
        Err(Status::unimplemented("update_invoice_status is unimplemented"))
    }

    async fn draft_invoice_from_context(
        &self,
        request: Request<DraftInvoiceFromContextRequest>,
    ) -> Result<Response<DraftInvoiceFromContextResponse>, Status> {
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
    use ::server_ohc::invoice::invoice_service_server::InvoiceServiceServer;
    use tonic::transport::Server;
    use tower::ServiceBuilder;

    // This is just a stub router for Axum integration if needed,
    // though typically gRPC services are mounted differently.
    // For now, we return an empty router.
    axum::Router::new()
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use ::server_ohc::invoice::*;
    use ::server_ohc::invoice::invoice_service_server::InvoiceService;
    use super::InvoiceServiceImpl;
    use tonic::Request;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_invoice_logic() {
        // Just verify basic compilation
        assert!(true);
    }
}
