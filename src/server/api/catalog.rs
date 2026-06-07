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

#[derive(Deserialize)]
pub struct DraftOfferingRequest {
    pub intent: String,
}

#[derive(Serialize)]
pub struct DraftOfferingResponse {
    pub title: String,
    pub description: String,
    pub price: i32,
    pub r#type: String, // 'physical', 'service', 'digital'
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

    let insert_product = sqlx::query(
        "INSERT INTO products (id, tenant_id, title, description, type, inventory_count) VALUES ($1, $2, $3, $4, $5, 100)"
    )
    .bind(&product_id)
    .bind(&tenant_id)
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(&payload.item_type)
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

async fn handle_draft_offering(
    Extension(hub): Extension<Arc<Hub>>,
    Json(payload): Json<DraftOfferingRequest>,
) -> impl IntoResponse {
    // In a real implementation, this would call out to Minimax/Gemini with the intent.
    // For now, we mock the AI agent's response based on the intent string.
    let intent_lower = payload.intent.to_lowercase();

    let item_type = if intent_lower.contains("lesson") || intent_lower.contains("consultation") || intent_lower.contains("hour") || intent_lower.contains("service") || intent_lower.contains("booking") {
        "service"
    } else if intent_lower.contains("download") || intent_lower.contains("pdf") || intent_lower.contains("video") {
        "digital"
    } else {
        "physical"
    };

    let title = if payload.intent.len() > 40 {
        format!("{}...", &payload.intent[0..37])
    } else {
        payload.intent.clone()
    };

    let description = format!("A fantastic {} offering generated from your request: \"{}\". We've set this up so you can start selling immediately.", item_type, payload.intent);
    let price = 50;

    (
        StatusCode::OK,
        Json(DraftOfferingResponse {
            title,
            description,
            price,
            r#type: item_type.to_string(),
        }),
    )
        .into_response()
}

pub fn router<S: Clone + Send + Sync + 'static>(hub: Arc<Hub>) -> Router<S> {
    Router::new()
        .route("/product", post(handle_create_product))
        .route("/draft-offering", post(handle_draft_offering))
        .layer(Extension(hub))
}
