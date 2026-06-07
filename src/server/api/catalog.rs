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
pub struct ProductDto {
    pub id: String,
    pub name: String,
    pub price: String,
    pub status: String,
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

    let mut price_cents = 0;
    if let Ok(p) = payload.price.replace("$", "").parse::<f64>() {
        price_cents = (p * 100.0) as i64;
    }

    let insert_product = sqlx::query(
        "INSERT INTO products (id, tenant_id, title, description, type, inventory_count, price_cents) VALUES ($1, $2, $3, $4, $5, 100, $6)"
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
            "INSERT INTO subscription_plans (id, tenant_id, product_id, interval, discount_percentage) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(&plan_id)
        .bind(&tenant_id)
        .bind(&product_id)
        .bind(&interval)
        .bind(discount)
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

pub fn router<S: Clone + Send + Sync + 'static>(hub: Arc<Hub>) -> Router<S> {
    Router::new()
        .route("/product", post(handle_create_product))
        .route("/products", get(handle_list_products))
        .layer(Extension(hub))
}

async fn handle_list_products(
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
            return (StatusCode::INTERNAL_SERVER_ERROR, Json::<Vec<ProductDto>>(vec![])).into_response();
        }
    };

    let rows = sqlx::query("SELECT id, title, price, price_cents, inventory_count FROM products WHERE tenant_id = $1 ORDER BY created_at DESC")
        .bind(&tenant_id)
        .fetch_all(&mut *conn)
        .await;

    let mut products: Vec<ProductDto> = Vec::new();
    match rows {
        Ok(rows_data) => {
            for r in rows_data {
                let price_cents: i64 = r.try_get("price_cents").unwrap_or(0);
                let formatted_price = format!("${:.2}", (price_cents as f64) / 100.0);

                products.push(ProductDto {
                    id: r.try_get("id").unwrap_or_default(),
                    name: r.try_get("title").unwrap_or_default(),
                    price: formatted_price,
                    status: "Published".to_string(),
                });
            }
        },
        Err(e) => {
            tracing::error!("Failed to fetch products: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(products)).into_response();
        }
    }
    (StatusCode::OK, Json(products)).into_response()
}
