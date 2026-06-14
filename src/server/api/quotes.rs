use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::repository::models::{Quote, QuoteLineItem};

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    PgPool: axum::extract::FromRef<S>,
{
    Router::new()
        .route("/", post(create_quote))
        .route("/intake", post(create_quote_intake))
        .route("/{id}", get(get_quote))
        .route("/{id}", put(update_quote))
        .route("/{id}/accept", post(accept_quote))
}

#[derive(Serialize)]
pub struct QuoteResponse {
    pub quote: Quote,
    pub line_items: Vec<QuoteLineItem>,
}

#[derive(Deserialize)]
pub struct CreateQuoteRequest {
    pub tenant_id: String,
    pub customer_id: String,
    pub total_amount: Option<i64>,
    pub required_deposit: Option<i64>,
    pub checkout_url: Option<String>,
    pub line_items: Vec<QuoteLineItemRequest>,
}

#[derive(Deserialize)]
pub struct CreateQuoteIntakeRequest {
    pub tenant_id: String,
    pub customer_id: String,
    pub description: String,
    pub image_url: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateQuoteRequest {
    pub total_amount: Option<i64>,
    pub required_deposit: Option<i64>,
    pub checkout_url: Option<String>,
    pub status: Option<String>,
    pub line_items: Vec<QuoteLineItemRequest>,
}

#[derive(Deserialize)]
pub struct QuoteLineItemRequest {
    pub description: String,
    pub unit_price_cents: i64,
    pub quantity: i32,
    pub is_optional: bool,
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

    let quote_res = sqlx::query(
        "INSERT INTO quotes (id, tenant_id, customer_id, status, total_amount, required_deposit, checkout_url, created_at, updated_at) VALUES ($1, $2, $3, 'DRAFT', $4, $5, $6, NOW(), NOW())"
    )
    .bind(quote_id)
    .bind(&payload.tenant_id)
    .bind(&payload.customer_id)
    .bind(payload.total_amount)
    .bind(payload.required_deposit)
    .bind(&payload.checkout_url)
    .execute(&mut *tx)
    .await;

    if let Err(e) = quote_res {
        tracing::error!("Failed to insert quote: {}", e);
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    for item in payload.line_items {
        let item_id = Uuid::new_v4();
        let res = sqlx::query(
            "INSERT INTO quote_line_items (id, quote_id, description, unit_price_cents, quantity, is_optional, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())"
        )
        .bind(item_id)
        .bind(quote_id)
        .bind(&item.description)
        .bind(item.unit_price_cents)
        .bind(item.quantity)
        .bind(item.is_optional)
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

async fn create_quote_intake(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateQuoteIntakeRequest>,
) -> impl IntoResponse {
    let quote_id = Uuid::new_v4();

    let prompt = format!(
        "You are a quoting agent. A customer requests an estimate for: '{}'. Image: {}. Please output a strict JSON object with keys: 'suggested_price' (number), 'scope' (string), 'suggested_time' (string), 'required_deposit' (number), and 'line_items' (array of objects with 'description' (string), 'unit_price_cents' (number), 'quantity' (number)). Do not use markdown blocks.",
        payload.description, payload.image_url.unwrap_or_default()
    );

    let raw_response = match std::env::var("OHC_SALES_LLM_PROVIDER")
        .or_else(|_| std::env::var("OHC_LLM_PROVIDER"))
        .as_deref()
    {
        Ok("minimax") => {
            let api_key = std::env::var("MINIMAX_API_KEY").unwrap_or_default();
            crate::minimax::MinimaxClient::new(api_key).reason(&crate::pricing::compression::reduce_tokens(&prompt)).await.unwrap_or_default()
        }
        _ => {
            crate::minimax::LocalLLMClient::new().reason(&crate::pricing::compression::reduce_tokens(&prompt)).await.unwrap_or_default()
        }
    };

    let mut total_amount = 0;
    let mut required_deposit = 0;
    let mut scope = "Service estimate".to_string();
    let mut suggested_time = "TBD".to_string();
    let mut line_items_data: Vec<QuoteLineItemRequest> = Vec::new();

    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&raw_response) {
        if let Some(p) = parsed.get("suggested_price").and_then(|v| v.as_f64()) {
            total_amount = (p * 100.0) as i64;
        }
        if let Some(d) = parsed.get("required_deposit").and_then(|v| v.as_f64()) {
            required_deposit = (d * 100.0) as i64;
        }
        if let Some(s) = parsed.get("scope").and_then(|v| v.as_str()) {
            scope = s.to_string();
        }
        if let Some(t) = parsed.get("suggested_time").and_then(|v| v.as_str()) {
            suggested_time = t.to_string();
        }
        if let Some(items) = parsed.get("line_items").and_then(|v| v.as_array()) {
            for item in items {
                line_items_data.push(QuoteLineItemRequest {
                    description: item.get("description").and_then(|v| v.as_str()).unwrap_or("Item").to_string(),
                    unit_price_cents: item.get("unit_price_cents").and_then(|v| v.as_i64()).unwrap_or(0),
                    quantity: item.get("quantity").and_then(|v| v.as_i64()).unwrap_or(1) as i32,
                    is_optional: false,
                });
            }
        }
    }

    let mut tx = match pool.begin().await {
        Ok(t) => t,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db error"}))).into_response(),
    };

    if let Err(e) = sqlx::query(
        "INSERT INTO quotes (id, tenant_id, customer_id, status, total_amount, required_deposit, checkout_url, created_at, updated_at) VALUES ($1, $2, $3, 'DRAFT', $4, $5, NULL, NOW(), NOW())"
    )
    .bind(quote_id)
    .bind(&payload.tenant_id)
    .bind(&payload.customer_id)
    .bind(total_amount)
    .bind(required_deposit)
    .execute(&mut *tx)
    .await {
        tracing::error!("Failed to insert quote intake: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db insert failed"}))).into_response();
    }

