use crate::hub::Hub;
use axum::http::StatusCode;
use axum::{
    Router,
    extract::{Extension, Json},
    response::IntoResponse,
    routing::post,
};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::sync::Arc;
use std::sync::OnceLock;
use crate::utils::cache::HybridCache;

pub static CATALOG_CACHE: OnceLock<HybridCache<i64>> = OnceLock::new();


#[derive(Deserialize)]
pub struct GenerateOfferingRequest {
    pub prompt: String,
}

#[derive(Serialize)]
pub struct GenerateOfferingResponse {
    pub title: String,
    pub description: String,
    pub price: String,
    pub item_type: String,
    pub is_subscription: bool,
}

#[derive(Deserialize)]
pub struct CreateProductRequest {
    pub name: String,
    pub price: String,
    pub duration: Option<i32>,
    pub description: String,
    pub item_type: String,
    pub is_subscription: Option<bool>,
    pub subscription_interval: Option<String>,
    pub subscription_discount: Option<i32>,
}

#[derive(Serialize)]
pub struct CreateProductResponse {
    pub success: bool,
    pub message: Option<String>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

async fn count_tenant_products(
    conn: &mut sqlx::pool::PoolConnection<sqlx::Postgres>,
    tenant_id: &str,
) -> Result<i64, sqlx::Error> {
    let row = sqlx::query("SELECT COUNT(*)::BIGINT AS count FROM products WHERE tenant_id = $1")
        .bind(tenant_id)
        .fetch_one(&mut **conn)
        .await?;

    Ok(row.try_get::<i64, _>("count").unwrap_or(0))
}

fn plan_name(tier: &::server_pricing::rate_limit::PlanTier) -> &'static str {
    match tier {
        ::server_pricing::rate_limit::PlanTier::Free => "Free",
        ::server_pricing::rate_limit::PlanTier::Starter => "Starter",
        ::server_pricing::rate_limit::PlanTier::Pro => "Pro",
        ::server_pricing::rate_limit::PlanTier::Business => "Business",
    }
}

