use std::sync::Arc;

use ::server_common::Claims;
use ::server_omnisolo::invoice::invoice_service_server::InvoiceService;
use ::server_omnisolo::invoice::*;
use axum::{
    Json, Router,
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
};
use serde::Deserialize;
use tonic::{Request, Response, Status};

use crate::hub::Hub;
use crate::rpc_error::RpcError;

pub struct InvoiceServiceImpl {
    pub hub: Arc<Hub>,
}

// PostgreSQL stores timestamps; protobuf exposes Unix seconds. A decoding error
// must not silently turn a real due date into 1970 or manufacture a current date.
fn invoice_timestamp(row: &sqlx::postgres::PgRow, column: &str) -> Result<i64, RpcError> {
    use sqlx::Row;
    row.try_get::<Option<chrono::DateTime<chrono::Utc>>, _>(column)
        .map(|value| value.map_or(0, |date| date.timestamp()))
        .map_err(|_| {
            RpcError::internal("Invoice timestamp is incompatible with the runtime schema")
        })
}

fn invoice_tenant(claims: &Claims) -> Result<String, StatusCode> {
    claims
        .organization_id
        .as_deref()
        .map(str::trim)
        .filter(|tenant| !tenant.is_empty())
        .map(str::to_owned)
        .ok_or(StatusCode::UNAUTHORIZED)
}

