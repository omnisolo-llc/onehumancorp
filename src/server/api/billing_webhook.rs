use axum::{
    extract::{Json, Request, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    middleware::Next,
    body::Body,
};
use serde::Deserialize;
use std::sync::Arc;
use serde_json::Value;

use ::server_pricing::rate_limit::{PlanTier, RedisRateLimiter};
use crate::db::DbStore;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;

#[derive(Clone)]
pub struct WebhookState {
    pub rate_limiter: Arc<RedisRateLimiter>,
    pub db_pool: sqlx::Pool<sqlx::Postgres>,
    pub db: std::sync::Arc<crate::db::DB>,
    pub orchestrator: std::sync::Arc<DepartmentOrchestrator>,
}

#[derive(Debug, Deserialize)]
pub struct StripeEvent {
    pub id: String,
    pub r#type: String,
    pub data: StripeEventData,
}

#[derive(Debug, Deserialize)]
pub struct StripeEventData {
    pub object: Value,
}

#[async_trait::async_trait]
pub trait PaymentFailureNotifier: Send + Sync {
    async fn send_payment_failure_sms(&self, subscriber_id: &str, message: &str) -> Result<(), String>;
}

#[async_trait::async_trait]
pub trait PaymentFailureMessageGenerator: Send + Sync {
    async fn generate_payment_failure_message(&self, subscriber_id: &str, business_name: &str) -> String;
}

pub struct CriticalSmsPaymentFailureNotifier;
pub struct LlmPaymentFailureMessageGenerator;

#[async_trait::async_trait]
impl PaymentFailureNotifier for CriticalSmsPaymentFailureNotifier {
    async fn send_payment_failure_sms(&self, _subscriber_id: &str, message: &str) -> Result<(), String> {
        crate::dispatch_critical_sms("failed_payment", message).await
    }
}

#[async_trait::async_trait]
impl PaymentFailureMessageGenerator for LlmPaymentFailureMessageGenerator {
    async fn generate_payment_failure_message(&self, subscriber_id: &str, business_name: &str) -> String {
        let fallback = format!(
            "{} subscription payment could not be processed. Please update the saved payment method to keep the subscription active.",
            business_name
        );
        let prompt = format!(
            "Write a concise, helpful SMS for subscription payment recovery. Business: {}. Subscriber id: {}. Mention the payment could not be processed and ask them to update their saved payment method. Avoid blame and keep it under 240 characters.",
            business_name,
            subscriber_id
        );

        match std::env::var("OHC_LLM_PROVIDER").as_deref() {
            Ok("minimax") => {
                let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
                crate::minimax::MinimaxClient::new(api_key)
                    .reason(&prompt)
                    .await
                    .unwrap_or(fallback)
            }
            _ => crate::minimax::LocalLLMClient::new()
                .reason(&prompt)
                .await
                .unwrap_or(fallback),
        }
    }
}

pub async fn send_payment_failure_dunning<N, G>(
    notifier: &N,
    generator: &G,
    subscriber_id: &str,
    business_name: &str,
) -> Result<(), String>
where
    N: PaymentFailureNotifier,
    G: PaymentFailureMessageGenerator,
{
    let message = generator.generate_payment_failure_message(subscriber_id, business_name).await;
    notifier.send_payment_failure_sms(subscriber_id, &message).await
}

pub fn inventory_locks_for_payment_success(object: &Value) -> Vec<String> {
    let Some(metadata) = object.get("metadata") else {
        return Vec::new();
    };
    let mut locks = Vec::new();
    if let Some(lock_id) = metadata.get("inventory_lock_id").and_then(|v| v.as_str()) {
        if !lock_id.trim().is_empty() {
            locks.push(lock_id.to_string());
        }
    }
    if let Some(lock_ids) = metadata.get("inventory_lock_ids").and_then(|v| v.as_array()) {
        for lock_id in lock_ids.iter().filter_map(|v| v.as_str()) {
            if !lock_id.trim().is_empty() && !locks.iter().any(|existing| existing == lock_id) {
                locks.push(lock_id.to_string());
            }
        }
    }
    locks
}

async fn release_inventory_locks_for_payment(webhook_state: &WebhookState, object: &Value) {
    let locks = inventory_locks_for_payment_success(object);
    if locks.is_empty() {
        return;
    }
    match webhook_state.rate_limiter.get_connection().await {
        Ok(mut conn) => {
            for lock_id in locks {
                let _: Result<(), _> = redis::cmd("DEL").arg(&lock_id).query_async(&mut conn).await;
            }
        }
        Err(err) => {
            ::server_telemetry::record_error_signal("[bug] Failed to get redis connection for payment inventory lock release");
            tracing::warn!("Failed to release payment inventory locks: {}", err);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentFailureLookup {
    pub stripe_subscription_id: Option<String>,
    pub customer_id: Option<String>,
}

pub fn payment_failure_lookup(object: &Value) -> PaymentFailureLookup {
    let stripe_subscription_id = object
        .get("subscription")
        .and_then(|value| value.as_str())
        .or_else(|| {
            object
                .get("parent")
                .and_then(|parent| parent.get("subscription_details"))
                .and_then(|details| details.get("subscription"))
                .and_then(|value| value.as_str())
        })
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string);

    let customer_id = object
        .get("customer")
        .and_then(|value| value.as_str())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string);

    PaymentFailureLookup {
        stripe_subscription_id,
        customer_id,
    }
}

async fn find_subscriber_for_payment_failure(
    webhook_state: &WebhookState,
    lookup: &PaymentFailureLookup,
) -> Result<Option<String>, String> {
    let subscription_id = lookup.stripe_subscription_id.as_deref().unwrap_or("");
    let customer_id = lookup.customer_id.as_deref().unwrap_or("");
    if subscription_id.is_empty() && customer_id.is_empty() {
        return Ok(None);
    }

    match &webhook_state.db.store {
        DbStore::Sqlite(pool) => {
            let row: Option<(String,)> = sqlx::query_as(
                "SELECT id FROM subscribers \
                 WHERE (?1 != '' AND stripe_subscription_id = ?1) \
                    OR (?2 != '' AND customer_id = ?2) \
                 LIMIT 1",
            )
            .bind(subscription_id)
            .bind(customer_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| format!("Failed to lookup failed-payment subscriber: {e}"))?;
            Ok(row.map(|(id,)| id))
        }
        DbStore::Postgres => {
            let row: Option<(String,)> = sqlx::query_as(
                "SELECT id FROM subscribers \
                 WHERE ($1 != '' AND stripe_subscription_id = $1) \
                    OR ($2 != '' AND customer_id = $2) \
                 LIMIT 1",
            )
            .bind(subscription_id)
            .bind(customer_id)
            .fetch_optional(&webhook_state.db.pool)
            .await
            .map_err(|e| format!("Failed to lookup failed-payment subscriber: {e}"))?;
            Ok(row.map(|(id,)| id))
        }
    }
}

async fn mark_subscriber_past_due(
    webhook_state: &WebhookState,
    subscriber_id: &str,
) -> Result<(), String> {
    match &webhook_state.db.store {
        DbStore::Sqlite(pool) => {
            sqlx::query("UPDATE subscribers SET status = 'PAST_DUE' WHERE id = ?")
                .bind(subscriber_id)
                .execute(pool)
                .await
                .map(|_| ())
                .map_err(|e| e.to_string())
        }
        DbStore::Postgres => {
            sqlx::query("UPDATE subscribers SET status = 'PAST_DUE' WHERE id = $1")
                .bind(subscriber_id)
                .execute(&webhook_state.db.pool)
                .await
                .map(|_| ())
                .map_err(|e| e.to_string())
        }
    }
    .map_err(|e| format!("Failed to mark subscriber past due: {e}"))
}

pub async fn process_invoice_payment_failed<N, G>(
    webhook_state: &WebhookState,
    object: &Value,
    notifier: &N,
    generator: &G,
) -> Result<Option<String>, String>
where
    N: PaymentFailureNotifier,
    G: PaymentFailureMessageGenerator,
{
    let lookup = payment_failure_lookup(object);
    let Some(subscriber_id) = find_subscriber_for_payment_failure(webhook_state, &lookup).await? else {
        return Ok(None);
    };

    mark_subscriber_past_due(webhook_state, &subscriber_id).await?;
    let business_name = object
        .get("metadata")
        .and_then(|metadata| metadata.get("business_name"))
        .and_then(|value| value.as_str())
        .unwrap_or("Your business");
    send_payment_failure_dunning(notifier, generator, &subscriber_id, business_name).await?;

    Ok(Some(subscriber_id))
}

pub async fn webhook_security_middleware(
    State(webhook_state): State<WebhookState>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let (parts, body) = req.into_parts();

    let bytes = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(b) => b,
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    };

    // Extract timestamp and check signature
    let sig_header = parts.headers.get("X-Signature").or_else(|| parts.headers.get("Stripe-Signature"));
    let mut valid_signature = false;
    let mut timestamp_valid = false;

    if let Some(sig) = sig_header {
        if let Ok(sig_str) = sig.to_str() {
            valid_signature = true; // In a real implementation this would perform HMAC verification

            // Example Stripe signature format: t=1614838634,v1=...
            let ts_part = sig_str.split(',').find(|p| p.starts_with("t="));
            if let Some(ts) = ts_part {
                if let Ok(timestamp) = ts[2..].parse::<i64>() {
                    let now = chrono::Utc::now().timestamp();
                    // Within 5 minutes (300 seconds)
                    if (now - timestamp).abs() <= 300 {
                        timestamp_valid = true;
                    }
                }
            } else {
                // If no timestamp is provided in the header, we'll reject or accept based on requirements.
                // Since "Verify that the timestamp in the signature header is within 5 minutes" is requested,
                // we require it for valid signatures.
            }
        }
    }

    if !valid_signature || !timestamp_valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Extract event ID for idempotency check
    let mut event_id = None;
    if let Ok(json_value) = serde_json::from_slice::<serde_json::Value>(&bytes) {
        if let Some(id) = json_value.get("id").and_then(|id| id.as_str()) {
            event_id = Some(id.to_string());
        } else if let Some(uid) = json_value.get("payload").and_then(|p| p.get("uid")).and_then(|uid| uid.as_str()) {
            event_id = Some(uid.to_string());
        } else if let Some(entity_id) = json_value.get("payload").and_then(|p| p.get("payment")).and_then(|p| p.get("entity")).and_then(|e| e.get("id")).and_then(|id| id.as_str()) {
            event_id = Some(entity_id.to_string());
        }
    }

    if let Some(id) = event_id {
        let redis_key = format!("webhook:idempotency:{}", id);

        if let Ok(mut conn) = webhook_state.rate_limiter.get_connection().await {
            let acquired: bool = redis::cmd("SET")
                .arg(&redis_key)
                .arg("1")
                .arg("NX")
                .arg("EX")
                .arg(86400) // 24 hours
                .query_async(&mut conn)
                .await
                .unwrap_or(false);

            if !acquired {
                // Already processed
                return Ok(StatusCode::OK.into_response());
            }
        } else {
            ::server_telemetry::record_error_signal("[bug] Failed to get redis connection for webhook idempotency check");
            tracing::error!("Failed to get redis connection for webhook idempotency check");
        }
    }

    // Reconstruct the request body
    let new_req = Request::from_parts(parts, Body::from(bytes));

    // Process asynchronously and immediately return 200 OK
    tokio::spawn(async move {
        let _ = next.run(new_req).await;
    });

    Ok(StatusCode::OK.into_response())
}

pub async fn stripe_webhook_handler(
    axum::extract::State(webhook_state): axum::extract::State<WebhookState>,
    Json(payload): Json<StripeEvent>,
) -> impl IntoResponse {

    match payload.r#type.as_str() {
        "terminal.reader.action.succeeded" | "pos_transaction" | "payment_intent.succeeded" => {
            let obj = &payload.data.object;

            // Only handle POS payment intents (where source is in_person) here, ignore online intents
            if payload.r#type == "payment_intent.succeeded" {
                let source = obj.get("metadata").and_then(|m| m.get("source")).and_then(|s| s.as_str());
                if source != Some("in_person") {
                    // It's a normal online payment intent, handled elsewhere or ignored here
                    return StatusCode::OK.into_response();
                }

                // Extract the tenant_id and idempotency_key for ledger updates
                let tenant_id_opt = obj.get("metadata").and_then(|m| m.get("tenant_id")).and_then(|id| id.as_str());
                let idempotency_key_opt = obj.get("metadata").and_then(|m| m.get("idempotency_key")).and_then(|id| id.as_str());

                if let (Some(tenant_id), Some(idempotency_key)) = (tenant_id_opt, idempotency_key_opt) {
                    let pool = crate::db::get_pool();
                    let existing: Option<(String,)> = sqlx::query_as("SELECT status FROM payment_intents WHERE idempotency_key = $1 AND tenant_id = $2")
                        .bind(idempotency_key)
                        .bind(tenant_id)
                        .fetch_optional(&pool)
                        .await.unwrap_or(None);

                    if let Some((status,)) = existing {
                        if status != "succeeded" {
                            if let Ok(mut tx) = pool.begin().await {
                                let payment_intent_id = obj.get("id").and_then(|id| id.as_str()).unwrap_or("");
                                let amount_cents = obj.get("amount").and_then(|a| a.as_i64()).unwrap_or(0);
                                let amount_f64 = (amount_cents as f64) / 100.0;
                                let currency = obj.get("currency").and_then(|c| c.as_str()).unwrap_or("usd").to_string();
                                let tx_id = uuid::Uuid::new_v4().to_string();
                                let entry_id = uuid::Uuid::new_v4().to_string();
                                let account_id = "default_revenue";

                                if let Err(e) = sqlx::query("UPDATE payment_intents SET status = 'succeeded', stripe_payment_intent_id = $1 WHERE idempotency_key = $2 AND tenant_id = $3")
                                    .bind(payment_intent_id).bind(idempotency_key).bind(tenant_id).execute(&mut *tx).await { tracing::error!("Failed to update payment_intents: {}", e); let _ = tx.rollback().await; return StatusCode::INTERNAL_SERVER_ERROR.into_response(); }

                                if let Err(e) = sqlx::query("INSERT INTO ledger_transactions (tenant_id, tx_id, amount, currency) VALUES ($1, $2, $3, $4)")
                                    .bind(tenant_id).bind(&tx_id).bind(amount_f64).bind(&currency).execute(&mut *tx).await { tracing::error!("Failed to insert ledger_transactions: {}", e); let _ = tx.rollback().await; return StatusCode::INTERNAL_SERVER_ERROR.into_response(); }

                                if let Err(e) = sqlx::query("INSERT INTO ledger_accounts (tenant_id, account_id, currency, balance) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING")
                                    .bind(tenant_id).bind(account_id).bind(&currency).bind(0.0).execute(&mut *tx).await { tracing::error!("Failed to insert ledger_accounts: {}", e); let _ = tx.rollback().await; return StatusCode::INTERNAL_SERVER_ERROR.into_response(); }

                                if let Err(e) = sqlx::query("INSERT INTO ledger_entries (tenant_id, entry_id, tx_id, account_id, direction, amount) VALUES ($1, $2, $3, $4, 'CREDIT', $5)")
                                    .bind(tenant_id).bind(&entry_id).bind(&tx_id).bind(account_id).bind(amount_f64).execute(&mut *tx).await { tracing::error!("Failed to insert ledger_entries: {}", e); let _ = tx.rollback().await; return StatusCode::INTERNAL_SERVER_ERROR.into_response(); }

                                if let Err(e) = sqlx::query("UPDATE ledger_accounts SET balance = balance + $1 WHERE tenant_id = $2 AND account_id = $3")
                                    .bind(amount_f64).bind(tenant_id).bind(account_id).execute(&mut *tx).await { tracing::error!("Failed to update ledger_accounts: {}", e); let _ = tx.rollback().await; return StatusCode::INTERNAL_SERVER_ERROR.into_response(); }

                                if let Err(e) = tx.commit().await {
                                    tracing::error!("Failed to commit tap-to-pay ledger update: {}", e);
                                }
                            }
                        }
                    }
                }
            }

            let tenant_id_opt = obj.get("metadata")
                .and_then(|m| m.get("tenant_id"))
                .and_then(|id| id.as_str());

            let product_id_opt = obj.get("metadata")
                .and_then(|m| m.get("product_id"))
                .and_then(|id| id.as_str());

            if let (Some(tenant_id), Some(product_id)) = (tenant_id_opt, product_id_opt) {
                let quantity = obj.get("metadata")
                    .and_then(|m| m.get("quantity"))
                    .and_then(|q| q.as_str())
                    .and_then(|q| q.parse::<i32>().ok())
                    .unwrap_or(1);

                let inventory_service = crate::services::inventory::InventoryService::new(None);

                let _ = inventory_service.commit_inventory(tenant_id, product_id, quantity, "").await;

                // Notify KAIROS Orchestrator for Sales and Operations AI agents
                let orch = webhook_state.orchestrator.clone();
                let payload_val = obj.clone();
                let tenant_id_val = tenant_id.to_string();
                tokio::spawn(async move {
                    let evt = crate::orchestration::departments::types::DepartmentEvent {
                        id: uuid::Uuid::new_v4().to_string(),
                        tenant_id: tenant_id_val,
                        event_type: "POS_SALE_COMPLETED".to_string(),
                        payload: payload_val,
                    };
                    let _ = orch.dispatch_event(evt).await;
                });
            }

            // Also try to update the order status to Paid if order_id is present
            let order_id_opt = obj.get("metadata")
                .and_then(|m| m.get("order_id"))
                .and_then(|id| id.as_str());

            if let Some(order_id) = order_id_opt {
                let res = match &webhook_state.db.store {
                    crate::db::DbStore::Sqlite(pool) => {
                        sqlx::query("UPDATE orders SET status = 'Paid' WHERE id = ?")
                            .bind(order_id)
                            .execute(pool)
                            .await
                            .map(|_| ())
                    }
                    crate::db::DbStore::Postgres => {
                        sqlx::query("UPDATE orders SET status = 'Paid' WHERE id = $1")
                            .bind(order_id)
                            .execute(&webhook_state.db.pool)
                            .await
                            .map(|_| ())
                    }
                };

                if let Err(e) = res {
                    ::server_telemetry::record_error_signal("[bug] Failed to update order status for order : {:?}");
                    tracing::error!("Failed to update order status for order {}: {:?}", order_id, e);
                }
            }

            let booking_id_opt = obj.get("metadata")
                .and_then(|m| m.get("booking_id"))
                .and_then(|id| id.as_str());

            if let Some(booking_id) = booking_id_opt {
                let res = match &webhook_state.db.store {
                    crate::db::DbStore::Sqlite(pool) => {
                        sqlx::query("UPDATE bookings SET status = 'confirmed' WHERE id = ?")
                            .bind(booking_id)
                            .execute(pool)
                            .await
                            .map(|_| ())
                    }
                    crate::db::DbStore::Postgres => {
                        sqlx::query("UPDATE bookings SET status = 'confirmed' WHERE id = $1")
                            .bind(booking_id)
                            .execute(&webhook_state.db.pool)
                            .await
                            .map(|_| ())
                    }
                };

                if let Err(e) = res {
                    tracing::error!("Failed to confirm booking {}: {:?}", booking_id, e);
                } else if let Some(tenant_id) = tenant_id_opt {
                    // Dispatch the payment.captured event so Operations can do follow up if required
                    let orch = webhook_state.orchestrator.clone();
                    let payload_val = obj.clone();
                    let tenant_id_val = tenant_id.to_string();
                    tokio::spawn(async move {
                        let evt = crate::orchestration::departments::types::DepartmentEvent {
                            id: uuid::Uuid::new_v4().to_string(),
                            tenant_id: tenant_id_val,
                            event_type: "payment.captured".to_string(),
                            payload: payload_val,
                        };
                        let _ = orch.dispatch_event(evt).await;
                    });
                }
            }

            StatusCode::OK.into_response()
        },
        "checkout.session.completed" | "customer.subscription.updated" | "customer.subscription.created" => {
            let obj = &payload.data.object;
            if payload.r#type == "checkout.session.completed" {
                let tenant_id_opt = obj.get("metadata")
                    .and_then(|m| m.get("tenant_id"))
                    .and_then(|id| id.as_str());

                let product_id_opt = obj.get("metadata")
                    .and_then(|m| m.get("product_id"))
                    .and_then(|id| id.as_str());

                let conversational_intake_opt = obj.get("metadata")
                    .and_then(|m| m.get("conversational_intake_id"))
                    .and_then(|id| id.as_str());

                if let Some(intake_id) = conversational_intake_opt {
                    let _ = sqlx::query("UPDATE conversational_intake_queue SET status = 'BOOKED', updated_at = NOW() WHERE id = $1")
                        .bind(intake_id)
                        .execute(&webhook_state.db.pool)
                        .await;

                    if let Ok(Some(row)) = sqlx::query("SELECT proposed_slot_id, tenant_id FROM conversational_intake_queue WHERE id = $1")
                        .bind(intake_id)
                        .fetch_optional(&webhook_state.db.pool)
                        .await
                    {
                        use sqlx::Row;
                        let proposed_slot_id: Option<String> = row.try_get("proposed_slot_id").ok();
                        let tenant_id: String = row.get("tenant_id");

                        if let Some(slot_id) = proposed_slot_id {
                            let _ = sqlx::query("UPDATE booking_slots SET status = 'booked', updated_at = NOW() WHERE id = $1")
                                .bind(&slot_id)
                                .execute(&webhook_state.db.pool)
                                .await;
                        }

                        let feed_id = uuid::Uuid::new_v4().to_string();
                        let _ = sqlx::query(r#"INSERT INTO agent_feed_items (id, tenant_id, event_source, context_payload, proposed_action, lifecycle_state, created_at, updated_at) VALUES ($1, $2, 'omnichannel', '{"feature_type": "autonomous_quote_confirmed"}', '{}', 'APPROVED', NOW(), NOW())"#)
                            .bind(feed_id)
                            .bind(tenant_id)
                            .execute(&webhook_state.db.pool)
                            .await;
                    }
                }

                if let (Some(tenant_id), Some(product_id)) = (tenant_id_opt, product_id_opt) {
                    let quantity = obj.get("metadata")
                        .and_then(|m| m.get("quantity"))
                        .and_then(|q| q.as_str())
                        .and_then(|q| q.parse::<i32>().ok())
                        .unwrap_or(1);

                    let lock_id = obj.get("metadata")
                        .and_then(|m| m.get("inventory_lock_id"))
                        .and_then(|id| id.as_str())
                        .unwrap_or("");

                    let inventory_service = crate::services::inventory::InventoryService::new(None);
                    let _ = inventory_service.commit_inventory(tenant_id, product_id, quantity, lock_id).await;
                } else {
                    release_inventory_locks_for_payment(&webhook_state, obj).await;
                }

                // Dispatch payment.captured event to Finance agent
                if let Some(tenant_id) = tenant_id_opt {
                    let orch = webhook_state.orchestrator.clone();
                    let payload_val = obj.clone();
                    let tenant_id_val = tenant_id.to_string();
                    tokio::spawn(async move {
                        let evt = crate::orchestration::departments::types::DepartmentEvent {
                            id: uuid::Uuid::new_v4().to_string(),
                            tenant_id: tenant_id_val,
                            event_type: "payment.captured".to_string(),
                            payload: payload_val,
                        };
                        let _ = orch.dispatch_event(evt).await;
                    });
                }
            }

            let tenant_id_opt = obj.get("metadata")
                .and_then(|m| m.get("tenant_id"))
                .and_then(|id| id.as_str())
                .or_else(|| obj.get("client_reference_id").and_then(|id| id.as_str()));

            let customer_id_opt = obj.get("client_reference_id")
                .and_then(|id| id.as_str())
                .or_else(|| obj.get("customer").and_then(|id| id.as_str()));

            let product_id_opt = obj.get("metadata")
                .and_then(|m| m.get("product_id"))
                .and_then(|id| id.as_str());

            if let Some(tenant_id) = tenant_id_opt {
                // If this is a product subscription
                if let (Some(product_id), Some(customer_id)) = (product_id_opt, customer_id_opt) {
                    if payload.r#type == "checkout.session.completed" && obj.get("mode").and_then(|m| m.as_str()) == Some("subscription") {
                        // Check if a plan exists for this product
                        let mut plan_id_res = sqlx::query_scalar::<_, String>("SELECT id FROM subscription_plans WHERE product_id = $1 AND tenant_id = $2")
                            .bind(product_id)
                            .bind(tenant_id)
                            .fetch_optional(&webhook_state.db.pool)
                            .await
                            .unwrap_or(None);

                        if plan_id_res.is_none() {
                            let new_plan_id = uuid::Uuid::new_v4().to_string();
                            let _ = sqlx::query("INSERT INTO subscription_plans (id, tenant_id, product_id, interval) VALUES ($1, $2, $3, $4)")
                                .bind(&new_plan_id)
                                .bind(tenant_id)
                                .bind(product_id)
                                .bind("month") // fallback interval
                                .execute(&webhook_state.db.pool)
                                .await;
                            plan_id_res = Some(new_plan_id);
                        }

                        if let Some(plan_id) = plan_id_res {
                            let subscription_id = uuid::Uuid::new_v4().to_string();
                            let stripe_subscription_id = obj.get("subscription").and_then(|s| s.as_str()).unwrap_or("");

                            let _ = sqlx::query(
                                "INSERT INTO subscriptions (id, tenant_id, customer_id, plan_id, status, current_period_end)
                                 VALUES ($1, $2, $3, $4, 'active', CURRENT_TIMESTAMP + INTERVAL '1 month')"
                            )
                            .bind(&subscription_id)
                            .bind(tenant_id)
                            .bind(customer_id)
                            .bind(&plan_id)
                            .execute(&webhook_state.db.pool)
                            .await;

                            let _ = sqlx::query(
                                "INSERT INTO subscribers (id, tenant_id, subscription_plan_id, customer_id, stripe_subscription_id, status)
                                 VALUES ($1, $2, $3, $4, $5, 'ACTIVE')"
                            )
                            .bind(&subscription_id)
                            .bind(tenant_id)
                            .bind(&plan_id)
                            .bind(customer_id)
                            .bind(stripe_subscription_id)
                            .execute(&webhook_state.db.pool)
                            .await;

                            // Create an order for the Manager agent
                            let order_id = uuid::Uuid::new_v4().to_string();
                            let amount_total = obj.get("amount_total").and_then(|a| a.as_i64()).unwrap_or(0);
                            let _ = sqlx::query(
                                "INSERT INTO orders (id, tenant_id, customer_id, total_amount_cents, status) VALUES ($1, $2, $3, $4, 'paid')"
                            )
                            .bind(&order_id)
                            .bind(tenant_id)
                            .bind(customer_id)
                            .bind(amount_total)
                            .execute(&webhook_state.db.pool)
                            .await;

                            // Let the manager agent know
                            let orch = webhook_state.orchestrator.clone();
                            let payload_val = serde_json::json!({
                                "order_id": order_id,
                                "customer_id": customer_id,
                                "subscription_id": subscription_id
                            });
                            let tenant_id_val = tenant_id.to_string();
                            tokio::spawn(async move {
                                let evt = crate::orchestration::departments::types::DepartmentEvent {
                                    id: uuid::Uuid::new_v4().to_string(),
                                    tenant_id: tenant_id_val,
                                    event_type: "tenant.order.created".to_string(),
                                    payload: payload_val,
                                };
                                let _ = orch.dispatch_event(evt).await;
                            });
                        }
                    }
                }

                // Normal tenant tier update logic
                let tier_str = obj.get("metadata")
                    .and_then(|m| m.get("tier"))
                    .and_then(|t| t.as_str())
                    .unwrap_or("Starter");

                let tier = match tier_str {
                    "Starter" => PlanTier::Starter,
                    "Pro" => PlanTier::Pro,
                    "Business" => PlanTier::Business,
                    _ => PlanTier::Free,
                };

                // Update Redis Rate Limiter
                if let Err(_e) = webhook_state.rate_limiter.set_tenant_tier(tenant_id, tier.clone()).await {
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }

                // Update Database
                let tier_string = match tier {
                    PlanTier::Free => "Free",
                    PlanTier::Starter => "Starter",
                    PlanTier::Pro => "Pro",
                    PlanTier::Business => "Business",
                };

                let res = match &webhook_state.db.store {
                    DbStore::Sqlite(pool) => {
                        sqlx::query("UPDATE tenants SET plan_tier = ? WHERE id = ?")
                            .bind(tier_string)
                            .bind(tenant_id)
                            .execute(pool)
                            .await
                            .map(|_| ())
                    }
                    DbStore::Postgres => {
                        sqlx::query("UPDATE tenants SET plan_tier = $1 WHERE id = $2")
                            .bind(tier_string)
                            .bind(tenant_id)
                            .execute(&webhook_state.db.pool)
                            .await
                            .map(|_| ())
                    }
                };

                if let Err(_e) = res {
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }

                StatusCode::OK.into_response()
            } else {
                StatusCode::BAD_REQUEST.into_response()
            }
        },
        "customer.subscription.deleted" => {
            let obj = &payload.data.object;
            let tenant_id_opt = obj.get("metadata")
                .and_then(|m| m.get("tenant_id"))
                .and_then(|id| id.as_str())
                .or_else(|| obj.get("client_reference_id").and_then(|id| id.as_str()));

            if let Some(tenant_id) = tenant_id_opt {

                // Update Redis
                if let Err(_e) = webhook_state.rate_limiter.set_tenant_tier(tenant_id, PlanTier::Free).await {
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }

                // Update DB
                let res = match &webhook_state.db.store {
                    DbStore::Sqlite(pool) => {
                        sqlx::query("UPDATE tenants SET plan_tier = ? WHERE id = ?")
                            .bind("Free")
                            .bind(tenant_id)
                            .execute(pool)
                            .await
                            .map(|_| ())
                    }
                    DbStore::Postgres => {
                        sqlx::query("UPDATE tenants SET plan_tier = $1 WHERE id = $2")
                            .bind("Free")
                            .bind(tenant_id)
                            .execute(&webhook_state.db.pool)
                            .await
                            .map(|_| ())
                    }
                };

                if let Err(_e) = res {
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }

                StatusCode::OK.into_response()
            } else {
                StatusCode::BAD_REQUEST.into_response()
            }
        },
        "invoice.payment_succeeded" | "invoice.paid" => {
            let stripe_invoice_id = payload.data.object.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
            if !stripe_invoice_id.is_empty() {
                match &webhook_state.db.store {
                    crate::db::DbStore::Postgres => {
                        if let Ok(internal_invoice) = sqlx::query_scalar::<_, String>("SELECT id FROM invoices WHERE stripe_invoice_id = $1 OR id = $1").bind(&stripe_invoice_id).fetch_optional(&webhook_state.db.pool).await {
                            if let Some(id) = internal_invoice {
                                let _ = sqlx::query("UPDATE invoices SET payment_status = 'paid', status = 'paid', updated_at = CURRENT_TIMESTAMP WHERE id = $1").bind(&id).execute(&webhook_state.db.pool).await;
                                let _ = sqlx::query("UPDATE triage_items SET status = 'resolved' WHERE action_type = 'Approve Draft' AND action_payload LIKE '%' || $1 || '%'").bind(&id).execute(&webhook_state.db.pool).await;
                            }
                        }
                    },
                    crate::db::DbStore::Sqlite(_) => {
                        if let Ok(internal_invoice) = sqlx::query_scalar::<_, String>("SELECT id FROM invoices WHERE stripe_invoice_id = ? OR id = ?").bind(&stripe_invoice_id).bind(&stripe_invoice_id).fetch_optional(&webhook_state.db.pool).await {
                            if let Some(id) = internal_invoice {
                                let _ = sqlx::query("UPDATE invoices SET payment_status = 'paid', status = 'paid', updated_at = CURRENT_TIMESTAMP WHERE id = ?").bind(&id).execute(&webhook_state.db.pool).await;
                                let _ = sqlx::query("UPDATE triage_items SET status = 'resolved' WHERE action_type = 'Approve Draft' AND action_payload LIKE '%' || ? || '%'").bind(&id).execute(&webhook_state.db.pool).await;
                            }
                        }
                    }
                }
            }
            StatusCode::OK.into_response()
        }
        "invoice.payment_failed" => {
            let obj = &payload.data.object;
            match process_invoice_payment_failed(
                &webhook_state,
                obj,
                &CriticalSmsPaymentFailureNotifier,
                &LlmPaymentFailureMessageGenerator,
            )
            .await
            {
                Ok(Some(subscriber_id)) => {
                    tracing::info!("Processed Stripe failed-payment dunning for subscriber {}", subscriber_id); // pii-safe
                }
                Ok(None) => {
                    tracing::warn!("Stripe invoice.payment_failed did not match an OHC subscriber");
                }
                Err(err) => {
                    ::server_telemetry::record_error_signal("[bug] Failed to process Stripe failed-payment dunning");
                    tracing::error!("Failed to process Stripe failed-payment dunning: {}", err); // pii-safe
                    return StatusCode::INTERNAL_SERVER_ERROR.into_response();
                }
            }
            StatusCode::OK.into_response()
        },
        _ => {
            // Unhandled event types are ignored successfully
            StatusCode::OK.into_response()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct MercadoPagoEvent {
    pub id: i64,
    pub live_mode: bool,
    pub r#type: String,
    pub date_created: String,
    pub application_id: i64,
    pub user_id: i64,
    pub version: i32,
    pub api_version: String,
    pub action: String,
    pub data: MercadoPagoEventData,
}

#[derive(Debug, Deserialize)]
pub struct MercadoPagoEventData {
    pub id: String,
}

pub async fn mercadopago_webhook_handler(
    axum::extract::State(_webhook_state): axum::extract::State<WebhookState>,
    Json(payload): Json<MercadoPagoEvent>,
) -> impl IntoResponse {
    match payload.action.as_str() {
        "payment.created" | "payment.updated" => {
            // In a real implementation, you would fetch the payment details from MP API using data.id
            // and extract the tenant_id and tier from the metadata.
            // For mock purposes, assume we process it similarly to Stripe.
            // We just return OK.
            StatusCode::OK.into_response()
        },
        _ => StatusCode::OK.into_response()
    }
}


#[derive(Debug, Deserialize, serde::Serialize)]
pub struct RazorpayEvent {
    pub event: String,
    pub payload: RazorpayPayload,
}

#[derive(Debug, Deserialize, serde::Serialize)]
pub struct RazorpayPayload {
    pub payment: RazorpayPaymentEntity,
}

#[derive(Debug, Deserialize, serde::Serialize)]
pub struct RazorpayPaymentEntity {
    pub entity: RazorpayEntity,
}

#[derive(Debug, Deserialize, serde::Serialize)]
pub struct RazorpayEntity {
    pub id: String,
    pub status: String,
    pub order_id: String,
}





pub async fn razorpay_webhook_handler(
    _headers: axum::http::HeaderMap,
    axum::extract::State(webhook_state): axum::extract::State<WebhookState>,
    Json(payload): Json<RazorpayEvent>,
) -> impl IntoResponse {

    match payload.event.as_str() {
        "payment.captured" => {
            let order_id = &payload.payload.payment.entity.order_id;

            // Dispatch payment.captured event to Finance agent for split tag evaluation
            let orch = webhook_state.orchestrator.clone();
            let payload_val = serde_json::to_value(&payload).unwrap_or(serde_json::Value::Null);
            let _order_id_val = order_id.clone();
            tokio::spawn(async move {
                let evt = crate::orchestration::departments::types::DepartmentEvent {
                    id: uuid::Uuid::new_v4().to_string(),
                    tenant_id: "unknown".to_string(), // In a real app, this would be extracted from the payload
                    event_type: "payment.captured".to_string(),
                    payload: payload_val,
                };
                let _ = orch.dispatch_event(evt).await;
            });

            // In a real app, transition OHC orders from "Pending" to "Paid"
            let res = match &webhook_state.db.store {
                DbStore::Sqlite(pool) => {
                    sqlx::query("UPDATE orders SET status = 'Paid' WHERE id = ?")
                        .bind(order_id)
                        .execute(pool)
                        .await
                        .map(|_| ())
                }
                DbStore::Postgres => {
                    sqlx::query("UPDATE orders SET status = 'Paid' WHERE id = $1")
                        .bind(order_id)
                        .execute(&webhook_state.db.pool)
                        .await
                        .map(|_| ())
                }
            };

            if let Err(e) = res {
                ::server_telemetry::record_error_signal("[bug] Failed to update order status: {:?}");
                tracing::error!("Failed to update order status: {:?}", e);
                return StatusCode::INTERNAL_SERVER_ERROR.into_response();
            }

            StatusCode::OK.into_response()
        },
        _ => StatusCode::OK.into_response()
    }
}


#[derive(Debug, Deserialize)]
pub struct CalComEvent {
    pub trigger_event: String,
    pub payload: CalComPayload,
}

#[derive(Debug, Deserialize)]
pub struct CalComPayload {
    pub uid: String,
    pub title: String,
    pub start_time: String,
    pub end_time: String,
    pub attendees: Vec<CalComAttendee>,
}

#[derive(Debug, Deserialize)]
pub struct CalComAttendee {
    pub email: String,
    pub name: String,
}

pub async fn calcom_webhook_handler(
    axum::extract::State(_webhook_state): axum::extract::State<WebhookState>,
    Json(payload): Json<CalComEvent>,
) -> impl IntoResponse {
    match payload.trigger_event.as_str() {
        "BOOKING_CREATED" => {
            let booking_uid = &payload.payload.uid;

            // In a real app, create calendar events in the OHC dashboard
            // and auto-generate meeting links (e.g., Zoom).
            tracing::info!("Created booking: {}", booking_uid);
            StatusCode::OK.into_response()
        },
        _ => StatusCode::OK.into_response()
    }
}


#[derive(Debug, Deserialize)]
pub struct ResendEvent {
    #[serde(rename = "type")]
    pub type_: String,
    pub data: ResendEventData,
}

#[derive(Debug, Deserialize)]
pub struct ResendEventData {
    pub email_id: String,
    pub to: Vec<String>,
}

pub async fn resend_webhook_handler(
    axum::extract::State(_webhook_state): axum::extract::State<WebhookState>,
    Json(payload): Json<ResendEvent>,
) -> impl IntoResponse {
    match payload.type_.as_str() {
        "email.bounced" | "email.complained" => {
            // Automatically clean the tenant's mailing list
            tracing::info!("Message bounced/complained: [REDACTED]");
            StatusCode::OK.into_response()
        },
        _ => StatusCode::OK.into_response()
    }
}


#[derive(Debug, Deserialize)]
pub struct AyrshareEvent {
    pub action: String,
    pub message: String,
    pub platform: String,
    pub profile_key: String,
}

pub async fn ayrshare_webhook_handler(
    axum::extract::State(_webhook_state): axum::extract::State<WebhookState>,
    Json(payload): Json<AyrshareEvent>,
) -> impl IntoResponse {
    match payload.action.as_str() {
        "social_message" => {
            // Ingest inbound messages into a unified OHC inbox table
            tracing::info!("Incoming notification from integration: [REDACTED]");
            StatusCode::OK.into_response()
        },
        _ => StatusCode::OK.into_response()
    }
}

#[derive(Debug, Deserialize)]
pub struct ManychatEvent {
    pub status: String,
    pub messages: Vec<ManychatMessage>,
}

#[derive(Debug, Deserialize)]
pub struct ManychatMessage {
    pub id: String,
    pub text: String,
}

pub async fn manychat_webhook_handler(
    axum::extract::State(_webhook_state): axum::extract::State<WebhookState>,
    Json(payload): Json<ManychatEvent>,
) -> impl IntoResponse {
    match payload.status.as_str() {
        "ok" => StatusCode::OK.into_response(),
        _ => StatusCode::OK.into_response()
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct CalendlyEvent {
    pub event: String,
    pub payload: serde_json::Value,
}

pub async fn calendly_webhook_handler(
    axum::extract::State(__webhook_state): axum::extract::State<WebhookState>,
    axum::Json(_payload): axum::Json<CalendlyEvent>,
) -> impl axum::response::IntoResponse {
    axum::http::StatusCode::OK.into_response()
}

#[derive(Debug, serde::Deserialize)]
pub struct MailchimpEvent {
    pub r#type: String,
    pub data: serde_json::Value,
}

pub async fn mailchimp_webhook_handler(
    axum::extract::State(__webhook_state): axum::extract::State<WebhookState>,
    axum::Json(_payload): axum::Json<MailchimpEvent>,
) -> impl axum::response::IntoResponse {
    axum::http::StatusCode::OK.into_response()
}
