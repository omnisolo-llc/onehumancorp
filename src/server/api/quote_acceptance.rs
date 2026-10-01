//! Quote acceptance is a committed local receipt, not evidence of payment.
use super::{QUOTE_COLUMNS, QUOTE_LINE_ITEM_COLUMNS, Quote, QuoteLineItem};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use sqlx::{PgConnection, PgPool};

#[derive(Debug)]
pub(super) enum Error {
    NotFound,
    Conflict(&'static str),
    Database(sqlx::Error),
    Commit(sqlx::Error),
}
impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self::Database(value)
    }
}
impl Error {
    pub(super) fn response(self) -> axum::response::Response {
        use axum::response::IntoResponse;
        let (status, reason) = match self {
            Self::NotFound => (axum::http::StatusCode::NOT_FOUND, "quote_not_found"),
            Self::Conflict(reason) => (axum::http::StatusCode::CONFLICT, reason),
            Self::Commit(error) => {
                tracing::error!(%error, "Quote acceptance commit outcome requires reconciliation");
                (
                    axum::http::StatusCode::SERVICE_UNAVAILABLE,
                    "commit_outcome_unknown",
                )
            }
            Self::Database(error) => {
                tracing::error!(%error, "Quote acceptance database operation failed; prior phases may be committed");
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "database_operation_failed",
                )
            }
        };
        (
            status,
            axum::Json(json!({"success":false,"status":"reconciliation","reason":reason})),
        )
            .into_response()
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct Receipt {
    version: u32,
    tenant_id: String,
    quote_id: String,
    accepted_by: String,
    invoice_id: String,
    operation_id: String,
    terms: Value,
    checkout_status: String,
    reason: Option<String>,
    stripe_payment_link: Option<String>,
    checkout_id: Option<String>,
    invoice_status: String,
    payment_status: String,
}
impl Receipt {
    pub(super) fn public(&self) -> Value {
        json!({"success":true,"status":"accepted","quote_id":self.quote_id,
            "invoice_id":self.invoice_id,"invoice_status":self.invoice_status,"payment_status":self.payment_status,
            "stripe_payment_link":self.stripe_payment_link.as_deref().unwrap_or(""),
            "checkout_status":self.checkout_status,"reason":self.reason})
    }
}
#[async_trait::async_trait]
pub(super) trait Checkout: Sync {
    fn configured(&self) -> bool;
    async fn create(
        &self,
        quote_id: &str,
        amount: i64,
        operation: &str,
    ) -> Result<crate::integrations::stripe::safe_checkout::CheckoutReceipt, String>;
}
pub(super) struct StripeCheckout(pub crate::integrations::stripe::client::StripeClient);
#[async_trait::async_trait]
impl Checkout for StripeCheckout {
    fn configured(&self) -> bool {
        self.0.require_api_key().is_ok()
    }
    async fn create(
        &self,
        quote_id: &str,
        amount: i64,
        operation: &str,
    ) -> Result<crate::integrations::stripe::safe_checkout::CheckoutReceipt, String> {
        self.0
            .create_checkout_session_idempotent(
                crate::integrations::stripe::safe_checkout::CheckoutRequest {
                    name: &format!("Invoice for Quote #{quote_id}"),
                    reference: quote_id,
                    amount_cents: amount,
                    interval: None,
                    product: None,
                    currency: "usd",
                    operation_id: operation,
                },
            )
            .await
    }
}
async fn begin(
    pool: &PgPool,
    tenant: &str,
) -> Result<sqlx::Transaction<'static, sqlx::Postgres>, Error> {
    let mut tx = pool.begin().await?;
    ::server_common::auth_utils::set_org_context(&mut *tx, tenant).await?;
    Ok(tx)
}
async fn locked_quote(tx: &mut PgConnection, tenant: &str, id: &str) -> Result<Quote, Error> {
    let quote: Quote = sqlx::query_as(&format!(
        "SELECT {QUOTE_COLUMNS} FROM quotes WHERE id::text=$1 AND tenant_id=$2 FOR UPDATE"
    ))
    .bind(id)
    .bind(tenant)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(Error::NotFound)?;
    let customer: Option<String> = sqlx::query_scalar(
        "SELECT id::text FROM customers WHERE id::text=$1 AND tenant_id=$2 FOR SHARE",
    )
    .bind(&quote.customer_id)
    .bind(tenant)
    .fetch_optional(tx)
    .await?;
    if customer.is_none() {
        return Err(Error::NotFound);
    }
    Ok(quote)
}

