use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use axum::{
    extract::{State, Path},
    routing::{get, post, patch},
    Json, Router,
};
use serde::{Deserialize, Serialize};

pub fn router(pool: PgPool) -> Router {
    Router::new()
        .route("/quotes", post(create_quote))
        .route("/quotes/{id}", get(get_quote))
        .route("/quotes/{id}/approve", patch(approve_quote))
        .route("/pricing-rules", get(get_pricing_rules))
        .route("/pricing-rules", post(create_pricing_rule))
        .route("/quote-requests", post(create_quote_request))
        .route("/quote-requests/{id}/generate-proposal", post(generate_proposal))
        .with_state(pool)
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Quote {
    pub id: Uuid,
    pub tenant_id: String,
    pub customer_id: Uuid,
    pub status: String,
    pub valid_until: Option<DateTime<Utc>>,
    pub total_amount_cents: i64,
    pub required_deposit_cents: i64,
    pub stripe_payment_link: Option<String>,
    pub proposed_slot_id: Option<String>,
    pub service_id: Option<String>,
}


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct QuoteRequest {
    pub id: Uuid,
    pub tenant_id: String,
    pub customer_id: Option<Uuid>,
    pub status: String,
    pub source: String,
    pub message: String,
    pub images: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQuoteRequestReq {
    pub customer_id: Option<Uuid>,
    pub source: String,
    pub message: String,
    pub images: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateQuoteReq {
    pub customer_id: Uuid,
    pub status: String,
    pub line_items: Vec<QuoteLineItemReq>,
    pub proposed_slot_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuoteLineItemReq {
    pub description: String,
    pub unit_price_cents: i64,
    pub quantity: i32,
    pub is_optional: bool,
    pub service_item_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct PricingRule {
    pub id: Uuid,
    pub tenant_id: String,
    pub name: String,
    pub base_price_cents: i64,
    pub rules_json: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePricingRuleReq {
    pub name: String,
    pub base_price_cents: i64,
    pub rules_json: serde_json::Value,
}

async fn get_pricing_rules(
    State(pool): State<PgPool>,
    axum::extract::Extension(claims): axum::extract::Extension<crate::common::Claims>,
) -> Result<Json<Vec<PricingRule>>, axum::http::StatusCode> {
    let tenant_id = claims.organization_id;

    let rules = sqlx::query_as::<_, PricingRule>(
        "SELECT id, tenant_id, name, base_price_cents, rules_json, created_at, updated_at FROM pricing_rules WHERE tenant_id = $1"
    )
    .bind(&tenant_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch pricing rules: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(rules))
}

async fn create_pricing_rule(
    State(pool): State<PgPool>,
    axum::extract::Extension(claims): axum::extract::Extension<crate::common::Claims>,
    Json(payload): Json<CreatePricingRuleReq>,
) -> Result<Json<PricingRule>, axum::http::StatusCode> {
    let rule_id = Uuid::new_v4();
    let tenant_id = claims.organization_id;

    let rule = sqlx::query_as::<_, PricingRule>(
        "INSERT INTO pricing_rules (id, tenant_id, name, base_price_cents, rules_json) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(rule_id)
    .bind(&tenant_id)
    .bind(&payload.name)
    .bind(payload.base_price_cents)
    .bind(&payload.rules_json)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to insert pricing rule: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(rule))
}


async fn create_quote_request(
    State(pool): State<PgPool>,
    axum::extract::Extension(claims): axum::extract::Extension<crate::common::Claims>,
    Json(payload): Json<CreateQuoteRequestReq>,
) -> Result<Json<QuoteRequest>, axum::http::StatusCode> {
    let request_id = Uuid::new_v4();
    let tenant_id = claims.organization_id;

    let request = sqlx::query_as::<_, QuoteRequest>(
        r#"INSERT INTO quote_requests (id, tenant_id, customer_id, status, source, message, images)
           VALUES ($1, $2, $3, $4, $5, $6, $7)
           RETURNING id, tenant_id, customer_id, status, source, message, images, created_at, updated_at"#
    )
    .bind(request_id)
    .bind(tenant_id)
    .bind(payload.customer_id)
    .bind("NEW")
    .bind(payload.source)
    .bind(payload.message)
    .bind(payload.images)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create quote request: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(request))
}

async fn generate_proposal(
    State(pool): State<PgPool>,
    axum::extract::Extension(claims): axum::extract::Extension<crate::common::Claims>,
    Path(id): Path<Uuid>,
) -> Result<Json<Quote>, axum::http::StatusCode> {
    // 1. Fetch the QuoteRequest
    let request = sqlx::query_as::<_, QuoteRequest>(
        r#"SELECT id, tenant_id, customer_id, status, source, message, images, created_at, updated_at
           FROM quote_requests WHERE id = $1 AND tenant_id = $2"#
    )
    .bind(id)
    .bind(claims.organization_id.clone())
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch quote request: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?
    .ok_or(axum::http::StatusCode::NOT_FOUND)?;

    // 2. AI integration logic using LLM for Estimator Agent
    // We dynamically parse the incoming message to look up matching prices from the services table.

    let mut mock_price = 25000; // $250 default
    let mut matched_service_name = "Custom Service Base Fee".to_string();

    let request_lower = request.message.to_lowercase();
    let mut generated_scope = String::new();

    // Find all service items for the tenant
    #[derive(FromRow, serde::Serialize)]
    struct ServiceItem {
        id: Uuid,
        name: String,
        base_price_cents: i64,
    }

    let mut matched_service_item_id: Option<Uuid> = None;

    if let Ok(services) = sqlx::query_as::<_, ServiceItem>("SELECT id, name, base_price_cents FROM service_items WHERE tenant_id = $1")
        .bind(&claims.organization_id)
        .fetch_all(&pool)
        .await
    {
        // Try LLM parsing
        let catalog_json = serde_json::to_string(&services).unwrap_or_default();

        let key = std::env::var("GEMINI_API_KEY")
            .or_else(|_| std::env::var("OPENAI_API_KEY"))
            .unwrap_or_default();

        let mut llm_matched = false;

        if !key.is_empty() {
            let model = std::env::var("OHC_LLM_MODEL").unwrap_or_else(|_| "gemini-pro".to_string());
            let endpoint = std::env::var("OHC_LLM_ENDPOINT").ok();

            let mut config = if let Some(endpoint) = endpoint {
                ohc_builtin_agent::llm::openai::OpenAIClientConfig::openai_compatible(key.clone(), endpoint, Some(model.clone()))
            } else {
                ohc_builtin_agent::llm::openai::OpenAIClientConfig::openai(key.clone())
            };
            config.default_model = Some(model.clone());
            let llm = ohc_builtin_agent::llm::openai::OpenAIClient::from_config(config);

            let prompt = format!(
                "You are the Estimator Agent for a service business. You have the following service catalog:
{0}

Customer Inquiry: '{1}'

Task: Extract the scope of work and identify the closest matching service from the catalog based on the inquiry. Respond ONLY in valid JSON format: {{ \"matched_service_title\": \"string\", \"matched_price_cents\": 15000, \"scope\": \"string\" }}",
                catalog_json, request.message
            );

            let req = ohc_builtin_agent::types::ChatRequest {
                messages: vec![ohc_builtin_agent::types::Message::user(&prompt)],
                model,
                temperature: 0.0,
                max_tokens: 512,
                system: "You are an Estimator Agent. Parse scopes of work and match with service catalog. Output pure JSON.".to_string(),
                tools: vec![],
            };

            use ohc_builtin_agent::llm::LlmClient;
            if let Ok(resp) = llm.chat(req).await {
                let content = resp.message.content.trim();
                let clean_content = if content.starts_with("```json") {
                    content.trim_start_matches("```json").trim_end_matches("```").trim()
                } else {
                    content
                };
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(clean_content) {
                    if let Some(title) = parsed.get("matched_service_title").and_then(|t| t.as_str()) {
                        matched_service_name = title.to_string();
                        llm_matched = true;

                        // Try to find the matched service by title to get the ID
                        if let Some(matched_service) = services.iter().find(|s| s.name.to_lowercase() == title.to_lowercase()) {
                            matched_service_item_id = Some(matched_service.id);
                        }
                    }
                    if let Some(price) = parsed.get("matched_price_cents").and_then(|p| p.as_i64()) {
                        mock_price = price;
                        llm_matched = true;
                    }
                    if let Some(scope) = parsed.get("scope").and_then(|s| s.as_str()) {
                        generated_scope = scope.to_string();
                        llm_matched = true;
                    }
                }
            }
        }

        if !llm_matched {
            // Fallback naive matching
            for service in services {
                if request_lower.contains(&service.name.to_lowercase()) {
                    mock_price = service.base_price_cents;
                    matched_service_name = service.name.clone();
                    matched_service_item_id = Some(service.id);
                    break;
                }
            }
        }
    } else {
        if request_lower.contains("door") {
            mock_price = 15000;
        }
    }

    // Agent parsing: Attempt to find and reserve an available slot
    let mut proposed_slot_id = None;
    let mut _lock_val = None;

    // Find first available booking slot for this tenant
    #[derive(FromRow)]
    struct BookingSlot {
        id: String,
    }

    let slot = sqlx::query_as::<_, BookingSlot>(
        "SELECT id FROM booking_slots WHERE tenant_id = $1 AND status = 'available' ORDER BY start_time ASC LIMIT 1"
    )
    .bind(&claims.organization_id)
    .fetch_optional(&pool)
    .await
    .unwrap_or(None);

    if let Some(s) = slot {
        // Attempt to acquire Redis Redlock
        let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
        if let Ok(redis_lock) = crate::orchestration::queue::RedisLock::new(&redis_url) {
            let org_id = claims.organization_id.clone().unwrap_or_default();
            if let Ok(Some(val)) = redis_lock.acquire_lock(&org_id, "booking_slot", &s.id, 600).await {
                // Lock acquired, update slot status to soft_locked
                let updated = sqlx::query(
                    "UPDATE booking_slots SET status = 'soft_locked' WHERE id = $1 AND tenant_id = $2 AND status = 'available'"
                )
                .bind(&s.id)
                .bind(&org_id)
                .execute(&pool)
                .await;

                if let Ok(res) = updated {
                    if res.rows_affected() > 0 {
                        proposed_slot_id = Some(s.id.clone());
                        _lock_val = Some(val);
                    } else {
                        // Slot was snatched before we updated the DB, release the redis lock
                        let _ = redis_lock.release_lock(&org_id, "booking_slot", &s.id, &val).await;
                    }
                }
            }
        }
    }

    let create_req = CreateQuoteReq {
        customer_id: request.customer_id.unwrap_or_else(Uuid::new_v4),
        status: "DRAFT".to_string(),
        line_items: vec![
            QuoteLineItemReq {
                description: format!("AI Generated Proposal for: {} - Scope: {}", matched_service_name, generated_scope),
                unit_price_cents: mock_price,
                quantity: 1,
                is_optional: false,
                service_item_id: matched_service_item_id,
            }
        ],
        proposed_slot_id,
    };

    // 3. Create the quote
    create_quote(State(pool), axum::extract::Extension(claims.clone()), Json(create_req)).await
}

async fn create_quote(
    State(pool): State<PgPool>,
    axum::extract::Extension(claims): axum::extract::Extension<crate::common::Claims>,
    Json(payload): Json<CreateQuoteReq>,
) -> Result<Json<Quote>, axum::http::StatusCode> {
    let quote_id = Uuid::new_v4();
    let tenant_id = claims.organization_id;

    let mut tx = pool.begin().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut line_items = payload.line_items;

    // Check if TaxJar integration is active for this tenant
    // In a real implementation we would fetch the integration credentials,
    // for now we'll check if the tenant has a TAXJAR_API_KEY env var (or similar config)
    // Here we'll simulate adding tax if the first item isn't already tax
    if let Ok(api_key) = std::env::var("TAXJAR_API_KEY") {
        if !api_key.is_empty() {
            let provider = crate::integrations::taxjar::provider::TaxJarProvider::new(api_key);
            let total_pre_tax = line_items.iter().map(|li| li.unit_price_cents * li.quantity as i64).sum::<i64>();
            let total_pre_tax_usd = (total_pre_tax as f64) / 100.0;

            // Hardcoding dummy from/to zip codes for automated tax calculation via API
            if let Ok(tax_rate) = provider.calculate_tax(total_pre_tax_usd, 0.0, "US", "90002", "CA", "US", "92093", "CA").await {
                if tax_rate.amount_to_collect > 0.0 {
                    line_items.push(QuoteLineItemReq {
                        description: "Automated Sales Tax (TaxJar)".to_string(),
                        unit_price_cents: (tax_rate.amount_to_collect * 100.0) as i64,
                        quantity: 1,
                        is_optional: false,
                        service_item_id: None,
                    });
                }
            }
        }
    }

    let total_amount_cents = line_items.iter().map(|li| li.unit_price_cents * li.quantity as i64).sum::<i64>();
    let required_deposit_cents = total_amount_cents / 3; // Default 33% deposit

    let quote = sqlx::query_as::<_, Quote>(
        "INSERT INTO quotes (id, tenant_id, customer_id, status, total_amount_cents, required_deposit_cents, proposed_slot_id) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *"
    )
    .bind(quote_id)
    .bind(&tenant_id)
    .bind(payload.customer_id)
    .bind(&payload.status)
    .bind(total_amount_cents)
    .bind(required_deposit_cents)
    .bind(payload.proposed_slot_id)
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create quote: {}", e);
        axum::http::StatusCode::INTERNAL_SERVER_ERROR
    })?;

    for item in line_items {
        sqlx::query(
            "INSERT INTO quote_line_items (id, quote_id, description, unit_price_cents, quantity, is_optional, tenant_id, service_item_id) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(Uuid::new_v4())
        .bind(quote_id)
        .bind(item.description)
        .bind(item.unit_price_cents)
        .bind(item.quantity)
        .bind(item.is_optional)
        .bind(tenant_id.clone())
        .bind(item.service_item_id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create quote line item: {}", e);
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        })?;
    }

    tx.commit().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(quote))
}

async fn get_quote(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Quote>, axum::http::StatusCode> {
    let quote = sqlx::query_as::<_, Quote>("SELECT * FROM quotes WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    match quote {
        Some(q) => Ok(Json(q)),
        None => Err(axum::http::StatusCode::NOT_FOUND),
    }
}

async fn approve_quote(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<Quote>, axum::http::StatusCode> {
    let quote = sqlx::query_as::<_, Quote>(
        "UPDATE quotes SET status = 'ACCEPTED', updated_at = NOW() WHERE id = $1 RETURNING *"
    )
    .bind(id)
    .fetch_optional(&pool)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    // Integrate Stripe deposit logic
    if let Some(mut q) = quote {
        if q.status == "ACCEPTED" {
            let amount_usd = (q.total_amount_cents as f64) / 100.0;
            // Use Stripe client to create a checkout session
            let stripe_key = std::env::var("STRIPE_API_KEY").unwrap_or_else(|_| "sk_test_mock".to_string());
            let stripe_client = crate::integrations::stripe::client::StripeClient::new(stripe_key);

            match stripe_client.create_checkout_session(
                &format!("Quote #{}", q.id),
                &q.customer_id.to_string(),
                amount_usd,
                None,
                None
            ).await {
                Ok(url) => {
                    q.stripe_payment_link = Some(url.clone());
                    let _ = sqlx::query("UPDATE quotes SET stripe_payment_link = $1 WHERE id = $2")
                        .bind(&url)
                        .bind(q.id)
                        .execute(&pool)
                        .await;
                },
                Err(e) => {
                    tracing::error!("Failed to create Stripe checkout session: {}", e); // pii-safe
                }
            }
        }
        Ok(Json(q))
    } else {
        Err(axum::http::StatusCode::NOT_FOUND)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_quote_struct_serialization() {
        let quote = Quote {
            id: Uuid::new_v4(),
            tenant_id: "test-tenant".to_string(),
            customer_id: Uuid::new_v4(),
            status: "DRAFT".to_string(),
            valid_until: None,
            total_amount_cents: 1000,
            required_deposit_cents: 333,
            stripe_payment_link: Some("http://stripe.com".to_string()),
            proposed_slot_id: Some("slot-1".to_string()),
            service_id: Some("srv-1".to_string()),
        };
        let serialized = serde_json::to_string(&quote).unwrap();
        assert!(serialized.contains("total_amount_cents"));
    }
}
