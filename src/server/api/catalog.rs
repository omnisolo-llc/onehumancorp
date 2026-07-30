use crate::hub::Hub;
use crate::persistence::catalog::CatalogRepository;
use axum::http::StatusCode;
use axum::{
    Router,
    extract::{Extension, Json},
    response::IntoResponse,
    routing::{get, post},
};

use crate::utils::cache::HybridCache;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use std::sync::Arc;
use std::sync::OnceLock;

pub static CATALOG_CACHE: OnceLock<HybridCache<i64>> = OnceLock::new();

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
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
    pub variants: Option<Vec<ProductVariantRequest>>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ProductVariantRequest {
    pub name: String,
    pub price_modifier: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateProductRequest {
    pub name: String,
    pub price: String,
    pub duration: Option<i32>,
    pub description: String,
    pub item_type: String,
    pub is_subscribable: Option<bool>,
    pub subscription_frequency: Option<String>,
    pub subscription_discount_percent: Option<i32>,
    pub variants: Option<Vec<ProductVariantRequest>>,
    pub smart_pricing_enabled: Option<bool>,
    pub base_price: Option<String>,
    pub min_price: Option<String>,
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

#[derive(Serialize)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub title: String,
    pub description: Option<String>,
    pub item_type: Option<String>,
    pub price_cents: Option<i64>,
    pub price: f64,
    pub inventory_count: Option<i32>,
    pub image_url: Option<String>,
    pub variants: Option<Vec<ProductVariantRequest>>,
}

fn bounded_product_image_url(metadata: Option<&serde_json::Value>) -> Option<String> {
    let image_url = metadata?.get("image_url")?.as_str()?.trim();
    let is_safe_path = image_url.starts_with('/')
        && !image_url.starts_with("//")
        && image_url.len() <= 2_048
        && !image_url.contains(['\\', '\r', '\n', '\0'])
        && !image_url
            .split('/')
            .any(|segment| matches!(segment, "." | ".."));

    is_safe_path.then(|| image_url.to_string())
}

async fn handle_get_products(
    Extension(repository): Extension<Option<Arc<CatalogRepository>>>,
    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let Some(tenant_id) = claims
        .organization_id
        .as_deref()
        .map(str::trim)
        .filter(|tenant_id| !tenant_id.is_empty() && !tenant_id.eq_ignore_ascii_case("system"))
        .map(str::to_string)
    else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let Some(repository) = repository else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(vec![] as Vec<Product>),
        )
            .into_response();
    };
    match repository.list_products(&tenant_id).await {
        Ok(rows) => {
            let products = rows
                .into_iter()
                .map(|row| Product {
                    id: row.id,
                    name: row.title.clone(),
                    title: row.title,
                    description: row.description,
                    item_type: row.item_type,
                    price_cents: Some(row.price_cents),
                    price: row.price_cents as f64 / 100.0,
                    inventory_count: Some(row.inventory_count),
                    image_url: None,
                    variants: None,
                })
                .collect();
            (StatusCode::OK, Json(products)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to fetch products: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(vec![] as Vec<Product>),
            )
                .into_response()
        }
    }
}

fn validated_product_price(payload: &CreateProductRequest) -> Option<f64> {
    let invalid_smart_pricing = payload.smart_pricing_enabled.unwrap_or(false) && {
        let base_price = payload
            .base_price
            .as_deref()
            .map(|price| parse_bounded_price(price, false))
            .unwrap_or_else(|| parse_bounded_price(&payload.price, false));
        let min_price = payload
            .min_price
            .as_deref()
            .map(|price| parse_bounded_price(price, false))
            .unwrap_or(Some(0.0));

        base_price
            .zip(min_price)
            .is_none_or(|(base_price, min_price)| min_price > base_price)
    };
    let invalid_text = payload.name.trim().is_empty()
        || payload.name.chars().count() > 200
        || payload.description.chars().count() > 10_000
        || !matches!(payload.item_type.as_str(), "Product" | "Service")
        || payload.duration.is_some_and(|duration| duration < 0)
        || payload
            .subscription_discount_percent
            .is_some_and(|discount| !(0..=100).contains(&discount))
        || payload
            .subscription_frequency
            .as_deref()
            .is_some_and(|frequency| {
                !matches!(
                    frequency.to_ascii_lowercase().as_str(),
                    "daily" | "weekly" | "monthly" | "yearly"
                )
            })
        || payload.variants.as_ref().is_some_and(|variants| {
            variants.len() > 50
                || variants.iter().any(|variant| {
                    variant.name.trim().is_empty()
                        || variant.name.chars().count() > 200
                        || parse_bounded_price(&variant.price_modifier, true).is_none()
                })
        })
        || invalid_smart_pricing;

    (!invalid_text)
        .then(|| parse_bounded_price(&payload.price, false))
        .flatten()
}