async fn items(tx: &mut PgConnection, tenant: &str, id: &str) -> Result<Vec<QuoteLineItem>, Error> {
    Ok(sqlx::query_as(&format!("SELECT {QUOTE_LINE_ITEM_COLUMNS} FROM quote_line_items WHERE quote_id::text=$1 AND tenant_id=$2 ORDER BY id FOR SHARE"))
        .bind(id).bind(tenant).fetch_all(tx).await?)
}
fn terms(quote: &Quote, lines: &[QuoteLineItem]) -> Value {
    json!({"customer_id":quote.customer_id,"total_amount_cents":quote.total_amount_cents,
        "required_deposit_cents":quote.required_deposit_cents,"valid_until":quote.valid_until,
        "proposed_slot_id":quote.proposed_slot_id,"service_id":quote.service_id,
        "line_items":lines.iter().map(|line|json!({"id":line.id,"description":line.description,
            "unit_price_cents":line.unit_price_cents,"quantity":line.quantity,
            "is_optional":line.is_optional,"service_item_id":line.service_item_id})).collect::<Vec<_>>()})
}
async fn stored(tx: &mut PgConnection, tenant: &str, id: &str) -> Result<Option<Receipt>, Error> {
    let raw: Option<Value> = sqlx::query_scalar(
        "SELECT acceptance_receipt FROM quotes WHERE id::text=$1 AND tenant_id=$2",
    )
    .bind(id)
    .bind(tenant)
    .fetch_one(tx)
    .await?;
    raw.map(|v| {
        serde_json::from_value(v).map_err(|_| Error::Conflict("invalid_acceptance_receipt"))
    })
    .transpose()
}
async fn persist(tx: &mut PgConnection, receipt: &Receipt) -> Result<(), Error> {
    let value =
        serde_json::to_value(receipt).map_err(|_| Error::Conflict("invalid_acceptance_receipt"))?;
    let updated =
        sqlx::query("UPDATE quotes SET acceptance_receipt=$1 WHERE id::text=$2 AND tenant_id=$3")
            .bind(value)
            .bind(&receipt.quote_id)
            .bind(&receipt.tenant_id)
            .execute(tx)
            .await?;
    if updated.rows_affected() != 1 {
        return Err(Error::NotFound);
    }
    Ok(())
}
#[derive(sqlx::FromRow)]
struct InvoiceRecord {
    id: String,
    total_amount_cents: Option<i32>,
    customer_id: String,
    currency: String,
    total_amount: f64,
    status: String,
    payment_status: Option<String>,
    stripe_payment_link: Option<String>,
}
#[derive(sqlx::FromRow)]
struct InvoiceLineRecord {
    description: String,
    quantity: i32,
    unit_price_cents: Option<i64>,
    amount_cents: Option<i64>,
    unit_price: f64,
    amount: f64,
}
async fn verify_receipt(
    tx: &mut PgConnection,
    quote: &Quote,
    lines: &[QuoteLineItem],
    receipt: &mut Receipt,
) -> Result<(), Error> {
    if receipt.version != 1
        || receipt.quote_id != quote.id
        || receipt.tenant_id != quote.tenant_id
        || quote.status != "ACCEPTED"
        || receipt.terms != terms(quote, lines)
    {
        return Err(Error::Conflict("accepted_quote_terms_changed"));
    }
    let rows:Vec<InvoiceRecord>=sqlx::query_as("SELECT id::text,total_amount_cents,customer_id,currency,total_amount,status,payment_status,stripe_payment_link FROM invoices WHERE quote_id::text=$1 AND tenant_id=$2")
        .bind(&quote.id).bind(&quote.tenant_id).fetch_all(&mut *tx).await?;
    if rows.len() != 1
        || rows[0].id != receipt.invoice_id
        || rows[0].total_amount_cents.map(i64::from) != quote.total_amount_cents
        || rows[0].customer_id != quote.customer_id
        || !rows[0].currency.eq_ignore_ascii_case("usd")
        || rows[0].total_amount != quote.total_amount_cents.unwrap_or(0) as f64 / 100.0
        || rows[0].stripe_payment_link != receipt.stripe_payment_link
    {
        return Err(Error::Conflict("accepted_invoice_requires_reconciliation"));
    }
    receipt.invoice_status = rows[0].status.clone();
    receipt.payment_status = rows[0]
        .payment_status
        .clone()
        .unwrap_or_else(|| "unverified".into());
    let saved:Vec<InvoiceLineRecord>=sqlx::query_as("SELECT description,quantity,unit_price_cents,amount_cents,unit_price,amount FROM invoice_line_items WHERE invoice_id::text=$1 AND tenant_id=$2 ORDER BY id")
        .bind(&receipt.invoice_id).bind(&quote.tenant_id).fetch_all(&mut *tx).await?;
    let mut expected: Vec<_> = lines
        .iter()
        .map(|line| {
            (
                line.description.clone(),
                line.quantity,
                line.unit_price_cents,
                line.unit_price_cents.checked_mul(i64::from(line.quantity)),
            )
        })
        .collect();
    expected.sort();
    let mut actual = vec![];
    for InvoiceLineRecord {
        description,
        quantity,
        unit_price_cents: price,
        amount_cents: amount,
        unit_price: legacy_price,
        amount: legacy_amount,
    } in saved
    {
        let (Some(price), Some(amount)) = (price, amount) else {
            return Err(Error::Conflict("accepted_invoice_requires_reconciliation"));
        };
        if legacy_price != price as f64 / 100.0 || legacy_amount != amount as f64 / 100.0 {
            return Err(Error::Conflict("accepted_invoice_requires_reconciliation"));
        }
        actual.push((description, quantity, price, Some(amount)));
    }
    actual.sort();
    if actual != expected {
        return Err(Error::Conflict("accepted_invoice_requires_reconciliation"));
    }
    Ok(())
}
pub(super) async fn read_locked(
    tx: &mut PgConnection,
    quote: &Quote,
    lines: &[QuoteLineItem],
) -> Result<Option<Value>, Error> {
    let mut receipt = stored(tx, &quote.tenant_id, &quote.id).await?;
    if let Some(receipt) = &mut receipt {
        verify_receipt(tx, quote, lines, receipt).await?;
    }
    Ok(receipt.map(|r| r.public()))
}
pub(super) async fn ensure_editable(tx: &mut PgConnection, quote: &Quote) -> Result<(), Error> {
    let invoices: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM invoices WHERE quote_id::text=$1 AND tenant_id=$2)",
    )
    .bind(&quote.id)
    .bind(&quote.tenant_id)
    .fetch_one(&mut *tx)
    .await?;
    if quote.status.eq_ignore_ascii_case("accepted")
        || invoices
        || stored(tx, &quote.tenant_id, &quote.id).await?.is_some()
    {
        return Err(Error::Conflict("accepted_quote_is_immutable"));
    }
    Ok(())
}
/// The local invoice and terms commit before any checkout attempt. Replayed or
/// interrupted attempts never automatically repeat an external effect.
pub(super) async fn accept<C: Checkout>(
    pool: &PgPool,
    tenant: &str,
    user: &str,
    id: &str,
    expected: Option<DateTime<Utc>>,
    provider: &C,
) -> Result<Receipt, Error> {
    let mut tx = begin(pool, tenant).await?;
    let quote = locked_quote(&mut tx, tenant, id).await?;
    let lines = items(&mut tx, tenant, id).await?;
    if let Some(mut receipt) = stored(&mut tx, tenant, id).await? {
        verify_receipt(&mut tx, &quote, &lines, &mut receipt).await?;
        tx.commit().await.map_err(Error::Commit)?;
        return Ok(receipt);
    }
    ensure_editable(&mut tx, &quote).await?;
    if expected.is_none() || quote.updated_at != expected {
        return Err(Error::Conflict("reviewed_quote_version_required"));
    }
    if !matches!(
        quote.status.to_ascii_uppercase().as_str(),
        "DRAFT" | "SENT" | "APPROVED"
    ) {
        return Err(Error::Conflict("quote_not_open_for_acceptance"));
    }
    let amount = quote
        .total_amount_cents
        .ok_or(Error::Conflict("quote_amount_unavailable"))?;
    let amount_i32 =
        i32::try_from(amount).map_err(|_| Error::Conflict("quote_amount_out_of_range"))?;
    // Keep signed line adjustments and the owner's explicit total. Do not
    // silently recompute pricing, deposits or optional-item policy here.
    for line in &lines {
        line.unit_price_cents
            .checked_mul(i64::from(line.quantity))
            .ok_or(Error::Conflict("line_amount_out_of_range"))?;
    }
    let invoice_id = uuid::Uuid::new_v4().to_string();
    let operation_id = format!(
        "ohc_quote_accept_{:x}",
        Sha256::digest(format!("{}:{tenant}:{id}", tenant.len()).as_bytes())
    );
    let existing_checkout = quote
        .stripe_payment_link
        .as_ref()
        .is_some_and(|s| !s.is_empty());
    let (checkout_status, reason) = if existing_checkout {
        ("reconciliation", Some("existing_checkout_is_unbound"))
    } else if !provider.configured() {
        ("not_configured", Some("checkout_not_configured"))
    } else {
        ("pending", None)
    };
    let mut receipt = Receipt {
        version: 1,
        tenant_id: tenant.into(),
        quote_id: id.into(),
        accepted_by: user.into(),
        invoice_id: invoice_id.clone(),
        operation_id,
        terms: terms(&quote, &lines),
        checkout_status: checkout_status.into(),
        reason: reason.map(str::to_string),
        stripe_payment_link: None,
        checkout_id: None,
        invoice_status: "Draft".into(),
        payment_status: "unverified".into(),
    };
    sqlx::query("INSERT INTO invoices(id,tenant_id,customer_id,client_id,quote_id,total_amount,total_amount_cents,currency,status,payment_status)VALUES($1,$2,$3,$3,$4,$5,$6,'USD','Draft','unverified')")
        .bind(&invoice_id).bind(tenant).bind(&quote.customer_id).bind(id).bind(amount as f64/100.0).bind(amount_i32).execute(&mut *tx).await?;
    for line in &lines {
        let cents = line
            .unit_price_cents
            .checked_mul(i64::from(line.quantity))
            .ok_or(Error::Conflict("line_amount_out_of_range"))?;
        sqlx::query("INSERT INTO invoice_line_items(id,tenant_id,invoice_id,description,quantity,unit_price,amount,unit_price_cents,amount_cents)VALUES($1,$2,$3,$4,$5,$6,$7,$8,$9)")
            .bind(uuid::Uuid::new_v4().to_string()).bind(tenant).bind(&invoice_id).bind(&line.description).bind(line.quantity).bind(line.unit_price_cents as f64/100.0).bind(cents as f64/100.0).bind(line.unit_price_cents).bind(cents).execute(&mut *tx).await?;
    }
    sqlx::query("UPDATE quotes SET status='ACCEPTED',updated_at=GREATEST(clock_timestamp(),updated_at+INTERVAL '1 microsecond') WHERE id::text=$1 AND tenant_id=$2")
        .bind(id).bind(tenant).execute(&mut *tx).await?;
    persist(&mut tx, &receipt).await?;
    tx.commit().await.map_err(Error::Commit)?;
    if receipt.checkout_status != "pending" {
        return Ok(receipt);
    }

    // A committed claim precedes the provider boundary. If this task disappears
    // afterwards, GET/replay exposes reconciliation and cannot send it again.
    let mut tx = begin(pool, tenant).await?;
    let quote = locked_quote(&mut tx, tenant, id).await?;
    let lines = items(&mut tx, tenant, id).await?;
    receipt = stored(&mut tx, tenant, id)
        .await?
        .ok_or(Error::Conflict("acceptance_receipt_missing"))?;
    verify_receipt(&mut tx, &quote, &lines, &mut receipt).await?;
    if receipt.checkout_status != "pending" {
        return Ok(receipt);
    }
    receipt.checkout_status = "reconciliation".into();
    receipt.reason = Some("checkout_outcome_unconfirmed".into());
    persist(&mut tx, &receipt).await?;
    tx.commit().await.map_err(Error::Commit)?;
    let checkout = match provider.create(id, amount, &receipt.operation_id).await {
        Ok(checkout) => checkout,
        Err(_) => return Ok(receipt),
    };
    let mut tx = begin(pool, tenant).await?;
    let quote = locked_quote(&mut tx, tenant, id).await?;
    let lines = items(&mut tx, tenant, id).await?;
    verify_receipt(&mut tx, &quote, &lines, &mut receipt).await?;
    receipt.checkout_status = "available".into();
    receipt.reason = None;
    receipt.stripe_payment_link = Some(checkout.url);
    receipt.checkout_id = Some(checkout.id);
    sqlx::query("UPDATE invoices SET stripe_payment_link=$1,updated_at=clock_timestamp() WHERE id::text=$2 AND tenant_id=$3")
        .bind(&receipt.stripe_payment_link).bind(&receipt.invoice_id).bind(tenant).execute(&mut *tx).await?;
    persist(&mut tx, &receipt).await?;
    tx.commit().await.map_err(Error::Commit)?;
    Ok(receipt)
}
