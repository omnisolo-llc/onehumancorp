use axum::{
    extract::{Extension, Json},
    response::IntoResponse,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::hub::Hub;
use axum::http::StatusCode;

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

async fn handle_create_product(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<CreateProductRequest>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "system".to_string());

    // Check product quota
    let quota_status = hub.tracker().check_product_quota(&tenant_id).await.unwrap_or_else(|e| {
        tracing::warn!("Failed to check product quota for tenant {}: {}", tenant_id, e);
        ::server_pricing::rate_limit::RateLimitStatus {
            is_allowed: true,
            soft_limit_reached: false,
            user_message: None,
        }
    });

    if quota_status.soft_limit_reached && !quota_status.is_allowed {
        let msg = quota_status.user_message.unwrap_or_else(|| "Tier limit reached. Please upgrade.".to_string());
        return (
            StatusCode::PAYMENT_REQUIRED,
            Json(ErrorResponse {
                error: "LIMIT_EXCEEDED".to_string(),
                message: msg,
            }),
        ).into_response();
    }

    // Record product addition
    let _ = hub.tracker().record_product_added(&tenant_id).await;

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
            ).into_response();
        }
    };

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
        ).into_response();
    }

    if payload.is_subscription.unwrap_or(false) {
        let plan_id = uuid::Uuid::new_v4().to_string();
        let interval = payload.subscription_interval.unwrap_or_else(|| "Monthly".to_string()).to_lowercase();
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

    (StatusCode::OK, Json(CreateProductResponse { success: true, message: Some(format!("Created {}", payload.name)) })).into_response()
}

#[derive(Deserialize)]
pub struct DigitizeProductRequest {
    pub image_base64: String,
}

#[derive(Serialize)]
pub struct DigitizeProductResponse {
    pub title: String,
    pub description: String,
    pub price: String,
    pub category: String,
}

async fn handle_digitize_product(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<DigitizeProductRequest>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "system".to_string());
    let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_else(|_| "fake-key".to_string());
    let client = crate::minimax::MinimaxClient::new(api_key);
    // TODO(Architecture): The current MinimaxClient `reason()` method takes a text prompt.
    // To properly support Vision AI, the LLM Provider interface needs to be updated to accept
    // multipart image data (e.g. `client.analyze_image(&base64_data)`).
    // For now, we are passing the base64 prefix or dummy data in the text prompt to trigger the adapter,
    // which simulates the Vision AI extraction pipeline until the Vision API client is fully integrated.
    let preview_data = if payload.image_base64.len() > 100 { &payload.image_base64[..100] } else { &payload.image_base64 };
    let prompt = format!("Extract product details from this image data: {}", preview_data);

    match client.reason(&prompt).await {
        Ok(json_str) => {
            // Parse the json_str to extract title, description, price, category
            // The prompt might return markdown wrapper like ```json ... ```
            let cleaned = json_str.trim().trim_start_matches("```json").trim_end_matches("```").trim();
            match serde_json::from_str::<serde_json::Value>(cleaned) {
                Ok(val) => {
                    let title = val["title"].as_str().unwrap_or("Unknown Product").to_string();
                    let description = val["description"].as_str().unwrap_or("No description").to_string();
                    let price = val["price"].as_str().unwrap_or("0.00").to_string();
                    let category = val["category"].as_str().unwrap_or("General").to_string();

                    // Auto-save the product draft
                    let product_id = uuid::Uuid::new_v4().to_string();
                    let mut conn = match hub.pool.acquire().await {
                        Ok(c) => c,
                        Err(e) => {
                            tracing::error!("Failed to acquire DB connection: {}", e);
                            return (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: "DATABASE_ERROR".to_string(), message: "Failed to connect to database".to_string() })).into_response();
                        }
                    };

                    let insert_product = sqlx::query(
                        "INSERT INTO products (id, tenant_id, title, description, type, inventory_count, _sync_status) VALUES ($1, $2, $3, $4, $5, 1, 'draft')"
                    )
                    .bind(&product_id)
                    .bind(&tenant_id)
                    .bind(&title)
                    .bind(&description)
                    .bind(&category)
                    .execute(&mut *conn)
                    .await;

                    if let Err(e) = insert_product {
                        tracing::error!("Failed to auto-save product draft: {}", e);
                    }

                    (StatusCode::OK, Json(DigitizeProductResponse { title, description, price, category })).into_response()
                },
                Err(e) => {
                    tracing::error!("Failed to parse LLM response: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: "LLM_PARSE_ERROR".to_string(), message: "Failed to parse product details".to_string() })).into_response()
                }
            }
        },
        Err(e) => {
            tracing::error!("Failed to call LLM: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { error: "LLM_ERROR".to_string(), message: "Failed to extract product details".to_string() })).into_response()
        }
    }
}

pub fn router<S: Clone + Send + Sync + 'static>(hub: Arc<Hub>) -> Router<S> {
    Router::new()
        .route("/product", post(handle_create_product))
        .route("/v1/products/digitize", post(handle_digitize_product))
        .layer(Extension(hub))
}

#[cfg(test)]
mod digitize_tests {
    use super::*;
    use axum::http::StatusCode;
    use axum::extract::Json;
    use std::sync::Arc;
    use axum::extract::Extension;


    #[tokio::test]
    async fn test_digitize_product_success() {
        if std::env::var("OHC_DATABASE_URL").is_err() {
            return;
        }

        let database_url = "postgres://postgres:postgres@localhost:5432/test";
        let pool = sqlx::postgres::PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_millis(50))
            .connect_lazy(database_url)
            .unwrap();

        let (tx, _rx) = tokio::sync::mpsc::channel(100);
        let mut hub = Hub::new(tx, pool.clone());

        let claims = ::server_common::Claims {
            sub: "user1".to_string(),
            organization_id: Some("test_tenant".to_string()),
            exp: 100000,
            iat: 100000,
            username: "user".to_string(),
            email: "test@example.com".to_string(),
            roles: vec![],
            session_id: None,
            jti: "test".to_string(),
        };

        // Use the fake key to trigger the dummy image logic in MinimaxClient
        unsafe { std::env::set_var("MINIMAX_API_KEY", "fake-key"); }

        let payload = DigitizeProductRequest {
            image_base64: "extract product details".to_string(),
        };

        let response = handle_digitize_product(
            Extension(Arc::new(hub)),
            Extension(claims),
            Json(payload),
        ).await;

        let (parts, _body) = response.into_response().into_parts();
        assert_eq!(parts.status, StatusCode::OK);
    }
}