fn parse_bounded_price(price: &str, allow_negative: bool) -> Option<f64> {
    let price = price.parse::<f64>().ok()?;
    (price.is_finite()
        && price <= 10_000_000.0
        && (allow_negative || price >= 0.0)
        && (!allow_negative || price >= -10_000_000.0))
        .then_some(price)
}

async fn count_tenant_products(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    tenant_id: &str,
) -> Result<i64, sqlx::Error> {
    let row = sqlx::query("SELECT COUNT(*)::BIGINT AS count FROM products WHERE tenant_id = $1")
        .bind(tenant_id)
        .fetch_one(&mut **tx)
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
    let Some(tenant_id) = claims
        .organization_id
        .as_deref()
        .map(str::trim)
        .filter(|tenant_id| !tenant_id.is_empty() && !tenant_id.eq_ignore_ascii_case("system"))
        .map(str::to_string)
    else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    let Some(price) = validated_product_price(&payload) else {
        return (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "INVALID_PRODUCT".to_string(),
                message: "Product fields are invalid".to_string(),
            }),
        )
            .into_response();
    };

    let mut tx = match hub.pool.begin().await {
        Ok(c) => c,
        Err(e) => {
            ::server_telemetry::record_error_signal("[bug] Failed to acquire DB connection");
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
    if let Err(error) = ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id).await {
        tracing::error!("Failed to bind catalog tenant context: {error:?}"); // pii-safe
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "DATABASE_ERROR".to_string(),
                message: "Failed to create product".to_string(),
            }),
        )
            .into_response();
    }

    let tier = hub
        .tracker()
        .get_tenant_tier(&tenant_id)
        .await
        .unwrap_or(::server_pricing::rate_limit::PlanTier::Free);

    let total_products = if let Some(limit) = tier.max_products() {
        if let Err(e) = sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended($1, 0))")
            .bind(&tenant_id)
            .execute(&mut *tx)
            .await
        {
            ::server_telemetry::record_error_signal("[bug] Failed to lock product quota");
            tracing::error!("Failed to lock product quota: {e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "DATABASE_ERROR".to_string(),
                    message: "Failed to verify product limit".to_string(),
                }),
            )
                .into_response();
        }
        let total_products = match count_tenant_products(&mut tx, &tenant_id).await {
            Ok(c) => c,
            Err(e) => {
                ::server_telemetry::record_error_signal(
                    "[bug] Failed to count products for quota check",
                );
                tracing::error!("Failed to count products for tenant {}: {}", tenant_id, e); // pii-safe
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "DATABASE_ERROR".to_string(),
                        message: "Failed to verify product limit".to_string(),
                    }),
                )
                    .into_response();
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
        Some(total_products)
    } else {
        None
    };

    let product_id = uuid::Uuid::new_v4().to_string();

    let price_cents = (price * 100.0).round() as i64;
    let insert_product = sqlx::query(
        "INSERT INTO products (id, tenant_id, title, description, type, price_cents, inventory_count, is_subscribable, subscription_frequency, subscription_discount_percent) VALUES ($1, $2, $3, $4, $5, $6, 100, $7, $8, $9)"
    )
    .bind(&product_id)
    .bind(&tenant_id)
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(&payload.item_type)
    .bind(price_cents)
    .bind(payload.is_subscribable.unwrap_or(false))
    .bind(payload.subscription_frequency.clone())
    .bind(payload.subscription_discount_percent)
    .execute(&mut *tx)
    .await;

    if let Err(e) = insert_product {
        ::server_telemetry::record_error_signal("[bug] Failed to insert product");
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

    if let Some(variants) = payload.variants {
        for variant in variants {
            let variant_id = format!("var-{}", uuid::Uuid::new_v4());
            let v_price_mod =
                (parse_bounded_price(&variant.price_modifier, true).unwrap_or_default() * 100.0)
                    .round() as i64;
            let insert_variant = sqlx::query("INSERT INTO product_variants (id, tenant_id, product_id, name, price_modifier, inventory_count) VALUES ($1, $2, $3, $4, $5, 100)")
                .bind(&variant_id)
                .bind(&tenant_id)
                .bind(&product_id)
                .bind(&variant.name)
                .bind(v_price_mod)
                .execute(&mut *tx)
                .await;
            if let Err(e) = insert_variant {
                ::server_telemetry::record_error_signal("[bug] Failed to insert product variant");
                tracing::error!("Failed to insert product variant: {e}");
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error: "DATABASE_ERROR".to_string(),
                        message: "Failed to create product".to_string(),
                    }),
                )
                    .into_response();
            }
        }
    }

    if payload.is_subscribable.unwrap_or(false) {
        let plan_id = uuid::Uuid::new_v4().to_string();
        let frequency = payload
            .subscription_frequency
            .clone()
            .unwrap_or_else(|| "Monthly".to_string())
            .to_lowercase();
        let discount = payload.subscription_discount_percent.unwrap_or(0);

        let insert_plan = sqlx::query(
            "INSERT INTO subscription_plans (id, tenant_id, product_id, name, price_cents, frequency, interval, discount_percentage) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(&plan_id)
        .bind(&tenant_id)
        .bind(&product_id)
        .bind(&payload.name)
        .bind(price_cents)
        .bind(&frequency)
        .bind(&frequency)
        .bind(discount)
        .execute(&mut *tx)
        .await;

        if let Err(e) = insert_plan {
            ::server_telemetry::record_error_signal("[bug] Failed to insert subscription plan");
            tracing::error!("Failed to insert subscription plan: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "DATABASE_ERROR".to_string(),
                    message: "Failed to create product".to_string(),
                }),
            )
                .into_response();
        }
    }

    if payload.smart_pricing_enabled.unwrap_or(false) {
        let base_price_cents = (payload
            .base_price
            .as_deref()
            .and_then(|price| parse_bounded_price(price, false))
            .unwrap_or(price)
            * 100.0)
            .round() as i64;
        let rules_id = uuid::Uuid::new_v4().to_string();

        let insert_pricing_rule = sqlx::query("INSERT INTO pricing_rules (id, tenant_id, target_id, name, base_price_cents, is_active, rules_json) VALUES ($1, $2, $3, $4, $5, TRUE, $6)")
            .bind(&rules_id)
            .bind(&tenant_id)
            .bind(&product_id)
            .bind(format!("Smart Pricing: {}", payload.name))
            .bind(base_price_cents)
            .bind(serde_json::json!([{
                "type": "InventoryThreshold",
                "config": { "threshold": 10, "adjustment_percent": -10.0 }
            }]))
            .execute(&mut *tx)
            .await;
        if let Err(e) = insert_pricing_rule {
            ::server_telemetry::record_error_signal("[bug] Failed to insert pricing rule");
            tracing::error!("Failed to insert pricing rule: {e}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "DATABASE_ERROR".to_string(),
                    message: "Failed to create product".to_string(),
                }),
            )
                .into_response();
        }
    }

    if let Err(error) = tx.commit().await {
        ::server_telemetry::record_error_signal("[bug] Failed to commit product creation");
        tracing::error!("Failed to commit product creation: {error}");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "DATABASE_ERROR".to_string(),
                message: "Failed to create product".to_string(),
            }),
        )
            .into_response();
    }

    let cache = CATALOG_CACHE.get_or_init(|| HybridCache::new(None));
    if let Some(total_products) = total_products {
        cache
            .set(
                &tenant_id,
                total_products + 1,
                std::time::Duration::from_secs(30),
            )
            .await;
    } else {
        cache.invalidate(&tenant_id).await;
    }

    let edge_cache = crate::builder::edge::get_edge_cache();
    edge_cache
        .invalidate_by_tag(&format!("tenant-id:{}", tenant_id))
        .await;
    edge_cache
        .invalidate_by_tag(&format!("entity:product:{}", product_id))
        .await;
    let cdn_cache = crate::utils::edge_caching_middleware::get_cdn_cache();
    cdn_cache
        .invalidate_by_tag(&format!("tenant-id:{}", tenant_id))
        .await;
    cdn_cache
        .invalidate_by_tag(&format!("entity:product:{}", product_id))
        .await;

    if let Err(e) = hub.tracker().record_product_added(&tenant_id).await {
        tracing::warn!(
            "Failed to update product usage counter for tenant {}: {}",
            tenant_id,
            e
        );
    }

    let event_payload = serde_json::json!({
        "product_id": product_id,
        "name": payload.name,
        "description": payload.description,
        "item_type": payload.item_type,
        "price": price,
        "organization_id": tenant_id,
    });

    let event = ::server_ohc::orchestration::TeammateMeshEvent {
        agent_id: "system".to_string(),
        action: "ProductCreated".to_string(),
        status: "success".to_string(),
        payload: serde_json::to_vec(&event_payload).unwrap_or_default(),
        msg_id: uuid::Uuid::new_v4().to_string(),
    };

    let _ = hub
        .publish_teammate_event("products_inbox".to_string(), event)
        .await;

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
    let prompt_input = payload.prompt.trim();
    if prompt_input.is_empty() || prompt_input.chars().count() > 4_000 {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "invalid prompt"})),
        )
            .into_response();
    }
    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
    let optimized_prompt = ::server_pricing::compression::reduce_tokens(prompt_input);
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
        variants: None,
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
            if let Some(variants) = parsed.get("variants").and_then(|v| v.as_array()) {
                let mut v_list = Vec::new();
                for v in variants.iter().take(20) {
                    let mut req = ProductVariantRequest {
                        name: "".to_string(),
                        price_modifier: "0.00".to_string(),
                    };
                    if let Some(n) = v.get("name").and_then(|n| n.as_str()) {
                        req.name = n.to_string();
                    }
                    if let Some(p) = v.get("price_modifier").and_then(|p| p.as_str()) {
                        req.price_modifier = p.to_string();
                    } else if let Some(p) = v.get("price_modifier").and_then(|p| p.as_f64()) {
                        req.price_modifier = format!("{:.2}", p);
                    }
                    v_list.push(req);
                }
                if !v_list.is_empty() {
                    response_json.variants = Some(v_list);
                }
            }
        } else {
            tracing::warn!("Failed to parse LLM JSON: {}", cleaned);
        }
    }

    (axum::http::StatusCode::OK, Json(response_json)).into_response()
}

