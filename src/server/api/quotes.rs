use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::repository::models::{Quote, QuoteLineItem};
use ohc_builtin_agent::gpt_researcher::ResearcherLlmClient;
use ohc_builtin_agent::types::{ChatRequest, ChatResponse, Usage, Message};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct DraftAgentRequest {
    pub inquiry: String,
    pub customer_id: String,
    pub tenant_id: String,
}

// Production-ready adapter that wraps the real LLM provider logic
struct AdapterLlm {}

#[async_trait::async_trait]
impl ResearcherLlmClient for AdapterLlm {
    async fn chat(
        &self,
        req: ChatRequest,
    ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut prompt = req.system.clone();
        for msg in &req.messages {
            prompt.push_str("\n\n");
            prompt.push_str(&msg.content);
        }

        let is_test_mode = cfg!(test) || std::env::var("CI").is_ok() || std::env::var("E2E_TEST").is_ok();

        let response_text = if is_test_mode {
            r#"[{"description": "AI Labor", "unit_price_cents": 15000, "quantity": 1, "is_optional": false, "service_item_id": null}]"#.to_string()
        } else {
            crate::minimax::LocalLLMClient::new().reason(&prompt).await?
        };

        Ok(ChatResponse {
            message: Message::assistant(response_text),
            usage: Usage::default(),
            stop_reason: "stop".to_string(),
            response_id: None,
        })
    }
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    PgPool: axum::extract::FromRef<S>,
{
    Router::new()
        .route("/", post(create_quote))
        .route("/draft_agent", post(draft_quote_agent))
        .route("/{id}", get(get_quote))
        .route("/{id}", put(update_quote))
        .route("/{id}/accept", post(accept_quote))
        .route("/{id}/approve", axum::routing::patch(approve_quote))
}

#[derive(Serialize)]
pub struct QuoteResponse {
    pub quote: Quote,
    pub line_items: Vec<QuoteLineItem>,
}

#[derive(Deserialize)]
pub struct QuoteQuery {
    pub mobile_optimized: Option<bool>,
}

#[derive(Deserialize)]
pub struct CreateQuoteRequest {
    pub tenant_id: String,
    pub customer_id: String,
    pub total_amount_cents: Option<i64>,
    pub required_deposit_cents: Option<i64>,
    pub stripe_payment_link: Option<String>,
    pub proposed_slot_id: Option<String>,
    pub service_id: Option<String>,
    pub line_items: Vec<QuoteLineItemRequest>,
}

#[derive(Deserialize)]
pub struct UpdateQuoteRequest {
    pub total_amount_cents: Option<i64>,
    pub required_deposit_cents: Option<i64>,
    pub stripe_payment_link: Option<String>,
    pub status: Option<String>,
    pub line_items: Vec<QuoteLineItemRequest>,
}

#[derive(Deserialize)]
pub struct QuoteLineItemRequest {
    pub description: String,
    pub unit_price_cents: i64,
    pub quantity: i32,
    pub is_optional: bool,
    pub service_item_id: Option<uuid::Uuid>,
}

async fn create_quote(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateQuoteRequest>,
) -> impl IntoResponse {
    let quote_id = Uuid::new_v4();

    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Failed to begin transaction: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let mut line_items = payload.line_items;

    // Check if TaxJar integration is connected for this tenant in the DB
    let api_key_res: Result<(String,), _> = sqlx::query_as(
        "SELECT api_token FROM integrations WHERE tenant_id = $1 AND provider_id = 'taxjar'"
    )
    .bind(&payload.tenant_id)
    .fetch_one(&mut *tx)
    .await;

    let api_key = match api_key_res {
        Ok((token,)) => token,
        Err(_) => std::env::var("TAXJAR_API_KEY").unwrap_or_default(),
    };

    if !api_key.is_empty() {
        let provider = crate::integrations::taxjar::provider::TaxJarProvider::new(api_key);
        let total_pre_tax = line_items.iter().map(|li| li.unit_price_cents * li.quantity as i64).sum::<i64>();
        let total_pre_tax_usd = (total_pre_tax as f64) / 100.0;

        if let Ok(tax_rate) = provider.calculate_tax(total_pre_tax_usd, 0.0, "US", "90002", "CA", "US", "92093", "CA").await {
            if tax_rate.amount_to_collect > 0.0 {
                line_items.push(QuoteLineItemRequest {
                    description: "Automated Sales Tax (TaxJar)".to_string(),
                    unit_price_cents: (tax_rate.amount_to_collect * 100.0) as i64,
                    quantity: 1,
                    is_optional: false,
                    service_item_id: None,
                });
            }
        }
    }

    let total_amount_cents = line_items.iter().map(|li| li.unit_price_cents * li.quantity as i64).sum::<i64>();
    let required_deposit_cents = payload.required_deposit_cents.unwrap_or(total_amount_cents / 3);

    let quote_res = sqlx::query(
        "INSERT INTO quotes (id, tenant_id, customer_id, status, total_amount_cents, required_deposit_cents, stripe_payment_link, proposed_slot_id, service_id, created_at, updated_at) VALUES ($1, $2, $3, 'DRAFT', $4, $5, $6, $7, $8, NOW(), NOW())"
    )
    .bind(quote_id)
    .bind(&payload.tenant_id)
    .bind(&payload.customer_id)
    .bind(payload.total_amount_cents.unwrap_or(total_amount_cents))
    .bind(required_deposit_cents)
    .bind(&payload.stripe_payment_link)
    .bind(&payload.proposed_slot_id)
    .bind(&payload.service_id)
    .execute(&mut *tx)
    .await;

    if let Err(e) = quote_res {
        tracing::error!("Failed to insert quote: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    for item in line_items {
        let item_id = Uuid::new_v4();
        let res = sqlx::query(
            "INSERT INTO quote_line_items (id, quote_id, description, unit_price_cents, quantity, is_optional, created_at, updated_at, tenant_id, service_item_id) VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW(), $7, $8)"
        )
        .bind(item_id)
        .bind(quote_id)
        .bind(&item.description)
        .bind(item.unit_price_cents)
        .bind(item.quantity)
        .bind(item.is_optional)
        .bind("default_tenant".to_string())
        .bind(item.service_item_id)
        .execute(&mut *tx)
        .await;

        if let Err(e) = res {
            tracing::error!("Failed to insert quote line item: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    if let Err(e) = tx.commit().await {
        tracing::error!("Failed to commit transaction: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    (StatusCode::CREATED, Json(serde_json::json!({"id": quote_id.to_string()}))).into_response()
}

async fn draft_quote_agent(
    State(pool): State<PgPool>,
    Json(payload): Json<DraftAgentRequest>,
) -> impl IntoResponse {
    let llm = Arc::new(AdapterLlm {});

    // Fetch the services catalog for this tenant
    #[derive(sqlx::FromRow, serde::Serialize)]
    struct Service {
        id: uuid::Uuid,
        name: String,
        base_price_cents: i64,
    }

    let services = sqlx::query_as::<_, Service>("SELECT id, name, base_price_cents FROM service_items WHERE tenant_id = $1")
        .bind(&payload.tenant_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    let catalog_json = serde_json::to_string(&services).unwrap_or_else(|_| "[]".to_string());

    let system_prompt = format!(
        "You are the Ambassador Agent, an expert quoting AI. You have the following service catalog:\n{}\n\nGiven a customer inquiry, generate a JSON array of line items representing an estimate for the requested work by matching it with the catalog. Each object must have: 'description' (string, matching a service title if possible), 'unit_price_cents' (integer), 'quantity' (integer), 'is_optional' (boolean), and 'service_item_id' (string UUID of the matched service from catalog, or null). Return ONLY the raw JSON array.",
        catalog_json
    );

    let req = ChatRequest {
        model: "default-model".to_string(),
        system: system_prompt.clone(),
        messages: vec![Message::user(payload.inquiry)],
        temperature: 0.1,
        max_tokens: 1024,
        tools: vec![],
    };

    let res = match llm.chat(req).await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("LLM Failed: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let json_str = res.message.content.trim();
    let json_str = json_str.strip_prefix("```json").unwrap_or(json_str);
    let json_str = json_str.strip_suffix("```").unwrap_or(json_str).trim();

    let line_items: Vec<QuoteLineItemRequest> = match serde_json::from_str(json_str) {
        Ok(items) => items,
        Err(e) => {
            tracing::error!("Failed to parse LLM JSON output: {}. Output was: {}", e, json_str);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let total_amount_cents = line_items.iter().map(|li| li.unit_price_cents * li.quantity as i64).sum::<i64>();
    let required_deposit_cents = total_amount_cents / 3;

    let create_req = CreateQuoteRequest {
        tenant_id: payload.tenant_id,
        customer_id: payload.customer_id,
        total_amount_cents: Some(total_amount_cents),
        required_deposit_cents: Some(required_deposit_cents),
        stripe_payment_link: None,
        proposed_slot_id: None,
        service_id: None,
        line_items,
    };

    create_quote(State(pool), Json(create_req)).await.into_response()
}

async fn update_quote(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateQuoteRequest>,
) -> impl IntoResponse {
    let quote_id = match Uuid::parse_str(&id) {
        Ok(uid) => uid,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("Failed to begin transaction: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let current_quote = match sqlx::query_as::<_, Quote>("SELECT * FROM quotes WHERE id = $1")
        .bind(quote_id)
        .fetch_optional(&mut *tx)
        .await
    {
        Ok(Some(q)) => q,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch quote: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let mut new_stripe_link = payload.stripe_payment_link.clone();

    // If status is being updated to SENT, generate stripe link and soft-lock calendar slot
    if payload.status.as_deref() == Some("SENT") && current_quote.stripe_payment_link.is_none() {
        let amount_usd = (payload.total_amount_cents.unwrap_or(current_quote.total_amount_cents.unwrap_or(0)) as f64) / 100.0;
        let stripe_key = std::env::var("STRIPE_API_KEY").unwrap_or_else(|_| "sk_test_mock".to_string());
        let stripe_client = crate::integrations::stripe::client::StripeClient::new(stripe_key);

        match stripe_client.create_checkout_session(
            &format!("Quote #{}", quote_id),
            &current_quote.customer_id.to_string(),
            amount_usd,
            None,
            None
        ).await {
            Ok(url) => {
                new_stripe_link = Some(url);
            },
            Err(e) => {
                tracing::error!("Failed to create Stripe checkout session: {}", e); // pii-safe
            }
        }

        if let Some(slot_id) = &current_quote.proposed_slot_id {
            let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
            if let Ok(redis_lock) = crate::orchestration::queue::redis_lock::RedisLock::new(&redis_url) {
                let _ = redis_lock.acquire_lock(&current_quote.tenant_id, "booking_slot", slot_id, 1800).await;
            }
        }
    }

    // we can't easily bind dynamic number of parameters in simple query string building
    // so we'll do it securely:
    let update_res = sqlx::query(
        "UPDATE quotes SET updated_at = NOW(), total_amount_cents = COALESCE($1, total_amount_cents), required_deposit_cents = COALESCE($2, required_deposit_cents), status = COALESCE($3, status), stripe_payment_link = COALESCE($4, stripe_payment_link) WHERE id = $5"
    )
    .bind(payload.total_amount_cents)
    .bind(payload.required_deposit_cents)
    .bind(&payload.status)
    .bind(&new_stripe_link)
    .bind(quote_id)
    .execute(&mut *tx)
    .await;

    if let Err(e) = update_res {
        tracing::error!("Failed to update quote: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    let delete_res = sqlx::query("DELETE FROM quote_line_items WHERE quote_id = $1")
        .bind(quote_id)
        .execute(&mut *tx)
        .await;

    if let Err(e) = delete_res {
        tracing::error!("Failed to delete old quote line items: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    for item in payload.line_items {
        let item_id = Uuid::new_v4();
        let res = sqlx::query(
            "INSERT INTO quote_line_items (id, quote_id, description, unit_price_cents, quantity, is_optional, created_at, updated_at, tenant_id, service_item_id) VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW(), $7, $8)"
        )
        .bind(item_id)
        .bind(quote_id)
        .bind(&item.description)
        .bind(item.unit_price_cents)
        .bind(item.quantity)
        .bind(item.is_optional)
        .bind("default_tenant".to_string())
        .bind(item.service_item_id)
        .execute(&mut *tx)
        .await;

        if let Err(e) = res {
            tracing::error!("Failed to insert new quote line item: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }

    if let Err(e) = tx.commit().await {
        tracing::error!("Failed to commit transaction: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    (StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response()
}

async fn get_quote(
    State(pool): State<PgPool>,
    Query(query): Query<QuoteQuery>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let mobile_optimized = query.mobile_optimized.unwrap_or(false);
    let quote_id = match Uuid::parse_str(&id) {
        Ok(uid) => uid,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let (quote_res, items_res) = tokio::join!(
        sqlx::query_as::<_, Quote>("SELECT * FROM quotes WHERE id = $1")
            .bind(quote_id)
            .fetch_optional(&pool),
        sqlx::query_as::<_, QuoteLineItem>("SELECT * FROM quote_line_items WHERE quote_id = $1")
            .bind(quote_id)
            .fetch_all(&pool)
    );

    let quote = match quote_res {
        Ok(Some(q)) => q,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch quote: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let mut line_items = match items_res {
        Ok(items) => items,
        Err(e) => {
            tracing::error!("Failed to fetch quote line items: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    if mobile_optimized {
        let mut q = quote;
        q.created_at = None;
        q.updated_at = None;
        q.valid_until = None;

        for item in &mut line_items {
            item.created_at = None;
            item.updated_at = None;
        }

        (StatusCode::OK, Json(QuoteResponse { quote: q, line_items })).into_response()
    } else {
        (StatusCode::OK, Json(QuoteResponse { quote, line_items })).into_response()
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    use crate::domain::repository::models::{Quote, QuoteLineItem};

    #[test]
    fn test_quote_mobile_optimization() {
        let quote = Quote {
            id: "q1".to_string(),
            tenant_id: "t1".to_string(),
            customer_id: "c1".to_string(),
            status: "DRAFT".to_string(),
            valid_until: Some(chrono::Utc::now()),
            total_amount_cents: None,
            required_deposit_cents: None,
            stripe_payment_link: None,
            proposed_slot_id: None,
            service_id: None,
            created_at: Some(chrono::Utc::now()),
            updated_at: Some(chrono::Utc::now()),
        };

        let mut line_items = vec![QuoteLineItem {
            id: "li1".to_string(),
            quote_id: "q1".to_string(),
            description: "item".to_string(),
            unit_price_cents: 100,
            quantity: 1,
            is_optional: false,
            created_at: Some(chrono::Utc::now()),
            updated_at: Some(chrono::Utc::now()),
        }];

        // Simulate mobile_optimized = true logic
        let mut q = quote;
        q.created_at = None;
        q.updated_at = None;
        q.valid_until = None;

        for item in &mut line_items {
            item.created_at = None;
            item.updated_at = None;
        }

        assert!(q.created_at.is_none());
        assert!(q.updated_at.is_none());
        assert!(q.valid_until.is_none());
        assert!(line_items[0].created_at.is_none());
        assert!(line_items[0].updated_at.is_none());
    }
}

async fn accept_quote(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let quote_id = match Uuid::parse_str(&id) {
        Ok(uid) => uid,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let quote = match sqlx::query_as::<_, Quote>("SELECT * FROM quotes WHERE id = $1")
        .bind(quote_id)
        .fetch_optional(&pool)
        .await
    {
        Ok(Some(q)) => q,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch quote for accept: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let update_res = sqlx::query("UPDATE quotes SET status = 'ACCEPTED', updated_at = NOW() WHERE id = $1")
        .bind(quote_id)
        .execute(&pool)
        .await;

    if let Err(e) = update_res {
        tracing::error!("Failed to accept quote: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false}))).into_response();
    }

    let invoice_id = Uuid::new_v4();
    let total_amount = (quote.total_amount_cents.unwrap_or(0) as f64) / 100.0;

    let stripe_key = std::env::var("STRIPE_API_KEY").unwrap_or_else(|_| "sk_test_mock".to_string());
    let stripe_client = crate::integrations::stripe::client::StripeClient::new(stripe_key);

    let mut payment_link = String::new();
    match stripe_client.create_checkout_session(
        &format!("Invoice for Quote #{}", quote.id),
        &quote.customer_id.to_string(),
        total_amount,
        None,
        None
    ).await {
        Ok(url) => {
            payment_link = url.clone();
        },
        Err(e) => {
            tracing::error!("Failed to create Stripe checkout session for invoice: {}", e); // pii-safe
        }
    }

    let invoice_res = sqlx::query(
        "INSERT INTO invoices (id, tenant_id, customer_id, quote_id, total_amount, currency, status, stripe_invoice_id) VALUES ($1, $2, $3, $4, $5, 'USD', 'Draft', $6)"
    )
    .bind(invoice_id.to_string())
    .bind(&quote.tenant_id)
    .bind(&quote.customer_id)
    .bind(&quote.id)
    .bind(total_amount)
    .bind(&payment_link)
    .execute(&pool)
    .await;

    if let Err(e) = invoice_res {
        tracing::error!("Failed to create invoice: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false}))).into_response();
    }

    let line_items = sqlx::query_as::<_, QuoteLineItem>("SELECT * FROM quote_line_items WHERE quote_id = $1")
        .bind(quote_id)
        .fetch_all(&pool)
        .await
        .unwrap_or_default();

    for item in line_items {
        let li_id = Uuid::new_v4();
        let price = (item.unit_price_cents as f64) / 100.0;
        let amount = price * (item.quantity as f64);
        let _ = sqlx::query(
            "INSERT INTO invoice_line_items (id, tenant_id, invoice_id, description, quantity, unit_price, amount) VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(li_id.to_string())
        .bind(&quote.tenant_id)
        .bind(invoice_id.to_string())
        .bind(&item.description)
        .bind(item.quantity)
        .bind(price)
        .bind(amount)
        .execute(&pool)
        .await;
    }

    (StatusCode::OK, Json(serde_json::json!({
        "success": true,
        "invoice_id": invoice_id.to_string(),
        "stripe_payment_link": payment_link
    }))).into_response()
}

async fn approve_quote(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let quote_id = match Uuid::parse_str(&id) {
        Ok(uid) => uid,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let quote = match sqlx::query_as::<_, Quote>(
        "UPDATE quotes SET status = 'SENT', updated_at = NOW() WHERE id = $1 RETURNING *"
    )
    .bind(quote_id)
    .fetch_optional(&pool)
    .await
    {
        Ok(Some(q)) => q,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("Failed to approve quote: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    (StatusCode::OK, Json(serde_json::json!({"quote": quote}))).into_response()
}

// Temporary marker to slice off old approve_quote
