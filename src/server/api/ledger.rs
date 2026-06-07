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
        .route("/api/ledger/invoice/:id", get(get_invoice))
        .route("/api/ledger/invoice/draft", post(create_invoice_draft))
        .route("/api/ledger/invoice/:id/update", put(update_invoice_status))
        .route("/api/ledger/invoice/:id/pay", post(apply_payment))
        .route("/api/ledger/entries/:tenant_id", get(get_ledger_entries))
        .route("/api/v1/ledger/balance", get(get_balance))
        .route("/api/v1/ledger/statement", get(get_statement))

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


#[derive(Deserialize)]
pub struct GetBalanceApiQuery {
    pub tenant_id: String,
    pub account_id: String,
}

#[derive(Serialize)]
pub struct ApiAccountBalance {
    pub tenant_id: String,
    pub account_id: String,
    pub currency: String,
    pub balance_cents: i64,
}

async fn get_balance(
    State(state): State<AppState>,
    axum::extract::Extension(claims): axum::extract::Extension<::server_common::Claims>,
    axum::extract::Query(query): axum::extract::Query<GetBalanceApiQuery>,
) -> impl IntoResponse {
    let auth_tenant_id = claims.organization_id.unwrap_or(query.tenant_id.clone());
    if auth_tenant_id != query.tenant_id && auth_tenant_id != "SYSTEM" {
        return (StatusCode::FORBIDDEN, "Unauthorized tenant access").into_response();
    }

    let mut tx = match state.db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &query.tenant_id).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    use sqlx::Row;
    let record = match sqlx::query("SELECT balance, currency FROM accounts WHERE tenant_id = $1 AND account_id = $2")
        .bind(&query.tenant_id)
        .bind(&query.account_id)
        .fetch_optional(&mut *tx)
        .await
    {
        Ok(r) => r,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let balance = if let Some(r) = record {
        ApiAccountBalance {
            tenant_id: query.tenant_id.clone(),
            account_id: query.account_id.clone(),
            currency: r.get("currency"),
            balance_cents: r.get("balance"),
        }
    } else {
        ApiAccountBalance {
            tenant_id: query.tenant_id.clone(),
            account_id: query.account_id.clone(),
            currency: "USD".to_string(),
            balance_cents: 0,
        }
    };

    (StatusCode::OK, Json(balance)).into_response()
}

#[derive(Deserialize)]
pub struct GetStatementApiQuery {
    pub tenant_id: String,
    pub account_id: String,
}

#[derive(Serialize)]
pub struct ApiLedgerEntry {
    pub entry_id: String,
    pub tx_id: String,
    pub account_id: String,
    pub direction: String,
    pub amount_cents: i64,
}

#[derive(Serialize)]
pub struct ApiTransaction {
    pub tx_id: String,
    pub amount_cents: i64,
    pub currency: String,
    pub timestamp: i64,
    pub entries: Vec<ApiLedgerEntry>,
}

async fn get_statement(
    State(state): State<AppState>,
    axum::extract::Extension(claims): axum::extract::Extension<::server_common::Claims>,
    axum::extract::Query(query): axum::extract::Query<GetStatementApiQuery>,
) -> impl IntoResponse {
    let auth_tenant_id = claims.organization_id.unwrap_or(query.tenant_id.clone());
    if auth_tenant_id != query.tenant_id && auth_tenant_id != "SYSTEM" {
        return (StatusCode::FORBIDDEN, "Unauthorized tenant access").into_response();
    }

    let mut tx = match state.db.pool.begin().await {
        Ok(tx) => tx,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    if let Err(e) = ::server_common::auth_utils::set_org_context(&mut *tx, &query.tenant_id).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response();
    }

    use sqlx::Row;
    let entries_records = match sqlx::query("SELECT entry_id, tx_id, direction, amount FROM entries WHERE tenant_id = $1 AND account_id = $2")
        .bind(&query.tenant_id)
        .bind(&query.account_id)
        .fetch_all(&mut *tx)
        .await
    {
        Ok(r) => r,
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    };

    let mut tx_ids = std::collections::HashSet::new();
    for e in &entries_records {
        tx_ids.insert(e.get::<String, _>("tx_id"));
    }

    let mut transactions = vec![];

    for tx_id in tx_ids {
        let tx_record = match sqlx::query("SELECT amount, currency, timestamp FROM transactions WHERE tenant_id = $1 AND tx_id = $2")
            .bind(&query.tenant_id)
            .bind(&tx_id)
            .fetch_optional(&mut *tx)
            .await
        {
            Ok(r) => r,
            Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        };

        if let Some(r) = tx_record {
            let tx_entries: Vec<ApiLedgerEntry> = entries_records
                .iter()
                .filter(|e| e.get::<String, _>("tx_id") == tx_id)
                .map(|e| ApiLedgerEntry {
                    entry_id: e.get("entry_id"),
                    tx_id: tx_id.clone(),
                    account_id: query.account_id.clone(),
                    direction: e.get("direction"),
                    amount_cents: e.get("amount"),
                })
                .collect();

            transactions.push(ApiTransaction {
                tx_id: tx_id,
                amount_cents: r.get("amount"),
                currency: r.get("currency"),
                timestamp: r.get::<chrono::DateTime<Utc>, _>("timestamp").timestamp(),
                entries: tx_entries,
            });
        }
    }

    (StatusCode::OK, Json(transactions)).into_response()
}
