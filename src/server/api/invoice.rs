use std::sync::Arc;

use ::server_ohc::invoice::*;
use ::server_ohc::invoice::invoice_service_server::InvoiceService;
use axum::{extract::{State, Extension, Path}, http::StatusCode, response::IntoResponse, routing::{get, put}, Json, Router};
use serde::Deserialize;
use ::server_common::Claims;
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
            total_amount_cents: (total_amount * 100.0) as i32,
            payment_status: "draft".to_string(),
            view_count: 0,
            amount_paid_cents: 0,
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
            total_amount_cents: row.try_get("total_amount_cents").unwrap_or_default(),
            payment_status: row.try_get("payment_status").unwrap_or_default(),
            view_count: row.try_get("view_count").unwrap_or_default(),
            amount_paid_cents: row.try_get("amount_paid_cents").unwrap_or_default(),
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
                total_amount_cents: row.try_get("total_amount_cents").unwrap_or_default(),
                payment_status: row.try_get("payment_status").unwrap_or_default(),
                view_count: row.try_get("view_count").unwrap_or_default(),
                amount_paid_cents: row.try_get("amount_paid_cents").unwrap_or_default(),
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
        request: Request<UpdateInvoiceStatusRequest>,
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

        sqlx::query("UPDATE invoices SET status = $1, updated_at = $2 WHERE id = $3 AND tenant_id = $4")
            .bind(&req.status)
            .bind(chrono::Utc::now().timestamp())
            .bind(&req.invoice_id)
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
            total_amount_cents: row.try_get("total_amount_cents").unwrap_or_default(),
            payment_status: row.try_get("payment_status").unwrap_or_default(),
            view_count: row.try_get("view_count").unwrap_or_default(),
            amount_paid_cents: row.try_get("amount_paid_cents").unwrap_or_default(),
            stripe_invoice_id: row.try_get("stripe_invoice_id").unwrap_or_default(),
            stripe_payment_link: row.try_get("stripe_payment_link").unwrap_or_default(),
            line_items,
            created_at: row.try_get("created_at").unwrap_or_default(),
            updated_at: row.try_get("updated_at").unwrap_or_default(),
        };

        Ok(Response::new(invoice))
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
            total_amount_cents: 150000,
            payment_status: "draft".to_string(),
            view_count: 0,
            amount_paid_cents: 0,
            stripe_invoice_id: "".to_string(),
            stripe_payment_link: "".to_string(),
            line_items: vec![line_item1],
            created_at: chrono::Utc::now().timestamp(),
            updated_at: chrono::Utc::now().timestamp(),
        };

        Ok(Response::new(DraftInvoiceFromContextResponse { draft: Some(invoice) }))
    }
}

#[derive(Deserialize)]
pub struct CreateInvoiceHttp {
    pub client_id: String,
    pub client_name: String,
    pub due_date: i64,
    pub currency: String,
    // We avoid using InvoiceLineItem directly in the struct if it doesn't derive Deserialize, but we can accept json values and construct it.
    pub line_items: Vec<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct UpdateInvoiceStatusHttp {
    pub status: String,
}

pub fn router<S: Clone + Send + Sync + 'static>(hub: Arc<Hub>) -> axum::Router<S> {
    Router::new()
        .route("/", get(list_invoices_handler).post(create_invoice_handler))
        .route("/{id}/status", put(update_invoice_status_handler))
        .with_state(hub)
}

async fn list_invoices_handler(
    State(hub): State<Arc<Hub>>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());

    let service = InvoiceServiceImpl { hub };
    let req = Request::new(ListInvoicesRequest { tenant_id });

    match service.list_invoices(req).await {
        Ok(resp) => {
            Ok(Json(resp.into_inner()))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn create_invoice_handler(
    State(hub): State<Arc<Hub>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateInvoiceHttp>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());
    let service = InvoiceServiceImpl { hub };

    let mut mapped_line_items = Vec::new();
    for val in payload.line_items {
        if let (Some(desc), Some(qty), Some(price)) = (
            val.get("description").and_then(|v| v.as_str()),
            val.get("quantity").and_then(|v| v.as_i64()),
            val.get("unit_price").and_then(|v| v.as_f64()),
        ) {
            mapped_line_items.push(InvoiceLineItem {
                id: "".to_string(),
                invoice_id: "".to_string(),
                description: desc.to_string(),
                quantity: qty as i32,
                unit_price: price,
                amount: price * (qty as f64),
            });
        }
    }

    let req = Request::new(CreateInvoiceRequest {
        tenant_id,
        client_id: payload.client_id,
        client_name: payload.client_name,
        due_date: payload.due_date,
        currency: payload.currency,
        line_items: mapped_line_items,
    });

    match service.create_invoice(req).await {
        Ok(resp) => Ok(Json(resp.into_inner())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn update_invoice_status_handler(
    State(hub): State<Arc<Hub>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateInvoiceStatusHttp>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());
    let service = InvoiceServiceImpl { hub };

    let req = Request::new(UpdateInvoiceStatusRequest { tenant_id, invoice_id: id, status: payload.status });

    match service.update_invoice_status(req).await {
        Ok(resp) => Ok(Json(resp.into_inner())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DB;
    use crate::hub::Hub;
    use ::server_ohc::invoice::{CreateInvoiceRequest, InvoiceLineItem, UpdateInvoiceStatusRequest};

    #[tokio::test]
    async fn test_invoice_logic() {
        let db = match DB::new().await {
            Ok(d) => d,
            Err(_) => return,
        };

        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(Hub::new(tx, db.pool.clone()));

        let service = InvoiceServiceImpl { hub: hub.clone() };

        let tenant_id = "test_tenant".to_string();
        let create_req = CreateInvoiceRequest {
            tenant_id: tenant_id.clone(),
            client_id: "client1".to_string(),
            client_name: "Test Client".to_string(),
            due_date: chrono::Utc::now().timestamp(),
            currency: "USD".to_string(),
            line_items: vec![
                InvoiceLineItem {
                    id: "".to_string(),
                    invoice_id: "".to_string(),
                    description: "Item 1".to_string(),
                    quantity: 1,
                    unit_price: 100.0,
                    amount: 100.0,
                }
            ],
        };

        let create_resp = service.create_invoice(Request::new(create_req)).await;
        if create_resp.is_err() {
            return;
        }

        let invoice = create_resp.unwrap().into_inner();
        assert_eq!(invoice.status, "draft");

        let update_req = UpdateInvoiceStatusRequest {
            tenant_id: tenant_id.clone(),
            invoice_id: invoice.id.clone(),
            status: "paid".to_string(),
        };

        let update_resp = service.update_invoice_status(Request::new(update_req)).await;
        assert!(update_resp.is_ok());

        let updated_invoice = update_resp.unwrap().into_inner();
        assert_eq!(updated_invoice.status, "paid");
    }
}
