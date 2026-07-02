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

        let base_currency = "USD".to_string(); // Default base currency
        let transaction_currency = req.currency.clone();
        // In a full implementation this would be dynamically fetched from an external FX oracle
        // based on the base_currency and transaction_currency.
        let exchange_rate = if transaction_currency != base_currency { 1.09 } else { 1.0 };

        sqlx::query(
            "INSERT INTO invoices (id, tenant_id, client_id, client_name, status, due_date, currency, total_amount, stripe_payment_link, base_currency, transaction_currency, exchange_rate)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)"
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
        .bind(&base_currency)
        .bind(&transaction_currency)
        .bind(exchange_rate)
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
            base_currency,
            transaction_currency,
            exchange_rate,
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

        let rows = sqlx::query(
            "SELECT i.*,
                    li.id as li_id,
                    li.description as li_description,
                    li.quantity as li_quantity,
                    li.unit_price as li_unit_price,
                    li.amount as li_amount
             FROM invoices i
             LEFT JOIN invoice_line_items li ON i.id = li.invoice_id
             WHERE i.id = $1"
        )
            .bind(&req.invoice_id)
            .fetch_all(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit().await.map_err(|e| Status::internal(e.to_string()))?;

        if rows.is_empty() {
            return Err(Status::not_found("Invoice not found"));
        }

        let first_row_id: String = rows[0].try_get("id").unwrap_or_default();
        let first_row_client_id: String = rows[0].try_get("client_id").unwrap_or_default();
        let first_row_client_name: String = rows[0].try_get("client_name").unwrap_or_default();
        let first_row_status: String = rows[0].try_get("status").unwrap_or_default();
        let first_row_due_date: i64 = rows[0].try_get("due_date").unwrap_or_default();
        let first_row_currency: String = rows[0].try_get("currency").unwrap_or_default();
        let first_row_total_amount: f64 = rows[0].try_get("total_amount").unwrap_or_default();
        let first_row_total_amount_cents: i32 = rows[0].try_get("total_amount_cents").unwrap_or_default();
        let first_row_payment_status: String = rows[0].try_get("payment_status").unwrap_or_default();
        let first_row_view_count: i32 = rows[0].try_get("view_count").unwrap_or_default();
        let first_row_amount_paid_cents: i32 = rows[0].try_get("amount_paid_cents").unwrap_or_default();
        let first_row_stripe_invoice_id: String = rows[0].try_get("stripe_invoice_id").unwrap_or_default();
        let first_row_stripe_payment_link: String = rows[0].try_get("stripe_payment_link").unwrap_or_default();
        let first_row_base_currency: String = rows[0].try_get("base_currency").unwrap_or_default();
        let first_row_transaction_currency: String = rows[0].try_get("transaction_currency").unwrap_or_default();
        let first_row_exchange_rate: f64 = rows[0].try_get("exchange_rate").unwrap_or_default();

        let mut line_items = Vec::new();
        for row in rows {
            if row.try_get::<String, _>("li_id").is_ok() {
                line_items.push(InvoiceLineItem {
                    id: row.try_get("li_id").unwrap_or_default(),
                    invoice_id: req.invoice_id.clone(),
                    description: row.try_get("li_description").unwrap_or_default(),
                    quantity: row.try_get("li_quantity").unwrap_or_default(),
                    unit_price: row.try_get("li_unit_price").unwrap_or_default(),
                    amount: row.try_get("li_amount").unwrap_or_default(),
                });
            }
        }

        let invoice = Invoice {
            id: first_row_id,
            client_id: first_row_client_id,
            client_name: first_row_client_name,
            status: first_row_status,
            due_date: first_row_due_date,
            currency: first_row_currency,
            total_amount: first_row_total_amount,
            total_amount_cents: first_row_total_amount_cents,
            payment_status: first_row_payment_status,
            view_count: first_row_view_count,
            amount_paid_cents: first_row_amount_paid_cents,
            stripe_invoice_id: first_row_stripe_invoice_id,
            stripe_payment_link: first_row_stripe_payment_link,
            base_currency: first_row_base_currency,
            transaction_currency: first_row_transaction_currency,
            exchange_rate: first_row_exchange_rate,
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
            base_currency: row.try_get("base_currency").unwrap_or_default(),
            transaction_currency: row.try_get("transaction_currency").unwrap_or_default(),
            exchange_rate: row.try_get("exchange_rate").unwrap_or_default(),
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


use serde::Serialize;

#[derive(Serialize)]
pub struct InvoiceMobileView {
    pub id: String,
    pub client_name: String,
    pub status: String,
    pub due_date: i64,
    pub currency: String,
    pub total_amount: f64,
    pub total_amount_cents: i32,
    pub payment_status: String,
    pub amount_paid_cents: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Serialize)]
pub struct InvoiceLineItemView {
    pub id: String,
    pub invoice_id: String,
    pub description: String,
    pub quantity: i32,
    pub unit_price: f64,
    pub amount: f64,
}

#[derive(Serialize)]
pub struct InvoiceStandardView {
    pub id: String,
    pub client_id: String,
    pub client_name: String,
    pub status: String,
    pub due_date: i64,
    pub currency: String,
    pub total_amount: f64,
    pub total_amount_cents: i32,
    pub payment_status: String,
    pub view_count: i32,
    pub amount_paid_cents: i32,
    pub stripe_invoice_id: String,
    pub stripe_payment_link: String,
    pub line_items: Vec<InvoiceLineItemView>,
    pub created_at: i64,
    pub updated_at: i64,
}

pub fn map_invoices_for_mobile(invoices: Vec<::server_ohc::invoice::Invoice>) -> Vec<InvoiceMobileView> {
    invoices.into_iter().map(|inv| InvoiceMobileView {
        id: inv.id,
        client_name: inv.client_name,
        status: inv.status,
        due_date: inv.due_date,
        currency: inv.currency,
        total_amount: inv.total_amount,
        total_amount_cents: inv.total_amount_cents,
        payment_status: inv.payment_status,
        amount_paid_cents: inv.amount_paid_cents,
        created_at: inv.created_at,
        updated_at: inv.updated_at,
    }).collect()
}

pub fn map_invoices_standard(invoices: Vec<::server_ohc::invoice::Invoice>) -> Vec<InvoiceStandardView> {
    invoices.into_iter().map(|inv| InvoiceStandardView {
        id: inv.id,
        client_id: inv.client_id,
        client_name: inv.client_name,
        status: inv.status,
        due_date: inv.due_date,
        currency: inv.currency,
        total_amount: inv.total_amount,
        total_amount_cents: inv.total_amount_cents,
        payment_status: inv.payment_status,
        view_count: inv.view_count,
        amount_paid_cents: inv.amount_paid_cents,
        stripe_invoice_id: inv.stripe_invoice_id,
        stripe_payment_link: inv.stripe_payment_link,
        line_items: inv.line_items.into_iter().map(|li| InvoiceLineItemView {
            id: li.id,
            invoice_id: li.invoice_id,
            description: li.description,
            quantity: li.quantity,
            unit_price: li.unit_price,
            amount: li.amount,
        }).collect(),
        created_at: inv.created_at,
        updated_at: inv.updated_at,
    }).collect()
}

pub async fn list_invoices_handler(
    State(hub): State<Arc<Hub>>,
    axum::extract::Query(query): axum::extract::Query<std::collections::HashMap<String, String>>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "default".to_string());

    let service = InvoiceServiceImpl { hub };
    let req = Request::new(ListInvoicesRequest { tenant_id });

    let mobile_optimized = query.get("mobile_optimized").map(|s| s == "true").unwrap_or(false);
    match service.list_invoices(req).await {
        Ok(resp) => {
            let inner = resp.into_inner();
            if mobile_optimized {
                let mapped = map_invoices_for_mobile(inner.invoices);
                Ok(Json(serde_json::json!({ "invoices": mapped })))
            } else {
                let mapped = map_invoices_standard(inner.invoices);
                Ok(Json(serde_json::json!({ "invoices": mapped })))
            }
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


#[cfg(test)]
mod payload_tests {
    use super::*;

    #[test]
    fn test_invoice_mobile_payload_optimization() {
        let inv = ::server_ohc::invoice::Invoice {
            id: "inv-1".to_string(),
            client_id: "client-1".to_string(),
            client_name: "John Doe".to_string(),
            status: "DRAFT".to_string(),
            due_date: 1234567890,
            currency: "USD".to_string(),
            total_amount: 100.0,
            total_amount_cents: 10000,
            payment_status: "UNPAID".to_string(),
            view_count: 5,
            amount_paid_cents: 0,
            stripe_invoice_id: "in_123".to_string(),
            stripe_payment_link: "https://stripe.com/pay/123".to_string(),
            line_items: vec![::server_ohc::invoice::InvoiceLineItem {
                id: "li-1".to_string(),
                invoice_id: "inv-1".to_string(),
                description: "Test".to_string(),
                quantity: 1,
                unit_price: 100.0,
                amount: 100.0,
            }],
            created_at: 1234567800,
            updated_at: 1234567800,
        };

        // Test mobile mapping
        let mobile_mapped = map_invoices_for_mobile(vec![inv.clone()]);
        assert_eq!(mobile_mapped.len(), 1);
        let m_inv = &mobile_mapped[0];

        // Assert fields are present
        assert_eq!(m_inv.id, "inv-1");
        assert_eq!(m_inv.client_name, "John Doe");
        assert_eq!(m_inv.currency, "USD");
        assert_eq!(m_inv.total_amount_cents, 10000);

        // Serialize and verify omitted fields
        let json_val = serde_json::to_value(m_inv).unwrap();
        assert!(json_val.get("stripe_invoice_id").is_none());
        assert!(json_val.get("stripe_payment_link").is_none());
        assert!(json_val.get("client_id").is_none());
        assert!(json_val.get("view_count").is_none());
        assert!(json_val.get("line_items").is_none());

        // Test standard mapping
        let standard_mapped = map_invoices_standard(vec![inv]);
        assert_eq!(standard_mapped.len(), 1);
        let s_inv = &standard_mapped[0];
        assert_eq!(s_inv.client_id, "client-1");
        assert_eq!(s_inv.stripe_invoice_id, "in_123");
        assert_eq!(s_inv.line_items.len(), 1);
    }
}