    for item in line_items_data {
        if let Err(e) = sqlx::query(
            "INSERT INTO quote_line_items (id, quote_id, description, unit_price_cents, quantity, is_optional, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, false, NOW(), NOW())"
        )
        .bind(Uuid::new_v4())
        .bind(quote_id)
        .bind(&item.description)
        .bind(item.unit_price_cents)
        .bind(item.quantity)
        .execute(&mut *tx)
        .await {
            tracing::error!("Failed to insert quote intake line item: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db insert line item failed"}))).into_response();
        }
    }

    let payload_json = serde_json::json!({
        "quote_id": quote_id.to_string(),
        "feature_type": "quote_draft",
        "customer_inquiry": payload.description,
        "scope": scope,
        "suggested_time": suggested_time,
        "suggested_price": total_amount as f64 / 100.0,
    });

    if let Err(e) = sqlx::query(
        "INSERT INTO agent_approvals (id, tenant_id, department, description, status, action_risk, payload, created_at, updated_at) VALUES ($1, $2, 'sales', 'Draft Quote for Customer', 'PENDING', 'DraftForReview', $3, NOW(), NOW())"
    )
    .bind(Uuid::new_v4().to_string())
    .bind(&payload.tenant_id)
    .bind(payload_json)
    .execute(&mut *tx)
    .await {
        tracing::error!("Failed to insert quote intake agent approval: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db insert agent approval failed"}))).into_response();
    }

    if let Err(e) = tx.commit().await {
        tracing::error!("Failed to commit quote intake tx: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "db tx commit failed"}))).into_response();
    }

    (StatusCode::CREATED, Json(serde_json::json!({"id": quote_id.to_string()}))).into_response()
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

    // we can't easily bind dynamic number of parameters in simple query string building
    // so we'll do it securely:
    let update_res = sqlx::query(
        "UPDATE quotes SET updated_at = NOW(), total_amount = COALESCE($1, total_amount), required_deposit = COALESCE($2, required_deposit), status = COALESCE($3, status), checkout_url = COALESCE($4, checkout_url) WHERE id = $5"
    )
    .bind(payload.total_amount)
    .bind(payload.required_deposit)
    .bind(&payload.status)
    .bind(&payload.checkout_url)
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
            "INSERT INTO quote_line_items (id, quote_id, description, unit_price_cents, quantity, is_optional, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())"
        )
        .bind(item_id)
        .bind(quote_id)
        .bind(&item.description)
        .bind(item.unit_price_cents)
        .bind(item.quantity)
        .bind(item.is_optional)
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
    Path(id): Path<String>,
) -> impl IntoResponse {
    let quote_id = match Uuid::parse_str(&id) {
        Ok(uid) => uid,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let quote_res = sqlx::query_as::<_, Quote>("SELECT * FROM quotes WHERE id = $1")
        .bind(quote_id)
        .fetch_optional(&pool)
        .await;

    let quote = match quote_res {
        Ok(Some(q)) => q,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch quote: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let items_res = sqlx::query_as::<_, QuoteLineItem>("SELECT * FROM quote_line_items WHERE quote_id = $1")
        .bind(quote_id)
        .fetch_all(&pool)
        .await;

    let line_items = match items_res {
        Ok(items) => items,
        Err(e) => {
            tracing::error!("Failed to fetch quote line items: {}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    (StatusCode::OK, Json(QuoteResponse { quote, line_items })).into_response()
}

async fn accept_quote(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let quote_id = match Uuid::parse_str(&id) {
        Ok(uid) => uid,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    match sqlx::query("UPDATE quotes SET status = 'ACCEPTED', updated_at = NOW() WHERE id = $1")
        .bind(quote_id)
        .execute(&pool)
        .await
    {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({"success": true}))).into_response(),
        Err(e) => {
            tracing::error!("Failed to accept quote: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"success": false}))).into_response()
        }
    }
}
