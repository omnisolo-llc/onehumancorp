//! Amount-only terminal operations. An uncertain provider write is never retried
//! automatically. Catalog/cart settlement needs a separate durable stock contract.
use crate::integrations::stripe::{
    client::StripeClient,
    terminal::{TerminalIntentReceipt, TerminalPaymentRequest},
};
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::{PgPool, Postgres, Transaction};

#[derive(Debug)]
pub struct Error(pub StatusCode, pub &'static str, pub &'static str);
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let mut response = (
            self.0,
            Json(serde_json::json!({"success":false,"status":self.1,"error":self.2})),
        )
            .into_response();
        response.headers_mut().insert(
            axum::http::header::CACHE_CONTROL,
            axum::http::HeaderValue::from_static("private, no-store"),
        );
        response
    }
}
fn unavailable() -> Error {
    Error(
        StatusCode::SERVICE_UNAVAILABLE,
        "unknown",
        "Payment outcome could not be verified. Reconcile the original operation before retrying.",
    )
}
fn rejected(message: &'static str) -> Error {
    Error(StatusCode::UNPROCESSABLE_ENTITY, "rejected", message)
}
fn conflict() -> Error {
    Error(
        StatusCode::CONFLICT,
        "reconciliation_required",
        "This payment operation already exists or requires reconciliation. Do not create a replacement charge.",
    )
}
fn not_found() -> Error {
    Error(StatusCode::NOT_FOUND, "rejected", "Payment not found.")
}
fn valid_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-'))
}
fn valid_intent_id(value: &str) -> bool {
    value.starts_with("pi_") && value.len() > 3 && valid_id(value)
}
pub fn provider_operation(tenant: &str, operation: &str) -> String {
    format!(
        "terminal:{:x}",
        Sha256::digest(format!("{tenant}\0{operation}"))
    )
}
pub fn connection_fingerprint(client: &StripeClient) -> Result<String, Error> {
    let key = client.require_api_key().map_err(|_| unavailable())?;
    Ok(format!("{:x}", Sha256::digest(key.as_bytes())))
}