fn require_invoice_owner(claims: &Claims) -> Result<(), StatusCode> {
    if claims
        .roles
        .iter()
        .any(|role| role.eq_ignore_ascii_case("owner") || role.eq_ignore_ascii_case("admin"))
    {
        Ok(())
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

fn invoice_http_error(error: Status) -> StatusCode {
    match error.code() {
        tonic::Code::InvalidArgument => StatusCode::UNPROCESSABLE_ENTITY,
        tonic::Code::PermissionDenied => StatusCode::FORBIDDEN,
        tonic::Code::Unauthenticated => StatusCode::UNAUTHORIZED,
        tonic::Code::NotFound => StatusCode::NOT_FOUND,
        tonic::Code::FailedPrecondition => StatusCode::CONFLICT,
        tonic::Code::Unimplemented => StatusCode::NOT_IMPLEMENTED,
        _ => StatusCode::SERVICE_UNAVAILABLE,
    }
}

fn validate_manual_invoice_status(status: &str) -> Result<(), RpcError> {
    // This endpoint manages unpaid local drafts, never payment or delivery.
    if matches!(status, "draft" | "void") {
        Ok(())
    } else {
        Err(RpcError::failed_precondition(
            "Payment and delivery status require verified evidence",
        ))
    }
}

fn validate_invoice_items(items: &[InvoiceLineItem]) -> Result<i32, RpcError> {
    if items.is_empty() || items.len() > 100 {
        return Err(RpcError::invalid_argument(
            "One to 100 invoice line items are required",
        ));
    }
    let mut total = 0_i64;
    for item in items {
        if item.description.trim().is_empty()
            || item.description.len() > 4000
            || item.quantity <= 0
            || !item.unit_price.is_finite()
            || item.unit_price < 0.0
            || item.unit_price > 999_999.99
            || !item.amount.is_finite()
            || item.amount < 0.0
        {
            return Err(RpcError::invalid_argument("Invalid invoice line item"));
        }
        let unit = (item.unit_price * 100.0).round();
        if (item.unit_price * 100.0 - unit).abs() > 0.00001 {
            return Err(RpcError::invalid_argument(
                "Unit price must use currency minor units",
            ));
        }
        let amount = (unit as i64)
            .checked_mul(i64::from(item.quantity))
            .ok_or_else(|| RpcError::invalid_argument("Invoice amount overflow"))?;
        if (item.amount * 100.0 - amount as f64).abs() > 0.00001 {
            return Err(RpcError::invalid_argument(
                "Line amount must equal quantity times unit price",
            ));
        }
        total = total
            .checked_add(amount)
            .filter(|value| *value <= 99_999_999)
            .ok_or_else(|| RpcError::invalid_argument("Invoice total exceeds supported amount"))?;
    }
    i32::try_from(total).map_err(|_| RpcError::invalid_argument("Invoice total overflow"))
}

#[tonic::async_trait]
impl InvoiceService for InvoiceServiceImpl {
    async fn create_invoice(
        &self,
        request: Request<CreateInvoiceRequest>,
    ) -> Result<Response<Invoice>, Status> {
        let req = request.into_inner();
        if req.tenant_id.trim().is_empty()
            || req.client_id.trim().is_empty()
            || req.client_id.len() > 255
            || req.client_name.trim().is_empty()
            || req.client_name.len() > 500
            || req.due_date <= 0
        {
            return Err(Status::invalid_argument(
                "Valid tenant, client and due date are required",
            ));
        }
        if !matches!(
            req.currency.as_str(),
            "USD" | "EUR" | "GBP" | "CAD" | "AUD" | "NZD" | "CHF" | "SGD" | "HKD"
        ) || (!req.base_currency.is_empty() && req.base_currency != req.currency)
            || (!req.transaction_currency.is_empty() && req.transaction_currency != req.currency)
            || !req.exchange_rate.is_finite()
            || !(req.exchange_rate == 0.0 || req.exchange_rate == 1.0)
        {
            return Err(Status::invalid_argument(
                "Use a supported two-decimal currency without unverified conversion",
            ));
        }
        let total_cents = validate_invoice_items(&req.line_items)?;
        let due_date = chrono::DateTime::from_timestamp(req.due_date, 0)
            .ok_or_else(|| Status::invalid_argument("Invoice due date is out of range"))?;
        let pool = &self.hub.pool;
        let mut tx = pool
            .begin()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // Set tenant context for RLS
        ::server_common::auth_utils::set_org_context(&mut *tx, &req.tenant_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let invoice_id = uuid::Uuid::new_v4().to_string();
        let total_amount = f64::from(total_cents) / 100.0;

        let status = "draft".to_string();

        let mut stripe_payment_link = String::new();
        let mut stripe_invoice_id = String::new();

        if total_cents > 0 {
            let db = crate::db::DB {
                pool: self.hub.pool.clone(),
                store: crate::db::DbStore::Postgres,
            };
            if let Ok(stripe_key) =
                crate::api::tool_integrations::stripe_key_for_tenant(&db, &req.tenant_id).await
            {
                let stripe_client =
                    crate::integrations::stripe::client::StripeClient::new(stripe_key);
                if stripe_client.require_api_key().is_ok() {
                    use sha2::{Digest, Sha256};
                    let operation_id = format!(
                        "invoice:{:x}",
                        Sha256::digest(format!("{}:{}", req.tenant_id, invoice_id))
                    );
                    if let Ok(receipt) = stripe_client
                        .create_checkout_session_idempotent(
                            crate::integrations::stripe::safe_checkout::CheckoutRequest {
                                name: &format!("Invoice for {}", req.client_name),
                                reference: &invoice_id,
                                amount_cents: total_cents as i64,
                                interval: None,
                                product: None,
                                currency: &req.currency,
                                operation_id: &operation_id,
                            },
                        )
                        .await
                    {
                        stripe_payment_link = receipt.url;
                        stripe_invoice_id = receipt.id;
                    }
                }
            }
        }

        let base_currency = if req.base_currency.is_empty() {
            "USD".to_string()
        } else {
            req.base_currency.clone()
        };
        let transaction_currency = if req.transaction_currency.is_empty() {
            req.currency.clone()
        } else {
            req.transaction_currency.clone()
        };
        let exchange_rate = if req.exchange_rate == 0.0 {
            1.0
        } else {
            req.exchange_rate
        };

        sqlx::query(
            "INSERT INTO invoices (id, tenant_id, client_id, client_name, status, due_date, currency, base_currency, transaction_currency, exchange_rate, total_amount, stripe_payment_link, total_amount_cents, amount_paid_cents, payment_status, stripe_invoice_id)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, 0, 'draft', $14)"
        )
        .bind(&invoice_id)
        .bind(&req.tenant_id)
        .bind(&req.client_id)
        .bind(&req.client_name)
        .bind(&status)
        .bind(due_date)
        .bind(&req.currency)
        .bind(&base_currency)
        .bind(&transaction_currency)
        .bind(exchange_rate)
        .bind(total_amount)
        .bind(&stripe_payment_link)
        .bind(total_cents)
        .bind(&stripe_invoice_id)
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

        tx.commit()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(Invoice {
            id: invoice_id,
            client_id: req.client_id,
            client_name: req.client_name,
            status,
            due_date: req.due_date,
            currency: req.currency,
            base_currency,
            transaction_currency,
            exchange_rate,
            total_amount,
            total_amount_cents: total_cents,
            payment_status: "draft".to_string(),
            view_count: 0,
            amount_paid_cents: 0,
            stripe_invoice_id,
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
        let mut tx = pool
            .begin()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // Set tenant context for RLS
        ::server_common::auth_utils::set_org_context(&mut *tx, &req.tenant_id)
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
             LEFT JOIN invoice_line_items li ON i.id = li.invoice_id AND i.tenant_id = li.tenant_id
             WHERE i.id = $1 AND i.tenant_id = $2",
        )
        .bind(&req.invoice_id)
        .bind(&req.tenant_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        if rows.is_empty() {
            return Err(Status::not_found("Invoice not found"));
        }

        let first_row_id: String = rows[0].try_get("id").unwrap_or_default();
        let first_row_client_id: String = rows[0].try_get("client_id").unwrap_or_default();
        let first_row_client_name: String = rows[0].try_get("client_name").unwrap_or_default();
        let first_row_status: String = rows[0].try_get("status").unwrap_or_default();
        let first_row_due_date = invoice_timestamp(&rows[0], "due_date")?;
        let created_at = invoice_timestamp(&rows[0], "created_at")?;
        let updated_at = invoice_timestamp(&rows[0], "updated_at")?;
        let first_row_currency: String = rows[0].try_get("currency").unwrap_or_default();
        let first_row_base_currency: String = rows[0].try_get("base_currency").unwrap_or_default();
        let first_row_transaction_currency: String =
            rows[0].try_get("transaction_currency").unwrap_or_default();
        let first_row_exchange_rate: f64 = rows[0].try_get("exchange_rate").unwrap_or_default();
        let first_row_total_amount: f64 = rows[0].try_get("total_amount").unwrap_or_default();
        let first_row_total_amount_cents: i32 =
            rows[0].try_get("total_amount_cents").unwrap_or_default();
        let first_row_payment_status: String =
            rows[0].try_get("payment_status").unwrap_or_default();
        let first_row_view_count: i32 = rows[0].try_get("view_count").unwrap_or_default();
        let first_row_amount_paid_cents: i32 =
            rows[0].try_get("amount_paid_cents").unwrap_or_default();
        let first_row_stripe_invoice_id: String =
            rows[0].try_get("stripe_invoice_id").unwrap_or_default();
        let first_row_stripe_payment_link: String =
            rows[0].try_get("stripe_payment_link").unwrap_or_default();

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
            base_currency: first_row_base_currency,
            transaction_currency: first_row_transaction_currency,
            exchange_rate: first_row_exchange_rate,
            total_amount: first_row_total_amount,
            total_amount_cents: first_row_total_amount_cents,
            payment_status: first_row_payment_status,
            view_count: first_row_view_count,
            amount_paid_cents: first_row_amount_paid_cents,
            stripe_invoice_id: first_row_stripe_invoice_id,
            stripe_payment_link: first_row_stripe_payment_link,
            line_items,
            created_at,
            updated_at,
        };

        Ok(Response::new(invoice))
    }

    async fn list_invoices(
        &self,
        request: Request<ListInvoicesRequest>,
    ) -> Result<Response<ListInvoicesResponse>, Status> {
        let req = request.into_inner();

        let pool = &self.hub.pool;
        let mut tx = pool
            .begin()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // Set tenant context for RLS
        ::server_common::auth_utils::set_org_context(&mut *tx, &req.tenant_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        use sqlx::Row;

        let rows =
            sqlx::query("SELECT * FROM invoices WHERE tenant_id = $1 ORDER BY created_at DESC")
                .bind(&req.tenant_id)
                .fetch_all(&mut *tx)
                .await
                .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let mut invoices = Vec::new();
        for row in rows {
            invoices.push(Invoice {
                id: row.try_get("id").unwrap_or_default(),
                client_id: row.try_get("client_id").unwrap_or_default(),
                client_name: row.try_get("client_name").unwrap_or_default(),
                status: row.try_get("status").unwrap_or_default(),
                due_date: invoice_timestamp(&row, "due_date")?,
                currency: row.try_get("currency").unwrap_or_default(),
                base_currency: row.try_get("base_currency").unwrap_or_default(),
                transaction_currency: row.try_get("transaction_currency").unwrap_or_default(),
                exchange_rate: row.try_get("exchange_rate").unwrap_or_default(),
                total_amount: row.try_get("total_amount").unwrap_or_default(),
                total_amount_cents: row.try_get("total_amount_cents").unwrap_or_default(),
                payment_status: row.try_get("payment_status").unwrap_or_default(),
                view_count: row.try_get("view_count").unwrap_or_default(),
                amount_paid_cents: row.try_get("amount_paid_cents").unwrap_or_default(),
                stripe_invoice_id: row.try_get("stripe_invoice_id").unwrap_or_default(),
                stripe_payment_link: row.try_get("stripe_payment_link").unwrap_or_default(),
                line_items: vec![],
                created_at: invoice_timestamp(&row, "created_at")?,
                updated_at: invoice_timestamp(&row, "updated_at")?,
            });
        }

        Ok(Response::new(ListInvoicesResponse { invoices }))
    }

    async fn update_invoice_status(
        &self,
        request: Request<UpdateInvoiceStatusRequest>,
    ) -> Result<Response<Invoice>, Status> {
        let req = request.into_inner();
        validate_manual_invoice_status(&req.status)?;

        let pool = &self.hub.pool;
        let mut tx = pool
            .begin()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        // Set tenant context for RLS
        ::server_common::auth_utils::set_org_context(&mut *tx, &req.tenant_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let changed = sqlx::query(
            "UPDATE invoices SET status = $1, updated_at = NOW() WHERE id = $2 AND tenant_id = $3 AND status = 'draft' AND COALESCE(amount_paid_cents, 0) = 0",
        )
        .bind(&req.status).bind(&req.invoice_id).bind(&req.tenant_id)
        .execute(&mut *tx).await.map_err(|_| Status::unavailable("Invoice update unavailable"))?
        .rows_affected();
        if changed != 1 {
            return Err(Status::failed_precondition(
                "Only an existing unpaid draft can be changed",
            ));
        }

        use sqlx::Row;

        let row = sqlx::query("SELECT * FROM invoices WHERE id = $1 AND tenant_id = $2")
            .bind(&req.invoice_id)
            .bind(&req.tenant_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        let items_rows = sqlx::query(
            "SELECT * FROM invoice_line_items WHERE invoice_id = $1 AND tenant_id = $2",
        )
        .bind(&req.invoice_id)
        .bind(&req.tenant_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(|e| Status::internal(e.to_string()))?;

        tx.commit()
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

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
            due_date: invoice_timestamp(&row, "due_date")?,
            currency: row.try_get("currency").unwrap_or_default(),
            base_currency: row.try_get("base_currency").unwrap_or_default(),
            transaction_currency: row.try_get("transaction_currency").unwrap_or_default(),
            exchange_rate: row.try_get("exchange_rate").unwrap_or_default(),
            total_amount: row.try_get("total_amount").unwrap_or_default(),
            total_amount_cents: row.try_get("total_amount_cents").unwrap_or_default(),
            payment_status: row.try_get("payment_status").unwrap_or_default(),
            view_count: row.try_get("view_count").unwrap_or_default(),
            amount_paid_cents: row.try_get("amount_paid_cents").unwrap_or_default(),
            stripe_invoice_id: row.try_get("stripe_invoice_id").unwrap_or_default(),
            stripe_payment_link: row.try_get("stripe_payment_link").unwrap_or_default(),
            line_items,
            created_at: invoice_timestamp(&row, "created_at")?,
            updated_at: invoice_timestamp(&row, "updated_at")?,
        };

        Ok(Response::new(invoice))
    }

    async fn draft_invoice_from_context(
        &self,
        _request: Request<DraftInvoiceFromContextRequest>,
    ) -> Result<Response<DraftInvoiceFromContextResponse>, Status> {
        // Free-form context cannot authorize fabricated hours, rates or an FX
        // conversion. The supported creation path requires explicit line items.
        Err(Status::failed_precondition(
            "Provide owner-approved line items through invoice creation; no invoice was generated from unverified context",
        ))
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateInvoiceHttp {
    pub client_id: String,
    pub client_name: String,
    pub due_date: i64,
    pub currency: String,
    pub line_items: Vec<CreateInvoiceLineHttp>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateInvoiceLineHttp {
    pub description: String,
    pub quantity: i32,
    pub unit_price: f64,
}

fn http_invoice_items(
    items: Vec<CreateInvoiceLineHttp>,
) -> Result<Vec<InvoiceLineItem>, StatusCode> {
    let mapped: Vec<_> = items
        .into_iter()
        .map(|item| InvoiceLineItem {
            id: String::new(),
            invoice_id: String::new(),
            description: item.description,
            quantity: item.quantity,
            unit_price: item.unit_price,
            amount: item.unit_price * f64::from(item.quantity),
        })
        .collect();
    validate_invoice_items(&mapped).map_err(|error| invoice_http_error(error.into()))?;
    Ok(mapped)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateInvoiceStatusHttp {
    pub status: String,
}

#[derive(Deserialize)]
pub struct GenerateInvoiceHttp {
    pub job_id: String,
    pub customer_id: String,
}

async fn generate_invoice_handler(
    State(_hub): State<Arc<Hub>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<GenerateInvoiceHttp>,
) -> Result<impl IntoResponse, StatusCode> {
    invoice_tenant(&claims)?;
    require_invoice_owner(&claims)?;
    if payload.job_id.trim().is_empty()
        || payload.customer_id.trim().is_empty()
        || payload.job_id.len() > 255
        || payload.customer_id.len() > 255
    {
        return Err(StatusCode::UNPROCESSABLE_ENTITY);
    }
    // Keep an offline job pending until actual authorized billable work exists.
    // Acknowledging it as invoiced would lose the client's unsynchronized work.
    Ok((
        StatusCode::NOT_IMPLEMENTED,
        Json(serde_json::json!({
            "success": false, "code": "approved_line_items_required",
            "message": "Create an invoice with approved line items; no invoice was generated for this job."
        })),
    ))
}

pub fn router<S: Clone + Send + Sync + 'static>(hub: Arc<Hub>) -> axum::Router<S> {
    Router::new()
        .route("/", get(list_invoices_handler).post(create_invoice_handler))
        .route("/generate", post(generate_invoice_handler))
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

pub fn map_invoices_for_mobile(
    invoices: Vec<::server_omnisolo::invoice::Invoice>,
) -> Vec<InvoiceMobileView> {
    invoices
        .into_iter()
        .map(|inv| InvoiceMobileView {
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
        })
        .collect()
}

pub fn map_invoices_standard(
    invoices: Vec<::server_omnisolo::invoice::Invoice>,
) -> Vec<InvoiceStandardView> {
    invoices
        .into_iter()
        .map(|inv| InvoiceStandardView {
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
            line_items: inv
                .line_items
                .into_iter()
                .map(|li| InvoiceLineItemView {
                    id: li.id,
                    invoice_id: li.invoice_id,
                    description: li.description,
                    quantity: li.quantity,
                    unit_price: li.unit_price,
                    amount: li.amount,
                })
                .collect(),
            created_at: inv.created_at,
            updated_at: inv.updated_at,
        })
        .collect()
}

pub async fn list_invoices_handler(
    State(_hub): State<Arc<Hub>>,
    axum::extract::Query(query): axum::extract::Query<std::collections::HashMap<String, String>>,
    Extension(claims): Extension<Claims>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = invoice_tenant(&claims)?;

    let service = InvoiceServiceImpl { hub: _hub };
    let req = Request::new(ListInvoicesRequest { tenant_id });

    let mobile_optimized = query
        .get("mobile_optimized")
        .map(|s| s == "true")
        .unwrap_or(false);
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
    State(_hub): State<Arc<Hub>>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateInvoiceHttp>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = invoice_tenant(&claims)?;
    require_invoice_owner(&claims)?;
    let service = InvoiceServiceImpl { hub: _hub };

    let mapped_line_items = http_invoice_items(payload.line_items)?;

    let req = Request::new(CreateInvoiceRequest {
        tenant_id,
        client_id: payload.client_id,
        client_name: payload.client_name,
        due_date: payload.due_date,
        currency: payload.currency.clone(),
        base_currency: payload.currency.clone(),
        transaction_currency: payload.currency,
        exchange_rate: 1.0,
        line_items: mapped_line_items,
    });

    service
        .create_invoice(req)
        .await
        .map(|resp| Json(resp.into_inner()))
        .map_err(invoice_http_error)
}

async fn update_invoice_status_handler(
    State(_hub): State<Arc<Hub>>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateInvoiceStatusHttp>,
) -> Result<impl IntoResponse, StatusCode> {
    let tenant_id = invoice_tenant(&claims)?;
    require_invoice_owner(&claims)?;
    let service = InvoiceServiceImpl { hub: _hub };

    let req = Request::new(UpdateInvoiceStatusRequest {
        tenant_id,
        invoice_id: id,
        status: payload.status,
    });

    service
        .update_invoice_status(req)
        .await
        .map(|resp| Json(resp.into_inner()))
        .map_err(invoice_http_error)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invoice_rejects_invalid_and_inconsistent_amounts() {
        let mut item = InvoiceLineItem {
            id: String::new(),
            invoice_id: String::new(),
            description: "Service".into(),
            quantity: 3,
            unit_price: 10.25,
            amount: 30.75,
        };
        assert_eq!(validate_invoice_items(&[item.clone()]).unwrap(), 3075);
        item.amount = 10.25;
        assert!(validate_invoice_items(&[item.clone()]).is_err());
        item.amount = f64::NAN;
        assert!(validate_invoice_items(&[item.clone()]).is_err());
        item.unit_price = -1.0;
        assert!(validate_invoice_items(&[item]).is_err());
        assert!(validate_invoice_items(&[]).is_err());
    }
    use crate::hub::Hub;
    use ::server_omnisolo::invoice::{InvoiceLineItem, UpdateInvoiceStatusRequest};

    #[tokio::test]
    async fn test_invoice_logic_rejects_fabricated_payment_before_database_access() {
        // No hidden early-success returns on missing databases. Persistence is
        // exercised separately by native_business_regression on the real stack.
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://unused:unused@127.0.0.1:1/unused")
            .unwrap();
        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        let service = InvoiceServiceImpl {
            hub: Arc::new(Hub::new(tx, pool)),
        };
        for status in ["paid", "sent", "refunded", "partially_paid", "unknown"] {
            let error = service
                .update_invoice_status(Request::new(UpdateInvoiceStatusRequest {
                    tenant_id: "tenant-a".into(),
                    invoice_id: "invoice-a".into(),
                    status: status.into(),
                }))
                .await
                .unwrap_err();
            assert_eq!(error.code(), tonic::Code::FailedPrecondition);
        }
        let error = service
            .draft_invoice_from_context(Request::new(DraftInvoiceFromContextRequest::default()))
            .await
            .unwrap_err();
        assert_eq!(error.code(), tonic::Code::FailedPrecondition);
    }

    #[test]
    fn invoice_http_payload_rejects_wrapped_quantity_and_malformed_items() {
        for json in [
            r#"{"description":"x","quantity":4294967297,"unit_price":1}"#,
            r#"{"description":"x","quantity":1,"unit_price":1,"tenant_id":"other"}"#,
            r#"{"description":"x","quantity":1}"#,
        ] {
            assert!(serde_json::from_str::<CreateInvoiceLineHttp>(json).is_err());
        }
        assert!(
            http_invoice_items(vec![CreateInvoiceLineHttp {
                description: "Item".into(),
                quantity: -1,
                unit_price: 10.0,
            }])
            .is_err()
        );
        let items = http_invoice_items(vec![CreateInvoiceLineHttp {
            description: "Item".into(),
            quantity: 2,
            unit_price: 12.34,
        }])
        .unwrap();
        assert_eq!(validate_invoice_items(&items).unwrap(), 2468);
        assert!(validate_manual_invoice_status("draft").is_ok());
        assert!(validate_manual_invoice_status("void").is_ok());
    }
}

#[cfg(test)]
mod payload_tests {
    use super::*;

    #[test]
    fn test_invoice_mobile_payload_optimization() {
        let inv = ::server_omnisolo::invoice::Invoice {
            id: "inv-1".to_string(),
            client_id: "client-1".to_string(),
            client_name: "John Doe".to_string(),
            status: "DRAFT".to_string(),
            due_date: 1234567890,
            currency: "USD".to_string(),
            base_currency: "USD".to_string(),
            transaction_currency: "USD".to_string(),
            exchange_rate: 1.0,
            total_amount: 100.0,
            total_amount_cents: 10000,
            payment_status: "UNPAID".to_string(),
            view_count: 5,
            amount_paid_cents: 0,
            stripe_invoice_id: "in_123".to_string(),
            stripe_payment_link: "https://stripe.com/pay/123".to_string(),
            line_items: vec![::server_omnisolo::invoice::InvoiceLineItem {
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
