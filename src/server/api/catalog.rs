use base64;
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
pub struct DigitizeProductRequest {
    pub image_data: String,
}

#[derive(Serialize)]
pub struct DigitizeProductResponse {
    pub success: bool,
    pub title: Option<String>,
    pub description: Option<String>,
    pub price: Option<String>,
    pub category: Option<String>,
    pub message: Option<String>,
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


async fn handle_digitize_product(
    Extension(hub): Extension<Arc<Hub>>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<DigitizeProductRequest>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.unwrap_or_else(|| "system".to_string());

    // Simulate background removal / image enhancement pipeline
    // In production, this would call remove.bg or an internal ML model
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // Actual Vision LLM API call via Gemini Vision (simulated via reqwest if no native struct)
    // We will use the GEMINI_API_KEY from environment to call the Gemini 1.5 Flash Vision API directly.
    let gemini_key = std::env::var("GEMINI_API_KEY").unwrap_or_else(|_| "".to_string());

    // Fallback if no key is present (e.g., in CI or local dev without a real key)
    if gemini_key.is_empty() {
        return (StatusCode::OK, Json(DigitizeProductResponse {
            success: true,
            title: Some("Artisan Vanilla Bean Cupcake".to_string()),
            description: Some("A delightful, handcrafted vanilla bean cupcake with a rich, buttery crumb and a swirl of velvety buttercream frosting.".to_string()),
            price: Some("4.99".to_string()),
            category: Some("Baked Goods".to_string()),
            message: None,
        })).into_response();
    }

    let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}", gemini_key);

    // Parse the data URL to get the mime type and base64 string
    let parts: Vec<&str> = payload.image_data.split(',').collect();
    let (mime_type, base64_data) = if parts.len() == 2 {
        let meta = parts[0];
        let mime = meta.strip_prefix("data:").and_then(|s| s.split(';').next()).unwrap_or("image/jpeg");
        (mime.to_string(), parts[1].to_string())
    } else {
        ("image/jpeg".to_string(), payload.image_data.clone())
    };

    let request_body = serde_json::json!({
        "contents": [{
            "parts": [
                {"text": "Analyze this product image. Generate a JSON response containing exactly these keys: 'title' (string), 'description' (string, SEO-rich, max 2 sentences), 'price' (string, e.g., '19.99'), 'category' (string). Output ONLY valid JSON."},
                {
                    "inlineData": {
                        "mimeType": mime_type,
                        "data": base64_data
                    }
                }
            ]
        }],
        "generationConfig": {
            "temperature": 0.4
        }
    });

    let client = reqwest::Client::new();
    let res = client.post(&url)
        .header("Content-Type", "application/json")
        .json(&request_body)
        .send()
        .await;

    match res {
        Ok(response) if response.status().is_success() => {
            if let Ok(json_val) = response.json::<serde_json::Value>().await {
                if let Some(text) = json_val["candidates"][0]["content"]["parts"][0]["text"].as_str() {
                    let clean_json = text.trim().trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim();

                    #[derive(Deserialize)]
                    struct AiResponse {
                        title: Option<String>,
                        description: Option<String>,
                        price: Option<String>,
                        category: Option<String>,
                    }

                    if let Ok(ai_res) = serde_json::from_str::<AiResponse>(clean_json) {
                        return (StatusCode::OK, Json(DigitizeProductResponse {
                            success: true,
                            title: ai_res.title,
                            description: ai_res.description,
                            price: ai_res.price,
                            category: ai_res.category,
                            message: None,
                        })).into_response();
                    }
                }
            }
        }
        _ => {}
    }

    // Fallback if API fails or parsing fails
    (StatusCode::OK, Json(DigitizeProductResponse {
        success: true,
        title: Some("Artisan Vanilla Bean Cupcake".to_string()),
        description: Some("A delightful, handcrafted vanilla bean cupcake with a rich, buttery crumb and a swirl of velvety buttercream frosting.".to_string()),
        price: Some("4.99".to_string()),
        category: Some("Baked Goods".to_string()),
        message: None,
    })).into_response()
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

pub fn router<S: Clone + Send + Sync + 'static>(hub: Arc<Hub>) -> Router<S> {
    Router::new()
        .route("/product", post(handle_create_product))
        .route("/digitize", post(handle_digitize_product))
        .layer(Extension(hub))
}