async fn handle_create_product(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<CreateProductRequest>,
) -> impl IntoResponse {
    let tenant_id = claims
        .organization_id
        .unwrap_or_else(|| ::server_common::auth_utils::get_default_tenant());

    let mut conn = match hub.pool.acquire().await {
        Ok(c) => c,
        Err(e) => {
            ::server_telemetry::record_error_signal("Failed to acquire DB connection");
            tracing::error!("Failed to acquire DB connection: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "DATABASE_ERROR".to_string(),
                    message: "Failed to connect to database".to_string(),
                }),
            )
                .into_response();
        }
    };

    let tier = hub
        .tracker()
        .get_tenant_tier(&tenant_id)
        .await
        .unwrap_or(::server_pricing::rate_limit::PlanTier::Free);

    if let Some(limit) = tier.max_products() {
        let cache = CATALOG_CACHE.get_or_init(|| HybridCache::new(None));
        let count_opt = cache.get(&tenant_id).await;

        let total_products = if let Some(count) = count_opt {
            count
        } else {
            match count_tenant_products(&mut conn, &tenant_id).await {
                Ok(c) => {
                    cache.set(&tenant_id, c, std::time::Duration::from_secs(30)).await;
                    c
                }
                Err(e) => {
                    ::server_telemetry::record_error_signal("Failed to count products for quota check");
                    tracing::error!("Failed to count products for tenant {}: {}", tenant_id, e);
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            error: "DATABASE_ERROR".to_string(),
                            message: "Failed to verify product limit".to_string(),
                        }),
                    )
                        .into_response();
                }
            }
        };

        if total_products >= limit as i64 {
            return (
                StatusCode::PAYMENT_REQUIRED,
                Json(ErrorResponse {
                    error: "LIMIT_EXCEEDED".to_string(),
                    message: format!(
                        "You've reached your {} tier limit of {} products. Upgrade your plan to add more products.",
                        plan_name(&tier),
                        limit
                    ),
                }),
            ).into_response();
        }
    }

    let product_id = uuid::Uuid::new_v4().to_string();

    let price_cents = (payload.price.parse::<f64>().unwrap_or(0.0) * 100.0).round() as i64;
    let insert_product = sqlx::query(
        "INSERT INTO products (id, tenant_id, title, description, type, price_cents, inventory_count) VALUES ($1, $2, $3, $4, $5, $6, 100)"
    )
    .bind(&product_id)
    .bind(&tenant_id)
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(&payload.item_type)
    .bind(price_cents)
    .execute(&mut *conn)
    .await;

    if let Err(e) = insert_product {
        ::server_telemetry::record_error_signal("Failed to insert product");
        tracing::error!("Failed to insert product: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "DATABASE_ERROR".to_string(),
                message: "Failed to create product".to_string(),
            }),
        )
            .into_response();
    }

    // Invalidate cache
    let cache = CATALOG_CACHE.get_or_init(|| HybridCache::new(None));
    cache.invalidate(&tenant_id).await;

    if let Err(e) = hub.tracker().record_product_added(&tenant_id).await {
        tracing::warn!(
            "Failed to update product usage counter for tenant {}: {}",
            tenant_id,
            e
        );
    }

    if payload.is_subscription.unwrap_or(false) {
        let plan_id = uuid::Uuid::new_v4().to_string();
        let interval = payload
            .subscription_interval
            .unwrap_or_else(|| "Monthly".to_string())
            .to_lowercase();
        let discount = payload.subscription_discount.unwrap_or(0);

        let insert_plan = sqlx::query(
            "INSERT INTO subscription_plans (id, tenant_id, product_id, interval, discount_percentage, name, price_cents, frequency) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(&plan_id)
        .bind(&tenant_id)
        .bind(&product_id)
        .bind(&interval)
        .bind(discount)
        .bind(&payload.name)
        .bind(price_cents)
        .bind(&interval)
        .execute(&mut *conn)
        .await;

        if let Err(e) = insert_plan {
            ::server_telemetry::record_error_signal("Failed to insert subscription plan");
            tracing::error!("Failed to insert subscription plan: {}", e);
            // Non-fatal, just log it. The product was created.
        }
    }

    let event_payload = serde_json::json!({
        "product_id": product_id,
        "name": payload.name,
        "organization_id": tenant_id,
    });

    let event = ::server_ohc::orchestration::TeammateMeshEvent {
        agent_id: "system".to_string(),
        action: "ProductCreated".to_string(),
        status: "success".to_string(),
        payload: serde_json::to_vec(&event_payload).unwrap_or_default(),
        msg_id: uuid::Uuid::new_v4().to_string(),
    };

    let _ = hub.publish_teammate_event("products_inbox".to_string(), event);

    (
        StatusCode::OK,
        Json(CreateProductResponse {
            success: true,
            message: Some(format!("Created {}", payload.name)),
        }),
    )
        .into_response()
}


async fn handle_generate_offering(
    Json(payload): Json<GenerateOfferingRequest>,
) -> impl IntoResponse {
    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
    let optimized_prompt = ::server_pricing::compression::reduce_tokens(&payload.prompt);
    let prompt = format!(
        "Extract the product or service offering details from the following text:\n\n'{}'\n\nOutput ONLY a raw JSON object (do not wrap in markdown or backticks) matching this exact schema: {{\"title\": \"string\", \"description\": \"string\", \"price\": \"string\", \"item_type\": \"string (either Product or Service)\", \"is_subscription\": \"boolean\"}}. Suggest an appropriate market price if none is provided.",
        optimized_prompt
    );

    let client = crate::minimax::MinimaxClient::new(api_key);
    let mut response_json = GenerateOfferingResponse {
        title: "Generated Offering".to_string(),
        description: "AI description".to_string(),
        price: "10.00".to_string(),
        item_type: "Service".to_string(),
        is_subscription: false,
    };

    if let Ok(reasoned) = client.reason(&prompt).await {
        let cleaned = reasoned.replace("", "").trim().to_string();
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&cleaned) {
            if let Some(title) = parsed.get("title").and_then(|v| v.as_str()) {
                response_json.title = title.to_string();
            }
            if let Some(description) = parsed.get("description").and_then(|v| v.as_str()) {
                response_json.description = description.to_string();
            }
            if let Some(price) = parsed.get("price").and_then(|v| v.as_str()) {
                response_json.price = price.to_string();
            } else if let Some(price) = parsed.get("price").and_then(|v| v.as_f64()) {
                response_json.price = format!("{:.2}", price);
            }
            if let Some(item_type) = parsed.get("item_type").and_then(|v| v.as_str()) {
                response_json.item_type = item_type.to_string();
            }
            if let Some(is_sub) = parsed.get("is_subscription").and_then(|v| v.as_bool()) {
                response_json.is_subscription = is_sub;
            }
        } else {
             tracing::warn!("Failed to parse LLM JSON: {}", cleaned);
        }
    }

    (axum::http::StatusCode::OK, Json(response_json)).into_response()
}

