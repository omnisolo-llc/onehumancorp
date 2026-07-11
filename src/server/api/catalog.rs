use crate::hub::Hub;
use axum::http::StatusCode;
use axum::{
    Router,
    extract::{Extension, Json},
    response::IntoResponse,
    routing::{get, post},
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
    pub variants: Option<Vec<ProductVariantRequest>>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ProductVariantRequest {
    pub name: String,
    pub price_modifier: String,
}

#[derive(Deserialize)]
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
    pub title: String,
    pub description: Option<String>,
    pub item_type: Option<String>,
    pub price_cents: Option<i64>,
    pub inventory_count: Option<i32>,
    pub variants: Option<Vec<ProductVariantRequest>>,
}

async fn handle_get_products(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = claims
        .organization_id
        .unwrap_or_else(|| ::server_common::auth_utils::get_default_tenant());

    let mut conn = match hub.pool.acquire().await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to acquire DB connection: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(vec![] as Vec<Product>),
            )
                .into_response();
        }
    };

    let rows = sqlx::query(
        "SELECT id, title, description, type as item_type, price_cents, inventory_count FROM products WHERE tenant_id = $1"
    )
    .bind(&tenant_id)
    .fetch_all(&mut *conn)
    .await;

    match rows {
        Ok(rows) => {
            // Collect product IDs to scope the variants query (safer if pagination is used)
            let product_ids: Vec<String> = rows.iter().map(|row| row.try_get("id").unwrap_or_default()).collect();

            // N+1 Query Optimization: Fetch all variants for the retrieved products in a single query
            // In sqlite we use `IN (SELECT value FROM json_each($1))` or similar, but since we support Postgres we use ANY
            // Instead of string concatenation which might not be safe, we just fetch all for the tenant since we know the schema
            let v_rows = if product_ids.is_empty() {
                vec![]
            } else {
                // For safety we just do the tenant filter since products endpoint usually doesn't paginate heavily
                // and it's cleaner in sqlx than `IN` across generic DB pools.
                sqlx::query("SELECT product_id, name, price_modifier FROM product_variants WHERE tenant_id = $1")
                    .bind(&tenant_id)
                    .fetch_all(&mut *conn)
                    .await
                    .unwrap_or_default()
            };

            let mut variants_map: std::collections::HashMap<String, Vec<ProductVariantRequest>> = std::collections::HashMap::new();
            for vr in v_rows {
                let p_id: String = vr.try_get("product_id").unwrap_or_default();
                let modifier: i64 = vr.try_get("price_modifier").unwrap_or(0);
                let modifier_str = format!("{:.2}", (modifier as f64) / 100.0);

                variants_map.entry(p_id).or_default().push(ProductVariantRequest {
                    name: vr.try_get("name").unwrap_or_default(),
                    price_modifier: modifier_str,
                });
            }

            let mut products = Vec::new();
            for row in rows {
                let p_id: String = row.try_get("id").unwrap_or_default();
                let variants = variants_map.remove(&p_id);

                products.push(Product {
                    id: p_id,
                    title: row.try_get("title").unwrap_or_default(),
                    description: row.try_get("description").ok(),
                    item_type: row.try_get("item_type").ok(),
                    price_cents: row.try_get("price_cents").ok(),
                    inventory_count: row.try_get("inventory_count").ok(),
                    variants: variants.filter(|v| !v.is_empty()),
                });
            }
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
                    ::server_telemetry::record_error_signal("[bug] Failed to count products for quota check");
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
    .execute(&mut *conn)
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
            let v_price_mod = (variant.price_modifier.parse::<f64>().unwrap_or(0.0) * 100.0).round() as i64;
            let _ = sqlx::query("INSERT INTO product_variants (id, tenant_id, product_id, name, price_modifier, inventory_count) VALUES ($1, $2, $3, $4, $5, 100)")
                .bind(&variant_id)
                .bind(&tenant_id)
                .bind(&product_id)
                .bind(&variant.name)
                .bind(v_price_mod)
                .execute(&mut *conn)
                .await;
        }
    }

    // Invalidate cache
    let cache = CATALOG_CACHE.get_or_init(|| HybridCache::new(None));
    cache.invalidate(&tenant_id).await;

    // Edge Cache Invalidation
    let edge_cache = crate::builder::edge::get_edge_cache();
    edge_cache.invalidate_by_tag(&format!("tenant-id:{}", tenant_id)).await;
    edge_cache.invalidate_by_tag(&format!("entity:product:{}", product_id)).await;
    let cdn_cache = crate::utils::edge_caching_middleware::get_cdn_cache();
    cdn_cache.invalidate_by_tag(&format!("tenant-id:{}", tenant_id)).await;
    cdn_cache.invalidate_by_tag(&format!("entity:product:{}", product_id)).await;

    if let Err(e) = hub.tracker().record_product_added(&tenant_id).await {
        tracing::warn!(
            "Failed to update product usage counter for tenant {}: {}",
            tenant_id,
            e
        );
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
        .execute(&mut *conn)
        .await;

        if let Err(e) = insert_plan {
            ::server_telemetry::record_error_signal("[bug] Failed to insert subscription plan");
            tracing::error!("Failed to insert subscription plan: {}", e);
            // Non-fatal, just log it. The product was created.
        }
    }

    if payload.smart_pricing_enabled.unwrap_or(false) {
        let base_price_cents = (payload.base_price.unwrap_or(payload.price.clone()).parse::<f64>().unwrap_or(0.0) * 100.0).round() as i64;
        let _min_price_cents = (payload.min_price.unwrap_or("0".to_string()).parse::<f64>().unwrap_or(0.0) * 100.0).round() as i64;
        let rules_id = uuid::Uuid::new_v4().to_string();

        let _ = sqlx::query("INSERT INTO pricing_rules (id, tenant_id, target_id, name, base_price_cents, is_active, rules_json) VALUES ($1, $2, $3, $4, $5, TRUE, $6)")
            .bind(&rules_id)
            .bind(&tenant_id)
            .bind(&product_id)
            .bind(format!("Smart Pricing: {}", payload.name))
            .bind(base_price_cents)
            .bind(serde_json::json!([{
                "type": "InventoryThreshold",
                "config": { "threshold": 10, "adjustment_percent": -10.0 }
            }]))
            .execute(&mut *conn)
            .await;
    }

    let event_payload = serde_json::json!({
        "product_id": product_id,
        "name": payload.name,
        "description": payload.description,
        "item_type": payload.item_type,
        "price": payload.price.parse::<f64>().unwrap_or(0.0),
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
                for v in variants {
                    let mut req = ProductVariantRequest { name: "".to_string(), price_modifier: "0.00".to_string() };
                    if let Some(n) = v.get("name").and_then(|n| n.as_str()) { req.name = n.to_string(); }
                    if let Some(p) = v.get("price_modifier").and_then(|p| p.as_str()) { req.price_modifier = p.to_string(); }
                    else if let Some(p) = v.get("price_modifier").and_then(|p| p.as_f64()) { req.price_modifier = format!("{:.2}", p); }
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

pub fn router<S: Clone + Send + Sync + 'static>(hub: Arc<Hub>) -> Router<S> {
    Router::new()
        .route("/products", get(handle_get_products))
        .route("/product", post(handle_create_product))
        .route("/generate", post(handle_generate_offering))
        .layer(Extension(hub))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::Extension;
    use crate::hub::Hub;
    use std::sync::Arc;

    #[test]
    fn test_token_reduction_integration() {
        let input = "This is a long sentence with many unnecessary words that should be reduced.";
        let optimized = ::server_pricing::compression::reduce_tokens(input);

        assert!(optimized.len() < input.len());
        assert!(!optimized.contains(" is "));
        assert!(!optimized.contains(" a "));
        assert!(!optimized.contains(" with "));
    }

    #[tokio::test]
    async fn test_handle_get_products_optimization() {
        let pool = crate::db::create_dummy_pg_pool().await;

        // Let's create the tables and insert test data
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS products (id TEXT PRIMARY KEY, tenant_id TEXT, title TEXT, description TEXT, type TEXT, price_cents INTEGER, inventory_count INTEGER);"
        ).execute(&pool).await.unwrap();

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS product_variants (id TEXT PRIMARY KEY, tenant_id TEXT, product_id TEXT, name TEXT, price_modifier INTEGER, inventory_count INTEGER);"
        ).execute(&pool).await.unwrap();

        sqlx::query(
            "INSERT INTO products (id, tenant_id, title, description, type, price_cents, inventory_count) VALUES ('p1', 't1', 'P1', 'Desc', 'product', 1000, 5);"
        ).execute(&pool).await.unwrap();

        sqlx::query(
            "INSERT INTO product_variants (id, tenant_id, product_id, name, price_modifier, inventory_count) VALUES ('v1', 't1', 'p1', 'Red', 200, 5);"
        ).execute(&pool).await.unwrap();

        sqlx::query(
            "INSERT INTO product_variants (id, tenant_id, product_id, name, price_modifier, inventory_count) VALUES ('v2', 't1', 'p1', 'Blue', 0, 5);"
        ).execute(&pool).await.unwrap();

        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        let hub = Arc::new(Hub::new(tx, pool.clone()));

        let claims = ::server_common::Claims {
            sub: "test-user".to_string(),
            organization_id: Some("t1".to_string()),
            roles: vec![],
            iat: 0,
            username: "test".to_string(),
            email: "test@example.com".to_string(),
            exp: 0,
            jti: "test".to_string(),
            session_id: None,
        };

        let response = handle_get_products(Extension(hub), Extension(claims)).await;
        let response = response.into_response();
        assert_eq!(response.status(), axum::http::StatusCode::OK);

        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let products: Vec<Product> = serde_json::from_slice(&body_bytes).unwrap();

        assert_eq!(products.len(), 1);
        assert_eq!(products[0].id, "p1");
        assert_eq!(products[0].title, "P1");
        assert_eq!(products[0].price_cents, Some(1000));

        let variants = products[0].variants.as_ref().unwrap();
        assert_eq!(variants.len(), 2);

        let red_variant = variants.iter().find(|v| v.name == "Red").unwrap();
        assert_eq!(red_variant.price_modifier, "2.00");
    }
}