pub fn router<S: Clone + Send + Sync + 'static>(
    hub: Arc<Hub>,
    repository: Option<Arc<CatalogRepository>>,
) -> Router<S> {
    Router::new()
        .route("/products", get(handle_get_products))
        .route(
            "/product",
            get(handle_get_products).post(handle_create_product),
        )
        .route("/generate", post(handle_generate_offering))
        .layer(Extension(repository))
        .layer(Extension(hub))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_reduction_integration() {
        let input = "This is a long sentence with many unnecessary words that should be reduced.";
        let optimized = ::server_pricing::compression::reduce_tokens(input);

        assert!(optimized.len() < input.len());
        assert!(!optimized.contains(" is "));
        assert!(!optimized.contains(" a "));
        assert!(!optimized.contains(" with "));
    }

    #[test]
    fn product_validation_rejects_non_finite_and_out_of_range_prices() {
        let request = |price: &str| CreateProductRequest {
            name: "Window cleaning".to_string(),
            price: price.to_string(),
            duration: Some(30),
            description: "Exterior window cleaning".to_string(),
            item_type: "Service".to_string(),
            is_subscribable: None,
            subscription_frequency: None,
            subscription_discount_percent: None,
            variants: None,
            smart_pricing_enabled: None,
            base_price: None,
            min_price: None,
        };
        let base = request("25.00");

        assert_eq!(validated_product_price(&base), Some(25.0));

        for price in ["NaN", "inf", "-0.01", "10000000.01"] {
            let payload = request(price);
            assert_eq!(
                validated_product_price(&payload),
                None,
                "{price} must be rejected"
            );
        }
    }

    #[test]
    fn product_validation_rejects_smart_pricing_floor_above_base_price() {
        let payload = CreateProductRequest {
            name: "Window cleaning".to_string(),
            price: "25.00".to_string(),
            duration: Some(30),
            description: "Exterior window cleaning".to_string(),
            item_type: "Service".to_string(),
            is_subscribable: None,
            subscription_frequency: None,
            subscription_discount_percent: None,
            variants: None,
            smart_pricing_enabled: Some(true),
            base_price: Some("25.00".to_string()),
            min_price: Some("30.00".to_string()),
        };

        assert_eq!(validated_product_price(&payload), None);
    }

    #[test]
    fn product_image_url_accepts_only_bounded_same_origin_paths() {
        let valid = serde_json::json!({"image_url": "/dashboard_with_charts.png"});
        assert_eq!(
            bounded_product_image_url(Some(&valid)).as_deref(),
            Some("/dashboard_with_charts.png")
        );

        for invalid in [
            serde_json::json!({}),
            serde_json::json!({"image_url": "https://example.com/product.png"}),
            serde_json::json!({"image_url": "//example.com/product.png"}),
            serde_json::json!({"image_url": "/../secret"}),
            serde_json::json!({"image_url": format!("/{}", "x".repeat(2_048))}),
        ] {
            assert_eq!(bounded_product_image_url(Some(&invalid)), None);
        }
    }
}