#[derive(Deserialize)]
pub struct AnalyzeImageRequest {
    pub image_url: String,
}

#[derive(Serialize)]
pub struct AnalyzeImageResponse {
    pub id: String,
    pub title: String,
    pub description: String,
    pub price: String,
    pub item_type: String,
}

async fn handle_analyze_image(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<AnalyzeImageRequest>,
) -> impl IntoResponse {
    let tenant_id = claims
        .organization_id
        .unwrap_or_else(|| ::server_common::auth_utils::get_default_tenant());

    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();

    // In a real production system, this would upload the base64 to GCS/S3, get a public URL,
    // and pass the URL to MiniMax's multi-modal endpoint (like abab6.5s).
    // For now, we simulate the Vision AI by passing the length/hash of the base64 or a structured prompt
    // to the standard model if it's large, but we MUST pass real data, not a mocked fake URL.

    let prompt = format!(
        "Analyze this base64 image data starting with 'data:image/...'. Act as a Visualizer and Promoter AI Agent. Output ONLY a raw JSON object matching this exact schema: {{\"title\": \"string\", \"description\": \"string\", \"price\": \"string\", \"item_type\": \"string\"}}. Generate a catchy title, a 2-sentence description, an estimated price based on market data, and the item_type (Product).|||{}",
        payload.image_url
    );

    let client = crate::minimax::MinimaxClient::new(api_key);
    let mut generated_item = AnalyzeImageResponse {
        id: uuid::Uuid::new_v4().to_string(),
        title: "Generated Product".to_string(),
        description: "A great product from photo".to_string(),
        price: "25.00".to_string(),
        item_type: "Product".to_string(),
    };

    if let Ok(reasoned) = client.reason(&prompt).await {
        let cleaned = reasoned.replace("```json", "").replace("```", "").trim().to_string();
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&cleaned) {
            if let Some(title) = parsed.get("title").and_then(|v| v.as_str()) {
                generated_item.title = title.to_string();
            }
            if let Some(description) = parsed.get("description").and_then(|v| v.as_str()) {
                generated_item.description = description.to_string();
            }
            if let Some(price) = parsed.get("price").and_then(|v| v.as_str()) {
                generated_item.price = price.to_string();
            }
            if let Some(item_type) = parsed.get("item_type").and_then(|v| v.as_str()) {
                generated_item.item_type = item_type.to_string();
            }
        }
    }

    // Queue in Action Feed
    let repo = crate::domain::repository::agent_feed_repo::AgentFeedRepository::new(hub.pool.clone());
    let proposed_action = serde_json::json!({
        "feature_type": "product_creation",
        "action_details": {
            "name": generated_item.title.clone(),
            "description": generated_item.description.clone(),
            "price": generated_item.price.clone(),
            "item_type": generated_item.item_type.clone(),
            "image_url": payload.image_url.clone()
        }
    });

    let feed_item = crate::domain::repository::agent_feed_repo::AgentFeedItem {
        id: generated_item.id.clone(),
        tenant_id: tenant_id.clone(),
        event_source: "invisible_magic_catalog".to_string(),
        context_payload: Some(sqlx::types::Json(serde_json::json!({"image_url": payload.image_url}))),
        proposed_action: Some(sqlx::types::Json(proposed_action)),
        lifecycle_state: "PENDING_APPROVAL".to_string(),
        created_at: Some(chrono::Utc::now()),
        updated_at: Some(chrono::Utc::now()),
    };

    if let Err(e) = repo.create(feed_item).await {
        tracing::error!("Failed to create agent feed item for magic catalog: {}", e);
    }

    (axum::http::StatusCode::OK, Json(generated_item)).into_response()
}

pub fn router<S: Clone + Send + Sync + 'static>(hub: Arc<Hub>) -> Router<S> {
    Router::new()
        .route("/product", post(handle_create_product))
        .route("/generate", post(handle_generate_offering))
        .route("/analyze-image", post(handle_analyze_image))
        .layer(Extension(hub))
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_token_reduction_integration() {
        let input = "This is a long sentence with many unnecessary words that should be reduced.";
        let optimized = ::server_pricing::compression::reduce_tokens(input);

        assert!(optimized.len() < input.len());
        assert!(!optimized.contains(" is "));
        assert!(!optimized.contains(" a "));
        assert!(!optimized.contains(" with "));
    }
}