#[async_trait::async_trait]
pub trait Provider: Send + Sync {
    async fn create(
        &self,
        tenant: &str,
        operation: &str,
        amount: i64,
        currency: &str,
    ) -> Result<TerminalIntentReceipt, String>;
    async fn retrieve(&self, id: &str) -> Result<TerminalIntentReceipt, String>;
    async fn capture(&self, id: &str, amount_cents: i64) -> Result<TerminalIntentReceipt, String>;
}
#[async_trait::async_trait]
impl Provider for StripeClient {
    async fn create(
        &self,
        tenant: &str,
        operation: &str,
        amount: i64,
        currency: &str,
    ) -> Result<TerminalIntentReceipt, String> {
        self.create_terminal_payment_intent_receipt(TerminalPaymentRequest {
            tenant_id: tenant,
            amount_cents: amount,
            currency,
            product_id: None,
            quantity: None,
            order_id: None,
            idempotency_key: operation,
        })
        .await
    }
    async fn retrieve(&self, id: &str) -> Result<TerminalIntentReceipt, String> {
        self.retrieve_terminal_payment_intent(id).await
    }
    async fn capture(&self, id: &str, amount_cents: i64) -> Result<TerminalIntentReceipt, String> {
        self.capture_terminal_payment_intent_receipt(id, amount_cents)
            .await
    }
}
#[derive(Clone, Debug, Deserialize)]
pub struct IntentInput {
    pub operation_id: String,
    pub amount_cents: i64,
    pub currency: String,
}
#[derive(Debug, Serialize)]
pub struct IntentResponse {
    pub client_secret: String,
    pub payment_intent_id: String,
    pub operation_id: String,
    pub amount_cents: i64,
    pub currency: String,
    pub lock_id: Option<String>,
}
#[derive(Clone, Debug)]
pub struct CaptureInput {
    pub payment_intent_id: String,
    pub amount_cents: Option<i64>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CaptureResponse {
    pub success: bool,
    pub status: String,
    pub payment_intent_id: String,
    pub operation_id: String,
    pub amount_cents: i64,
    pub currency: String,
}
#[derive(Debug, sqlx::FromRow)]
struct Operation {
    operation_id: String,
    amount_cents: i64,
    currency: String,
    provider_fingerprint: String,
    state: String,
    provider_receipt: Option<serde_json::Value>,
}
async fn scoped(pool: &PgPool, tenant: &str) -> Result<Transaction<'static, Postgres>, Error> {
    if tenant.trim().is_empty() || tenant.trim().eq_ignore_ascii_case("system") {
        return Err(Error(
            StatusCode::UNAUTHORIZED,
            "rejected",
            "Authentication required.",
        ));
    }
    let mut tx = pool.begin().await.map_err(|_| unavailable())?;
    ::server_common::auth_utils::set_org_context(&mut *tx, tenant)
        .await
        .map_err(|_| unavailable())?;
    Ok(tx)
}
fn validate_input(input: &IntentInput) -> Result<(), Error> {
    if !valid_id(&input.operation_id)
        || !(1..=99_999_999).contains(&input.amount_cents)
        || !matches!(
            input.currency.as_str(),
            "usd" | "eur" | "gbp" | "cad" | "aud" | "nzd" | "chf" | "sgd" | "hkd"
        )
    {
        return Err(rejected(
            "A stable operation ID, positive bounded minor-unit amount and supported currency are required.",
        ));
    }
    Ok(())
}
fn matches_receipt(
    receipt: &TerminalIntentReceipt,
    tenant: &str,
    operation: &str,
    amount: i64,
    currency: &str,
) -> bool {
    valid_intent_id(&receipt.id)
        && receipt.amount == amount
        && receipt.currency == currency
        && receipt.metadata.get("tenant_id").map(String::as_str) == Some(tenant)
        && receipt.metadata.get("idempotency_key").map(String::as_str)
            == Some(provider_operation(tenant, operation).as_str())
}
async fn mark_unknown(pool: &PgPool, tenant: &str, operation: &str) {
    if let Ok(mut tx) = scoped(pool, tenant).await {
        let result=sqlx::query("UPDATE terminal_payment_operations SET state='reconciliation_required', updated_at=CURRENT_TIMESTAMP WHERE tenant_id=$1 AND operation_id=$2 AND state IN ('creating','capturing')")
            .bind(tenant).bind(operation).execute(&mut *tx).await;
        if result.is_ok() {
            let _ = tx.commit().await;
        }
    }
}
pub async fn create(
    pool: &PgPool,
    tenant: &str,
    input: IntentInput,
    fingerprint: &str,
    provider: &dyn Provider,
) -> Result<IntentResponse, Error> {
    validate_input(&input)?;
    let mut tx = scoped(pool, tenant).await?;
    // The operation is committed BEFORE any provider effect. Only its first
    // claimant may create; even a crash immediately afterward requires review.
    let inserted=sqlx::query("INSERT INTO terminal_payment_operations (tenant_id,operation_id,amount_cents,currency,provider_fingerprint,state) VALUES ($1,$2,$3,$4,$5,'creating') ON CONFLICT (tenant_id,operation_id) DO NOTHING")
        .bind(tenant).bind(&input.operation_id).bind(input.amount_cents).bind(&input.currency).bind(fingerprint)
        .execute(&mut *tx).await.map_err(|_|unavailable())?;
    if inserted.rows_affected() != 1 {
        return Err(conflict());
    }
    tx.commit().await.map_err(|_| unavailable())?;
    let remote_operation = provider_operation(tenant, &input.operation_id);
    let receipt = match provider
        .create(
            tenant,
            &remote_operation,
            input.amount_cents,
            &input.currency,
        )
        .await
    {
        Ok(receipt)
            if matches_receipt(
                &receipt,
                tenant,
                &input.operation_id,
                input.amount_cents,
                &input.currency,
            ) && receipt.status == "requires_payment_method"
                && receipt.amount_received == 0
                && receipt.amount_capturable == 0
                && receipt.client_secret.as_ref().is_some_and(|secret| {
                    secret.starts_with(&format!("{}_secret_", receipt.id))
                }) =>
        {
            receipt
        }
        _ => {
            mark_unknown(pool, tenant, &input.operation_id).await;
            return Err(unavailable());
        }
    };
    let saved=async {
        let mut tx=scoped(pool,tenant).await?;
        sqlx::query("INSERT INTO payment_intents (tenant_id,payment_id,idempotency_key,amount,currency,status,source,stripe_payment_intent_id) VALUES ($1,$2,$3,$4,$5,'pending','in_person',$6)")
            .bind(tenant).bind(uuid::Uuid::new_v4().to_string()).bind(&remote_operation)
            .bind(input.amount_cents as f64/100.0).bind(&input.currency).bind(&receipt.id)
            .execute(&mut *tx).await.map_err(|_|unavailable())?;
        let updated=sqlx::query("UPDATE terminal_payment_operations SET stripe_payment_intent_id=$3,state='ready',updated_at=CURRENT_TIMESTAMP WHERE tenant_id=$1 AND operation_id=$2 AND state='creating'")
            .bind(tenant).bind(&input.operation_id).bind(&receipt.id).execute(&mut *tx).await.map_err(|_|unavailable())?;
        if updated.rows_affected()!=1{return Err(unavailable());}
        tx.commit().await.map_err(|_|unavailable())
    }.await;
    if saved.is_err() {
        mark_unknown(pool, tenant, &input.operation_id).await;
        return Err(unavailable());
    }
    Ok(IntentResponse {
        client_secret: receipt.client_secret.unwrap_or_default(),
        payment_intent_id: receipt.id,
        operation_id: input.operation_id,
        amount_cents: input.amount_cents,
        currency: input.currency,
        lock_id: None,
    })
}

