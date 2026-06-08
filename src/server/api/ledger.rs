use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

use crate::db::DB;
use crate::domain::repository::models::{Invoice, InvoiceLineItem, PaymentEvent, LedgerEntry};
use crate::domain::repository::ledger_repo::LedgerRepository;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DB>,
}

#[derive(Deserialize)]
pub struct CreateInvoiceDraftRequest {
    pub tenant_id: String,
    pub customer_id: String,
    pub due_date: Option<chrono::DateTime<Utc>>,
    pub items: Vec<CreateInvoiceLineItem>,
    pub tax_nexus: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateInvoiceLineItem {
    pub description: String,
    pub quantity: i32,
    pub unit_price: f64,
}

#[derive(Deserialize)]
pub struct UpdateInvoiceStatusRequest {
    pub tenant_id: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct ApplyPaymentRequest {
    pub tenant_id: String,
    pub invoice_id: String,
    pub amount: f64,
    pub method: String,
}

#[derive(Deserialize)]
pub struct GetInvoiceQuery {
    pub tenant_id: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/ledger/invoice/{id}", get(get_invoice))
        .route("/api/ledger/invoice/draft", post(create_invoice_draft))
        .route("/api/ledger/invoice/{id}/update", put(update_invoice_status))
        .route("/api/ledger/invoice/{id}/pay", post(apply_payment))
        .route("/api/ledger/entries/{tenant_id}", get(get_ledger_entries))
}

async fn get_invoice(
    State(state): State<AppState>,
    Path(id): Path<String>,
    axum::extract::Query(query): axum::extract::Query<GetInvoiceQuery>,
) -> impl IntoResponse {
    let repo = LedgerRepository::new(state.db);
    match repo.get_invoice(&query.tenant_id, &id).await {
        Ok(Some(invoice)) => (StatusCode::OK, Json(invoice)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Invoice not found").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn create_invoice_draft(
    State(state): State<AppState>,
    Json(payload): Json<CreateInvoiceDraftRequest>,
) -> impl IntoResponse {
    let repo = LedgerRepository::new(state.db);

    let invoice_id = Uuid::new_v4().to_string();
    let mut total_amount = 0.0;
    let mut line_items = Vec::new();

    for item in payload.items {
        let amount = item.unit_price * (item.quantity as f64);
        total_amount += amount;
        line_items.push(InvoiceLineItem {
            id: Uuid::new_v4().to_string(),
            tenant_id: payload.tenant_id.clone(),
            invoice_id: invoice_id.clone(),
            description: item.description,
            quantity: Some(item.quantity),
            unit_price: Some(item.unit_price),
            amount: Some(amount),
            created_at: Some(Utc::now()),
        });
    }

    let invoice = Invoice {
        id: invoice_id,
        tenant_id: payload.tenant_id,
        customer_id: payload.customer_id,
        status: Some("Draft".to_string()),
        due_date: payload.due_date,
        total_amount: Some(total_amount),
        currency: Some("USD".to_string()),
        tax_nexus: payload.tax_nexus,
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };

    match repo.create_invoice(invoice.clone(), line_items).await {
        Ok(_) => (StatusCode::CREATED, Json(invoice)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn update_invoice_status(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateInvoiceStatusRequest>,
) -> impl IntoResponse {
    let repo = LedgerRepository::new(state.db);
    match repo.update_invoice_status(&payload.tenant_id, &id, &payload.status).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn apply_payment(
    State(state): State<AppState>,
    Json(payload): Json<ApplyPaymentRequest>,
) -> impl IntoResponse {
    let repo = LedgerRepository::new(state.db);
    let event = PaymentEvent {
        id: Uuid::new_v4().to_string(),
        tenant_id: payload.tenant_id,
        invoice_id: payload.invoice_id,
        amount: payload.amount,
        method: payload.method,
        completed_at: Some(Utc::now()),
        created_at: Some(Utc::now()),
    };

    match repo.apply_payment_event(event).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

async fn get_ledger_entries(
    State(state): State<AppState>,
    Path(tenant_id): Path<String>,
) -> impl IntoResponse {
    let repo = LedgerRepository::new(state.db);
    match repo.get_ledger_entries(&tenant_id).await {
        Ok(entries) => (StatusCode::OK, Json(entries)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e).into_response(),
    }
}

#[derive(Serialize)]
pub struct DashboardLedgerResponse {
    pub balance: f64,
    pub currency: String,
    pub tax_obligations: Vec<TaxObligationDTO>,
    pub statement: Vec<StatementEntryDTO>,
}

#[derive(Serialize)]
pub struct TaxObligationDTO {
    pub jurisdiction: String,
    pub amount: f64,
    pub currency: String,
}

#[derive(Serialize)]
pub struct StatementEntryDTO {
    pub id: String,
    pub date: String,
    pub description: String,
    pub amount: f64,
    pub type_: String, // "CREDIT" or "DEBIT"
}

pub fn ui_router() -> Router<AppState> {
    Router::new()
        .route("/api/ledger", get(get_dashboard_ledger))
}

async fn get_dashboard_ledger(
    State(state): State<AppState>,
    // In a real scenario we'd extract tenant_id from auth context
) -> impl IntoResponse {
    let tenant_id = "default";
    let pool = match &state.db.store {
        crate::db::DbStore::Postgres => &state.db.pool,
        _ => return (StatusCode::NOT_IMPLEMENTED, "PostgreSQL required").into_response()
    };

    // Note: The actual query to fetch balance, obligations and statements from Postgres
    // For now we will return a minimal realistic response

    let balance_query = sqlx::query!("SELECT balance, currency FROM ledger_accounts WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT 1", tenant_id).fetch_optional(pool).await.unwrap_or(None);
    let balance = balance_query.as_ref().map(|b| b.balance).unwrap_or(0.0);
    let currency = balance_query.map(|b| b.currency).unwrap_or_else(|| "USD".to_string());

    let tax_query = sqlx::query!("SELECT jurisdiction, amount, currency FROM tax_obligations WHERE tenant_id = $1 AND status = 'PENDING'", tenant_id).fetch_all(pool).await.unwrap_or(vec![]);
    let tax_obligations = tax_query.into_iter().map(|t| TaxObligationDTO {
        jurisdiction: t.jurisdiction,
        amount: t.amount,
        currency: t.currency,
    }).collect();

    let statement_query = sqlx::query!("SELECT entry_id as id, created_at, account_id, amount, direction as type_ FROM ledger_entries WHERE tenant_id = $1 ORDER BY created_at DESC LIMIT 10", tenant_id).fetch_all(pool).await.unwrap_or(vec![]);
    let statement = statement_query.into_iter().map(|s| StatementEntryDTO {
        id: s.id,
        date: s.created_at.to_rfc3339(),
        description: format!("Account {}", s.account_id),
        amount: s.amount,
        type_: s.type_,
    }).collect();

    let response = DashboardLedgerResponse {
        balance,
        currency,
        tax_obligations,
        statement,
    };


    (StatusCode::OK, Json(response)).into_response()
}
