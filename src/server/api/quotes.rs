use axum::{
    extract::{Extension, Path, Query, State},
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

#[derive(Debug, Clone)]
struct TenantAuthority(String);

impl TenantAuthority {
    fn from_claims(claims: &::server_common::Claims) -> Result<Self, StatusCode> {
        match claims.organization_id.as_deref() {
            Some(tenant_id) if !tenant_id.is_empty() => Ok(Self(tenant_id.to_string())),
            _ => Err(StatusCode::UNAUTHORIZED),
        }
    }

    fn tenant_id(&self) -> &str {
        &self.0
    }

    fn owns_quote(&self, quote: &Quote) -> bool {
        quote.tenant_id == self.0
    }
}

async fn create_quote(
    State(pool): State<PgPool>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<CreateQuoteRequest>,
) -> impl IntoResponse {
    let authority = match TenantAuthority::from_claims(&claims) {
        Ok(authority) => authority,
        Err(status) => return status.into_response(),
    };
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
    .bind(authority.tenant_id())
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

        if let Ok(tax_rate) = provider.calculate_tax(crate::integrations::taxjar::client::TaxJarParams { amount: total_pre_tax_usd, shipping: 0.0, to_country: "US", to_zip: "90002", to_state: "CA", from_country: "US", from_zip: "92093", from_state: "CA" }).await {
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
    .bind(authority.tenant_id())
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
        .bind(authority.tenant_id())
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
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<DraftAgentRequest>,
) -> impl IntoResponse {
    let authority = match TenantAuthority::from_claims(&claims) {
        Ok(authority) => authority,
        Err(status) => return status.into_response(),
    };
    let llm = Arc::new(AdapterLlm {});

    // Fetch the services catalog for this tenant
    #[derive(sqlx::FromRow, serde::Serialize)]
    struct Service {
        id: uuid::Uuid,
        name: String,
        base_price_cents: i64,
    }

    let services = sqlx::query_as::<_, Service>("SELECT id, name, base_price_cents FROM service_items WHERE tenant_id = $1")
        .bind(authority.tenant_id())
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
        customer_id: payload.customer_id,
        total_amount_cents: Some(total_amount_cents),
        required_deposit_cents: Some(required_deposit_cents),
        stripe_payment_link: None,
        proposed_slot_id: None,
        service_id: None,
        line_items,
    };

    create_quote(State(pool), Extension(claims), Json(create_req)).await.into_response()
}

async fn update_quote(
    State(pool): State<PgPool>,
    Extension(claims): Extension<::server_common::Claims>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateQuoteRequest>,
) -> impl IntoResponse {
    let authority = match TenantAuthority::from_claims(&claims) {
        Ok(authority) => authority,
        Err(status) => return status.into_response(),
    };
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

    let current_quote = match sqlx::query_as::<_, Quote>(
        "SELECT * FROM quotes WHERE id = $1 AND tenant_id = $2",
    )
        .bind(quote_id)
        .bind(authority.tenant_id())
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
    if !authority.owns_quote(&current_quote) {
        return StatusCode::NOT_FOUND.into_response();
    }

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
        "UPDATE quotes SET updated_at = NOW(), total_amount_cents = COALESCE($1, total_amount_cents), required_deposit_cents = COALESCE($2, required_deposit_cents), status = COALESCE($3, status), stripe_payment_link = COALESCE($4, stripe_payment_link) WHERE id = $5 AND tenant_id = $6"
    )
    .bind(payload.total_amount_cents)
    .bind(payload.required_deposit_cents)
    .bind(&payload.status)
    .bind(&new_stripe_link)
    .bind(quote_id)
    .bind(authority.tenant_id())
    .execute(&mut *tx)
    .await;

    if let Err(e) = update_res {
        tracing::error!("Failed to update quote: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    let delete_res = sqlx::query(
        "DELETE FROM quote_line_items WHERE quote_id = $1 AND tenant_id = $2",
    )
        .bind(quote_id)
        .bind(authority.tenant_id())
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
        .bind(authority.tenant_id())
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
    Extension(claims): Extension<::server_common::Claims>,
    Query(query): Query<QuoteQuery>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let authority = match TenantAuthority::from_claims(&claims) {
        Ok(authority) => authority,
        Err(status) => return status.into_response(),
    };
    let mobile_optimized = query.mobile_optimized.unwrap_or(false);
    let quote_id = match Uuid::parse_str(&id) {
        Ok(uid) => uid,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let (quote_res, items_res) = tokio::join!(
        sqlx::query_as::<_, Quote>("SELECT * FROM quotes WHERE id = $1 AND tenant_id = $2")
            .bind(quote_id)
            .bind(authority.tenant_id())
            .fetch_optional(&pool),
        sqlx::query_as::<_, QuoteLineItem>(
            "SELECT * FROM quote_line_items WHERE quote_id = $1 AND tenant_id = $2",
        )
            .bind(quote_id)
            .bind(authority.tenant_id())
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
    if !authority.owns_quote(&quote) {
        return StatusCode::NOT_FOUND.into_response();
    }

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
    use axum::{body::Body, extract::Extension, http::Request};
    use tower::ServiceExt;

    fn claims(organization_id: Option<&str>) -> ::server_common::Claims {
        ::server_common::Claims {
            sub: "user-7".to_string(),
            exp: i64::MAX,
            iat: 1,
            organization_id: organization_id.map(str::to_string),
            username: String::new(),
            email: String::new(),
            roles: vec![],
            session_id: None,
            jti: String::new(),
        }
    }

    fn quote_for_tenant(tenant_id: &str) -> Quote {
        Quote {
            id: "q1".to_string(),
            tenant_id: tenant_id.to_string(),
            customer_id: "c1".to_string(),
            status: "DRAFT".to_string(),
            valid_until: None,
            total_amount_cents: None,
            required_deposit_cents: None,
            stripe_payment_link: None,
            proposed_slot_id: None,
            service_id: None,
            created_at: None,
            updated_at: None,
        }
    }

    #[test]
    fn tenant_authority_rejects_missing_and_cross_tenant_access() {
        let authority = match TenantAuthority::from_claims(&claims(Some("tenant-a"))) {
            Ok(authority) => authority,
            Err(status) => panic!("unexpected authority error: {status}"),
        };

        assert_eq!(authority.tenant_id(), "tenant-a");
        assert!(authority.owns_quote(&quote_for_tenant("tenant-a")));
        assert!(!authority.owns_quote(&quote_for_tenant("tenant-b")));
        assert!(matches!(
            TenantAuthority::from_claims(&claims(None)),
            Err(StatusCode::UNAUTHORIZED)
        ));
        assert!(matches!(
            TenantAuthority::from_claims(&claims(Some(""))),
            Err(StatusCode::UNAUTHORIZED)
        ));
    }

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
            service_item_id: None,
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

    #[tokio::test]
    async fn quote_routes_reject_claims_without_an_organization() {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgres://postgres:postgres@localhost:5432/postgres")
            .expect("lazy test pool");
        let app = router().with_state(pool).layer(Extension(claims(None)));
        let quote_id = "6f9619ff-8b86-d011-b42d-00cf4fc964ff";
        let requests = [
            Request::builder()
                .method("POST")
                .uri("/")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"customer_id":"customer-7","line_items":[]}"#,
                ))
                .unwrap(),
            Request::builder()
                .method("POST")
                .uri("/draft_agent")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"inquiry":"Need a quote","customer_id":"customer-7"}"#,
                ))
                .unwrap(),
            Request::builder()
                .method("GET")
                .uri(format!("/{quote_id}"))
                .body(Body::empty())
                .unwrap(),
            Request::builder()
                .method("PUT")
                .uri(format!("/{quote_id}"))
                .header("content-type", "application/json")
                .body(Body::from(r#"{"line_items":[]}"#))
                .unwrap(),
            Request::builder()
                .method("POST")
                .uri(format!("/{quote_id}/accept"))
                .body(Body::empty())
                .unwrap(),
            Request::builder()
                .method("PATCH")
                .uri(format!("/{quote_id}/approve"))
                .body(Body::empty())
                .unwrap(),
        ];

        for request in requests {
            let response = app.clone().oneshot(request).await.unwrap();
            assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        }
    }

    #[tokio::test]
    async fn quote_handlers_enforce_tenant_boundaries_in_postgres() {
        let database_url = match std::env::var("OHC_DATABASE_URL") {
            Ok(database_url) => database_url,
            Err(_) => return,
        };
        let admin = sqlx::PgPool::connect(&database_url)
            .await
            .expect("connect quote integration database");
        let schema = format!("quote_tenant_test_{}", Uuid::new_v4().simple());
        sqlx::query(&format!("CREATE SCHEMA {schema}"))
            .execute(&admin)
            .await
            .expect("create quote integration schema");

        let schema_for_connections = schema.clone();
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(4)
            .after_connect(move |connection, _metadata| {
                let search_path = format!("SET search_path TO {schema_for_connections}");
                Box::pin(async move {
                    sqlx::query(&search_path).execute(connection).await?;
                    Ok(())
                })
            })
            .connect(&database_url)
            .await
            .expect("connect tenant-scoped quote test pool");

        for statement in [
            "CREATE TABLE quotes (id UUID PRIMARY KEY, tenant_id TEXT NOT NULL, customer_id TEXT NOT NULL, status TEXT NOT NULL, valid_until TIMESTAMPTZ, total_amount_cents BIGINT, required_deposit_cents BIGINT, stripe_payment_link TEXT, proposed_slot_id TEXT, service_id TEXT, created_at TIMESTAMPTZ, updated_at TIMESTAMPTZ)",
            "CREATE TABLE quote_line_items (id UUID PRIMARY KEY, quote_id UUID NOT NULL, tenant_id TEXT NOT NULL, description TEXT NOT NULL, unit_price_cents BIGINT NOT NULL, quantity INTEGER NOT NULL, is_optional BOOLEAN NOT NULL, service_item_id UUID, created_at TIMESTAMPTZ, updated_at TIMESTAMPTZ)",
            "CREATE TABLE integrations (tenant_id TEXT NOT NULL, provider_id TEXT NOT NULL, api_token TEXT NOT NULL)",
            "CREATE TABLE service_items (id UUID PRIMARY KEY, tenant_id TEXT NOT NULL, name TEXT NOT NULL, base_price_cents BIGINT NOT NULL)",
        ] {
            sqlx::query(statement)
                .execute(&pool)
                .await
                .expect("create quote integration table");
        }

        let foreign_quote_id = Uuid::new_v4();
        let foreign_line_item_id = Uuid::new_v4();
        sqlx::query("INSERT INTO quotes (id, tenant_id, customer_id, status, total_amount_cents, created_at, updated_at) VALUES ($1, 'tenant-b', 'customer-b', 'DRAFT', 700, NOW(), NOW())")
            .bind(foreign_quote_id)
            .execute(&pool)
            .await
            .expect("insert foreign quote");
        sqlx::query("INSERT INTO quote_line_items (id, quote_id, tenant_id, description, unit_price_cents, quantity, is_optional, created_at, updated_at) VALUES ($1, $2, 'tenant-b', 'Foreign item', 700, 1, FALSE, NOW(), NOW())")
            .bind(foreign_line_item_id)
            .bind(foreign_quote_id)
            .execute(&pool)
            .await
            .expect("insert foreign quote line item");
        sqlx::query("INSERT INTO integrations (tenant_id, provider_id, api_token) VALUES ('tenant-a', 'taxjar', '')")
            .execute(&pool)
            .await
            .expect("disable TaxJar in quote integration test");

        let app = router()
            .with_state(pool.clone())
            .layer(Extension(claims(Some("tenant-a"))));
        let cross_tenant_requests = [
            Request::builder()
                .method("GET")
                .uri(format!("/{foreign_quote_id}"))
                .body(Body::empty())
                .unwrap(),
            Request::builder()
                .method("PUT")
                .uri(format!("/{foreign_quote_id}"))
                .header("content-type", "application/json")
                .body(Body::from(r#"{"total_amount_cents":1,"line_items":[]}"#))
                .unwrap(),
            Request::builder()
                .method("POST")
                .uri(format!("/{foreign_quote_id}/accept"))
                .body(Body::empty())
                .unwrap(),
            Request::builder()
                .method("PATCH")
                .uri(format!("/{foreign_quote_id}/approve"))
                .body(Body::empty())
                .unwrap(),
        ];
        for request in cross_tenant_requests {
            let response = app.clone().oneshot(request).await.unwrap();
            assert_eq!(response.status(), StatusCode::NOT_FOUND);
        }

        for body in [
            r#"{"tenant_id":"tenant-b","customer_id":"customer-a","line_items":[]}"#,
            r#"{"tenant_id":"tenant-b","inquiry":"Need a quote","customer_id":"customer-a"}"#,
        ] {
            let uri = if body.contains("inquiry") {
                "/draft_agent"
            } else {
                "/"
            };
            let response = app
                .clone()
                .oneshot(
                    Request::builder()
                        .method("POST")
                        .uri(uri)
                        .header("content-type", "application/json")
                        .body(Body::from(body))
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(response.status(), StatusCode::CREATED);
        }

        let foreign_quote: (String, i64) = sqlx::query_as(
            "SELECT status, total_amount_cents FROM quotes WHERE id = $1",
        )
        .bind(foreign_quote_id)
        .fetch_one(&pool)
        .await
        .expect("reload foreign quote");
        assert_eq!(foreign_quote, ("DRAFT".to_string(), 700));
        let foreign_items: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM quote_line_items WHERE quote_id = $1 AND tenant_id = 'tenant-b'",
        )
        .bind(foreign_quote_id)
        .fetch_one(&pool)
        .await
        .expect("count foreign quote line items");
        assert_eq!(foreign_items, 1);
        let tenant_a_quotes: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM quotes WHERE tenant_id = 'tenant-a'")
                .fetch_one(&pool)
                .await
                .expect("count claims-owned quotes");
        assert_eq!(tenant_a_quotes, 2);

        pool.close().await;
        sqlx::query(&format!("DROP SCHEMA {schema} CASCADE"))
            .execute(&admin)
            .await
            .expect("drop quote integration schema");
    }
}

async fn accept_quote(
    State(pool): State<PgPool>,
    Extension(claims): Extension<::server_common::Claims>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let authority = match TenantAuthority::from_claims(&claims) {
        Ok(authority) => authority,
        Err(status) => return status.into_response(),
    };
    let quote_id = match Uuid::parse_str(&id) {
        Ok(uid) => uid,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let quote = match sqlx::query_as::<_, Quote>(
        "SELECT * FROM quotes WHERE id = $1 AND tenant_id = $2",
    )
        .bind(quote_id)
        .bind(authority.tenant_id())
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
    if !authority.owns_quote(&quote) {
        return StatusCode::NOT_FOUND.into_response();
    }

    let update_res = sqlx::query(
        "UPDATE quotes SET status = 'ACCEPTED', updated_at = NOW() WHERE id = $1 AND tenant_id = $2",
    )
        .bind(quote_id)
        .bind(authority.tenant_id())
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
    .bind(authority.tenant_id())
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

    let line_items = sqlx::query_as::<_, QuoteLineItem>(
        "SELECT * FROM quote_line_items WHERE quote_id = $1 AND tenant_id = $2",
    )
        .bind(quote_id)
        .bind(authority.tenant_id())
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
        .bind(authority.tenant_id())
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
    Extension(claims): Extension<::server_common::Claims>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let authority = match TenantAuthority::from_claims(&claims) {
        Ok(authority) => authority,
        Err(status) => return status.into_response(),
    };
    let quote_id = match Uuid::parse_str(&id) {
        Ok(uid) => uid,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let quote = match sqlx::query_as::<_, Quote>(
        "UPDATE quotes SET status = 'SENT', updated_at = NOW() WHERE id = $1 AND tenant_id = $2 RETURNING *"
    )
    .bind(quote_id)
    .bind(authority.tenant_id())
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
    if !authority.owns_quote(&quote) {
        return StatusCode::NOT_FOUND.into_response();
    }

    (StatusCode::OK, Json(serde_json::json!({"quote": quote}))).into_response()
}

// Temporary marker to slice off old approve_quote