// Ownership is deliberately a separate preflight callable before loading any
// provider connection. Foreign/missing/old unbound records are indistinguishable.
pub async fn require_owned(pool: &PgPool, tenant: &str, id: &str) -> Result<(), Error> {
    if !valid_intent_id(id) {
        return Err(not_found());
    }
    let mut tx = scoped(pool, tenant).await?;
    let exists=sqlx::query_scalar::<_,bool>("SELECT EXISTS(SELECT 1 FROM terminal_payment_operations WHERE tenant_id = $1 AND stripe_payment_intent_id = $2)")
        .bind(tenant).bind(id).fetch_one(&mut *tx).await.map_err(|_|unavailable())?;
    if !exists {
        return Err(not_found());
    }
    Ok(())
}
pub async fn capture(
    pool: &PgPool,
    tenant: &str,
    input: CaptureInput,
    fingerprint: &str,
    provider: &dyn Provider,
) -> Result<CaptureResponse, Error> {
    if !valid_intent_id(&input.payment_intent_id) {
        return Err(not_found());
    }
    let mut tx = scoped(pool, tenant).await?;
    let operation=sqlx::query_as::<_,Operation>("SELECT operation_id,amount_cents,currency,provider_fingerprint,state,provider_receipt FROM terminal_payment_operations WHERE tenant_id = $1 AND stripe_payment_intent_id = $2 FOR UPDATE")
        .bind(tenant).bind(&input.payment_intent_id).fetch_optional(&mut *tx).await.map_err(|_|unavailable())?.ok_or_else(not_found)?;
    if operation.provider_fingerprint != fingerprint
        || input
            .amount_cents
            .is_some_and(|amount| amount != operation.amount_cents)
    {
        return Err(rejected(
            "Payment binding does not match the saved operation.",
        ));
    }
    let saved=sqlx::query_as::<_,(f64,String,String,String)>("SELECT amount,currency,idempotency_key,source FROM payment_intents WHERE tenant_id=$1 AND stripe_payment_intent_id=$2 FOR UPDATE")
        .bind(tenant).bind(&input.payment_intent_id).fetch_optional(&mut *tx).await.map_err(|_|unavailable())?.ok_or_else(not_found)?;
    if saved.0 != operation.amount_cents as f64 / 100.0
        || saved.1 != operation.currency
        || saved.2 != provider_operation(tenant, &operation.operation_id)
        || saved.3 != "in_person"
    {
        return Err(conflict());
    }
    let response = CaptureResponse {
        success: true,
        status: "succeeded".into(),
        payment_intent_id: input.payment_intent_id.clone(),
        operation_id: operation.operation_id.clone(),
        amount_cents: operation.amount_cents,
        currency: operation.currency.clone(),
    };
    if operation.state == "succeeded" {
        let receipt: TerminalIntentReceipt =
            serde_json::from_value(operation.provider_receipt.ok_or_else(conflict)?)
                .map_err(|_| conflict())?;
        if receipt.id != input.payment_intent_id
            || receipt.status != "succeeded"
            || receipt.amount_received != operation.amount_cents
            || receipt.amount_capturable != 0
            || !matches_receipt(
                &receipt,
                tenant,
                &operation.operation_id,
                operation.amount_cents,
                &operation.currency,
            )
        {
            return Err(conflict());
        }
        return Ok(response);
    }
    if operation.state != "ready" {
        return Err(conflict());
    }
    let claimed=sqlx::query("UPDATE terminal_payment_operations SET state = 'capturing',updated_at=CURRENT_TIMESTAMP WHERE tenant_id=$1 AND operation_id=$2 AND state = 'ready'")
        .bind(tenant).bind(&operation.operation_id).execute(&mut *tx).await.map_err(|_|unavailable())?;
    if claimed.rows_affected() != 1 {
        return Err(conflict());
    }
    tx.commit().await.map_err(|_| unavailable())?;
    // Check the provider's actual amount/account metadata before capture. A
    // changed key/account, modified provider intent or missing receipt blocks it.
    let current = provider.retrieve(&input.payment_intent_id).await;
    if !current.as_ref().is_ok_and(|receipt| {
        receipt.id == input.payment_intent_id
            && receipt.status == "requires_capture"
            && receipt.amount_received == 0
            && receipt.amount_capturable == operation.amount_cents
            && matches_receipt(
                receipt,
                tenant,
                &operation.operation_id,
                operation.amount_cents,
                &operation.currency,
            )
    }) {
        mark_unknown(pool, tenant, &operation.operation_id).await;
        return Err(conflict());
    }
    let receipt = match provider
        .capture(&input.payment_intent_id, operation.amount_cents)
        .await
    {
        Ok(receipt)
            if receipt.id == input.payment_intent_id
                && receipt.status == "succeeded"
                && receipt.amount_received == operation.amount_cents
                && receipt.amount_capturable == 0
                && matches_receipt(
                    &receipt,
                    tenant,
                    &operation.operation_id,
                    operation.amount_cents,
                    &operation.currency,
                ) =>
        {
            receipt
        }
        _ => {
            mark_unknown(pool, tenant, &operation.operation_id).await;
            return Err(unavailable());
        }
    };
    let saved=async {
        let mut tx=scoped(pool,tenant).await?;
        let receipt=serde_json::to_value(receipt).map_err(|_|unavailable())?;
        let changed=sqlx::query("UPDATE terminal_payment_operations SET state='succeeded',provider_receipt=$3,updated_at=CURRENT_TIMESTAMP WHERE tenant_id=$1 AND operation_id=$2 AND state='capturing'")
            .bind(tenant).bind(&operation.operation_id).bind(receipt).execute(&mut *tx).await.map_err(|_|unavailable())?;
        if changed.rows_affected()!=1{return Err(unavailable());}
        // payment_intents.status remains webhook-owned: marking it succeeded
        // here would suppress the existing ledger webhook's reconciliation.
        tx.commit().await.map_err(|_|unavailable())
    }.await;
    if saved.is_err() {
        mark_unknown(pool, tenant, &operation.operation_id).await;
        return Err(unavailable());
    }
    Ok(response)
}
